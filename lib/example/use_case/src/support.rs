use common_entity::Shard;
use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
use example_core_entity::{BalanceV2, BalanceV2Command, BalanceV2Error};

pub(crate) fn transition_balance(
    balance: &BalanceV2,
    lsn: u64,
    logical_ts: u64,
    command: BalanceV2Command,
) -> Result<BalanceV2, BalanceV2Error> {
    let metadata = CommandMetadata::new(
        lsn,
        logical_ts,
        balance.topic(),
        balance.partition(),
        balance.party_id().to_string(),
    );
    FsmV2::transition(balance, SequencedCommand::new(metadata, command)).map(|(after, _, ())| after)
}

pub(crate) fn concat2(a: &str, b: &str) -> String {
    let mut out = String::with_capacity(a.len().saturating_add(b.len()));
    out.push_str(a);
    out.push_str(b);
    out
}

pub(crate) fn concat3(a: &str, b: &str, c: &str) -> String {
    let mut out = String::with_capacity(a.len().saturating_add(b.len()).saturating_add(c.len()));
    out.push_str(a);
    out.push_str(b);
    out.push_str(c);
    out
}

pub(crate) fn concat4(a: &str, b: &str, c: &str, d: &str) -> String {
    let mut out = String::with_capacity(
        a.len()
            .saturating_add(b.len())
            .saturating_add(c.len())
            .saturating_add(d.len()),
    );
    out.push_str(a);
    out.push_str(b);
    out.push_str(c);
    out.push_str(d);
    out
}
