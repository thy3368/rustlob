use crate::cqrs::cqrs_types::CmdResp;

pub trait Handler<C, R, E>: Send + Sync {
    async fn handle(&self, cmd: C) -> Result<CmdResp<R>, E>;
}

// 业务功能 里面包含 取数，计算changelog,持久化，发消息等能力
pub trait CmdHandler<C, R, E>: Send + Sync {
    fn handle(&self, cmd: C) -> Result<R, E>;

    //todo 1. queue state; 2. cal changelog for state; 3. publish changelog
}

pub trait QueryHandler<C, R, E>: Send + Sync {
    fn handle(&self, cmd: C) -> Result<R, E>;
}

pub trait EventHandler<C, R, E>: Send + Sync {
    fn evn_handle(&self, cmd: C) -> Result<R, E> {
        //todo 调用cmd handler

        todo!()
    }
}

//todo command/changelog

//todo 1 load entity; 2 cal changelog; 3 persis changelog

//todo eventhandler

// receive changelog; invoke command;
