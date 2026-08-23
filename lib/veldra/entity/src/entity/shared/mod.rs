#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountAssetKey {
    pub account_id: String,
    pub asset: String,
}

impl AccountAssetKey {
    pub fn new(account_id: impl Into<String>, asset: impl Into<String>) -> Self {
        Self { account_id: account_id.into(), asset: asset.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountMarketKey {
    pub account_id: String,
    pub market: String,
}

impl AccountMarketKey {
    pub fn new(account_id: impl Into<String>, market: impl Into<String>) -> Self {
        Self { account_id: account_id.into(), market: market.into() }
    }
}
