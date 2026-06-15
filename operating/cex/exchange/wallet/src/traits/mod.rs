pub mod wallet_behavior;

pub use wallet_behavior::{
    AccountInfo,
    AccountSnapshot,
    AssetDetail,
    // Response Types
    CoinInfo,
    DepositAddressResponse,
    DepositRecord,
    DustTransferCmd,
    DustTransferResponse,
    DustTransferResult,
    FundingAsset,
    GetAccountInfoCmd,
    GetAccountSnapshotCmd,

    // Commands
    GetAllCoinsCmd,
    GetAssetDetailCmd,
    GetDepositAddressCmd,
    GetDepositHistoryCmd,
    GetFundingAssetCmd,
    GetTradeFeeCmd,
    GetUniversalTransferHistoryCmd,
    GetUserAssetsCmd,
    GetWithdrawHistoryCmd,
    NetworkInfo,
    SnapshotVo,

    TradeFee,
    UniversalTransferCmd,
    UniversalTransferRecord,
    UniversalTransferResponse,
    UserAsset,
    // Trait
    WalletBehavior,
    // Command Enum
    WalletCmdAny,

    // Error
    WalletCmdError,

    // Response Enum
    WalletRes,

    WithdrawCmd,
    WithdrawRecord,
    WithdrawResponse,
};
