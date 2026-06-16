mod block;
mod exchange;
mod perp;
mod shared;
mod spot;
mod support;
mod treasury;

pub use block::{BlockExecutionBody, NewBlock};
pub use exchange::{
    CommandEnvelope, ExchangeState, PerpCommand, ProductCommand, SpotCommand, TreasuryCommand,
    build_new_block,
};
pub use perp::PerpState;
pub use shared::{AccountAssetKey, AccountMarketKey};
pub use spot::{SpotAssetPair, SpotState};
pub(crate) use support::stable_hash_hex;
pub use treasury::{TreasuryState, WithdrawLockState};

pub(crate) fn event_commitment(event: &cmd_handler::EntityReplayableEvent) -> String {
    let fields = event
        .field_changes
        .iter()
        .map(|change| {
            let name = change.field_name_as_str().unwrap_or_default();
            let old_value = std::str::from_utf8(change.old_value_bytes()).unwrap_or_default();
            let new_value = std::str::from_utf8(change.new_value_bytes()).unwrap_or_default();
            format!("{name}:{old_value}:{new_value}:{}", change.field_type)
        })
        .collect::<Vec<_>>()
        .join("|");
    stable_hash_hex(&[
        event.timestamp.to_string(),
        event.sequence.to_string(),
        event.old_version.to_string(),
        event.new_version.to_string(),
        event.entity_id.to_string(),
        event.entity_type.to_string(),
        event.change_type.to_string(),
        fields,
    ])
}

#[cfg(test)]
mod tests {
    use super::{
        AccountAssetKey, ExchangeState, PerpState, SpotAssetPair, SpotState, TreasuryState,
        WithdrawLockState,
    };

    #[test]
    fn top_level_entity_exports_still_cover_core_state_types() {
        let _: ExchangeState = ExchangeState::default();
        let _: SpotState = SpotState::default();
        let _: PerpState = PerpState::default();
        let _: TreasuryState = TreasuryState::default();
        let _: AccountAssetKey = AccountAssetKey::new("account-1", "USDT");
        let _: SpotAssetPair = SpotAssetPair::new("BTC", "USDT");
        let _: WithdrawLockState = WithdrawLockState::default();
    }
}
