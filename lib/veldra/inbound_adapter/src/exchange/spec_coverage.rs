use crate::exchange::actions::SUPPORTED_ACTION_TYPES;

#[test]
fn supported_actions_match_official_exchange_endpoint_spec() {
    // 基于官方 exchange-endpoint 页面在 2026-06-18 的 action.type 清单整理。
    // `topUpIsolatedOnlyMargin` 虽然不是独立章节，但在 updateIsolatedMargin 说明里被官方明确列为可用 action。
    let expected = [
        "agentEnableDexAbstraction",
        "agentSendAsset",
        "agentSetAbstraction",
        "approveAgent",
        "approveBuilderFee",
        "authorizeAqav2Role",
        "batchModify",
        "cDeposit",
        "cWithdraw",
        "cancel",
        "cancelByCloid",
        "claimRewards",
        "hip3LiquidatorTransfer",
        "modify",
        "noop",
        "order",
        "reserveRequestWeight",
        "scheduleCancel",
        "sendAsset",
        "sendToEvmWithData",
        "spotSend",
        "tokenDelegate",
        "topUpIsolatedOnlyMargin",
        "twapCancel",
        "twapOrder",
        "updateIsolatedMargin",
        "updateLeverage",
        "usdClassTransfer",
        "usdSend",
        "userDexAbstraction",
        "userOutcome",
        "userSetAbstraction",
        "validatorL1Stream",
        "vaultTransfer",
        "withdraw3",
    ];

    assert_eq!(SUPPORTED_ACTION_TYPES, expected);
}
