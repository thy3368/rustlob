use serde_json::{Value, json};

use crate::info::queries;

pub struct QueryContractCase {
    pub query_type: &'static str,
    pub request: Value,
    pub expected_response: Value,
}

fn addr1() -> &'static str {
    "0x0000000000000000000000000000000000000001"
}
fn addr2() -> &'static str {
    "0x0000000000000000000000000000000000000002"
}
fn vault_addr() -> &'static str {
    "0xdfc24b077bc1425ad1dea75bcb6f8158e10df303"
}
fn cloid() -> &'static str {
    "0x1234567890abcdef1234567890abcdef"
}

pub fn valid_all_mids_request_value() -> Value {
    json!({ "type": "allMids", "dex": "main" })
}

pub fn valid_order_status_with_cloid_value() -> Value {
    json!({ "type": "orderStatus", "user": addr1(), "oid": cloid() })
}

pub fn query_contract_cases() -> Vec<QueryContractCase> {
    vec![
        QueryContractCase {
            query_type: "allMids",
            request: valid_all_mids_request_value(),
            expected_response: serde_json::to_value(queries::all_mids::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "openOrders",
            request: json!({ "type": "openOrders", "user": addr1(), "dex": "main" }),
            expected_response: serde_json::to_value(queries::open_orders::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "frontendOpenOrders",
            request: json!({ "type": "frontendOpenOrders", "user": addr1(), "dex": "main" }),
            expected_response: serde_json::to_value(queries::frontend_open_orders::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "userFills",
            request: json!({ "type": "userFills", "user": addr1(), "aggregateByTime": true }),
            expected_response: serde_json::to_value(queries::user_fills::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "userFillsByTime",
            request: json!({ "type": "userFillsByTime", "user": addr1(), "startTime": 1710000000000u64, "endTime": 1710000001000u64, "aggregateByTime": true }),
            expected_response: serde_json::to_value(queries::user_fills_by_time::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "userRateLimit",
            request: json!({ "type": "userRateLimit", "user": addr1() }),
            expected_response: serde_json::to_value(queries::user_rate_limit::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "orderStatus",
            request: json!({ "type": "orderStatus", "user": addr1(), "oid": 1u64 }),
            expected_response: serde_json::to_value(queries::order_status::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "l2Book",
            request: json!({ "type": "l2Book", "coin": "BTC", "nSigFigs": 5, "mantissa": 1 }),
            expected_response: serde_json::to_value(queries::l2_book::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "candleSnapshot",
            request: json!({ "type": "candleSnapshot", "req": { "coin": "BTC", "interval": "15m", "startTime": 1681923600000u64, "endTime": 1681924499999u64 } }),
            expected_response: serde_json::to_value(queries::candle_snapshot::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "maxBuilderFee",
            request: json!({ "type": "maxBuilderFee", "user": addr1(), "builder": addr2() }),
            expected_response: serde_json::to_value(queries::max_builder_fee::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "historicalOrders",
            request: json!({ "type": "historicalOrders", "user": addr1() }),
            expected_response: serde_json::to_value(queries::historical_orders::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "userTwapSliceFills",
            request: json!({ "type": "userTwapSliceFills", "user": addr1() }),
            expected_response:
                serde_json::to_value(queries::user_twap_slice_fills::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "subAccounts",
            request: json!({ "type": "subAccounts", "user": addr1() }),
            expected_response: serde_json::to_value(queries::sub_accounts::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "vaultDetails",
            request: json!({ "type": "vaultDetails", "vaultAddress": vault_addr(), "user": addr1() }),
            expected_response: serde_json::to_value(queries::vault_details::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "userVaultEquities",
            request: json!({ "type": "userVaultEquities", "user": addr1() }),
            expected_response: serde_json::to_value(queries::user_vault_equities::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "userRole",
            request: json!({ "type": "userRole", "user": addr1() }),
            expected_response: serde_json::to_value(queries::user_role::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "portfolio",
            request: json!({ "type": "portfolio", "user": addr1() }),
            expected_response: serde_json::to_value(queries::portfolio::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "referral",
            request: json!({ "type": "referral", "user": addr1() }),
            expected_response: serde_json::to_value(queries::referral::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "userFees",
            request: json!({ "type": "userFees", "user": addr1() }),
            expected_response: serde_json::to_value(queries::user_fees::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "delegations",
            request: json!({ "type": "delegations", "user": addr1() }),
            expected_response: serde_json::to_value(queries::delegations::stub_response()).unwrap(),
        },
        QueryContractCase {
            query_type: "delegatorSummary",
            request: json!({ "type": "delegatorSummary", "user": addr1() }),
            expected_response: serde_json::to_value(queries::delegator_summary::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "delegatorHistory",
            request: json!({ "type": "delegatorHistory", "user": addr1() }),
            expected_response: serde_json::to_value(queries::delegator_history::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "delegatorRewards",
            request: json!({ "type": "delegatorRewards", "user": addr1() }),
            expected_response: serde_json::to_value(queries::delegator_rewards::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "userDexAbstraction",
            request: json!({ "type": "userDexAbstraction", "user": addr1() }),
            expected_response: serde_json::to_value(true).unwrap(),
        },
        QueryContractCase {
            query_type: "userAbstraction",
            request: json!({ "type": "userAbstraction", "user": addr1() }),
            expected_response: serde_json::to_value(queries::user_abstraction::stub_response())
                .unwrap(),
        },
        QueryContractCase {
            query_type: "alignedQuoteTokenInfo",
            request: json!({ "type": "alignedQuoteTokenInfo", "token": 0 }),
            expected_response: serde_json::to_value(
                queries::aligned_quote_token_info::stub_response(),
            )
            .unwrap(),
        },
        QueryContractCase {
            query_type: "borrowLendUserState",
            request: json!({ "type": "borrowLendUserState", "user": addr1() }),
            expected_response: serde_json::to_value(
                queries::borrow_lend_user_state::stub_response(),
            )
            .unwrap(),
        },
        QueryContractCase {
            query_type: "borrowLendReserveState",
            request: json!({ "type": "borrowLendReserveState", "token": 0 }),
            expected_response: serde_json::to_value(
                queries::borrow_lend_reserve_state::stub_response(),
            )
            .unwrap(),
        },
        QueryContractCase {
            query_type: "allBorrowLendReserveStates",
            request: json!({ "type": "allBorrowLendReserveStates" }),
            expected_response: serde_json::to_value(
                queries::all_borrow_lend_reserve_states::stub_response(),
            )
            .unwrap(),
        },
        QueryContractCase {
            query_type: "approvedBuilders",
            request: json!({ "type": "approvedBuilders", "user": addr1() }),
            expected_response: serde_json::to_value(queries::approved_builders::stub_response())
                .unwrap(),
        },
    ]
}
