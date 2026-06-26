use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges,
};
use common_entity::MiStateMachine;
use serde::{Deserialize, Serialize};

use crate::entity::spot::spot_order_mi_state_machine::{
    CancelSpotOrderChanges as EntityCancelSpotOrderChanges,
    PlaceSpotOrderChanges as EntityPlaceSpotOrderChanges, PlaceSpotOrderSettlementInput,
};
use crate::entity::spot::{
    CancelSpotOrderCmd as EntityCancelSpotOrderCmd, PlaceSpotOrderCmd as EntityPlaceSpotOrderCmd,
    SpotOrderMiChanges, SpotOrderMiCommand, SpotOrderMiStateMachineError,
};
use crate::entity::{Balance, SpotOrder};

/// 现货订单下单 MI 状态机所需的已加载状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderState {
    /// 当前待推进的 taker 订单。
    pub order: SpotOrder,
}

/// 推进现货订单 Place 状态机的命令。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceSpotOrderCmd {
    /// 发起下单的交易账户 ID。
    pub party_id: String,
    /// 按撮合优先级排序的 maker 订单。
    pub makers: Option<Vec<SpotOrder>>,
    /// taker base 余额快照。
    pub taker_base_balance: Balance,
    /// taker quote 余额快照。
    pub taker_quote_balance: Balance,
    /// 同步成交清结算所需的 maker 余额快照；本次下单没有成交时不会使用。
    pub settlement_inputs: Option<Vec<PlaceSpotOrderSettlementInput>>,
}

impl IssuedByParty for PlaceSpotOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 下单状态机变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderChanges {
    /// 实体状态机产出的下单变化。
    pub inner: EntityPlaceSpotOrderChanges,
}

impl ReplayableChanges for PlaceSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        common_entity::ReplayableChanges::to_replayable_events(&self.inner)
    }
}

/// 推进现货订单 Place MI 状态机的 use case。
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceSpotOrderUseCase;

impl CommandUseCase4 for PlaceSpotOrderUseCase {
    type Command = PlaceSpotOrderCmd;
    type GivenState = PlaceSpotOrderState;
    type Error = SpotOrderMiStateMachineError;
    type Changes = PlaceSpotOrderChanges;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "party_id must not be empty",
            });
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.order.belongs_to_account(&cmd.party_id) {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "order does not belong to command party",
            });
        }
        state.order.pre_check_command(&SpotOrderMiCommand::Place(EntityPlaceSpotOrderCmd {
            makers: cmd.makers.clone(),
            taker_base_balance: cmd.taker_base_balance.clone(),
            taker_quote_balance: cmd.taker_quote_balance.clone(),
            settlement_inputs: cmd.settlement_inputs.clone(),
        }))?;
        state.order.validate_state_transition(&SpotOrderMiCommand::Place(EntityPlaceSpotOrderCmd {
            makers: cmd.makers.clone(),
            taker_base_balance: cmd.taker_base_balance.clone(),
            taker_quote_balance: cmd.taker_quote_balance.clone(),
            settlement_inputs: cmd.settlement_inputs.clone(),
        }))
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let changes =
            state.order.compute_changes(&SpotOrderMiCommand::Place(EntityPlaceSpotOrderCmd {
                makers: cmd.makers.clone(),
                taker_base_balance: cmd.taker_base_balance.clone(),
                taker_quote_balance: cmd.taker_quote_balance.clone(),
                settlement_inputs: cmd.settlement_inputs.clone(),
            }))?;
        match changes {
            SpotOrderMiChanges::Place(inner) => Ok(PlaceSpotOrderChanges { inner }),
            SpotOrderMiChanges::Cancel(_) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "place use case received cancel changes",
                })
            }
        }
    }
}

/// 现货订单撤单 MI 状态机所需的已加载状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderMiState {
    /// 当前待撤销的订单。
    pub order: SpotOrder,
}

/// 推进现货订单 Cancel 状态机的命令。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderMiCmd {
    /// 发起撤单的交易账户 ID。
    pub party_id: String,
}

impl IssuedByParty for CancelSpotOrderMiCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 撤单状态机变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderMiChanges {
    /// 实体状态机产出的撤单变化。
    pub inner: EntityCancelSpotOrderChanges,
}

impl ReplayableChanges for CancelSpotOrderMiChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        common_entity::ReplayableChanges::to_replayable_events(&self.inner)
    }
}

/// 推进现货订单 Cancel MI 状态机的 use case。
#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderMiUseCase;

impl CommandUseCase4 for CancelSpotOrderMiUseCase {
    type Command = CancelSpotOrderMiCmd;
    type GivenState = CancelSpotOrderMiState;
    type Error = SpotOrderMiStateMachineError;
    type Changes = CancelSpotOrderMiChanges;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "party_id must not be empty",
            });
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.order.belongs_to_account(&cmd.party_id) {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "order does not belong to command party",
            });
        }
        state.order.validate_state_transition(&SpotOrderMiCommand::Cancel(EntityCancelSpotOrderCmd))
    }

    fn compute_changes(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let changes =
            state.order.compute_changes(&SpotOrderMiCommand::Cancel(EntityCancelSpotOrderCmd))?;
        match changes {
            SpotOrderMiChanges::Cancel(inner) => Ok(CancelSpotOrderMiChanges { inner }),
            SpotOrderMiChanges::Place(_) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "cancel use case received place changes",
                })
            }
        }
    }
}
