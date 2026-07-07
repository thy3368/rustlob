pub mod aligned_quote_token_info;
pub mod all_borrow_lend_reserve_states;
pub mod all_mids;
pub mod approved_builders;
pub mod borrow_lend_reserve_state;
pub mod borrow_lend_user_state;
pub mod candle_snapshot;
pub mod delegations;
pub mod delegator_history;
pub mod delegator_rewards;
pub mod delegator_summary;
pub mod frontend_open_orders;
pub mod historical_orders;
pub mod l2_book;
pub mod max_builder_fee;
pub mod open_orders;
pub mod order_status;
pub mod portfolio;
pub mod referral;
pub mod sub_accounts;
pub mod user_abstraction;
pub mod user_dex_abstraction;
pub mod user_fees;
pub mod user_fills;
pub mod user_fills_by_time;
pub mod user_rate_limit;
pub mod user_role;
pub mod user_twap_slice_fills;
pub mod user_vault_equities;
pub mod vault_details;

use serde_json::{Value as JsonValue, to_value};

use crate::info::error::InfoHttpError;

#[derive(Debug, Default, Clone, Copy)]
pub struct InfoQueryDeps;

pub const SUPPORTED_INFO_QUERY_TYPES: &[&str] = &[
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

pub enum InfoQueryReply {
    AllMids(all_mids::reply::ResponseWire),
    OpenOrders(open_orders::reply::ResponseWire),
    FrontendOpenOrders(frontend_open_orders::reply::ResponseWire),
    UserFills(user_fills::reply::ResponseWire),
    UserFillsByTime(user_fills_by_time::reply::ResponseWire),
    UserRateLimit(user_rate_limit::reply::ResponseWire),
    OrderStatus(order_status::reply::ResponseWire),
    L2Book(l2_book::reply::ResponseWire),
    CandleSnapshot(candle_snapshot::reply::ResponseWire),
    MaxBuilderFee(max_builder_fee::reply::ResponseWire),
    HistoricalOrders(historical_orders::reply::ResponseWire),
    UserTwapSliceFills(user_twap_slice_fills::reply::ResponseWire),
    SubAccounts(sub_accounts::reply::ResponseWire),
    VaultDetails(vault_details::reply::ResponseWire),
    UserVaultEquities(user_vault_equities::reply::ResponseWire),
    UserRole(user_role::reply::ResponseWire),
    Portfolio(portfolio::reply::ResponseWire),
    Referral(referral::reply::ResponseWire),
    UserFees(user_fees::reply::ResponseWire),
    Delegations(delegations::reply::ResponseWire),
    DelegatorSummary(delegator_summary::reply::ResponseWire),
    DelegatorHistory(delegator_history::reply::ResponseWire),
    DelegatorRewards(delegator_rewards::reply::ResponseWire),
    UserDexAbstraction(user_dex_abstraction::reply::ResponseWire),
    UserAbstraction(user_abstraction::reply::ResponseWire),
    AlignedQuoteTokenInfo(aligned_quote_token_info::reply::ResponseWire),
    BorrowLendUserState(borrow_lend_user_state::reply::ResponseWire),
    BorrowLendReserveState(borrow_lend_reserve_state::reply::ResponseWire),
    AllBorrowLendReserveStates(all_borrow_lend_reserve_states::reply::ResponseWire),
    ApprovedBuilders(approved_builders::reply::ResponseWire),
}

impl InfoQueryReply {
    pub fn into_json_value(self) -> JsonValue {
        match self {
            Self::AllMids(value) => to_value(value),
            Self::OpenOrders(value) => to_value(value),
            Self::FrontendOpenOrders(value) => to_value(value),
            Self::UserFills(value) => to_value(value),
            Self::UserFillsByTime(value) => to_value(value),
            Self::UserRateLimit(value) => to_value(value),
            Self::OrderStatus(value) => to_value(value),
            Self::L2Book(value) => to_value(value),
            Self::CandleSnapshot(value) => to_value(value),
            Self::MaxBuilderFee(value) => to_value(value),
            Self::HistoricalOrders(value) => to_value(value),
            Self::UserTwapSliceFills(value) => to_value(value),
            Self::SubAccounts(value) => to_value(value),
            Self::VaultDetails(value) => to_value(value),
            Self::UserVaultEquities(value) => to_value(value),
            Self::UserRole(value) => to_value(value),
            Self::Portfolio(value) => to_value(value),
            Self::Referral(value) => to_value(value),
            Self::UserFees(value) => to_value(value),
            Self::Delegations(value) => to_value(value),
            Self::DelegatorSummary(value) => to_value(value),
            Self::DelegatorHistory(value) => to_value(value),
            Self::DelegatorRewards(value) => to_value(value),
            Self::UserDexAbstraction(value) => to_value(value),
            Self::UserAbstraction(value) => to_value(value),
            Self::AlignedQuoteTokenInfo(value) => to_value(value),
            Self::BorrowLendUserState(value) => to_value(value),
            Self::BorrowLendReserveState(value) => to_value(value),
            Self::AllBorrowLendReserveStates(value) => to_value(value),
            Self::ApprovedBuilders(value) => to_value(value),
        }
        .expect("info stub response serializes")
    }
}

pub async fn dispatch_info_query(
    query_type: &str,
    body: &[u8],
    deps: &InfoQueryDeps,
) -> Result<InfoQueryReply, InfoHttpError> {
    match query_type {
        "allMids" => Ok(InfoQueryReply::AllMids(all_mids::handle(body, deps).await?)),
        "openOrders" => Ok(InfoQueryReply::OpenOrders(open_orders::handle(body, deps).await?)),
        "frontendOpenOrders" => {
            Ok(InfoQueryReply::FrontendOpenOrders(frontend_open_orders::handle(body, deps).await?))
        }
        "userFills" => Ok(InfoQueryReply::UserFills(user_fills::handle(body, deps).await?)),
        "userFillsByTime" => {
            Ok(InfoQueryReply::UserFillsByTime(user_fills_by_time::handle(body, deps).await?))
        }
        "userRateLimit" => {
            Ok(InfoQueryReply::UserRateLimit(user_rate_limit::handle(body, deps).await?))
        }
        "orderStatus" => Ok(InfoQueryReply::OrderStatus(order_status::handle(body, deps).await?)),
        "l2Book" => Ok(InfoQueryReply::L2Book(l2_book::handle(body, deps).await?)),
        "candleSnapshot" => {
            Ok(InfoQueryReply::CandleSnapshot(candle_snapshot::handle(body, deps).await?))
        }
        "maxBuilderFee" => {
            Ok(InfoQueryReply::MaxBuilderFee(max_builder_fee::handle(body, deps).await?))
        }
        "historicalOrders" => {
            Ok(InfoQueryReply::HistoricalOrders(historical_orders::handle(body, deps).await?))
        }
        "userTwapSliceFills" => {
            Ok(InfoQueryReply::UserTwapSliceFills(user_twap_slice_fills::handle(body, deps).await?))
        }
        "subAccounts" => Ok(InfoQueryReply::SubAccounts(sub_accounts::handle(body, deps).await?)),
        "vaultDetails" => {
            Ok(InfoQueryReply::VaultDetails(vault_details::handle(body, deps).await?))
        }
        "userVaultEquities" => {
            Ok(InfoQueryReply::UserVaultEquities(user_vault_equities::handle(body, deps).await?))
        }
        "userRole" => Ok(InfoQueryReply::UserRole(user_role::handle(body, deps).await?)),
        "portfolio" => Ok(InfoQueryReply::Portfolio(portfolio::handle(body, deps).await?)),
        "referral" => Ok(InfoQueryReply::Referral(referral::handle(body, deps).await?)),
        "userFees" => Ok(InfoQueryReply::UserFees(user_fees::handle(body, deps).await?)),
        "delegations" => Ok(InfoQueryReply::Delegations(delegations::handle(body, deps).await?)),
        "delegatorSummary" => {
            Ok(InfoQueryReply::DelegatorSummary(delegator_summary::handle(body, deps).await?))
        }
        "delegatorHistory" => {
            Ok(InfoQueryReply::DelegatorHistory(delegator_history::handle(body, deps).await?))
        }
        "delegatorRewards" => {
            Ok(InfoQueryReply::DelegatorRewards(delegator_rewards::handle(body, deps).await?))
        }
        "userDexAbstraction" => {
            Ok(InfoQueryReply::UserDexAbstraction(user_dex_abstraction::handle(body, deps).await?))
        }
        "userAbstraction" => {
            Ok(InfoQueryReply::UserAbstraction(user_abstraction::handle(body, deps).await?))
        }
        "alignedQuoteTokenInfo" => Ok(InfoQueryReply::AlignedQuoteTokenInfo(
            aligned_quote_token_info::handle(body, deps).await?,
        )),
        "borrowLendUserState" => Ok(InfoQueryReply::BorrowLendUserState(
            borrow_lend_user_state::handle(body, deps).await?,
        )),
        "borrowLendReserveState" => Ok(InfoQueryReply::BorrowLendReserveState(
            borrow_lend_reserve_state::handle(body, deps).await?,
        )),
        "allBorrowLendReserveStates" => Ok(InfoQueryReply::AllBorrowLendReserveStates(
            all_borrow_lend_reserve_states::handle(body, deps).await?,
        )),
        "approvedBuilders" => {
            Ok(InfoQueryReply::ApprovedBuilders(approved_builders::handle(body, deps).await?))
        }
        other => Err(InfoHttpError::UnsupportedQueryType(other.to_string())),
    }
}
