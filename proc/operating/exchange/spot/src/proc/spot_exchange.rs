use account::Balance;
use db_repo::MySqlDbRepo;
use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotCommand, SpotOrder, SpotOrderExchangeProc, SpotTrade};
use lob_repo::adapter::standalone_lob_repo::StandaloneLobRepo;

pub struct SpotOrderExchangeProcImpl{

    /// 余额仓储（依赖注入）
    balance_repo: MySqlDbRepo<Balance>,

    
    trade_repo: MySqlDbRepo<SpotTrade>,

    order_repo: MySqlDbRepo<SpotOrder>,

    lob_repo: StandaloneLobRepo<SpotOrder>,

    
}

impl SpotOrderExchangeProcImpl {

}

impl SpotOrderExchangeProc for SpotOrderExchangeProcImpl {
    fn handle(&mut self, cmd: IdempotentSpotCommand) -> IdempotentSpotResult {
        
        match cmd.payload {
            SpotCommand::LimitOrder { .. } => {
                
                todo!()
            }
            SpotCommand::MarketOrder { .. } => {
                todo!()
            }
            SpotCommand::CancelOrder { .. } => {
                todo!()
            }
            SpotCommand::CancelAllOrders { .. } => {
                todo!()
            }
        }
        todo!()
    }
}