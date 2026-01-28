use crate::proc::behavior::spot_trade_behavior::{AlgoTradeProc, IdemAlgoResult, IdemAlgoCmd};
use crate::proc::v1::spot_trade::SpotTradeBehaviorImpl;

impl AlgoTradeProc for SpotTradeBehaviorImpl {
    fn handle(&mut self, cmd: IdemAlgoCmd) -> IdemAlgoResult { todo!() }
}
