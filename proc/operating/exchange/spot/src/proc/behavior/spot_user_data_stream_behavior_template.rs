// 参考 ## Account Endpoints / Account information (USER_DATA) /Users/hongyaotang/src/rustlob/design/other/binance-spot-api-docs/rest-api.md 定义所有 user data 接口

use crate::proc::behavior::spot_trade_behavior::{CMetadata, CmdResp, SpotCmdError};

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
    pub recv_window: Option<f64>,
    /// 时间戳
    pub timestamp: i64,
}

/// User Data 响应枚举
#[derive(Debug, Clone)]
pub enum SpotUserDataRes {
    /// 账户信息响应
    Account(AccountInfo),
}

/// 账户信息
#[derive(Debug, Clone)]
pub struct AccountInfo {
    /// Maker 佣金
    pub maker_commission: i32,
    /// Taker 佣金
    pub taker_commission: i32,
    /// 买方佣金
    pub buyer_commission: i32,
    /// 卖方佣金
    pub seller_commission: i32,
    /// 佣金费率
    pub commission_rates: CommissionRates,
    /// 可交易
    pub can_trade: bool,
    /// 可提现
    pub can_withdraw: bool,
    /// 可充值
    pub can_deposit: bool,
    /// 经纪账户
    pub brokered: bool,
    /// 需要自成交防护
    pub require_self_trade_prevention: bool,
    /// 阻止 SOR
    pub prevent_sor: bool,
    /// 更新时间
    pub update_time: i64,
    /// 账户类型
    pub account_type: String,
    /// 余额列表
    pub balances: Vec<Balance>,
    /// 权限列表
    pub permissions: Vec<String>,
    /// 用户ID
    pub uid: i64,
}

/// User Data 行为接口
pub trait SpotUserDataStreamBehavior: Send + Sync {
    /// 处理 User Data 命令
    fn handle(&mut self, cmd: SpotUserDataCmdAny) -> Result<CmdResp<SpotUserDataRes>, SpotCmdError>;
}
