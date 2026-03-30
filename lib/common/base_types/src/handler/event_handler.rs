
pub trait QueryHandler<C, R, E>: Send + Sync {
    fn handle(&self, cmd: C) -> Result<R, E>;
}

pub trait EventHandler<C, R, E>: Send + Sync {
    fn evn_handle(&self, cmd: C) -> Result<R, E>;
}

//todo command/changelog

//todo 1 load entity; 2 cal changelog; 3 persis changelog

//todo eventhandler

// receive changelog; invoke command;
