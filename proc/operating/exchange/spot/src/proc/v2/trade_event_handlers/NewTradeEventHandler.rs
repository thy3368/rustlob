use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::handler::event_handler::EventHandler;
use diff::ChangeLog;
use diff::diff_types::DomainEvent;

pub struct NewTradeEventHandler;

//todo 处理change log 调用 撮合结算 command handler
impl EventHandler for NewTradeEventHandler {
    fn event_handle(&self, event:  DomainEvent<SpotTrade>) -> Result<R, E> {
        todo!()
    }
}
