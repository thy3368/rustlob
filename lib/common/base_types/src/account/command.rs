//! 账户命令定义
//!
//! 统一 LOB 和 Settlement 的所有账户操作

use crate::account::balance::Balance;
use crate::account::error::BalanceError;
use crate::{AccountId, AssetId, OrderId, OrderSide, Price, TradingPair};

/// 账户命令（统一）
///
/// 合并 LOB 和 Settlement 的所有账户操作：
/// - LOB 调用：CheckAndFreeze, Unfreeze（基于交易对）
/// - Settlement 调用：Credit, Debit, DebitFrozen, Transfer 等（直接操作资产）
#[derive(Debug, Clone)]
pub enum AccountCommand {
    // ==================== LOB 调用（基于交易对） ====================
    /// 检查并冻结（原子操作，防止 TOCTOU 竞态）
    /// 下单时使用，一步完成检查+冻结
    CheckAndFreeze {
        account_id: AccountId,
        order_id: OrderId,
        pair: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Price,
    },

    /// 解冻资金（撤单时释放）
    Unfreeze {
        account_id: AccountId,
        order_id: OrderId,
        pair: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Price,
    },

    // ==================== Settlement 调用（直接操作资产） ====================
    /// 冻结指定资产（可用 → 冻结）
    Freeze { account_id: AccountId, asset_id: AssetId, amount: Price, reference_id: u64 },

    /// 解冻指定资产（冻结 → 可用）
    UnfreezeAsset { account_id: AccountId, asset_id: AssetId, amount: Price, reference_id: u64 },

    /// 增加可用余额（入金、收款、成交收入）
    Credit { account_id: AccountId, asset_id: AssetId, amount: Price, reference_id: u64 },

    /// 扣减可用余额（出金、付款）
    Debit { account_id: AccountId, asset_id: AssetId, amount: Price, reference_id: u64 },

    /// 扣减冻结余额（成交扣款、强平）
    DebitFrozen { account_id: AccountId, asset_id: AssetId, amount: Price, reference_id: u64 },

    /// 转账（同用户不同账户间）
    Transfer {
        from_account_id: AccountId,
        to_account_id: AccountId,
        asset_id: AssetId,
        amount: Price,
        reference_id: u64,
    },

    /// 结算盈亏（可正可负）
    SettlePnl { account_id: AccountId, asset_id: AssetId, pnl: Price, reference_id: u64 },

    // ==================== 查询 ====================
    /// 查询余额
    GetBalance { account_id: AccountId, asset_id: AssetId },
}

impl AccountCommand {
    /// 获取账户ID
    pub fn account_id(&self) -> AccountId {
        match self {
            AccountCommand::CheckAndFreeze { account_id, .. }
            | AccountCommand::Unfreeze { account_id, .. }
            | AccountCommand::Freeze { account_id, .. }
            | AccountCommand::UnfreezeAsset { account_id, .. }
            | AccountCommand::Credit { account_id, .. }
            | AccountCommand::Debit { account_id, .. }
            | AccountCommand::DebitFrozen { account_id, .. }
            | AccountCommand::SettlePnl { account_id, .. }
            | AccountCommand::GetBalance { account_id, .. } => *account_id,
            AccountCommand::Transfer { from_account_id, .. } => *from_account_id,
        }
    }

    /// 获取资产ID（如果适用）
    pub fn asset_id(&self) -> Option<AssetId> {
        match self {
            AccountCommand::Freeze { asset_id, .. }
            | AccountCommand::UnfreezeAsset { asset_id, .. }
            | AccountCommand::Credit { asset_id, .. }
            | AccountCommand::Debit { asset_id, .. }
            | AccountCommand::DebitFrozen { asset_id, .. }
            | AccountCommand::Transfer { asset_id, .. }
            | AccountCommand::SettlePnl { asset_id, .. }
            | AccountCommand::GetBalance { asset_id, .. } => Some(*asset_id),
            AccountCommand::CheckAndFreeze { .. } | AccountCommand::Unfreeze { .. } => None,
        }
    }

    /// 获取关联ID（订单ID或结算ID）
    pub fn reference_id(&self) -> Option<u64> {
        match self {
            AccountCommand::CheckAndFreeze { order_id, .. }
            | AccountCommand::Unfreeze { order_id, .. } => Some(*order_id),
            AccountCommand::Freeze { reference_id, .. }
            | AccountCommand::UnfreezeAsset { reference_id, .. }
            | AccountCommand::Credit { reference_id, .. }
            | AccountCommand::Debit { reference_id, .. }
            | AccountCommand::DebitFrozen { reference_id, .. }
            | AccountCommand::Transfer { reference_id, .. }
            | AccountCommand::SettlePnl { reference_id, .. } => Some(*reference_id),
            AccountCommand::GetBalance { .. } => None,
        }
    }

    /// 计算交易对命令的冻结资产和金额
    /// 仅适用于 CheckAndFreeze 和 Unfreeze
    #[inline]
    pub fn trading_pair_amount(&self) -> Option<(AssetId, Price)> {
        match self {
            AccountCommand::CheckAndFreeze { pair, side, price, quantity, .. }
            | AccountCommand::Unfreeze { pair, side, price, quantity, .. } => match side {
                OrderSide::Buy => {
                    let amount = price.checked_mul(*quantity)?;
                    Some((pair.quote_asset(), amount))
                }
                OrderSide::Sell => Some((pair.base_asset(), *quantity)),
            },
            _ => None,
        }
    }
}

/// 账户命令执行结果
#[derive(Debug, Clone)]
pub enum AccountCommandResult {
    /// 冻结成功（CheckAndFreeze, Freeze）
    Frozen {
        reference_id: u64,
        asset_id: AssetId,
        amount: Price,
        new_available: Price,
        new_frozen: Price,
    },

    /// 解冻成功（Unfreeze, UnfreezeAsset）
    Unfrozen {
        reference_id: u64,
        asset_id: AssetId,
        amount: Price,
        new_available: Price,
        new_frozen: Price,
    },

    /// 入账成功（Credit）
    Credited { reference_id: u64, asset_id: AssetId, amount: Price, new_available: Price },

    /// 扣款成功（Debit, DebitFrozen）
    Debited {
        reference_id: u64,
        asset_id: AssetId,
        amount: Price,
        from_frozen: bool,
        new_available: Price,
        new_frozen: Price,
    },

    /// 转账成功（Transfer）
    Transferred {
        reference_id: u64,
        from_account_id: AccountId,
        to_account_id: AccountId,
        asset_id: AssetId,
        amount: Price,
    },

    /// 盈亏结算成功（SettlePnl）
    PnlSettled { reference_id: u64, asset_id: AssetId, pnl: Price, new_available: Price },

    /// 余额查询结果
    Balance(Option<Balance>),

    /// 错误
    Error(BalanceError),
}

impl AccountCommandResult {
    /// 检查是否成功
    #[inline]
    pub fn is_ok(&self) -> bool {
        !matches!(self, AccountCommandResult::Error(_))
    }

    /// 检查是否失败
    #[inline]
    pub fn is_err(&self) -> bool {
        matches!(self, AccountCommandResult::Error(_))
    }

    /// 获取错误（如果有）
    pub fn err(&self) -> Option<&BalanceError> {
        match self {
            AccountCommandResult::Error(e) => Some(e),
            _ => None,
        }
    }
}
