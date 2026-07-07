use crate::info::SUPPORTED_INFO_QUERY_TYPES;

#[test]
fn supported_queries_match_local_info_endpoint_spec_snapshot() {
    let expected = [
        "allMids",
        "openOrders",
        "frontendOpenOrders",
        "userFills",
        "userFillsByTime",
        "userRateLimit",
        "orderStatus",
        "l2Book",
        "candleSnapshot",
        "maxBuilderFee",
        "historicalOrders",
        "userTwapSliceFills",
        "subAccounts",
        "vaultDetails",
        "userVaultEquities",
        "userRole",
        "portfolio",
        "referral",
        "userFees",
        "delegations",
        "delegatorSummary",
        "delegatorHistory",
        "delegatorRewards",
        "userDexAbstraction",
        "userAbstraction",
        "alignedQuoteTokenInfo",
        "borrowLendUserState",
        "borrowLendReserveState",
        "allBorrowLendReserveStates",
        "approvedBuilders",
    ];

    assert_eq!(SUPPORTED_INFO_QUERY_TYPES, expected);
}
