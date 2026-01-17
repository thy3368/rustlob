use account::Balance;
use base_types::{Price, Quantity, Side};
use db_repo::{CmdRepo, MySqlDbRepo};
use diff::ChangeLogEntry;
use id_generator::generator::IdGenerator;
use lob::lob::domain::service::trading_spot_order_proc::LimitOrder;
use lob::lob::{
    Cmd, CmdResp, CommonError, IdempotentSpotCmd, IdempotentSpotResult, SpotCmdAny, SpotCmdError, SpotCmdResult,
    SpotOrder, SpotOrderExchangeProc, SpotTrade, TraderId,
};
use lob_repo::{adapter::standalone_lob_repo::StandaloneLobRepo, core::symbol_lob_repo::MultiSymbolLobRepo};

pub struct SpotOrderExchangeProcImpl {
    /// 余额仓储（依赖注入）
    balance_repo: MySqlDbRepo<Balance>,

    trade_repo: MySqlDbRepo<SpotTrade>,

    order_repo: MySqlDbRepo<SpotOrder>,

    lob_repo: StandaloneLobRepo<SpotOrder>,

    /// ID生成器（节点ID为0）
    id_generator: IdGenerator,
}

impl SpotOrderExchangeProcImpl {
    /// 生成订单ID
    fn generate_order_id(&self) -> u64 {
        self.id_generator.next_id() as u64
    }

    fn limit_order(&mut self, limitOrder: LimitOrder) -> Result<CmdResp<SpotCmdResult>, SpotCmdError> {
        // todo
        // 1 检查余额并下单（根据买单还是卖单
        // 买单冻结quote_asset余额，卖单冻结base_asset余额）
        // 2 匹配撮合（根据买卖单，冻结变成实付，买的增加base_asset余额，
        // 卖的增加quote_asset余额）    TimeInForce 逻辑处理（GTC/IOC/FOK/PostOnly）
        // 3 当前订单未撮合完则在lob中挂单
        // 4 通过entity trait 获得所有的实体变更changelog并持久化  生成事件（订单更新 +
        // 交易记录）持久化事件到仓储

        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        // cmd.validate().map_err(PrepCommandError::ValidationError)?;

        let order_id = self.generate_order_id();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;

        let mut internal_order = SpotOrder::create_limit(
            order_id,
            limitOrder.trader,
            limitOrder.account_id,
            limitOrder.trading_pair,
            limitOrder.side,
            limitOrder.price,
            limitOrder.quantity,
            limitOrder.time_in_force,
            limitOrder.client_order_id,
        );

        let frozen_asset_balance_id = internal_order.frozen_asset_balance_id();

        let base_asset_balance_id =
            format!("{}:{}", internal_order.account_id.0, internal_order.trading_pair.base_asset.0);

        let mut frozen_asset_balance = self.balance_repo.find_by_id(&frozen_asset_balance_id).ok().unwrap().unwrap();
        let mut base_asset_balance = self.balance_repo.find_by_id(&base_asset_balance_id).ok().unwrap().unwrap();

        // 2 风控检查 - 余额检查并冻结保证金
        internal_order.frozen_margin(&mut frozen_asset_balance, now);

        // todo time_in_force 没有用

        // 匹配
        let matched_orders = self.lob_repo.match_orders(
            internal_order.trading_pair,
            internal_order.side,
            internal_order.price.unwrap(),
            internal_order.total_qty,
        );

        if matched_orders.is_some() {
            // 如果匹配
            let mut trades = Vec::new();
            if let Some(matched) = matched_orders {
                // matched_order 的状态也要同步变更，生成 log event 放在一个数据里
                for matched_order in matched {
                    let quote_asset_balance_id =
                        format!("{}:{}", matched_order.account_id.0, matched_order.trading_pair.quote_asset.0);
                    let base_asset_balance_id = matched_order.frozen_asset_balance_id();

                    let mut o_quote_asset_balance = self.balance_repo.find_by_id(&quote_asset_balance_id).ok().unwrap().unwrap();
                    let mut o_base_asset_balance = self.balance_repo.find_by_id(&base_asset_balance_id).ok().unwrap().unwrap();

                    let mut matched_order_mut = matched_order.clone();
                    let trade = internal_order.make_trade(
                        &mut matched_order_mut,
                        &mut frozen_asset_balance,
                        &mut base_asset_balance,
                        &mut o_quote_asset_balance,
                        &mut o_base_asset_balance,
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

        // 所有数据持久化操作
        let all_events: Vec<ChangeLogEntry> = Vec::new();

        // 3. 一次性回放所有事件到数据库
        if !all_events.is_empty() {
            // 回放 matched_order 更新和 trade 创建事件到各自的 repo
            for event in &all_events {
                // 根据 entity_type 判断回放到哪个 repo
                // todo 增加balance position
                match event.entity_type.as_str() {
                    "SpotOrder" => {
                        if let Err(e) = self.order_repo.replay_event(event) {
                            log::error!("Failed to replay order event: {:?}", e);
                        }
                    }
                    "SpotTrade" => {
                        if let Err(e) = self.trade_repo.replay_event(event) {
                            log::error!("Failed to replay trade event: {:?}", e);
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

impl SpotOrderExchangeProc for SpotOrderExchangeProcImpl {
    fn handle(&mut self, cmd: SpotCmdAny) -> IdempotentSpotResult {
        match cmd {
            //todo 移动后使用的值 [E0382]
            SpotCmdAny::LimitOrder(limitOrder) => self.limit_order(limitOrder),
            SpotCmdAny::MarketOrder { .. } => Err(SpotCmdError::Common(CommonError::InvalidParameter {
                field: "payload",
                reason: "MarketOrder not yet implemented",
            })),
            SpotCmdAny::CancelOrder { .. } => Err(SpotCmdError::Common(CommonError::InvalidParameter {
                field: "payload",
                reason: "CancelOrder not yet implemented",
            })),
            SpotCmdAny::CancelAllOrders { .. } => Err(SpotCmdError::Common(CommonError::InvalidParameter {
                field: "payload",
                reason: "CancelAllOrders not yet implemented",
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use account::AccountId;
    use base_types::{AssetId, TradingPair};

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
        let trading_pair = TradingPair {
            base_asset: AssetId::BTC,   // BTC
            quote_asset: AssetId::USDT, // USDT
        };
        let price = Price::from_raw(1_000_000_000_000); // 10000.00 USDT
        let quantity = Quantity::from_raw(100_000_000); // 1.00 BTC

        // 2. 创建限价单
        let order = SpotOrder::create_limit(
            12345,                          // order_id
            trader_id,                      // trader
            account_id,                     // account_id
            trading_pair,                   // trading_pair
            Side::Buy,                      // Buy order
            price,                          // price
            quantity,                       // quantity
            lob::lob::TimeInForce::GTC,     // GTC: Good Till Cancel
            Some("CLIENT-001".to_string()), // client_order_id
        );

        // ========================================================================
        // 验证阶段
        // ========================================================================

        // 3. 验证订单基本信息
        assert_eq!(order.order_id, 12345, "订单ID应为12345");
        assert_eq!(order.trader_id, trader_id, "交易员ID应匹配");
        assert_eq!(order.account_id, account_id, "账户ID应为100");

        // 4. 验证订单方向和价格
        assert_eq!(order.side, Side::Buy, "订单方向应为Buy");
        assert_eq!(order.price, Some(price), "订单价格应为10000 USDT");
        assert_eq!(order.total_qty, quantity, "订单数量应为1.00 BTC");

        // 5. 验证初始状态
        assert_eq!(order.status, lob::lob::OrderStatus::Pending, "订单状态应为Pending");
        assert_eq!(order.unfilled_qty, quantity, "未成交数量应等于总数量");
        assert_eq!(order.executed_qty, Quantity::default(), "已成交数量应为0");
        assert_eq!(order.cumulative_quote_qty, Quantity::default(), "累计成交金额应为0");

        // 6. 验证交易对信息
        assert_eq!(order.trading_pair, trading_pair, "交易对信息应匹配");
        assert_eq!(order.trading_pair.base_asset, AssetId::BTC, "基础资产应为BTC");
        assert_eq!(order.trading_pair.quote_asset, AssetId::USDT, "计价资产应为USDT");

        // 7. 验证时间戳（应该是有效的毫秒时间戳）
        assert!(order.timestamp > 0, "时间戳应为正数");
        assert!(order.timestamp > 1_000_000_000, "时间戳应为合理的毫秒值");

        // 8. 验证客户端订单ID
        assert_eq!(order.client_order_id, Some("CLIENT-001".to_string()), "客户端订单ID应被保存");
    }
}
