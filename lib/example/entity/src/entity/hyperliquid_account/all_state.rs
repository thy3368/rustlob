use std::collections::BTreeMap;

use lob_repo_v2::imp::lob_btree::LocalLobBtree;

use crate::perp::trade::perp_position_v2::PerpPositionV2;
use crate::{Balance, HyperliquidPerpOrder, SpotAccount, SpotOrderV2};

/// 三户模型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllMatchState {
    /// 每owner一个state 三户模型
    pub spot_lob: BTreeMap<u64, LocalLobBtree<SpotOrderV2>>,

    ///条件单
    pub spot_con_orders: BTreeMap<u64, SpotOrderV2>,

    /// 每owner一个state 三户模型
    pub perp_lob: BTreeMap<u64, LocalLobBtree<HyperliquidPerpOrder>>,

    ///条件单
    pub perp_con_orders: BTreeMap<u64, HyperliquidPerpOrder>,
}

/// 三户模型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllClearinghouseState {
    /// 每owner一个state 三户模型
    pub state: BTreeMap<u64, OwnedClearinghouseState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedClearinghouseState {
    owner_id: u64,
    spot_balances: SpotWallet,
    funding_balances: SpotWallet,
    pending_withdrawals: PerpWallet,
}

// ─── 现货户 ───

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotWallet {
    pub account: SpotAccount,
    pub balances: BTreeMap<u64, Balance>,
}

// ─── 资金户（充值/提现入口） ───

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FundingWallet {
    pub account: SpotAccount,
    pub balances: BTreeMap<u64, Balance>,
}

// ─── Perp 合约户 ───

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpWallet {
    ///其实他只需要usdc的balance就行了，他的语义和上面的语义不一样，只有USDC
    pub cross: Balance, // 全仓余额（通常只有 USDC）
    pub isolated: BTreeMap<u64, PerpPositionV2>, // USDC
}
