use std::collections::BTreeMap;

use base_types_v2::Symbol;
use common_entity::{Entity, ReplayableDomainEvent, StateSinkV3};
use example_core_entity::spot::SpotTradeV3;
use example_core_entity::{BalanceV2, OrderSide, PartyId, SpotOrderV7};
use example_core_use_case::pipe_line::cancel_spot_order::{
    CancelSpotOrderFsmV3Changes, CancelSpotOrderFsmV3UseCase,
};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;
use thiserror::Error;

/// 独立use case通过source/sink影响了共享状态
#[derive(Debug, Clone, PartialEq, Eq)]
/// todo 数据组织可能还有更好的方式，比如balance可能被复用的情况现在可能有问题，
/// 每个容器都需要balance的所有权
/// 通过order找balance: 通过order的party_id 到balance的数组里面去找
pub struct State4BuildBlock {
    /// 下一笔成交使用的全局成交 ID。
    pub next_trade_id: u64,

    /// Maker 费率，单位为基点。
    pub maker_fee_bps: u64,

    /// Taker 费率，单位为基点。
    pub taker_fee_bps: u64,

    /// 每owner一个state
    pub spot_symbol_lob: BTreeMap<Symbol, LocalLobBtree<SpotOrderV7>>,

    /// todo 对于balance，use case的state_given有优化空间，可以减少转换过程

    /// party_id+symbol,balance
    pub user_symbol_quote_balance: BTreeMap<(PartyId, Symbol), BalanceV2>,

    /// party_id+symbol,balance
    pub user_symbol_base_balance: BTreeMap<(PartyId, Symbol), BalanceV2>,

    /// 可能不对
    pub user_symbol_trades: BTreeMap<(PartyId, Symbol), Vec<SpotTradeV3>>,

    // pub symbol_trades: BTreeMap<Symbol, Vec<SpotTradeV3>>,
    /// 每个用户每个symbol
    pub user_symbol_orders: BTreeMap<(PartyId, Symbol), Vec<SpotOrderV7>>,
}
