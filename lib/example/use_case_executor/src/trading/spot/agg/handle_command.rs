use cmd_handler::command_use_case_def2::{ExecutionError, ExecutionResult};
use common_entity::LogEntry;
use example_core_use_case::{
    CancelSpotOrderV2Changes, CancelSpotOrderV2Cmd, CancelSpotOrderV2Error,
    ModifySpotOrderV2Changes, ModifySpotOrderV2Cmd, ModifySpotOrderV2Error,
    PlaceMatchSpotOrderV2Changes, PlaceMatchSpotOrderV2Error, PlaceOnlySpotOrderV2Cmd,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2CancelOutboundError, DefaultSpotOrderV2ModifyOutboundError,
    DefaultSpotOrderV2PlaceOutboundError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotBlockCommand {
    Cancel(CancelSpotOrderV2Cmd),
    Modify(ModifySpotOrderV2Cmd),
    PlaceMatch(PlaceOnlySpotOrderV2Cmd),
}

/// 一条现货命令对应的 executor 结果，保留各 use case 的具体变化和错误类型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotBlockCommandResult {
    Cancel(
        Result<
            ExecutionResult<CancelSpotOrderV2Changes>,
            ExecutionError<CancelSpotOrderV2Error, DefaultSpotOrderV2CancelOutboundError>,
        >,
    ),
    Modify(
        Result<
            ExecutionResult<ModifySpotOrderV2Changes>,
            ExecutionError<ModifySpotOrderV2Error, DefaultSpotOrderV2ModifyOutboundError>,
        >,
    ),
    PlaceMatch(
        Result<
            ExecutionResult<PlaceMatchSpotOrderV2Changes>,
            ExecutionError<PlaceMatchSpotOrderV2Error, DefaultSpotOrderV2PlaceOutboundError>,
        >,
    ),
}

/// 按输入顺序逐条执行现货命令，并为每条命令保留独立结果。
pub fn handler(commands: Vec<LogEntry<SpotBlockCommand>>) -> Vec<SpotBlockCommandResult> {
    commands
        .into_iter()
        .map(|entry| match entry.payload.clone() {
            SpotBlockCommand::Cancel(command) => SpotBlockCommandResult::Cancel(
                super::super::cancel_spot_order_v2_executor::execute_cancel_spot_order_v2(
                    &entry.with_payload(command),
                ),
            ),
            SpotBlockCommand::Modify(command) => SpotBlockCommandResult::Modify(
                super::super::modify_spot_order_v2_executor::execute_modify_spot_order_v2(
                    &entry.with_payload(command),
                ),
            ),
            SpotBlockCommand::PlaceMatch(command) => SpotBlockCommandResult::PlaceMatch(
                super::super::place_match_spot_order_v2_executor::execute_place_match_spot_order_v2(
                    &entry.with_payload(command),
                ),
            ),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::ExecutionError;
    use common_entity::LogEntry;
    use example_core_use_case::{
        CancelSpotOrderV2Lookup, ModifySpotOrderV2OrderType, OrderId, PlaceOnlySpotOrderV2OrderCmd,
        PlaceOnlySpotOrderV2OrderType,
    };
    use example_outbound_adapter::{
        DefaultSpotOrderV2CancelOutboundError, DefaultSpotOrderV2PlaceOutboundError,
    };

    use super::{SpotBlockCommand, SpotBlockCommandResult, handler};

    fn cancel_command() -> SpotBlockCommand {
        SpotBlockCommand::Cancel(super::CancelSpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            lookup: CancelSpotOrderV2Lookup::Oid(1),
        })
    }

    fn modify_command(party_id: &str) -> SpotBlockCommand {
        SpotBlockCommand::Modify(super::ModifySpotOrderV2Cmd {
            party_id: party_id.to_owned(),
            asset: 10_001,
            order_id: OrderId::Oid(1),
            is_buy: true,
            price: "100".to_owned(),
            size: "1".to_owned(),
            order_type: ModifySpotOrderV2OrderType::Limit { tif: "gtc".to_owned() },
            cloid: None,
        })
    }

    fn place_match_command() -> SpotBlockCommand {
        SpotBlockCommand::PlaceMatch(super::PlaceOnlySpotOrderV2Cmd::Single(
            PlaceOnlySpotOrderV2OrderCmd {
                party_id: "buyer".to_owned(),
                asset: 10_001,
                order_id: 1,
                symbol: "BTCUSDT".to_owned(),
                is_buy: true,
                price: "100".to_owned(),
                size: "1".to_owned(),
                order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "ioc".to_owned() },
                reduce_only: false,
                cloid: None,
                base_asset_id: "BTC".to_owned(),
                quote_asset_id: "USDT".to_owned(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
            },
        ))
    }

    #[test]
    fn handler_dispatches_each_command_and_preserves_input_order_after_failure() {
        let results = handler(
            vec![cancel_command(), modify_command(""), place_match_command()]
                .into_iter()
                .enumerate()
                .map(|(index, payload)| LogEntry {
                    lsn: index as u64 + 1,
                    logical_ts: index as u64 + 1,
                    client_id: 1,
                    request_id: index as u64 + 1,
                    payload,
                })
                .collect(),
        );

        assert_eq!(results.len(), 3);
        assert!(matches!(
            &results[0],
            SpotBlockCommandResult::Cancel(Err(ExecutionError::LoadState(
                DefaultSpotOrderV2CancelOutboundError::StateUnavailable
            )))
        ));
        assert!(matches!(
            &results[1],
            SpotBlockCommandResult::Modify(Err(ExecutionError::Business(
                super::ModifySpotOrderV2Error::EmptyPartyId
            )))
        ));
        assert!(matches!(
            &results[2],
            SpotBlockCommandResult::PlaceMatch(Err(ExecutionError::LoadState(
                DefaultSpotOrderV2PlaceOutboundError::StateUnavailable
            )))
        ));
    }

    #[test]
    fn handler_returns_empty_results_for_empty_input() {
        assert!(handler(Vec::new()).is_empty());
    }
}
