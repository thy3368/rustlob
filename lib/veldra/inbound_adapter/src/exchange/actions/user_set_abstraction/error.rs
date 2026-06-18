use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserSetAbstractionContractError {
    #[error("Unexpected `action.type` for userSetAbstraction handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.user`. Expected a 42-character hexadecimal address.")]
    InvalidUserAddress,
    #[error(
        "Invalid `action.abstraction`. Expected one of `disabled`, `unifiedAccount`, `portfolioMargin`."
    )]
    InvalidAbstraction,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`vaultAddress` is not supported for `userSetAbstraction`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `userSetAbstraction`.")]
    ExpiresAfterNotSupported,
}
