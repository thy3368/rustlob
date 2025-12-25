use account::Balance;
use db_repo::MySqlDbRepo;
use lob::lob::{
    Command, CommandResponse, IdempotentSpotCommand, IdempotentSpotResult, SpotCommand, SpotCommandError,
    SpotCommandResult, SpotOrder, SpotOrderExchangeProc, SpotTrade
};
use lob_repo::adapter::standalone_lob_repo::StandaloneLobRepo;

pub struct SpotOrderExchangeProcImpl {
    /// 余额仓储（依赖注入）
    balance_repo: MySqlDbRepo<Balance>,

    trade_repo: MySqlDbRepo<SpotTrade>,

    order_repo: MySqlDbRepo<SpotOrder>,

    lob_repo: StandaloneLobRepo<SpotOrder>
}

impl SpotOrderExchangeProcImpl {
    fn limit_order(
        &mut self, cmd: Command<SpotCommand::LimitOrder>
    ) -> Result<CommandResponse<SpotCommandResult::LimitOrder>, SpotCommandError> {


        // todo
        //1 检查余额并下单（根据买单还是卖单 买单冻结quote_asset余额，卖单冻结base_asset余额）
        //2 匹配撮合（根据买卖单，冻结变成实付，买的增加base_asset余额，卖的增加quote_asset余额）    TimeInForce 逻辑处理（GTC/IOC/FOK/PostOnly）
        //3 当前订单未撮合完则在lob中挂单
        //4 通过entity trait 获得所有的实体变更changelog并持久化  生成事件（订单更新 + 交易记录）持久化事件到仓储


        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        // cmd.validate().map_err(PrepCommandError::ValidationError)?;

        let balance_id = format!("{}:{}", "self.account_id.0", cmd.payload.trading_pair.base_asset.0);
        let mut balance = match self.balance_repo.find_by_id(&balance_id).ok().flatten() {
            Some(b) => b,
            None => todo!() // todo 应该报错
        };
        
        
        // todo time_in_force 没有用
        let order_id = self.generate_order_id();

        // 1 创建订单
        let mut internal_order = SpotOrder::pending(
            order_id.clone(),
            self.account_id,
            cmd.trading_pair,
            cmd.side,
            cmd.order_type,
            cmd.quantity,
            cmd.price,
            cmd.leverage
        );




        let now = self.now();
        // 2 风控检查 - 余额检查并冻结保证金
        internal_order.frozen_margin(balance.clone(), now);
        
        
        todo!()

    }
}

impl SpotOrderExchangeProc for SpotOrderExchangeProcImpl {
    fn handle(&mut self, cmd: IdempotentSpotCommand) -> IdempotentSpotResult {
        match cmd.payload {
            SpotCommand::LimitOrder {
                ..
            } => {
                todo!()
            }
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
