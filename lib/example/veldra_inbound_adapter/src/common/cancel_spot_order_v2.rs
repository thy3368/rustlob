use cmd_handler::command_use_case_def2::{
    MiFamilyExecutionError, ExecutionResult, StateSink, StateSource,
    StateMachineExecutor,
};
use example_core_use_case::{
    SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3, SpotOrderV2UseCaseFamilyV3,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2CancelOutbound, DefaultSpotOrderV2CancelOutboundError,
};

pub fn execute_cancel_spot_order_v2(
    command: &SpotOrderV2CommandV3,
) -> Result<
    ExecutionResult<SpotOrderV2CaseChangesV3>,
    MiFamilyExecutionError<
        example_core_use_case::SpotOrderV2UseCaseFamilyV3Error,
        DefaultSpotOrderV2CancelOutboundError,
    >,
> {
    execute_cancel_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2CancelOutbound)
}

pub(crate) fn execute_cancel_spot_order_v2_with_outbound<OB>(
    command: &SpotOrderV2CommandV3,
    outbound: &OB,
) -> Result<
    ExecutionResult<SpotOrderV2CaseChangesV3>,
    MiFamilyExecutionError<
        example_core_use_case::SpotOrderV2UseCaseFamilyV3Error,
        <OB as StateSink<SpotOrderV2UseCaseFamilyV3>>::Error,
    >,
>
where
    OB: StateSource<
            SpotOrderV2UseCaseFamilyV3,
            Error = <OB as StateSink<SpotOrderV2UseCaseFamilyV3>>::Error,
        > + StateSink<SpotOrderV2UseCaseFamilyV3>,
{
    StateMachineExecutor.execute::<SpotOrderV2UseCaseFamilyV3, OB, OB>(
        &SpotOrderV2UseCaseFamilyV3,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use example_core_use_case::{
        CancelSpotOrderV2CmdV3, CancelSpotOrderV2LookupV3, SpotOrderStatus, SpotOrderV2CommandV3,
    };
    use example_outbound_adapter::FakeSpotOrderV2CancelOutbound;
    use rstest::rstest;

    use super::*;

    // Matrix: OID/CLOID lookup + default state not wired -> load-state failure.
    #[rstest]
    #[case::oid(CancelSpotOrderV2LookupV3::Oid(77738308))]
    #[case::cloid(CancelSpotOrderV2LookupV3::Cloid("client-order-1".to_string()))]
    fn execute_cancel_with_default_outbound_stops_at_load_state(
        #[case] lookup: CancelSpotOrderV2LookupV3,
    ) {
        let command = SpotOrderV2CommandV3::Cancel(CancelSpotOrderV2CmdV3 {
            party_id: "buyer".to_string(),
            asset: 10000,
            lookup,
        });

        let result = execute_cancel_spot_order_v2(&command);

        assert_eq!(
            result,
            Err(MiFamilyExecutionError::LoadState(
                DefaultSpotOrderV2CancelOutboundError::StateUnavailable,
            )),
        );
    }

    // Matrix: OID lookup + open order + releasable reservation -> canceled order and update events.
    #[rstest]
    fn execute_cancel_open_order_releases_reservation_and_projects_events() {
        // Rule: an accepted cancel command must cancel the order and release its remaining funds.
        // Given: the fake outbound loads an open buy order with a frozen USDT balance.
        // When: the shared cancel executor runs the command.
        // Then: Changes contain the before/after business truth and replay events contain updates.
        let command = SpotOrderV2CommandV3::Cancel(CancelSpotOrderV2CmdV3 {
            party_id: "buyer".to_string(),
            asset: 10000,
            lookup: CancelSpotOrderV2LookupV3::Oid(77738308),
        });
        let outbound = FakeSpotOrderV2CancelOutbound::default();

        let result = execute_cancel_spot_order_v2_with_outbound(&command, &outbound)
            .expect("open order cancellation should execute");

        let SpotOrderV2CaseChangesV3::Cancel(changes) = result.changes else {
            panic!("expected cancel changes");
        };
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
