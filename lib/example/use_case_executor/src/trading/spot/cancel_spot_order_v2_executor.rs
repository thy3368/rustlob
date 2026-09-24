use cmd_handler::command_use_case_def2::{
    ExecutionError, ExecutionResult, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    CancelSpotOrderV2Changes, CancelSpotOrderV2Cmd, CancelSpotOrderV2Error,
    CancelSpotOrderV2UseCase,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2CancelOutbound, DefaultSpotOrderV2CancelOutboundError,
};

pub fn execute_cancel_spot_order_v2(
    command: &CancelSpotOrderV2Cmd,
) -> Result<
    ExecutionResult<CancelSpotOrderV2Changes>,
    ExecutionError<CancelSpotOrderV2Error, DefaultSpotOrderV2CancelOutboundError>,
> {
    execute_cancel_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2CancelOutbound)
}

pub fn execute_cancel_spot_order_v2_with_outbound<OB>(
    command: &CancelSpotOrderV2Cmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<CancelSpotOrderV2Changes>,
    ExecutionError<CancelSpotOrderV2Error, <OB as StateSink<CancelSpotOrderV2UseCase>>::Error>,
>
where
    OB: StateSource<
            CancelSpotOrderV2UseCase,
            Error = <OB as StateSink<CancelSpotOrderV2UseCase>>::Error,
        > + StateSink<CancelSpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<CancelSpotOrderV2UseCase, OB, OB>(
        &CancelSpotOrderV2UseCase,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use example_core_use_case::{CancelSpotOrderV2Cmd, CancelSpotOrderV2Lookup, SpotOrderStatus};
    use example_outbound_adapter::FakeSpotOrderV2CancelOutbound;
    use rstest::rstest;

    use super::*;

    // Matrix: 本地 order_id / CLOID lookup + default state not wired -> load-state failure.
    #[rstest]
    #[case::order_id(CancelSpotOrderV2Lookup::Oid(1))]
    #[case::cloid(CancelSpotOrderV2Lookup::Cloid("client-order-1".to_string()))]
    fn execute_cancel_with_default_outbound_stops_at_load_state(
        #[case] lookup: CancelSpotOrderV2Lookup,
    ) {
        let command = CancelSpotOrderV2Cmd { party_id: "buyer".to_string(), asset: 10000, lookup };

        let result = execute_cancel_spot_order_v2(&command);

        assert_eq!(
            result,
            Err(
                ExecutionError::LoadState(DefaultSpotOrderV2CancelOutboundError::StateUnavailable,)
            ),
        );
    }

    // Matrix: 本地 order_id lookup + open order + releasable reservation -> canceled order and update events.
    #[rstest]
    fn execute_cancel_open_order_releases_reservation_and_projects_events() {
        // Rule: an accepted cancel command must cancel the order and release its remaining funds.
        // Given: the fake outbound loads an open buy order with a frozen USDT balance.
        // When: the shared cancel executor runs the command.
        // Then: Changes contain the before/after business truth and replay events contain updates.
        let command = CancelSpotOrderV2Cmd {
            party_id: "buyer".to_string(),
            asset: 10000,
            lookup: CancelSpotOrderV2Lookup::Oid(1),
        };
        let outbound = FakeSpotOrderV2CancelOutbound::default();

        let result = execute_cancel_spot_order_v2_with_outbound(&command, &outbound)
            .expect("open order cancellation should execute");

        let changes = result.changes;
        assert_eq!(changes.updated_order.before.status(), SpotOrderStatus::Open);
        assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Canceled);
        assert_eq!(changes.updated_order.after.reservation.remaining_amount, 0);
        assert_eq!(changes.updated_order.after.fee_reservation.remaining_amount, 0);
        assert!(!changes.updated_balances.is_empty());
        assert!(changes.updated_balances.iter().all(|pair| pair.after.frozen < pair.before.frozen));
        assert!(!changes.created_balance_ledger_entries.is_empty());

        assert!(!result.events.is_empty());
        let updated_events =
            result.events.iter().filter(|event| event.is_updated()).collect::<Vec<_>>();
        assert!(!updated_events.is_empty());
        assert!(
            updated_events.iter().all(|event| event.new_version == event.old_version + 1
                && !event.field_changes.is_empty())
        );
    }
}
