use crate::{EntityError, LogEntry, ReplayableDomainEvent, TrackDiff};

/// FSM v2：只接受已定序 command envelope 的单一路径。
///
/// 该契约独立定义完整执行路径。实现方必须提供 v2 自己的无提交业务迁移入口；
/// 新调用方显式传入 [`LogEntry`]。
pub trait Fsm: TrackDiff + Sized {
    type Command;
    type Error: From<EntityError>;

    /// 基于当前实体只读推导迁移后的完整实体。
    ///
    /// 实现不得修改当前实体、发布事件或产生外部副作用。
    #[doc(hidden)]
    fn derive_transition(&self, command: LogEntry<Self::Command>) -> Result<Self, Self::Error>;

    /// 返回 command 对应的稳定、非零业务事件码。
    fn event_type(command: &Self::Command) -> u8;

    /// 先纯计算 after entity，再完成 replay 投影；原实体始终保持不变。
    ///
    /// replay event 的 `topic` 和 `partition` 以已定序 command metadata 为权威来源，
    /// 不使用实体自身的 shard 覆盖日志路由。
    fn transition(
        &self,
        command: LogEntry<Self::Command>,
    ) -> Result<(Self, ReplayableDomainEvent), Self::Error> {
        let event_type = Self::event_type(command.payload());
        if event_type == 0 {
            return Err(EntityError::Custom("FSM event type must be non-zero".into()).into());
        }
        let logical_ts = command.logical_ts();
        let lsn = command.lsn();
        let command_index = command.command_index();
        let topic = command.topic();
        let partition = command.partition();
        let after = self.derive_transition(command)?;
        let mut replay_event = after.track_update_event_from_at(
            self,
            self.lsn(),
            self.command_index(),
            lsn,
            command_index,
            logical_ts,
        )?;
        replay_event.t = topic;
        replay_event.p = partition;
        replay_event.domain_event_type = event_type;

        Ok((after, replay_event))
    }

    fn transition_v2(&self, command: LogEntry<Self::Command>) -> Result<Self, Self::Error> {
        let event_type = Self::event_type(command.payload());
        if event_type == 0 {
            return Err(EntityError::Custom("FSM event type must be non-zero".into()).into());
        }
        let logical_ts = command.logical_ts();
        let lsn = command.lsn();
        let command_index = command.command_index();
        let topic = command.topic();
        let partition = command.partition();
        let mut after = self.derive_transition(command)?;
        let mut replay_event = after.track_update_event_from_at(
            self,
            self.lsn(),
            self.command_index(),
            lsn,
            command_index,
            logical_ts,
        )?;
        replay_event.t = topic;
        replay_event.p = partition;
        replay_event.domain_event_type = event_type;

        after.append_event(replay_event);

        Ok(after)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Entity, EntityFieldChange, LogEntrySequencer, Shard};

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEntity {
        value: u64,
        lsn: u64,
        command_index: u32,
        logical_ts: u64,
        replay_events: Vec<ReplayableDomainEvent>,
    }

    impl Shard for TestEntity {
        fn topic(&self) -> u8 {
            1
        }

        fn partition(&self) -> u8 {
            2
        }

        fn party_id(&self) -> String {
            "entity-party".to_owned()
        }
    }

    impl Entity for TestEntity {
        fn append_event(&mut self, event: ReplayableDomainEvent) {
            self.replay_events.push(event);
        }

        fn replay_events(&self) -> &[ReplayableDomainEvent] {
            &self.replay_events
        }

        fn entity_id(&self) -> u64 {
            11
        }

        fn owner_id(&self) -> String {
            "owner".to_owned()
        }

        fn op_id(&self) -> String {
            "operator".to_owned()
        }

        fn lsn(&self) -> u64 {
            self.lsn
        }

        fn command_index(&self) -> u32 {
            self.command_index
        }

        fn created_at(&self) -> u64 {
            3
        }

        fn updated_at(&self) -> u64 {
            self.logical_ts
        }

        fn is_derived() -> bool {
            false
        }

        fn entity_type() -> u8 {
            12
        }
    }

    impl TrackDiff for TestEntity {
        fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
            (self.value != other.value)
                .then(|| {
                    EntityFieldChange::new("value", self.value.to_string(), other.value.to_string())
                })
                .into_iter()
                .collect()
        }
    }

    impl Fsm for TestEntity {
        type Command = u64;
        type Error = EntityError;

        fn derive_transition(&self, command: LogEntry<Self::Command>) -> Result<Self, Self::Error> {
            if *command.payload() == u64::MAX {
                return Err(EntityError::Custom("derive transition failed".into()));
            }
            let mut after = self.clone();
            after.value = *command.payload();
            after.lsn = command.lsn();
            after.command_index = command.command_index();
            after.logical_ts = command.logical_ts();
            Ok(after)
        }

        fn event_type(command: &Self::Command) -> u8 {
            if *command == 0 { 0 } else { 9 }
        }
    }

    fn entity(value: u64, lsn: u64, command_index: u32, logical_ts: u64) -> TestEntity {
        TestEntity {
            value,
            lsn,
            command_index,
            logical_ts,
            replay_events: Vec::new(),
        }
    }

    fn command(value: u64, lsn: u64, command_index: u32, logical_ts: u64) -> LogEntry<u64> {
        LogEntrySequencer::from_metadata_with_topic_at(
            8,
            6,
            lsn,
            command_index,
            logical_ts,
            0,
            0,
            "command-party".to_owned(),
            value,
        )
    }

    #[test]
    fn transition_uses_command_routing_and_preserves_replay_metadata() {
        let before = entity(5, 20, 4, 30);
        let command = command(6, 21, 7, 31);

        let (after, event) = before
            .transition(command)
            .expect("FSM transition should succeed");

        assert_eq!(after.value, 6);
        assert_eq!((event.t, event.p), (8, 6));
        assert_eq!((event.lsn, event.command_index), (21, 7));
        assert_eq!((event.previous_lsn, event.previous_command_index), (20, 4));
        assert_eq!(event.logical_ts, 31);
        assert_eq!(event.domain_event_type, 9);
        assert!(after.replay_events().is_empty());
        assert!(before.replay_events().is_empty());
    }

    #[test]
    fn transition_v2_appends_complete_event_without_changing_original() {
        let before = entity(5, 20, 4, 30);

        let after = before
            .transition_v2(command(6, 21, 7, 31))
            .expect("FSM transition should succeed");

        assert_eq!(after.value, 6);
        assert!(before.replay_events().is_empty());
        let [event] = after.replay_events() else {
            panic!("transition_v2 should append exactly one event");
        };
        assert_eq!((event.t, event.p), (8, 6));
        assert_eq!((event.lsn, event.command_index), (21, 7));
        assert_eq!((event.previous_lsn, event.previous_command_index), (20, 4));
        assert_eq!(event.logical_ts, 31);
        assert_eq!(event.domain_event_type, 9);
        assert_eq!(event.field_changes.len(), 1);
        assert_eq!(event.field_changes[0].field_name_as_str().unwrap(), "value");
        assert_eq!(event.field_changes[0].old_value_bytes(), b"5");
        assert_eq!(event.field_changes[0].new_value_bytes(), b"6");
    }

    #[test]
    fn transition_v2_appends_events_in_transition_order() {
        let first = entity(5, 20, 4, 30)
            .transition_v2(command(6, 21, 7, 31))
            .expect("first transition should succeed");
        let second = first
            .transition_v2(command(7, 22, 8, 32))
            .expect("second transition should succeed");

        assert_eq!(second.replay_events().len(), 2);
        assert_eq!(second.replay_events()[0].lsn, 21);
        assert_eq!(second.replay_events()[1].lsn, 22);
    }

    #[test]
    fn transition_v2_failures_do_not_append_events() {
        let before = entity(5, 20, 4, 30);

        assert!(before.clone().transition_v2(command(0, 21, 7, 31)).is_err());
        assert!(
            before
                .clone()
                .transition_v2(command(u64::MAX, 21, 7, 31))
                .is_err()
        );
        assert!(before.clone().transition_v2(command(5, 21, 7, 31)).is_err());
        assert!(before.replay_events().is_empty());
    }
}
