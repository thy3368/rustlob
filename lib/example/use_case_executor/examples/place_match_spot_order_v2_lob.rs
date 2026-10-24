//! 运行：cargo run -p use_case_executor --example place_match_spot_order_v2_lob

use common_entity::stable_string_entity_id;
use example_core_use_case::{
    ActivatePendingSpotOrderV2Input, Balance, MarketRules, OrderSide, OrderTif, OrderType,
    PlaceMatchSpotOrderV2Changes, PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd,
    PlaceOnlySpotOrderV2OrderType, SpotOrderV2,
};
use example_outbound_adapter::{InMemoryPlaceOrderOutbound, InMemoryStore};
use use_case_executor::trading::spot::place_match_spot_order_v2_executor::execute_place_match_spot_order_v2_with_outbound;

const ASSET: u32 = 10_001;
const MAKER_ORDER_ID: u64 = 2;

fn outbound(with_maker: bool) -> Result<InMemoryPlaceOrderOutbound, Box<dyn std::error::Error>> {
    let store = InMemoryStore::new(ASSET);
    store.seed_market_rules(MarketRules { symbol: "BTCUSDT".into(), min_qty: 1 })?;
    for balance in [
        Balance::new(
            stable_string_entity_id("buyer"),
            stable_string_entity_id("buyer"),
            stable_string_entity_id("BTC"),
            0,
            0,
            1,
        ),
        Balance::new(
            stable_string_entity_id("buyer"),
            stable_string_entity_id("buyer"),
            stable_string_entity_id("USDT"),
            101,
            0,
            1,
        ),
        Balance::new(
            stable_string_entity_id("seller"),
            stable_string_entity_id("seller"),
            stable_string_entity_id("BTC"),
            0,
            1,
            1,
        ),
        Balance::new(
            stable_string_entity_id("seller"),
            stable_string_entity_id("seller"),
            stable_string_entity_id("USDT"),
            0,
            1,
            1,
        ),
    ] {
        store.seed_balance(balance)?;
    }

    if with_maker {
        let mut maker = SpotOrderV2::new_pending_limit(
            MAKER_ORDER_ID,
            ASSET,
            "seller".into(),
            "BTCUSDT".into(),
            OrderSide::Sell,
            1,
            100,
            OrderType::Limit { tif: OrderTif::Gtc },
            None,
            0,
            1,
        );
        maker.activate_pending(ActivatePendingSpotOrderV2Input {
            base_asset_id: "BTC".into(),
            quote_asset_id: "USDT".into(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
            timestamp: 1,
        })?;
        store.seed_order(maker)?;
    }
    Ok(InMemoryPlaceOrderOutbound::from_store(store))
}

fn command(asset: u32) -> PlaceOnlySpotOrderV2Cmd {
    PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
        party_id: "buyer".into(),
        asset,
        order_id: 1,
        symbol: "BTCUSDT".into(),
        is_buy: true,
        price: "100".into(),
        size: "1".into(),
        order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "ioc".into() },
        reduce_only: false,
        cloid: None,
        base_asset_id: "BTC".into(),
        quote_asset_id: "USDT".into(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let outbound = outbound(true)?;
    // executor -> outbound.load_given_state -> LocalLobBtree::maker_orders -> use case 撮合
    let result = execute_place_match_spot_order_v2_with_outbound(
        &common_entity::LogEntrySequencer::from_metadata(1, 1, 1, 1, command(ASSET)),
        &outbound,
    )
    .map_err(|error| format!("place-match failed: {error:?}"))?;
    let PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched { match_changes, .. } = result.changes
    else {
        return Err("expected a matched single order".into());
    };
    assert_eq!(match_changes.created_trades().len(), 1);
    assert_eq!(match_changes.created_trades()[0].maker_order_id, MAKER_ORDER_ID);
    println!("matched maker order {MAKER_ORDER_ID}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lob_has_no_match() -> Result<(), Box<dyn std::error::Error>> {
        let outbound = outbound(false)?;
        let result = execute_place_match_spot_order_v2_with_outbound(&command(ASSET), &outbound)
            .map_err(|error| format!("place-match failed: {error:?}"))?;
        match result.changes {
            PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched { match_changes, .. } => {
                assert!(match_changes.created_trades().is_empty());
            }
            _ => return Err("expected single order matching path".into()),
        }
        Ok(())
    }

    #[test]
    fn wrong_asset_is_rejected_by_lob() -> Result<(), Box<dyn std::error::Error>> {
        let outbound = outbound(true)?;
        let result = execute_place_match_spot_order_v2_with_outbound(&command(10_000), &outbound);
        assert!(result.is_err());
        Ok(())
    }
}
