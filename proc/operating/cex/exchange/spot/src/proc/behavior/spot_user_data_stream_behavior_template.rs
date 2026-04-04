// 参考 ## Account Endpoints / Account information (USER_DATA) /Users/hongyaotang/src/rustlob/design/other/binance-spot-api-docs/rest-api.md 定义所有 user data 接口

use crate::proc::behavior::spot_trade_behavior::{CMetadata, CmdResp, SpotCmdErrorAny};

/// User Data 命令枚举
#[derive(Debug, Clone)]
pub enum SpotUserDataCmdAny {
    /// 账户信息查询 GET /api/v3/account
    /// Weight: 20
    Account(AccountCmd),
}

/// 账户信息查询命令
/// GET /api/v3/account
/// Weight: 20
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
pub struct AccountCmd {
    pub metadata: CMetadata,
    /// 仅返回非零余额，默认 false
    pub omit_zero_balances: Option<bool>,
    /// 接收窗口（微秒精度），不超过 60000
    pub recv_window: Option<u64>,
    /// 时间戳
    pub timestamp: i64,
}

/// User Data 响应枚举
#[derive(Debug, Clone)]
pub enum SpotUserDataRes {}

/// User Data 行为接口
pub trait SpotUserDataStreamBehavior: Send + Sync {
    /// 处理 User Data 命令
    fn handle(
        &mut self,
        cmd: SpotUserDataCmdAny,
    ) -> Result<CmdResp<SpotUserDataRes>, SpotCmdErrorAny>;
}
