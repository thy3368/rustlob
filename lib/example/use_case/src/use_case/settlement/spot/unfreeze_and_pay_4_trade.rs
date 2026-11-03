use common_entity::{LogEntry, StateMachineV2};
use example_core_entity::spot::spot_order_primitives::OrderSide;
use example_core_entity::spot::spot_trade_v3::{
    SpotTradeSettlementError, SpotTradeSettlementStatus, SpotTradeV3,
};
use example_core_entity::{BalanceV2, BalanceV2Error, Open};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 现货成交结算在本次 use case 投影中的状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotSettlementState {
    Unsettled,
    Settled,
}

/// 结算状态机支持的命令。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotSettlementCommand {
    Settle,
    Inspect,
}

/// 四个余额变更 effect。金额由 `SpotTradeV3` 派生，中心只负责编排。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotSettlementEffect {
    DebitBuyerQuoteFrozen,
    CreditBuyerBaseAvailable,
    DebitSellerBaseFrozen,
    CreditSellerQuoteAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SpotSettlementCenterError {
    #[error("spot trade settlement was already completed")]
    DuplicateSettlement,
}

/// 现货成交结算的 tabular-center 投影。
///
/// 表格只决定状态转移和 effect 顺序，不计算金额，也不持有余额。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpotSettlementCenter {
    state: SpotSettlementState,
}

impl Default for SpotSettlementCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl SpotSettlementCenter {
    pub const fn new() -> Self {
        Self { state: SpotSettlementState::Unsettled }
    }

    pub const fn state(self) -> SpotSettlementState {
        self.state
    }

    pub fn dispatch(
        &mut self,
        command: SpotSettlementCommand,
    ) -> Result<&'static [SpotSettlementEffect], SpotSettlementCenterError> {
        const EFFECTS: [SpotSettlementEffect; 4] = [
            SpotSettlementEffect::DebitBuyerQuoteFrozen,
            SpotSettlementEffect::CreditBuyerBaseAvailable,
            SpotSettlementEffect::DebitSellerBaseFrozen,
            SpotSettlementEffect::CreditSellerQuoteAvailable,
        ];
        match (self.state, command) {
            (SpotSettlementState::Unsettled, SpotSettlementCommand::Settle) => {
                self.state = SpotSettlementState::Settled;
                Ok(&EFFECTS)
            }
            (SpotSettlementState::Unsettled, SpotSettlementCommand::Inspect)
            | (SpotSettlementState::Settled, SpotSettlementCommand::Inspect) => Ok(&[]),
            (SpotSettlementState::Settled, SpotSettlementCommand::Settle) => {
                Err(SpotSettlementCenterError::DuplicateSettlement)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UnFreezeSpotOrderV2Cmd {
    pub trade_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnFreezeSpotOrderV2State {
    pub trade: SpotTradeV3,
    pub buyer_base_balance: BalanceV2<Open>,
    pub buyer_quote_balance: BalanceV2<Open>,
    pub seller_base_balance: BalanceV2<Open>,
    pub seller_quote_balance: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnFreezeSpotOrderV2AfterChanges {
    pub trade_after: SpotTradeV3,
    pub buyer_base_balance_after: BalanceV2<Open>,
    pub buyer_quote_balance_after: BalanceV2<Open>,
    pub seller_base_balance_after: BalanceV2<Open>,
    pub seller_quote_balance_after: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UnFreezeSpotOrderV2Error {
    #[error("trade id must be greater than zero")]
    InvalidTradeId,
    #[error("command trade id does not match the loaded trade")]
    TradeIdMismatch,
    #[error("trade participants must be distinct")]
    SameParticipant,
    #[error("a participant's base and quote balances must belong to the same account")]
    AccountMismatch,
    #[error("balance party or asset does not match the trade")]
    BalanceIdentityMismatch,
    #[error("settlement balance identities must be distinct")]
    DuplicateBalance,
    #[error("settlement command was already applied")]
    DuplicateSettlement,
    #[error(transparent)]
    SettlementAmounts(#[from] SpotTradeSettlementError),
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UnFreeze4SpotOrderV2UseCase;

impl StateMachineV2 for UnFreeze4SpotOrderV2UseCase {
    type Command = LogEntry<UnFreezeSpotOrderV2Cmd>;
    type StateGiven = UnFreezeSpotOrderV2State;
    type Error = UnFreezeSpotOrderV2Error;
    type StateChanged = UnFreezeSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.trade_id == 0 {
            return Err(UnFreezeSpotOrderV2Error::InvalidTradeId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let trade = &given_state.trade.comon_trade;
        if given_state.trade.settlement_status == SpotTradeSettlementStatus::Settled {
            return Err(UnFreezeSpotOrderV2Error::DuplicateSettlement);
        }
        if cmd.trade_id != trade.trade_id {
            return Err(UnFreezeSpotOrderV2Error::TradeIdMismatch);
        }
        if trade.taker_user == trade.maker_user {
            return Err(UnFreezeSpotOrderV2Error::SameParticipant);
        }
        let base_asset = u64::from(trade.symbol.base.index());
        let quote_asset = u64::from(trade.symbol.quote.index());
        let (buyer_party, seller_party) = match trade.taker_side {
            OrderSide::Buy => (trade.taker_user.0, trade.maker_user.0),
            OrderSide::Sell => (trade.maker_user.0, trade.taker_user.0),
        };
        let balances = [
            (&given_state.buyer_base_balance, buyer_party, base_asset),
            (&given_state.buyer_quote_balance, buyer_party, quote_asset),
            (&given_state.seller_base_balance, seller_party, base_asset),
            (&given_state.seller_quote_balance, seller_party, quote_asset),
        ];
        for (balance, party, asset) in balances {
            if balance.party_id() != party || balance.coin_idx() != asset {
                return Err(UnFreezeSpotOrderV2Error::BalanceIdentityMismatch);
            }
        }
        if given_state.buyer_base_balance.account_id()
            != given_state.buyer_quote_balance.account_id()
            || given_state.seller_base_balance.account_id()
                != given_state.seller_quote_balance.account_id()
        {
            return Err(UnFreezeSpotOrderV2Error::AccountMismatch);
        }
        for left in 0..balances.len() {
            for right in (left + 1)..balances.len() {
                let (a, _, _) = balances[left];
                let (b, _, _) = balances[right];
                if a.account_id() == b.account_id() && a.coin_idx() == b.coin_idx() {
                    return Err(UnFreezeSpotOrderV2Error::DuplicateBalance);
                }
            }
        }
        given_state.trade.settlement_amounts()?;
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let amounts = given_state.trade.settlement_amounts()?;
        let mut trade_after = given_state.trade.clone();
        trade_after.mark_settled()?;
        let mut buyer_base_balance_after = given_state.buyer_base_balance.clone();
        let mut buyer_quote_balance_after = given_state.buyer_quote_balance.clone();
        let mut seller_base_balance_after = given_state.seller_base_balance.clone();
        let mut seller_quote_balance_after = given_state.seller_quote_balance.clone();

        buyer_quote_balance_after.debit_frozen(amounts.buyer_quote_payment, cmd.logical_ts())?;
        buyer_base_balance_after.credit_available(amounts.base_amount, cmd.logical_ts())?;
        seller_base_balance_after.debit_frozen(amounts.base_amount, cmd.logical_ts())?;
        if amounts.seller_quote_receipt > rust_decimal::Decimal::ZERO {
            seller_quote_balance_after.credit_available(amounts.seller_quote_receipt, cmd.logical_ts())?;
        }

        Ok(UnFreezeSpotOrderV2AfterChanges {
            trade_after,
            buyer_base_balance_after,
            buyer_quote_balance_after,
            seller_base_balance_after,
            seller_quote_balance_after,
        })
    }
}
