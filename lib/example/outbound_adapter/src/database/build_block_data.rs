use std::collections::BTreeMap;

use base_types_v2::Symbol;
use example_core_entity::spot::SpotTradeV3;
use example_core_entity::{BalanceV2, SpotOrderV7};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;

/// 独立use case通过source/sink影响了共享状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State4BuildBlock {
    /// 每owner一个state
    pub spot_symbol_lob: BTreeMap<Symbol, LocalLobBtree<SpotOrderV7>>,

    /// user_id+symbol,balance
    pub user_symbol_quote_balance: BTreeMap<String, BalanceV2>,

    /// user_id+symbol,balance
    pub user_symbol_base_balance: BTreeMap<String, BalanceV2>,

    /// 可能不对
    pub user_symbol_trades: BTreeMap<String, Vec<SpotTradeV3>>,

    /// 每个用户每个symbol
    pub user_symbol_orders: BTreeMap<String, Vec<SpotOrderV7>>,
}
