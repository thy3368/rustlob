use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApproveAgentContractError {
    #[error("Unexpected `action.type` for approveAgent handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.hyperliquidChain`. Expected `Mainnet` or `Testnet`.")]
    InvalidHyperliquidChain,
    #[error("Invalid `action.signatureChainId`. Expected a hexadecimal chain id like `0xa4b1`.")]
    InvalidSignatureChainId,
    #[error("Invalid `action.agentAddress`. Expected a 42-character hexadecimal address.")]
    InvalidAgentAddress,
    #[error("Invalid `action.nonce`. Expected it to match the outer `nonce`.")]
    NonceMismatch,
    #[error("`vaultAddress` is not supported for `approveAgent`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `approveAgent`.")]
    ExpiresAfterNotSupported,
}
