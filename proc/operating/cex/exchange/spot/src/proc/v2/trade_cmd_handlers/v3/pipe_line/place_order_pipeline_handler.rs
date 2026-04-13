use base_types::account::balance::Balance as AccountBalance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use cmd_handler::CmdHandlerForUpdate3;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::v2::spot_trade_error::SpotApiErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior::NewOrderCmd;
use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::match_order_handler::{
    MatchCmd, MatchOrderCmdHandler,
};
use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::place_order_handler::PlaceOrderCmdHandler;
use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::sett_order_handler::{
    SettOrderCmdHandler, SettlementCmd,
};

#[derive(Debug, Clone)]
pub struct PlaceOrderPipelineReply {
    pub order: DomainEvent<SpotOrder>,
    pub trades: Option<Vec<DomainEvent<SpotTrade>>>,
    pub balances: Option<Vec<DomainEvent<AccountBalance>>>,
}

pub struct PlaceOrderPipelineHandler<
    R: CmdRepo2 + Clone,
    P: EventPublisher2 + Clone,
    L: MultiSymbolLobRepo<Order = SpotOrder> + Send,
> {
    place_order_handler: PlaceOrderCmdHandler<R, P>,
    match_order_handler: MatchOrderCmdHandler<R, P, L>,
    sett_order_handler: SettOrderCmdHandler<R, P>,
}
impl<
    R: CmdRepo2 + Clone,
    P: EventPublisher2 + Clone,
    L: MultiSymbolLobRepo<Order = SpotOrder> + Send,
> PlaceOrderPipelineHandler<R, P, L>
{
    pub fn new(
        place_order_handler: PlaceOrderCmdHandler<R, P>,
        match_order_handler: MatchOrderCmdHandler<R, P, L>,
        sett_order_handler: SettOrderCmdHandler<R, P>,
    ) -> Self {
        Self { place_order_handler, match_order_handler, sett_order_handler }
    }

    // 规则：place/match/settlement 分阶段串联，后一阶段必须消费前一阶段输出
    pub fn exec(&self, cmd: NewOrderCmd) -> Result<PlaceOrderPipelineReply, SpotApiErrorAny> {
        let order = self.place_order_handler.cmd_handle(
            cmd,
            self.place_order_handler.repo.clone(),
            self.place_order_handler.publisher.clone(),
        )?;
        let trades = self.match_order_handler.cmd_handle(
            MatchCmd { taker_order: order.object().clone() },
            self.match_order_handler.repo.clone(),
            self.match_order_handler.publisher.clone(),
        )?;

        let balances = if let Some(ref trade_events) = trades {
            if trade_events.is_empty() {
                None
            } else {
                self.sett_order_handler.cmd_handle(
                    SettlementCmd {
                        trades: trade_events.iter().map(|trade| trade.object().clone()).collect(),
                    },
                    self.sett_order_handler.repo.clone(),
                    self.sett_order_handler.publisher.clone(),
                )?
            }
        } else {
            None
        };

        Ok(PlaceOrderPipelineReply { order, trades, balances })
    }
}

#[cfg(test)]
mod tests {
    use base_types::account::balance::Balance as AccountBalance;
    use base_types::base_types::TraderId;
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::{OrderSide, OrderType, TimeInForce, TradingPair};
    use base_types::{AssetId, Price, Quantity, Timestamp};
    use bdd::bdd_test;
    use db_repo::adapter::v2::memdb_repo::MemdbRepo;
    use db_repo::core::db_repo2::QueryRepo2;
    use diff::diff_types::{ChangeLog, ChangeType, DomainEvent};
    use diff::Entity;
    use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;
    use lob_repo::adapter::local_lob_impl::LocalLob;
    use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

    use super::*;
    use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::mock_repo::MockEventPublisher;

    fn create_test_cmd_with_quantity(client_order_id: &str, quantity: f64) -> NewOrderCmd {
        NewOrderCmd::new(
            CMetadata::default(),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(quantity)),
            None,
            Some(Price::from_f64(50000.0)),
            Some(client_order_id.to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    fn create_test_cmd(client_order_id: &str) -> NewOrderCmd {
        create_test_cmd_with_quantity(client_order_id, 1.0)
    }

    fn create_sell_order(order_id: u64, client_order_id: &str) -> SpotOrder {
        create_sell_order_with_quantity(order_id, client_order_id, 1.0)
    }

    fn create_sell_order_with_quantity(
        order_id: u64,
        client_order_id: &str,
        quantity: f64,
    ) -> SpotOrder {
        SpotOrder::create_order(
            order_id,
            TraderId::new([1u8; 8]),
            TradingPair::BtcUsdt,
            OrderSide::Sell,
            Price::from_f64(50000.0),
            Quantity::from_f64(quantity),
            TimeInForce::GTC,
            Some(client_order_id.to_string()),
            Quantity::default(),
        )
    }

    fn create_embedded_lob(maker_orders: Vec<SpotOrder>) -> EmbeddedLobRepo<SpotOrder> {
        let mut lob = EmbeddedLobRepo::new(vec![LocalLob::new(TradingPair::BtcUsdt)]);
        for maker_order in maker_orders {
            lob.add_order(TradingPair::BtcUsdt, maker_order)
                .expect("add maker order should succeed");
        }
        lob
    }

    #[test]
    fn test_pipeline_exec_returns_order_and_skips_settlement_when_no_trades() {
        let repo = MemdbRepo::default();
        let pipeline = PlaceOrderPipelineHandler::new(
            PlaceOrderCmdHandler::new(repo.clone(), MockEventPublisher),
            MatchOrderCmdHandler::new(
                repo.clone(),
                MockEventPublisher,
                create_embedded_lob(Vec::new()),
            ),
            SettOrderCmdHandler::new(repo.clone(), MockEventPublisher),
        );

        let reply = pipeline
            .exec(create_test_cmd("test_pipeline_001"))
            .expect("pipeline exec should succeed");

        assert_eq!(reply.order.object().trading_pair, TradingPair::BtcUsdt);
        assert_eq!(reply.order.object().side, OrderSide::Buy);
        assert_eq!(reply.order.object().price, Some(Price::from_f64(50000.0)));
        assert_eq!(reply.order.object().total_base_qty, Quantity::from_f64(1.0));
        let stored_order = repo
            .find_by_id::<SpotOrder>(&reply.order.object().entity_id().to_string())
            .expect("repo query should succeed")
            .expect("stored order should exist");
        assert_eq!(stored_order.order_id, reply.order.object().order_id);
        assert!(reply.trades.is_none());
        assert!(reply.balances.is_none());
    }

    #[test]
    #[bdd_test(
        feature = "订单管道",
        scenario = "返回成交和余额",
        given(订单成交),
        when = "执行管道",
        then(返回成交记录, 返回余额变更),
        tags(pipeline, trade),
        priority = "4"
    )]
    fn test_pipeline_reply_can_carry_trades_and_balances() {
        let order = SpotOrder::create_order(
            11,
            TraderId::new([0u8; 8]),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            Price::from_f64(50000.0),
            Quantity::from_f64(1.0),
            TimeInForce::GTC,
            Some("pipeline_trade_path".to_string()),
            Quantity::default(),
        );
        let trade = SpotTrade::new(
            1,
            TradingPair::BtcUsdt,
            order.order_id,
            22,
            Timestamp::default(),
            Price::from_f64(50000.0),
            Quantity::from_f64(1.0),
            OrderSide::Buy,
            Quantity::default(),
            Quantity::default(),
            AssetId::Usdt,
            0,
            0,
        );
        let balance = AccountBalance::new(1u64.into(), AssetId::Usdt, Timestamp::default());

        let reply = PlaceOrderPipelineReply {
            order: DomainEvent::new(
                ChangeLog::new(
                    "order-11".to_string(),
                    SpotOrder::entity_type().to_string(),
                    ChangeType::Created { fields: Vec::new() },
                    0,
                    0,
                ),
                order,
            ),
            trades: Some(vec![DomainEvent::new(
                ChangeLog::new(
                    "trade-1".to_string(),
                    SpotTrade::entity_type().to_string(),
                    ChangeType::Created { fields: Vec::new() },
                    0,
                    0,
                ),
                trade,
            )]),
            balances: Some(vec![DomainEvent::new(
                ChangeLog::new(
                    "balance-1".to_string(),
                    AccountBalance::entity_type().to_string(),
                    ChangeType::Created { fields: Vec::new() },
                    0,
                    0,
                ),
                balance,
            )]),
        };

        let trades = reply.trades.expect("trades should exist");
        let balances = reply.balances.expect("balances should exist");

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].object().trade_id, 1);
        assert_eq!(trades[0].object().trading_pair, TradingPair::BtcUsdt);
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[0].object().asset_id, AssetId::Usdt);
        assert_eq!(reply.order.object().trading_pair, TradingPair::BtcUsdt);
        assert_eq!(reply.order.object().trader_id, TraderId::new([0u8; 8]));
    }

    /// #规则BDD：先挂一笔卖单，再挂一笔限价买单，完全撮合
    /// -数据 /Users/hongyaotang/src/rustlob/lib/common/lob_repo/src/adapter/embedded_lob_repo.rs
    /// -lob 使用 EmbeddedLobRepo
    /// -repo 不使用MockMySqlRepo 使用/Users/hongyaotang/src/rustlob/lib/common/db_repo/src/adapter/mysql_repo.rs 下的MySqlRepo
    #[test]
    fn test_pipeline_exec_full_match_generates_trade() {
        let maker_order = create_sell_order(21, "maker_sell_001");
        let lob = create_embedded_lob(vec![maker_order.clone()]);

        let repo = MemdbRepo::default();
        let pipeline = PlaceOrderPipelineHandler::new(
            PlaceOrderCmdHandler::new(repo.clone(), MockEventPublisher),
            MatchOrderCmdHandler::new(repo.clone(), MockEventPublisher, lob),
            SettOrderCmdHandler::new(repo.clone(), MockEventPublisher),
        );

        let reply =
            pipeline.exec(create_test_cmd("taker_buy_001")).expect("pipeline exec should succeed");

        let trades = reply.trades.expect("full match should generate trades");
        let balances = reply.balances.expect("full match should generate balances");
        assert_eq!(trades.len(), 1);

        let trade = trades[0].object();
        assert_eq!(trade.trading_pair, TradingPair::BtcUsdt);
        assert_eq!(trade.maker_order_id, maker_order.order_id);
        assert_eq!(trade.taker_order_id, reply.order.object().order_id);
        assert_eq!(trade.price, Price::from_f64(50000.0));
        assert_eq!(trade.base_qty, Quantity::from_f64(1.0));
        assert_eq!(trade.taker_side, OrderSide::Buy);
        let stored_order = repo
            .find_by_id::<SpotOrder>(&reply.order.object().entity_id().to_string())
            .expect("order query should succeed")
            .expect("stored order should exist");
        let stored_trade = repo
            .find_by_id::<SpotTrade>(&trade.entity_id().to_string())
            .expect("trade query should succeed")
            .expect("stored trade should exist");
        assert_eq!(stored_order.order_id, reply.order.object().order_id);
        assert_eq!(stored_trade.trade_id, trade.trade_id);
        assert_eq!(balances.len(), 2);
        let asset_ids: Vec<AssetId> =
            balances.iter().map(|balance| balance.object().asset_id).collect();
        assert!(asset_ids.contains(&AssetId::Usdt));
        assert!(asset_ids.contains(&AssetId::Btc));
        for balance in &balances {
            let stored_balance = repo
                .find_by_id::<AccountBalance>(&balance.object().entity_id().to_string())
                .expect("balance query should succeed")
                .expect("stored balance should exist");
            assert_eq!(stored_balance.asset_id, balance.object().asset_id);
        }
    }

    /// #规则BDD：先挂2笔卖单，再挂一笔限价买单，完全撮合，生成两笔成交
    #[test]
    fn test_pipeline_exec_full_match_generates_trade2() {
        let maker_order_one = create_sell_order_with_quantity(31, "maker_sell_002", 1.0);
        let maker_order_two = create_sell_order_with_quantity(32, "maker_sell_003", 1.0);
        let lob = create_embedded_lob(vec![maker_order_one.clone(), maker_order_two.clone()]);

        let repo = MemdbRepo::default();
        let pipeline = PlaceOrderPipelineHandler::new(
            PlaceOrderCmdHandler::new(repo.clone(), MockEventPublisher),
            MatchOrderCmdHandler::new(repo.clone(), MockEventPublisher, lob),
            SettOrderCmdHandler::new(repo.clone(), MockEventPublisher),
        );

        let reply = pipeline
            .exec(create_test_cmd_with_quantity("taker_buy_002", 2.0))
            .expect("pipeline exec should succeed");

        let trades = reply.trades.expect("full match should generate two trades");
        let balances = reply.balances.expect("full match should generate balances");

        assert_eq!(trades.len(), 2);
        assert_ne!(trades[0].object().trade_id, trades[1].object().trade_id);
        let maker_order_ids: Vec<_> = trades.iter().map(|trade| trade.object().maker_order_id).collect();
        assert!(maker_order_ids.contains(&maker_order_one.order_id));
        assert!(maker_order_ids.contains(&maker_order_two.order_id));
        for trade in &trades {
            assert_eq!(trade.object().taker_order_id, reply.order.object().order_id);
            assert_eq!(trade.object().price, Price::from_f64(50000.0));
            assert_eq!(trade.object().base_qty, Quantity::from_f64(1.0));
        }
        let total_base_qty = trades
            .iter()
            .fold(Quantity::default(), |acc, trade| acc + trade.object().base_qty);
        assert_eq!(total_base_qty, Quantity::from_f64(2.0));

        let stored_first_trade = repo
            .find_by_id::<SpotTrade>(&trades[0].object().entity_id().to_string())
            .expect("first trade query should succeed")
            .expect("first trade should exist");
        let stored_second_trade = repo
            .find_by_id::<SpotTrade>(&trades[1].object().entity_id().to_string())
            .expect("second trade query should succeed")
            .expect("second trade should exist");
        assert_eq!(stored_first_trade.trade_id, trades[0].object().trade_id);
        assert_eq!(stored_second_trade.trade_id, trades[1].object().trade_id);

        assert_eq!(balances.len(), 3);
        let taker_btc = repo
            .find_by_id::<AccountBalance>(&format!("{}:{}", reply.order.object().order_id, u32::from(AssetId::Btc)))
            .expect("taker btc query should succeed")
            .expect("taker btc balance should exist");
        let maker_one_usdt = repo
            .find_by_id::<AccountBalance>(&format!("{}:{}", maker_order_one.order_id, u32::from(AssetId::Usdt)))
            .expect("maker one usdt query should succeed")
            .expect("maker one usdt balance should exist");
        let maker_two_usdt = repo
            .find_by_id::<AccountBalance>(&format!("{}:{}", maker_order_two.order_id, u32::from(AssetId::Usdt)))
            .expect("maker two usdt query should succeed")
            .expect("maker two usdt balance should exist");
        assert_eq!(taker_btc.available, Quantity::from_f64(2.0));
        assert_eq!(maker_one_usdt.available, Quantity::from_f64(50000.0));
        assert_eq!(maker_two_usdt.available, Quantity::from_f64(50000.0));
    }
}
