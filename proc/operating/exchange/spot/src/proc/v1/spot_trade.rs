use base_types::{account::balance::Balance, exchange::spot::spot_types::{SpotOrder, SpotTrade, TimeInForce}, OrderId, OrderSide, Price, Quantity, Timestamp, TradingPair};
use db_repo::{CmdRepo, MySqlDbRepo};
use diff::ChangeLogEntry;
use id_generator::generator::IdGenerator;
use lob_repo::{adapter::embedded_lob_repo::EmbeddedLobRepo, core::symbol_lob_repo::MultiSymbolLobRepo};

use crate::proc::behavior::spot_trade_behavior::{
    CancelAllOrders, CancelAllOrdersRes, CancelOrder, CancelOrderRes, CmdResp, CommonError, IdemSpotResult, LimitOrder,
    LimitOrderRes, MarketOrder, MarketOrderRes, SpotCmdErrorAny, SpotTradeBehavior, SpotTradeCmdAny, SpotTradeResAny
};

pub struct SpotTradeBehaviorImpl {
    /// 余额仓储（依赖注入）
    pub balance_repo: MySqlDbRepo<Balance>,
    pub trade_repo: MySqlDbRepo<SpotTrade>,
    pub order_repo: MySqlDbRepo<SpotOrder>,
    pub lob_repo: EmbeddedLobRepo<SpotOrder>,
    /// ID生成器（节点ID为0）
    pub id_generator: IdGenerator
}

impl SpotTradeBehaviorImpl {
    /// 创建新的 SpotOrderExchBehaviorImpl 实例
    pub fn new(
        balance_repo: MySqlDbRepo<Balance>, trade_repo: MySqlDbRepo<SpotTrade>, order_repo: MySqlDbRepo<SpotOrder>,
        lob_repo: EmbeddedLobRepo<SpotOrder>, id_generator: IdGenerator
    ) -> Self {
        Self {
            balance_repo,
            trade_repo,
            order_repo,
            lob_repo,
            id_generator
        }
    }
}

impl SpotTradeBehaviorImpl {
    /// 生成订单ID
    fn generate_order_id(&self) -> u64 { self.id_generator.next_id() as u64 }

    pub(crate) fn handle_cancel_order(&self, _p0: CancelOrder) -> Result<CmdResp<CancelOrderRes>, SpotCmdErrorAny> {
        todo!()
    }

    pub(crate) fn handle_market_order(&self, _p0: MarketOrder) -> Result<CmdResp<MarketOrderRes>, SpotCmdErrorAny> {
        // todo 风控检查
        // todo 匹配
        todo!()
    }

    pub(crate) fn handle_cancel_all_orders(
        &self, _p0: CancelAllOrders
    ) -> Result<CmdResp<CancelAllOrdersRes>, SpotCmdErrorAny> {
        todo!()
    }
}

impl SpotTradeBehaviorImpl {
    // Result<CmdResp<SpotCmdRes>, SpotCmdError>;
    fn handle_limit_order(&mut self, limit_order: LimitOrder) -> Result<CmdResp<LimitOrderRes>, SpotCmdErrorAny> {
        // ========================================================================
        // 1. 命令验证 风控检查 - 余额检查并冻结保证金
        // ========================================================================
        // cmd.validate().map_err(PrepCommandError::ValidationError)?;
        //
        let order_id = self.generate_order_id();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;

        let mut internal_order = SpotOrder::create_limit(
            order_id,
            limit_order.trader,
            limit_order.account_id,
            limit_order.trading_pair,
            limit_order.side,
            limit_order.price,
            limit_order.quantity,
            limit_order.time_in_force,
            limit_order.client_order_id
        );

        let frozen_asset_balance_id = internal_order.frozen_asset_balance_id();
        let base_asset_balance_id =
            format!("{}:{}", internal_order.account_id.0, internal_order.trading_pair.base_asset());

        let mut frozen_asset_balance =
            self.balance_repo.find_by_id_4_update(&frozen_asset_balance_id).ok().unwrap().unwrap();
        let mut base_asset_balance =
            self.balance_repo.find_by_id_4_update(&base_asset_balance_id).ok().unwrap().unwrap();

        internal_order.frozen_margin(&mut frozen_asset_balance, Timestamp::now_as_nanos());

        // 匹配
        let matched_orders = self.lob_repo.match_orders(
            internal_order.trading_pair,
            internal_order.side,
            internal_order.price.unwrap(),
            internal_order.total_qty
        );

        // 如果匹配
        let mut trades = Vec::new();

        match matched_orders {
            None => {
                todo!()
            }
            Some(matched) => {
                // todo time_in_force 没有用
                match internal_order.time_in_force {
                    TimeInForce::GTC => {}
                    TimeInForce::IOC => {}
                    TimeInForce::FOK => {}
                    TimeInForce::GTX => {}
                    TimeInForce::GTD => {}
                }

                // matched_order 的状态也要同步变更，生成 log event 放在一个数据里
                for matched_order in matched {
                    let quote_asset_balance_id =
                        format!("{}:{}", matched_order.account_id.0, matched_order.trading_pair.quote_asset());
                    let base_asset_balance_id = matched_order.frozen_asset_balance_id();

                    let mut o_quote_asset_balance =
                        self.balance_repo.find_by_id_4_update(&quote_asset_balance_id).ok().unwrap().unwrap();
                    let mut o_base_asset_balance =
                        self.balance_repo.find_by_id_4_update(&base_asset_balance_id).ok().unwrap().unwrap();

                    let mut matched_order_mut = matched_order.clone();
                    let trade = internal_order.make_trade(
                        &mut matched_order_mut,
                        &mut frozen_asset_balance,
                        &mut base_asset_balance,
                        &mut o_quote_asset_balance,
                        &mut o_base_asset_balance
                    );

                    trades.push(trade);
                    if internal_order.is_all_filled() {
                        break;
                    }
                }
            }
        }

        // 没成交挂单
        if !internal_order.is_all_filled() {
            let _ = self.lob_repo.add_order(internal_order.trading_pair, internal_order);
        }

        // 所有数据持久化操作，一次性回放所有事件到数据库
        let all_events: Vec<ChangeLogEntry> = Vec::new();
        if !all_events.is_empty() {
            // 回放 matched_order 更新和 v1 创建事件到各自的 repo
            for event in &all_events {
                // 根据 entity_type 判断回放到哪个 repo
                // todo 增加balance position
                match event.entity_type().as_str() {
                    "SpotOrder" => {
                        if let Err(e) = self.order_repo.replay_event(event) {
                            log::error!("Failed to replay order event: {:?}", e);
                        }
                    }
                    "SpotTrade" => {
                        if let Err(e) = self.trade_repo.replay_event(event) {
                            log::error!("Failed to replay v1 event: {:?}", e);
                        }
                    }
                    "Balance" => {
                        if let Err(e) = self.balance_repo.replay_event(event) {
                            log::error!("Failed to replay balance event: {:?}", e);
                        }
                    }

                    _ => {}
                }
            }
        }

        todo!()
    }
}

impl SpotTradeBehavior for SpotTradeBehaviorImpl {
    fn handle(&mut self, cmd: SpotTradeCmdAny) -> IdemSpotResult {
        match cmd {
            SpotTradeCmdAny::LimitOrder(limit_order) => {
                // 将 LimitOrderRes 包装到 SpotCmdRes::LimitOrder 中
                self.handle_limit_order(limit_order).map(|resp| resp.map(SpotTradeResAny::LimitOrder))
            }
            SpotTradeCmdAny::MarketOrder(market_order) => {
                self.handle_market_order(market_order).map(|resp| resp.map(SpotTradeResAny::MarketOrder))
            }
            SpotTradeCmdAny::CancelOrder(cancel_order) => {
                self.handle_cancel_order(cancel_order).map(|resp| resp.map(SpotTradeResAny::CancelOrder))
            }
            SpotTradeCmdAny::CancelAllOrders(cancel_all_orders) => {
                self.handle_cancel_all_orders(cancel_all_orders).map(|resp| resp.map(SpotTradeResAny::CancelAllOrders))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use base_types::{
        base_types::TraderId,
        exchange::spot::spot_types::{OrderStatus, TimeInForce},
        AccountId, AssetId, TradingPair
    };

    use super::*;

    /// 测试：创建限价订单并验证基本流程
    ///
    /// 测试场景：
    /// 1. 创建BTC/USDT买单（价格10000，数量1个BTC）
    /// 2. 验证订单被正确创建
    /// 3. 验证订单ID被生成
    /// 4. 验证订单状态为Pending（待成交）
    #[test]
    fn test_limit_order_creation_and_validation() {
        // ========================================================================
        // 准备阶段
        // ========================================================================

        // 1. 创建测试数据
        let trader_id = TraderId::new([1, 2, 3, 4, 5, 6, 7, 8]);
        let account_id = AccountId(100);
        let trading_pair = TradingPair::BtcUsdt;
        let price = Price::from_raw(1_000_000_000_000); // 10000.00 USDT
        let quantity = Quantity::from_raw(100_000_000); // 1.00 BTC

        // 2. 创建限价单
        let order = SpotOrder::create_limit(
            12345,                          // order_id
            trader_id,                      // trader
            account_id,                     // account_id
            trading_pair,                   // trading_pair
            OrderSide::Buy,                 // Buy order
            price,                          // price
            quantity,                       // quantity
            TimeInForce::GTC,               // GTC: Good Till Cancel
            Some("CLIENT-001".to_string())  // client_order_id
        );

        // ========================================================================
        // 验证阶段
        // ========================================================================

        // 3. 验证订单基本信息
        assert_eq!(order.order_id, 12345, "订单ID应为12345");
        assert_eq!(order.trader_id, trader_id, "交易员ID应匹配");
        assert_eq!(order.account_id, account_id, "账户ID应为100");

        // 4. 验证订单方向和价格
        assert_eq!(order.side, OrderSide::Buy, "订单方向应为Buy");
        assert_eq!(order.price, Some(price), "订单价格应为10000 USDT");
        assert_eq!(order.total_qty, quantity, "订单数量应为1.00 BTC");

        // 5. 验证初始状态
        assert_eq!(order.status, OrderStatus::Pending, "订单状态应为Pending");
        assert_eq!(order.unfilled_qty, quantity, "未成交数量应等于总数量");
        assert_eq!(order.executed_qty, Quantity::default(), "已成交数量应为0");
        assert_eq!(order.cumulative_quote_qty, Quantity::default(), "累计成交金额应为0");

        // 6. 验证交易对信息
        assert_eq!(order.trading_pair, trading_pair, "交易对信息应匹配");
        assert_eq!(order.trading_pair.base_asset(), AssetId::Btc, "基础资产应为BTC");
        assert_eq!(order.trading_pair.quote_asset(), AssetId::Usdt, "计价资产应为USDT");

        // 7. 验证时间戳（应该是有效的毫秒时间戳）
        assert!(order.timestamp.0 > 0, "时间戳应为正数");
        assert!(order.timestamp.0 > 1_000_000_000, "时间戳应为合理的毫秒值");

        // 8. 验证客户端订单ID
        assert_eq!(order.client_order_id, Some("CLIENT-001".to_string()), "客户端订单ID应被保存");
    }
}
