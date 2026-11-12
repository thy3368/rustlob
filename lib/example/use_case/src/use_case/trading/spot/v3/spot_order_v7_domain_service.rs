use common_entity::{EntityError, Fsm};
use example_core_entity::spot::spot_order_v7::{
    ActivateSpotOrderV7Output, SpotOrderV7FreezeRequirement, SpotOrderV7Output,
};
use example_core_entity::{BalanceV2, BalanceV2Event, SpotOrderV7, SpotOrderV7Event};

use super::ActivateSpotOrderV7Error;

pub(crate) struct SpotOrderV7DomainService;

impl SpotOrderV7DomainService {
    pub(crate) fn freeze(
        mut order: SpotOrderV7,
        mut balance: BalanceV2,
        timestamp: u64,
    ) -> Result<(SpotOrderV7, BalanceV2, SpotOrderV7FreezeRequirement), ActivateSpotOrderV7Error>
    {
        let output = order.apply(SpotOrderV7Event::Activate { timestamp })?;
        let SpotOrderV7Output::Activated(ActivateSpotOrderV7Output { freeze_requirement }) = output
        else {
            return Err(
                EntityError::Custom("activate event produced mismatched output".into()).into()
            );
        };

        if balance.coin_idx() != u64::from(freeze_requirement.currency().index()) {
            return Err(ActivateSpotOrderV7Error::CoinMismatch);
        }
        balance.apply(BalanceV2Event::Freeze { amount: freeze_requirement.amount(), timestamp })?;

        Ok((order, balance, freeze_requirement))
    }
}
