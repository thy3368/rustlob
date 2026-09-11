use std::sync::{Arc, Mutex};

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{StateSink, StateSource};
use example_core_use_case::{
    Balance, CancelSpotOrderV2LookupV3, SpotOrderExecution, SpotOrderSide, SpotOrderStatus,
    SpotOrderTimeInForce, SpotOrderV2, SpotOrderV2CommandV3, SpotOrderV2GivenStateV3,
    SpotOrderV2UseCaseFamilyV3,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("fake outbound error")]
pub struct FakeSpotOrderV2CancelOutboundError;

#[derive(Debug, Default)]
pub struct FakeSpotOrderV2CancelOutbound {
    observed_lookup: Arc<Mutex<Option<CancelSpotOrderV2LookupV3>>>,
}

impl FakeSpotOrderV2CancelOutbound {
    pub fn observed_lookup(&self) -> Option<CancelSpotOrderV2LookupV3> {
        self.observed_lookup.lock().map(|lookup| lookup.clone()).unwrap_or(None)
    }
}

impl StateSource<SpotOrderV2UseCaseFamilyV3> for FakeSpotOrderV2CancelOutbound {
    type Error = FakeSpotOrderV2CancelOutboundError;

    fn load_given_state(
        &self,
        cmd: &SpotOrderV2CommandV3,
    ) -> Result<SpotOrderV2GivenStateV3, Self::Error> {
        let SpotOrderV2CommandV3::Cancel(request) = cmd else {
            return Err(FakeSpotOrderV2CancelOutboundError);
        };
        *self.observed_lookup.lock().map_err(|_| FakeSpotOrderV2CancelOutboundError)? =
            Some(request.lookup.clone());

        let principal_reservation = SpotOrderV2::principal_reservation(
            "order-1",
            request.party_id.as_str(),
            SpotOrderSide::Buy,
            2,
            100,
            "BTC",
            "USDT",
        )
        .map_err(|_| FakeSpotOrderV2CancelOutboundError)?;
        let fee_reservation = SpotOrderV2::fee_reservation(
            "order-1",
            request.party_id.as_str(),
            SpotOrderSide::Buy,
            2,
            100,
            "USDT",
            5,
            10,
        )
        .map_err(|_| FakeSpotOrderV2CancelOutboundError)?;
        let order = SpotOrderV2::new_with_fee_reservation(
            "order-1".to_string(),
            request.asset,
            Some(77738308),
            request.party_id.clone(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 100 },
            SpotOrderTimeInForce::Gtc,
            2,
            0,
            SpotOrderStatus::Open,
            None,
            principal_reservation,
            fee_reservation,
            None,
            1,
        );

        Ok(SpotOrderV2GivenStateV3::Cancel {
            balances: vec![Balance::new(
                request.party_id.clone(),
                "USDT".to_string(),
                1000,
                201,
                1,
            )],
            order,
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        })
    }
}

impl StateSink<SpotOrderV2UseCaseFamilyV3> for FakeSpotOrderV2CancelOutbound {
    type Error = FakeSpotOrderV2CancelOutboundError;

    fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}
