use std::sync::{Arc, Mutex};

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{StateSink, StateSource};
use example_core_use_case::{
    Balance, CancelSpotOrderV2Cmd, CancelSpotOrderV2Lookup, CancelSpotOrderV2State,
    CancelSpotOrderV2UseCase, SpotOrderSide, SpotOrderTif, SpotOrderType, SpotOrderV2,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("fake outbound error")]
pub struct FakeSpotOrderV2CancelOutboundError;

#[derive(Debug, Default)]
pub struct FakeSpotOrderV2CancelOutbound {
    observed_lookup: Arc<Mutex<Option<CancelSpotOrderV2Lookup>>>,
}

impl FakeSpotOrderV2CancelOutbound {
    pub fn observed_lookup(&self) -> Option<CancelSpotOrderV2Lookup> {
        self.observed_lookup.lock().map(|lookup| lookup.clone()).unwrap_or(None)
    }
}

impl StateSource<CancelSpotOrderV2UseCase> for FakeSpotOrderV2CancelOutbound {
    type Error = FakeSpotOrderV2CancelOutboundError;

    fn load_given_state(
        &self,
        request: &CancelSpotOrderV2Cmd,
    ) -> Result<CancelSpotOrderV2State, Self::Error> {
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
        let mut order = SpotOrderV2::new_pending_limit(
            "order-1".to_string(),
            request.asset,
            request.party_id.clone(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            2,
            100,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            Some("client-order-1".to_string()),
            0,
            1,
        );
        order.reservation = principal_reservation;
        order.fee_reservation = fee_reservation;
        order.status = example_core_use_case::SpotOrderStatus::Open;
        order.version = 1;
        order.updated_at = 1;

        Ok(CancelSpotOrderV2State {
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

impl StateSink<CancelSpotOrderV2UseCase> for FakeSpotOrderV2CancelOutbound {
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
