use std::cell::RefCell;

use example_veldra_inbound_adapter::exchange::error::ExchangeHttpError;
use example_veldra_inbound_adapter::exchange::{
    BatchModifyCancelPlaceExecutor, CancelSpotOrderV2LookupV3, CancelSpotOrderV2Request,
    CancelStatusWire, OrderStatusWire, PlaceSpotOrderV2Request,
    run_batch_modify_cancel_replace_with_executor,
};
use rstest::rstest;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ObservedAction {
    Cancel(CancelSpotOrderV2Request),
    Place(PlaceSpotOrderV2Request),
}

#[derive(Default)]
struct ObservingCancelPlaceExecutor {
    observed: RefCell<Vec<ObservedAction>>,
    fail_cancel_for_oid: Option<u64>,
    fail_place_for_asset: Option<u32>,
}

impl BatchModifyCancelPlaceExecutor for ObservingCancelPlaceExecutor {
    fn cancel(
        &self,
        request: CancelSpotOrderV2Request,
    ) -> Result<CancelStatusWire, ExchangeHttpError> {
        if matches!(request.lookup, CancelSpotOrderV2LookupV3::Oid(oid) if Some(oid) == self.fail_cancel_for_oid)
        {
            return Err(ExchangeHttpError::contract("cancel rejected"));
        }
        self.observed.borrow_mut().push(ObservedAction::Cancel(request));
        Ok(CancelStatusWire::Success("success"))
    }

    fn place(
        &self,
        request: PlaceSpotOrderV2Request,
    ) -> Result<OrderStatusWire, ExchangeHttpError> {
        if Some(request.asset) == self.fail_place_for_asset {
            return Err(ExchangeHttpError::contract("place rejected"));
        }

        let oid = if request.asset == 10000 { 77738308 } else { 77_001 };
        self.observed.borrow_mut().push(ObservedAction::Place(request));
        Ok(OrderStatusWire::Resting {
            resting: example_veldra_inbound_adapter::exchange::RestingOrderStatusWire { oid },
        })
    }
}

#[rstest]
fn batch_modify_executes_cancel_then_place_for_each_entry_in_request_order() {
    let executor = ObservingCancelPlaceExecutor::default();

    let response = run_batch_modify_cancel_replace_with_executor(valid_request_json(), &executor)
        .expect("batchModify should execute cancel/place sequence");

    let observed = executor.observed.borrow();
    assert_eq!(observed.len(), 4);
    assert!(
        matches!(
            &observed[0],
            ObservedAction::Cancel(CancelSpotOrderV2Request {
                lookup: CancelSpotOrderV2LookupV3::Oid(77738308),
                ..
            })
        ),
        "first entry must cancel the oid before placing replacement"
    );
    assert!(
        matches!(
            &observed[1],
            ObservedAction::Place(PlaceSpotOrderV2Request {
                asset: 10000,
                is_buy: true,
                price,
                size,
                tif,
                ..
            }) if price == "1891.4" && size == "0.02" && tif == "gtc"
        ),
        "first entry replacement order must keep wire order fields"
    );
    assert!(
        matches!(
            &observed[2],
            ObservedAction::Cancel(CancelSpotOrderV2Request {
                lookup: CancelSpotOrderV2LookupV3::Cloid(cloid),
                ..
            }) if cloid == "0x1234567890abcdef1234567890abcdef"
        ),
        "second entry must cancel the cloid before placing replacement"
    );
    assert!(
        matches!(
            &observed[3],
            ObservedAction::Place(PlaceSpotOrderV2Request {
                asset: 10001,
                is_buy: false,
                price,
                size,
                tif,
                ..
            }) if price == "1890.0" && size == "0.04" && tif == "ioc"
        ),
        "second entry replacement order must keep wire order fields"
    );
    assert_eq!(
        response.response.data.statuses,
        vec![
            OrderStatusWire::Resting {
                resting: example_veldra_inbound_adapter::exchange::RestingOrderStatusWire {
                    oid: 77738308,
                },
            },
            OrderStatusWire::Resting {
                resting: example_veldra_inbound_adapter::exchange::RestingOrderStatusWire {
                    oid: 77_001,
                },
            },
        ]
    );
}

#[rstest]
#[case::cancel_failure(Some(77738308), None, "cancel rejected", false)]
#[case::place_failure(None, Some(10000), "place rejected", true)]
fn batch_modify_single_entry_failure_returns_error_and_continues_following_entries(
    #[case] fail_cancel_for_oid: Option<u64>,
    #[case] fail_place_for_asset: Option<u32>,
    #[case] expected_error: &str,
    #[case] first_entry_cancel_observed: bool,
) {
    let executor = ObservingCancelPlaceExecutor {
        fail_cancel_for_oid,
        fail_place_for_asset,
        ..Default::default()
    };

    let response = run_batch_modify_cancel_replace_with_executor(valid_request_json(), &executor)
        .expect("batchModify should keep the order response envelope");

    assert!(matches!(
        &response.response.data.statuses[0],
        OrderStatusWire::Error { error } if error == expected_error
    ));
    assert!(matches!(
        &response.response.data.statuses[1],
        OrderStatusWire::Resting { resting } if resting.oid == 77_001
    ));

    let observed = executor.observed.borrow();
    if first_entry_cancel_observed {
        assert_eq!(observed.len(), 3);
        assert!(matches!(
            &observed[0],
            ObservedAction::Cancel(CancelSpotOrderV2Request {
                lookup: CancelSpotOrderV2LookupV3::Oid(77738308),
                ..
            })
        ));
    } else {
        assert_eq!(observed.len(), 2);
    }

    let second_entry_offset = usize::from(first_entry_cancel_observed);
    assert!(matches!(
        &observed[second_entry_offset],
        ObservedAction::Cancel(CancelSpotOrderV2Request {
            lookup: CancelSpotOrderV2LookupV3::Cloid(cloid),
            ..
        }) if cloid == "0x1234567890abcdef1234567890abcdef"
    ));
    assert!(matches!(
        &observed[second_entry_offset + 1],
        ObservedAction::Place(PlaceSpotOrderV2Request { asset: 10001, .. })
    ));
}

fn valid_request_json() -> &'static [u8] {
    br#"{
        "action": {
            "type": "batchModify",
            "modifies": [
                {
                    "oid": 77738308,
                    "order": {
                        "a": 10000,
                        "b": true,
                        "p": "1891.4",
                        "s": "0.02",
                        "r": false,
                        "t": { "limit": { "tif": "Gtc" } }
                    }
                },
                {
                    "oid": "0x1234567890abcdef1234567890abcdef",
                    "order": {
                        "a": 10001,
                        "b": false,
                        "p": "1890.0",
                        "s": "0.04",
                        "r": false,
                        "t": { "limit": { "tif": "Ioc" } }
                    }
                }
            ]
        },
        "nonce": 1710000000000,
        "signature": {
            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
            "v": 27
        }
    }"#
}
