use account::Balance;
use base_types::{Price, Quantity, Side};
use db_repo::{CmdRepo, MySqlDbRepo};
use diff::ChangeLogEntry;
use lob::lob::{
    Command, CommandResponse, IdempotentSpotCommand, IdempotentSpotResult, SpotCommand, SpotCommandError,
    SpotCommandResult, SpotOrder, SpotOrderExchangeProc, SpotTrade, TraderId
};
use lob_repo::{adapter::standalone_lob_repo::StandaloneLobRepo, core::symbol_lob_repo::MultiSymbolLobRepo};

pub struct SpotOrderExchangeProcImpl {
    /// 余额仓储（依赖注入）
    balance_repo: MySqlDbRepo<Balance>,

    trade_repo: MySqlDbRepo<SpotTrade>,

    order_repo: MySqlDbRepo<SpotOrder>,

    lob_repo: StandaloneLobRepo<SpotOrder>
}

impl SpotOrderExchangeProcImpl {
    fn limit_order_buy(
        &mut self, cmd: Command<SpotCommand::LimitOrder>
    ) -> Result<CommandResponse<SpotCommandResult::LimitOrder>, SpotCommandError> {
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



        let mut internal_order = SpotOrder::pending(
            order_id,
            TraderId([0; 8]), // TODO: 从请求或上下文获取真实trader_id
            cmd.quantity,     // frozen_qty
            frozen_asset,     // frozen_asset
            self.account_id,
            cmd.trading_pair,
            cmd.price,
            cmd.quantity,
            cmd.side,
            now
        );

        let quote_asset_balance_id = format!("{}:{}", "self.account_id.0", cmd.payload.trading_pair.quote_asset.0);
        let base_asset_balance_id = format!("{}:{}", "self.account_id.0", cmd.payload.trading_pair.base_asset.0);

        let mut quote_asset_balance = match self.balance_repo.find_by_id(&quote_asset_balance_id).ok().flatten() {
            Some(b) => b,
            None => todo!() // todo 应该报错
        };

        let mut base_asset_balance = match self.balance_repo.find_by_id(&quote_asset_balance_id).ok().flatten() {
            Some(b) => b,
            None => todo!() // todo 应该报错
        };


        // 2 风控检查 - 余额检查并冻结保证金
        internal_order.frozen_margin(&mut quote_asset_balance, now);


        // todo time_in_force 没有用


        // 匹配
        let matched_orders = self.lob_repo.match_orders(
            cmd.payload.trading_pair,
            internal_order.side,
            internal_order.price.unwrap_or_else(|| Price::from_f64(50000.0)),
            Quantity::from_raw(internal_order.total_qty)
        );

        if (matched_orders.is_some()) {
            // 如果匹配
            let mut trades = Vec::new();
            if let Some(matched) = matched_orders {
                // matched_order 的状态也要同步变更，生成 log event 放在一个数据里
                for matched_order in matched {
                    let quote_asset_balance_id =
                        format!("{}:{}", matched_order.account_id.0, cmd.payload.trading_pair.quote_asset.0);
                    let base_asset_balance_id =
                        format!("{}:{}", matched_order.account_id.0, cmd.payload.trading_pair.base_asset.0);

                    let mut o_quote_asset_balance =
                        match self.balance_repo.find_by_id(&quote_asset_balance_id).ok().flatten() {
                            Some(b) => b,
                            None => todo!() // todo 应该报错
                        };

                    let mut o_base_asset_balance =
                        match self.balance_repo.find_by_id(&quote_asset_balance_id).ok().flatten() {
                            Some(b) => b,
                            None => todo!() // todo 应该报错
                        };


                    let mut matched_order_mut = matched_order.clone();
                    let trade = internal_order.make_trade_4_buy(
                        &mut matched_order_mut,
                        &mut quote_asset_balance,
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
        if (!internal_order.is_all_filled()) {
            let _ = self.lob_repo.add_order(internal_order.trading_pair, internal_order);
        }


        // 所有数据持久化操作
        let mut all_events: Vec<ChangeLogEntry> = Vec::new();

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
                        if let Err(e) = self.trade_repo.replay_event(event) {
                            log::error!("Failed to replay trade event: {:?}", e);
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
    fn handle(&mut self, cmd: IdempotentSpotCommand) -> IdempotentSpotResult {
        match cmd.payload {
            SpotCommand::LimitOrder {
                ..
            } => self.limit_order_buy(cmd),
            SpotCommand::MarketOrder {
                ..
            } => {
                todo!()
            }
            SpotCommand::CancelOrder {
                ..
            } => {
                todo!()
            }
            SpotCommand::CancelAllOrders {
                ..
            } => {
                todo!()
            }
        }
        todo!()
    }
}
