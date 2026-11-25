use std::collections::BTreeMap;

use base_types_v2::Symbol;
use example_core_entity::{BalanceV2, SpotOrderV7};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;

/// 三户模型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State4BuildBlock {
    /// 每owner一个state 三户模型
    pub spot_symbol_lob: BTreeMap<Symbol, LocalLobBtree<SpotOrderV7>>,

    /// user_id+symbol,balance
    pub user_symbol_quote_balance: BTreeMap<String, BalanceV2>,

    /// user_id+symbol,balance
    pub user_symbol_base_balance: BTreeMap<String, BalanceV2>,
}
