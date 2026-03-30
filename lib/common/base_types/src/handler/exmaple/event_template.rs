use crate::handler::exmaple::cmd_template::CmdTemplate;
use crate::handler::handler::{CmdHandler, EventHandler};

pub struct EventHandlerTemplate{

    pub cmd:CmdTemplate;

}
impl EventHandler for EventHandlerTemplate{
    fn evn_handle(&self, cmd: C) -> Result<R, E> {

        cmd.handle();

    }
}

