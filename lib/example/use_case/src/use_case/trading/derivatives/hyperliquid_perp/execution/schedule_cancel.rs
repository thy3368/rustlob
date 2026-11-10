use common_entity::{
    Entity, EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::{ScheduledCancel, ScheduledCancelError, UserId};
use thiserror::Error;

const MIN_SCHEDULE_AHEAD_MS: u64 = 5_000;
const MAX_DAILY_TRIGGERS: u8 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleHyperliquidCancelCmd {
    pub user_id: UserId,
    /// `None` 清除当前计划，`Some` 设置或替换计划。
    pub cancel_at_ms: Option<u64>,
    pub now_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleHyperliquidCancelState {
    pub scheduled_cancel: ScheduledCancel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleHyperliquidCancelChanges {
    pub changed_schedule: UpdatedEntityPair<ScheduledCancel>,
    pub replaced_cancel_at_ms: Option<u64>,
}

impl ReplayableChanges for ScheduleHyperliquidCancelChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![
            self.changed_schedule.after.track_update_event_from(&self.changed_schedule.before)?,
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ScheduleHyperliquidCancelError {
    #[error("scheduled cancel must be at least five seconds in the future")]
    TooSoon,
    #[error("scheduled cancel timestamp overflow")]
    TimestampOverflow,
    #[error("scheduled cancel state belongs to a different user")]
    UserMismatch,
    #[error("daily scheduled cancel trigger limit reached")]
    DailyTriggerLimitReached,
    #[error(transparent)]
    Schedule(#[from] ScheduledCancelError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ScheduleHyperliquidCancelUseCase;

impl UseCase for ScheduleHyperliquidCancelUseCase {
    type Command = LogEntry<ScheduleHyperliquidCancelCmd>;
    type StateGiven = ScheduleHyperliquidCancelState;
    type Error = ScheduleHyperliquidCancelError;
    type StateChanged = ScheduleHyperliquidCancelChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if let Some(cancel_at_ms) = cmd.cancel_at_ms {
            let earliest = cmd
                .now_ms
                .checked_add(MIN_SCHEDULE_AHEAD_MS)
                .ok_or(ScheduleHyperliquidCancelError::TimestampOverflow)?;
            if cancel_at_ms < earliest {
                return Err(ScheduleHyperliquidCancelError::TooSoon);
            }
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.scheduled_cancel.user_id() != cmd.user_id.0 {
            return Err(ScheduleHyperliquidCancelError::UserMismatch);
        }
        if cmd.cancel_at_ms.is_some()
            && state.scheduled_cancel.trigger_count_at(cmd.now_ms) >= MAX_DAILY_TRIGGERS
        {
            return Err(ScheduleHyperliquidCancelError::DailyTriggerLimitReached);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let before = state.scheduled_cancel.clone();
        let mut after = before.clone();
        let replaced_cancel_at_ms = after.replace(cmd.cancel_at_ms, cmd.now_ms)?;
        Ok(ScheduleHyperliquidCancelChanges {
            changed_schedule: UpdatedEntityPair { before, after },
            replaced_cancel_at_ms,
        })
    }
}

impl StateDiff for ScheduleHyperliquidCancelUseCase {
    type StateDiff = ScheduleHyperliquidCancelChanges;

    fn do_compute_state_diff(
        _state_given: Self::StateGiven,
        state_changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(state_changed)
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntrySequencer, ReplayableChanges};
    use rstest::rstest;

    use super::*;

    const NOW_MS: u64 = 1_700_000_000_000;

    fn command(cancel_at_ms: Option<u64>) -> LogEntry<ScheduleHyperliquidCancelCmd> {
        LogEntrySequencer::from_metadata(
            1,
            NOW_MS * 1_000_000,
            7,
            1,
            ScheduleHyperliquidCancelCmd { user_id: UserId(7), cancel_at_ms, now_ms: NOW_MS },
        )
    }

    fn state() -> ScheduleHyperliquidCancelState {
        ScheduleHyperliquidCancelState { scheduled_cancel: ScheduledCancel::new(7) }
    }

    fn state_at_daily_limit() -> ScheduleHyperliquidCancelState {
        let mut scheduled_cancel = ScheduledCancel::new(7);
        for _ in 0..10 {
            scheduled_cancel.replace(Some(NOW_MS), NOW_MS).expect("fixture should arm");
            scheduled_cancel.trigger(NOW_MS).expect("fixture should trigger");
        }
        ScheduleHyperliquidCancelState { scheduled_cancel }
    }

    // competitor: hyperliquid; feature: scheduleCancel; state: GREEN; target: use_case
    #[rstest]
    #[case(NOW_MS + 5_000)]
    #[case(NOW_MS + 60_000)]
    fn schedules_at_or_beyond_five_second_boundary(#[case] cancel_at_ms: u64) {
        let changes = ScheduleHyperliquidCancelUseCase
            .compute_state_changed(&command(Some(cancel_at_ms)), &state())
            .expect("至少提前五秒的计划撤单应成功");

        assert_eq!(changes.changed_schedule.after.cancel_at_ms(), Some(cancel_at_ms));
        assert_eq!(changes.changed_schedule.after.version(), 1);
        assert_eq!(changes.to_replayable_events().expect("应生成回放事件").len(), 1);
    }

    #[test]
    fn rejects_a_schedule_less_than_five_seconds_ahead() {
        assert_eq!(
            ScheduleHyperliquidCancelUseCase
                .compute_state_changed(&command(Some(NOW_MS + 4_999)), &state()),
            Err(ScheduleHyperliquidCancelError::TooSoon)
        );
    }

    #[test]
    fn rejects_new_schedule_after_ten_triggers_but_allows_clearing() {
        assert_eq!(
            ScheduleHyperliquidCancelUseCase
                .compute_state_changed(&command(Some(NOW_MS + 5_000)), &state_at_daily_limit()),
            Err(ScheduleHyperliquidCancelError::DailyTriggerLimitReached)
        );
        let cleared = ScheduleHyperliquidCancelUseCase
            .compute_state_changed(&command(None), &state_at_daily_limit())
            .expect("达到日上限后仍应允许清除死人开关");
        assert_eq!(cleared.changed_schedule.after.cancel_at_ms(), None);
    }

    #[test]
    fn replacing_schedule_reports_previous_deadline() {
        let mut existing = ScheduledCancel::new(7);
        existing
            .replace(Some(NOW_MS + 10_000), NOW_MS - 1_000)
            .expect("fixture schedule should be valid");
        let state = ScheduleHyperliquidCancelState { scheduled_cancel: existing };

        let changes = ScheduleHyperliquidCancelUseCase
            .compute_state_changed(&command(Some(NOW_MS + 20_000)), &state)
            .expect("替换现有计划应成功");

        assert_eq!(changes.replaced_cancel_at_ms, Some(NOW_MS + 10_000));
        assert_eq!(changes.changed_schedule.after.cancel_at_ms(), Some(NOW_MS + 20_000));
    }
}
