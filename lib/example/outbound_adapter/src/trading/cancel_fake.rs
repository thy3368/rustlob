use std::sync::{Arc, Mutex};

use common_entity::{EntityReplayableEvent, StateSink, StateSource, stable_string_entity_id};
use example_core_use_case::{
    Balance, CancelSpotOrderV2Cmd, CancelSpotOrderV2Lookup, CancelSpotOrderV2State,
    CancelSpotOrderV2UseCase, OrderSide, OrderTif, OrderType, SpotOrderV2,
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
        request: &LogEntry<CancelSpotOrderV2Cmd>,
    ) -> Result<CancelSpotOrderV2State, Self::Error> {
        *self.observed_lookup.lock().map_err(|_| FakeSpotOrderV2CancelOutboundError)? =
            Some(request.lookup.clone());

        let mut order = SpotOrderV2::new_pending_limit(
            1,
            request.asset,
            request.party_id.clone(),
            "BTCUSDT".to_string(),
            OrderSide::Buy,
            2,
            100,
            OrderType::Limit { tif: OrderTif::Gtc },
            Some("client-order-1".to_string()),
            0,
            1,
        );

        order.status = example_core_use_case::SpotOrderStatus::Open;
        order.version = 1;
        order.updated_at = 1;

        Ok(CancelSpotOrderV2State {
            balances: vec![Balance::new(
                stable_string_entity_id(request.party_id.as_str()),
                stable_string_entity_id(request.party_id.as_str()),
                stable_string_entity_id("USDT"),
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
use common_entity::LogEntry;
