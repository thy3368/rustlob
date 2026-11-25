use common_entity::fsm_v2::Fsm as FsmV2;
use common_entity::{LogEntrySequencer, Shard};
use example_core_entity::{BalanceV2, BalanceV2Command, BalanceV2Error};

pub(crate) fn transition_balance(
    balance: &BalanceV2,
    lsn: u64,
    logical_ts: u64,
    command: BalanceV2Command,
) -> Result<BalanceV2, BalanceV2Error> {
    let command = LogEntrySequencer::from_metadata_with_topic(
        balance.topic(),
        balance.partition(),
        lsn,
        logical_ts,
        0,
        0,
        balance.party_id().to_string(),
        command,
    );
    FsmV2::transition(balance, command).map(|(after, _)| after)
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
