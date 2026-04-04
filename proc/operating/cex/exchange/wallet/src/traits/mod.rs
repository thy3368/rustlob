pub mod wallet_behavior;

pub use wallet_behavior::{
    // Commands
    GetAllCoinsCmd, WithdrawCmd, GetWithdrawHistoryCmd, GetDepositHistoryCmd,
    GetDepositAddressCmd, GetAssetDetailCmd, GetUserAssetsCmd, UniversalTransferCmd,
    GetUniversalTransferHistoryCmd, DustTransferCmd, GetTradeFeeCmd, GetFundingAssetCmd,
    GetAccountInfoCmd, GetAccountSnapshotCmd,

    // Command Enum
    WalletCmdAny,

    // Response Types
    CoinInfo, NetworkInfo, WithdrawResponse, WithdrawRecord, DepositRecord,
    DepositAddressResponse, AssetDetail, UserAsset, UniversalTransferResponse,
    UniversalTransferRecord, DustTransferResponse, DustTransferResult, TradeFee,
    FundingAsset, AccountInfo, AccountSnapshot, SnapshotVo,

    // Response Enum
    WalletRes,

    // Error
    WalletCmdError,

    // Trait
    WalletBehavior,
};