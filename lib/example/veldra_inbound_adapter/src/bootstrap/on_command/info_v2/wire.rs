use serde::Deserialize;

use super::InfoV2Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderIdV2 {
    Numeric(u64),
    Cloid(String),
}

impl<'de> Deserialize<'de> for OrderIdV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            Numeric(u64),
            Cloid(String),
        }
        Ok(match Raw::deserialize(deserializer)? {
            Raw::Numeric(value) => Self::Numeric(value),
            Raw::Cloid(value) => Self::Cloid(value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandleRequestV2 {
    pub coin: String,
    pub interval: String,
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde(rename = "endTime")]
    pub end_time: u64,
}

macro_rules! request_variants {
    ($($variant:ident($body:ty) => $name:literal),+ $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum InfoRequestV2 { $($variant($body),)+ Unknown { type_: String } }

        impl<'de> Deserialize<'de> for InfoRequestV2 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de> {
                let value = serde_json::Value::deserialize(deserializer)?;
                let type_ = value.get("type").and_then(serde_json::Value::as_str)
                    .ok_or_else(|| serde::de::Error::custom("missing or non-string field `type`"))?;
                match type_ {
                    $($name => serde_json::from_value(value).map(Self::$variant).map_err(serde::de::Error::custom),)+
                    other => Ok(Self::Unknown { type_: other.to_owned() }),
                }
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeOnly {
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DexRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub dex: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub user: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserDexRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub user: String,
    pub dex: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserAggregateRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub user: String,
    #[serde(rename = "aggregateByTime")]
    pub aggregate_by_time: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserTimeRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub user: String,
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    #[serde(rename = "aggregateByTime")]
    pub aggregate_by_time: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderStatusRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub user: String,
    pub oid: OrderIdV2,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BookRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub coin: String,
    #[serde(rename = "nSigFigs")]
    pub n_sig_figs: Option<u32>,
    pub mantissa: Option<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandleSnapshotRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub req: CandleRequestV2,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuilderFeeRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub user: String,
    pub builder: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VaultRequest {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "vaultAddress")]
    pub vault_address: String,
    pub user: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub token: u32,
}

request_variants! {
    AllMids(DexRequest) => "allMids", OpenOrders(UserDexRequest) => "openOrders",
    FrontendOpenOrders(UserDexRequest) => "frontendOpenOrders", UserFills(UserAggregateRequest) => "userFills",
    UserFillsByTime(UserTimeRequest) => "userFillsByTime", UserRateLimit(UserRequest) => "userRateLimit",
    OrderStatus(OrderStatusRequest) => "orderStatus", L2Book(BookRequest) => "l2Book",
    CandleSnapshot(CandleSnapshotRequest) => "candleSnapshot", MaxBuilderFee(BuilderFeeRequest) => "maxBuilderFee",
    HistoricalOrders(UserRequest) => "historicalOrders", UserTwapSliceFills(UserRequest) => "userTwapSliceFills",
    SubAccounts(UserRequest) => "subAccounts", VaultDetails(VaultRequest) => "vaultDetails",
    UserVaultEquities(UserRequest) => "userVaultEquities", UserRole(UserRequest) => "userRole",
    Portfolio(UserRequest) => "portfolio", Referral(UserRequest) => "referral", UserFees(UserRequest) => "userFees",
    Delegations(UserRequest) => "delegations", DelegatorSummary(UserRequest) => "delegatorSummary",
    DelegatorHistory(UserRequest) => "delegatorHistory", DelegatorRewards(UserRequest) => "delegatorRewards",
    UserDexAbstraction(UserRequest) => "userDexAbstraction", UserAbstraction(UserRequest) => "userAbstraction",
    AlignedQuoteTokenInfo(TokenRequest) => "alignedQuoteTokenInfo", BorrowLendUserState(UserRequest) => "borrowLendUserState",
    BorrowLendReserveState(TokenRequest) => "borrowLendReserveState", AllBorrowLendReserveStates(TypeOnly) => "allBorrowLendReserveStates",
    ApprovedBuilders(UserRequest) => "approvedBuilders",
}

impl InfoRequestV2 {
    pub fn validate(&self) -> Result<(), InfoV2Error> {
        match self {
            Self::Unknown { type_ } => return Err(InfoV2Error::UnsupportedType(type_.clone())),
            Self::AllMids(v) => optional_text("dex", v.dex.as_deref())?,
            Self::OpenOrders(v) | Self::FrontendOpenOrders(v) => {
                address("user", &v.user)?;
                optional_text("dex", v.dex.as_deref())?;
            }
            Self::UserFills(v) => address("user", &v.user)?,
            Self::UserFillsByTime(v) => {
                address("user", &v.user)?;
                positive("startTime", v.start_time)?;
                if let Some(end) = v.end_time {
                    positive("endTime", end)?;
                    if v.start_time > end {
                        return Err(InfoV2Error::InvalidTimeRange);
                    }
                }
            }
            Self::OrderStatus(v) => {
                address("user", &v.user)?;
                match &v.oid {
                    OrderIdV2::Numeric(0) => return Err(InfoV2Error::InvalidOid),
                    OrderIdV2::Cloid(v) if !valid_cloid(v) => return Err(InfoV2Error::InvalidOid),
                    _ => {}
                }
            }
            Self::L2Book(v) => {
                text("coin", &v.coin)?;
                if !matches!(v.n_sig_figs, None | Some(2..=5)) {
                    return Err(InfoV2Error::InvalidSigFigs);
                }
                if v.mantissa.is_some()
                    && (v.n_sig_figs != Some(5) || !matches!(v.mantissa, Some(1 | 2 | 5)))
                {
                    return Err(InfoV2Error::InvalidMantissa);
                }
            }
            Self::CandleSnapshot(v) => {
                text("req.coin", &v.req.coin)?;
                text("req.interval", &v.req.interval)?;
                positive("req.startTime", v.req.start_time)?;
                positive("req.endTime", v.req.end_time)?;
                if v.req.start_time > v.req.end_time {
                    return Err(InfoV2Error::InvalidTimeRange);
                }
            }
            Self::MaxBuilderFee(v) => {
                address("user", &v.user)?;
                address("builder", &v.builder)?;
            }
            Self::VaultDetails(v) => {
                address("vaultAddress", &v.vault_address)?;
                if let Some(user) = &v.user {
                    address("user", user)?;
                }
            }
            Self::AlignedQuoteTokenInfo(_)
            | Self::BorrowLendReserveState(_)
            | Self::AllBorrowLendReserveStates(_) => {}
            Self::UserRateLimit(v)
            | Self::HistoricalOrders(v)
            | Self::UserTwapSliceFills(v)
            | Self::SubAccounts(v)
            | Self::UserVaultEquities(v)
            | Self::UserRole(v)
            | Self::Portfolio(v)
            | Self::Referral(v)
            | Self::UserFees(v)
            | Self::Delegations(v)
            | Self::DelegatorSummary(v)
            | Self::DelegatorHistory(v)
            | Self::DelegatorRewards(v)
            | Self::UserDexAbstraction(v)
            | Self::UserAbstraction(v)
            | Self::BorrowLendUserState(v)
            | Self::ApprovedBuilders(v) => address("user", &v.user)?,
        }
        Ok(())
    }
}

fn address(field: &'static str, value: &str) -> Result<(), InfoV2Error> {
    if value.len() == 42
        && value.starts_with("0x")
        && value[2..].bytes().all(|b| b.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(InfoV2Error::InvalidAddress(field))
    }
}
fn text(field: &'static str, value: &str) -> Result<(), InfoV2Error> {
    if value.trim().is_empty() {
        Err(InfoV2Error::EmptyString(field))
    } else {
        Ok(())
    }
}
fn optional_text(field: &'static str, value: Option<&str>) -> Result<(), InfoV2Error> {
    value.map_or(Ok(()), |v| text(field, v))
}
fn positive(field: &'static str, value: u64) -> Result<(), InfoV2Error> {
    if value == 0 {
        Err(InfoV2Error::NonPositive(field))
    } else {
        Ok(())
    }
}
fn valid_cloid(value: &str) -> bool {
    value.len() == 34
        && value.starts_with("0x")
        && value[2..].bytes().all(|b| b.is_ascii_hexdigit())
}
