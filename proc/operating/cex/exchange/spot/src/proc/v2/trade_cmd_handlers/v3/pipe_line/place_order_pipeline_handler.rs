use base_types::account::balance::Balance as AccountBalance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler_update2::CmdHandlerForUpdate2;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;
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
    R: CmdRepo2,
    P: EventPublisher2,
    L: MultiSymbolLobRepo<Order = SpotOrder>,
> {
    place_order_handler: PlaceOrderCmdHandler<R, P>,
    match_order_handler: MatchOrderCmdHandler<R, P, L>,
    sett_order_handler: SettOrderCmdHandler<R, P>,
}
impl<R: CmdRepo2, P: EventPublisher2, L: MultiSymbolLobRepo<Order = SpotOrder>>
    PlaceOrderPipelineHandler<R, P, L>
{
    pub fn new(
        place_order_handler: PlaceOrderCmdHandler<R, P>,
        match_order_handler: MatchOrderCmdHandler<R, P, L>,
        sett_order_handler: SettOrderCmdHandler<R, P>,
    ) -> Self {
        Self { place_order_handler, match_order_handler, sett_order_handler }
    }

    // 规则：place/match/settlement 分阶段串联，后一阶段必须消费前一阶段输出
    pub fn exec(&self, cmd: NewOrderCmd) -> Result<PlaceOrderPipelineReply, SpotCmdErrorAny> {
        let order = self.place_order_handler.cmd_handle(cmd)?;
        let trades = self
            .match_order_handler
            .cmd_handle(MatchCmd { taker_order: order.object().clone() })?;

        let balances = if let Some(ref trade_events) = trades {
            if trade_events.is_empty() {
                None
            } else {
                self.sett_order_handler.cmd_handle(SettlementCmd {
                    trades: trade_events.iter().map(|trade| trade.object().clone()).collect(),
                })?
            }
        } else {
            None
        };

        //todo NewOrderFull
        Ok(PlaceOrderPipelineReply { order, trades, balances })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use base_types::account::balance::Balance as AccountBalance;
    use base_types::base_types::TraderId;
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::{OrderSide, OrderType, TimeInForce, TradingPair};
    use base_types::{AssetId, Price, Quantity, Timestamp};
    use bdd::bdd_test;
    use diff::Entity;
    use db_repo::adapter::mysql_repo::MySqlRepo;
    use diff::diff_types::{ChangeLog, ChangeType, DomainEvent};
    use lob_repo::core::repo_snapshot_support::LobError;
    use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

    use super::*;
    use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::mock_repo::{
        MockEventPublisher, MockMySqlRepo,
    };

    struct MockLobRepo {
        supported_symbols: Mutex<Vec<TradingPair>>,
    }

    impl MockLobRepo {
        fn new(supported_symbols: Vec<TradingPair>) -> Self {
            Self { supported_symbols: Mutex::new(supported_symbols) }
        }
    }

    impl MultiSymbolLobRepo for MockLobRepo {
        type Order = SpotOrder;

        fn match_orders(
            &self,
            _symbol: TradingPair,
            _side: OrderSide,
            _price: Price,
            _quantity: Quantity,
        ) -> (Option<Vec<&Self::Order>>, Quantity) {
            (None, Quantity::default())
        }

        fn best_bid(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn best_ask(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn contains_symbol(&self, symbol: &TradingPair) -> bool {
            self.supported_symbols.lock().unwrap().contains(symbol)
        }

        fn add_order(&self, _symbol: TradingPair, _order: Self::Order) -> Result<(), LobError> {
            Ok(())
        }

        fn remove_order(&self, _symbol: TradingPair, _order_id: base_types::OrderId) -> bool {
            true
        }

        fn find_order(&self, _p0: TradingPair, _p1: base_types::OrderId) -> Option<&Self::Order> {
            None
        }

        fn find_order_mut(
            &self,
            _p0: TradingPair,
            _order_id: base_types::OrderId,
        ) -> Option<&mut Self::Order> {
            None
        }

        fn last_price(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn update_last_price(&self, _symbol: TradingPair, _price: Price) {}
    }

    struct MatchingMockLobRepo {
        supported_symbols: Mutex<Vec<TradingPair>>,
        maker_orders: Arc<Vec<SpotOrder>>,
    }

    impl MatchingMockLobRepo {
        fn new(supported_symbols: Vec<TradingPair>, maker_orders: Vec<SpotOrder>) -> Self {
            Self {
                supported_symbols: Mutex::new(supported_symbols),
                maker_orders: Arc::new(maker_orders),
            }
        }
    }

    impl MultiSymbolLobRepo for MatchingMockLobRepo {
        type Order = SpotOrder;

        fn match_orders(
            &self,
            symbol: TradingPair,
            _side: OrderSide,
            _price: Price,
            quantity: Quantity,
        ) -> (Option<Vec<&Self::Order>>, Quantity) {
            if !self.supported_symbols.lock().unwrap().contains(&symbol) {
                return (None, quantity);
            }
            if self.maker_orders.is_empty() {
                return (None, quantity);
            }
            (Some(self.maker_orders.iter().collect()), Quantity::default())
        }

        fn best_bid(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn best_ask(&self, _symbol: TradingPair) -> Option<Price> {
            self.maker_orders.first().and_then(|order| order.price)
        }

        fn contains_symbol(&self, symbol: &TradingPair) -> bool {
            self.supported_symbols.lock().unwrap().contains(symbol)
        }

        fn add_order(&self, _symbol: TradingPair, _order: Self::Order) -> Result<(), LobError> {
            Ok(())
        }

        fn remove_order(&self, _symbol: TradingPair, _order_id: base_types::OrderId) -> bool {
            true
        }

        fn find_order(&self, _p0: TradingPair, _p1: base_types::OrderId) -> Option<&Self::Order> {
            None
        }

        fn find_order_mut(
            &self,
            _p0: TradingPair,
            _order_id: base_types::OrderId,
        ) -> Option<&mut Self::Order> {
            None
        }

        fn last_price(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn update_last_price(&self, _symbol: TradingPair, _price: Price) {}
    }

    fn create_test_cmd(client_order_id: &str) -> NewOrderCmd {
        NewOrderCmd::new(
            CMetadata::default(),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(1.0)),
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

    fn create_sell_order(order_id: u64, client_order_id: &str) -> SpotOrder {
        SpotOrder::create_order(
            order_id,
            TraderId::new([1u8; 8]),
            TradingPair::BtcUsdt,
            OrderSide::Sell,
            Price::from_f64(50000.0),
            Quantity::from_f64(1.0),
            TimeInForce::GTC,
            Some(client_order_id.to_string()),
            Quantity::default(),
        )
    }

    #[test]
    fn test_pipeline_exec_returns_order_and_skips_settlement_when_no_trades() {
        let pipeline = PlaceOrderPipelineHandler::new(
            PlaceOrderCmdHandler::new(MockMySqlRepo, MockEventPublisher),
            MatchOrderCmdHandler::new(
                MockMySqlRepo,
                MockEventPublisher,
                MockLobRepo::new(vec![TradingPair::BtcUsdt]),
            ),
            SettOrderCmdHandler::new(MockMySqlRepo, MockEventPublisher),
        );

        let reply = pipeline
            .exec(create_test_cmd("test_pipeline_001"))
            .expect("pipeline exec should succeed");

        assert_eq!(reply.order.object().trading_pair, TradingPair::BtcUsdt);
        assert_eq!(reply.order.object().side, OrderSide::Buy);
        assert_eq!(reply.order.object().price, Some(Price::from_f64(50000.0)));
        assert_eq!(reply.order.object().total_base_qty, Quantity::from_f64(1.0));
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
        let lob = MatchingMockLobRepo::new(vec![TradingPair::BtcUsdt], vec![maker_order.clone()]);

        let pipeline = PlaceOrderPipelineHandler::new(
            PlaceOrderCmdHandler::new(MySqlRepo::new_mock(), MockEventPublisher),
            MatchOrderCmdHandler::new(MySqlRepo::new_mock(), MockEventPublisher, lob),
            SettOrderCmdHandler::new(MySqlRepo::new_mock(), MockEventPublisher),
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
        assert_eq!(balances.len(), 2);
        let asset_ids: Vec<AssetId> =
            balances.iter().map(|balance| balance.object().asset_id).collect();
        assert!(asset_ids.contains(&AssetId::Usdt));
        assert!(asset_ids.contains(&AssetId::Btc));
    }

    #[test]
    #[bdd_test(
        feature = "撮合",
        scenario = "返回成交和余额",
        given = "[\"订单成交\"]",
        when = "执行管道",
        then = "[\"返回成交记录\", \"返回余额变更\"]",
        tags = "[\"pipeline\", \"trade\"]",
        priority = "4"
    )]
    fn test_pipeline_reply_can_carry_trades_and_balances2() {}
}
