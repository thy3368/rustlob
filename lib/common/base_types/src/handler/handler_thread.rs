pub trait CmdHandlerForCreate<C, R, E>: Send + Sync {
    fn cmd_handle(&self, cmd: C) -> Result<R, E>{

        todo!()
    }
}