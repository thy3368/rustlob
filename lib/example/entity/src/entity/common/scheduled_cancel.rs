use common_entity::{
    AggregateRole, Entity, EntityFieldChange, FieldDiff, FourColorArchetype, Shard,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SCHEDULED_CANCEL_ENTITY_TYPE: u8 = 19;
const UTC_DAY_MS: u64 = 86_400_000;
const MAX_DAILY_TRIGGERS: u8 = 10;

/// 账户级“死人开关”设置；到期执行撤单由独立用例消费。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledCancel {
    user_id: u64,
    cancel_at_ms: Option<u64>,
    trigger_day_utc: Option<u64>,
    trigger_count: u8,
    version: u64,
    updated_at_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ScheduledCancelError {
    #[error("scheduled cancel version overflow")]
    VersionOverflow,
    #[error("scheduled cancel is not armed")]
    NotArmed,
    #[error("scheduled cancel deadline has not arrived")]
    NotDue,
    #[error("daily scheduled cancel trigger limit reached")]
    DailyTriggerLimitReached,
    #[error("scheduled cancel trigger count overflow")]
    TriggerCountOverflow,
}

impl ScheduledCancel {
    pub const fn new(user_id: u64) -> Self {
        Self {
            user_id,
            cancel_at_ms: None,
            trigger_day_utc: None,
            trigger_count: 0,
            version: 0,
            updated_at_ms: 0,
        }
    }

    pub const fn user_id(&self) -> u64 {
        self.user_id
    }

    pub const fn cancel_at_ms(&self) -> Option<u64> {
        self.cancel_at_ms
    }

    pub const fn version(&self) -> u64 {
        self.version
    }

    pub const fn trigger_day_utc(&self) -> Option<u64> {
        self.trigger_day_utc
    }

    pub const fn trigger_count(&self) -> u8 {
        self.trigger_count
    }

    pub fn trigger_count_at(&self, now_ms: u64) -> u8 {
        if self.trigger_day_utc == Some(now_ms / UTC_DAY_MS) { self.trigger_count } else { 0 }
    }

    pub const fn updated_at_ms(&self) -> u64 {
        self.updated_at_ms
    }

    pub fn replace(
        &mut self,
        cancel_at_ms: Option<u64>,
        updated_at_ms: u64,
    ) -> Result<Option<u64>, ScheduledCancelError> {
        let previous = self.cancel_at_ms;
        let version = self.version.checked_add(1).ok_or(ScheduledCancelError::VersionOverflow)?;
        self.cancel_at_ms = cancel_at_ms;
        self.version = version;
        self.updated_at_ms = updated_at_ms;
        Ok(previous)
    }

    /// 到期触发一次死人开关，并按 UTC 自然日维护触发次数。
    pub fn trigger(&mut self, now_ms: u64) -> Result<u8, ScheduledCancelError> {
        let deadline = self.cancel_at_ms.ok_or(ScheduledCancelError::NotArmed)?;
        if now_ms < deadline {
            return Err(ScheduledCancelError::NotDue);
        }
        let day = now_ms / UTC_DAY_MS;
        let count = if self.trigger_day_utc == Some(day) { self.trigger_count } else { 0 };
        if count >= MAX_DAILY_TRIGGERS {
            return Err(ScheduledCancelError::DailyTriggerLimitReached);
        }
        let next_count = count.checked_add(1).ok_or(ScheduledCancelError::TriggerCountOverflow)?;
        let version = self.version.checked_add(1).ok_or(ScheduledCancelError::VersionOverflow)?;
        self.cancel_at_ms = None;
        self.trigger_day_utc = Some(day);
        self.trigger_count = next_count;
        self.version = version;
        self.updated_at_ms = now_ms;
        Ok(next_count)
    }
}

impl FieldDiff for ScheduledCancel {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("user_id", "", self.user_id.to_string()),
            EntityFieldChange::new("cancel_at_ms", "", optional_ms(self.cancel_at_ms)),
            EntityFieldChange::new("trigger_day_utc", "", optional_ms(self.trigger_day_utc)),
            EntityFieldChange::new("trigger_count", "", self.trigger_count.to_string()),
            EntityFieldChange::new("updated_at_ms", "", self.updated_at_ms.to_string()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        [
            ("cancel_at_ms", optional_ms(self.cancel_at_ms), optional_ms(other.cancel_at_ms)),
            (
                "trigger_day_utc",
                optional_ms(self.trigger_day_utc),
                optional_ms(other.trigger_day_utc),
            ),
            ("trigger_count", self.trigger_count.to_string(), other.trigger_count.to_string()),
            ("updated_at_ms", self.updated_at_ms.to_string(), other.updated_at_ms.to_string()),
        ]
        .into_iter()
        .filter(|(_, before, after)| before != after)
        .map(|(name, before, after)| EntityFieldChange::new(name, before, after))
        .collect()
    }
}

fn optional_ms(value: Option<u64>) -> String {
    value.map_or_else(String::new, |value| value.to_string())
}

impl Shard for ScheduledCancel {
    fn partition(&self) -> u8 {
        (self.user_id % 256) as u8
    }
}

impl Entity for ScheduledCancel {
    fn entity_id(&self) -> u64 {
        self.user_id
    }

    fn is_derived() -> bool {
        false
    }

    fn entity_type() -> u8 {
        SCHEDULED_CANCEL_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }

    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }

    fn lsn(&self) -> u64 {
        self.version
    }

    fn replay_field_type(field_name: &str) -> u8 {
        if matches!(
            field_name,
            "cancel_at_ms" | "trigger_day_utc" | "trigger_count" | "updated_at_ms"
        ) {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_overflow_is_rejected_without_mutation() {
        let mut schedule = ScheduledCancel {
            user_id: 7,
            cancel_at_ms: Some(10_000),
            trigger_day_utc: None,
            trigger_count: 0,
            version: u64::MAX,
            updated_at_ms: 1_000,
        };
        let before = schedule.clone();

        assert_eq!(
            schedule.replace(Some(20_000), 2_000),
            Err(ScheduledCancelError::VersionOverflow)
        );
        assert_eq!(schedule, before);
    }

    #[test]
    fn trigger_clears_deadline_and_increments_same_utc_day_count() {
        let day_start = 20 * UTC_DAY_MS;
        let mut schedule = ScheduledCancel::new(7);
        schedule.replace(Some(day_start + 5_000), day_start).expect("arming should succeed");

        assert_eq!(schedule.trigger(day_start + 5_000), Ok(1));
        assert_eq!(schedule.cancel_at_ms(), None);
        assert_eq!(schedule.trigger_day_utc(), Some(20));
        assert_eq!(schedule.trigger_count(), 1);
    }

    #[test]
    fn trigger_rejects_unarmed_and_not_yet_due_without_mutation() {
        let mut unarmed = ScheduledCancel::new(7);
        assert_eq!(unarmed.trigger(1_000), Err(ScheduledCancelError::NotArmed));

        let mut armed = ScheduledCancel::new(7);
        armed.replace(Some(10_000), 1_000).expect("arming should succeed");
        let before = armed.clone();
        assert_eq!(armed.trigger(9_999), Err(ScheduledCancelError::NotDue));
        assert_eq!(armed, before);
    }

    #[test]
    fn trigger_count_resets_at_utc_day_boundary() {
        let mut schedule = ScheduledCancel::new(7);
        for count in 1..=MAX_DAILY_TRIGGERS {
            schedule.replace(Some(5_000), 0).expect("arming should succeed");
            assert_eq!(schedule.trigger(5_000), Ok(count));
        }
        schedule.replace(Some(6_000), 5_000).expect("arming should succeed");
        assert_eq!(schedule.trigger(6_000), Err(ScheduledCancelError::DailyTriggerLimitReached));

        let next_day = UTC_DAY_MS;
        assert_eq!(schedule.trigger(next_day), Ok(1));
        assert_eq!(schedule.trigger_day_utc(), Some(1));
    }
}
