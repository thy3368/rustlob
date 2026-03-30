pub trait CmdHandlerForCreate<C, R, E>: Send + Sync {
    fn handle_create(&self, cmd: C) -> Result<R, E>{

        todo!()
    }
}