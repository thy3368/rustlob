// 参考 ## Account Endpoints / Account information (USER_DATA) /Users/hongyaotang/src/rustlob/design/other/binance-spot-api-docs/rest-api.md 定义所有 user data 接口；并用中文注shi

use crate::proc::behavior::spot_trade_behavior::{CMetadata, CmdResp, SpotCmdAny, SpotCmdError};

#[derive(Debug, Clone)]
pub enum SpotUserDataCmdAny {
    Account(AccountCmd),
    //todo
}

#[derive(Debug, Clone)]
pub struct AccountCmd {
    pub metadata: CMetadata,
}

#[derive(Debug, Clone)]
pub enum SpotUserDataRes {
    //todo
}

pub trait SpotUserDataBehavior: Send + Sync {
    fn handle(&mut self, cmd: SpotCmdAny) -> Result<CmdResp<SpotUserDataRes>, SpotCmdError>;
}
