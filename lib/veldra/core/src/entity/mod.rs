mod block;
mod exchange;
mod support;

pub use block::NewBlock;
pub use exchange::{
    AccountAssetKey, AccountMarketKey, CommandEnvelope, CommandExecutionResult, ExchangeState,
    PerpCommand, PerpCommandResult, PerpState, ProductCommand, ProductCommandResult, SpotAssetPair,
    SpotCancelExecution, SpotCommand, SpotCommandResult, SpotPipelineExecution, SpotState,
    TreasuryBalanceUpdate, TreasuryCommand, TreasuryCommandResult, TreasuryState,
    WithdrawLockState, build_new_block,
};
pub(crate) use support::stable_hash_hex;

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
