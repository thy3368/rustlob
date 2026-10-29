use common_entity::{LogEntry, StateMachineV2};
use example_core_entity::spot::spot_order_primitives::OrderSide;
use example_core_entity::spot::spot_trade_v3::{SpotTradeSettlementError, SpotTradeV3};
use example_core_entity::{BalanceV2, BalanceV2Error, Open};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
            if balance.party_id() != party || balance.asset_id() != asset {
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
                if a.account_id() == b.account_id() && a.asset_id() == b.asset_id() {
                    return Err(UnFreezeSpotOrderV2Error::DuplicateBalance);
                }
            }
        }
        given_state.trade.settlement_amounts()?;
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        _cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let amounts = given_state.trade.settlement_amounts()?;
        let mut buyer_base_balance_after = given_state.buyer_base_balance.clone();
        let mut buyer_quote_balance_after = given_state.buyer_quote_balance.clone();
        let mut seller_base_balance_after = given_state.seller_base_balance.clone();
        let mut seller_quote_balance_after = given_state.seller_quote_balance.clone();

        buyer_quote_balance_after.debit_frozen(amounts.buyer_quote_payment)?;
        buyer_base_balance_after.credit_available(amounts.base_amount)?;
        seller_base_balance_after.debit_frozen(amounts.base_amount)?;
        if amounts.seller_quote_receipt > rust_decimal::Decimal::ZERO {
            seller_quote_balance_after.credit_available(amounts.seller_quote_receipt)?;
        }

        Ok(UnFreezeSpotOrderV2AfterChanges {
            buyer_base_balance_after,
            buyer_quote_balance_after,
            seller_base_balance_after,
            seller_quote_balance_after,
        })
    }
}
