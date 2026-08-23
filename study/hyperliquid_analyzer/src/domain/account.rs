use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Hyperliquid 主账户身份，只表达“谁拥有一组子账户”。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AccountId(String);

impl AccountId {
    /// 用稳定账户标识构造账户身份。
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 返回账户标识的只读字符串视图。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for AccountId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for AccountId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 现货资产标识。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AssetId(String);

impl AssetId {
    /// 用稳定业务标识构造现货资产。
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 返回资产标识的只读字符串视图。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for AssetId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for AssetId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 永续合约资产标识。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PerpAssetId(String);

impl PerpAssetId {
    /// 用稳定业务标识构造 perp 资产。
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 返回 perp 资产标识的只读字符串视图。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for PerpAssetId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for PerpAssetId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for PerpAssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 子账户保证金模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginMode {
    Cross,
    Isolated,
}

/// 仓位方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionSide {
    Long,
    Short,
}

/// 领域风险语义。
///
/// 该值不要求与 Hyperliquid 原始返回一一对应，可由后续 adapter 或风险规则派生。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RiskState {
    Healthy,
    MarginConstrained,
    LiquidationRisk,
    Liquidating,
    #[default]
    Unknown,
}

/// 主账户身份对象。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MasterAccount {
    account_id: AccountId,
}

impl MasterAccount {
    /// 只装配主账户身份，不承载交易状态。
    pub fn new(account_id: AccountId) -> Self {
        Self { account_id }
    }

    /// 返回主账户身份。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 判断某个子账户是否归该主账户管理。
    pub fn owns(&self, sub_account: &SubAccountProfile) -> bool {
        sub_account.master_account_id() == self.account_id()
    }
}

/// 子账户身份与归属对象。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubAccountProfile {
    account_id: AccountId,
    master_account_id: AccountId,
    display_name: Option<String>,
}

impl SubAccountProfile {
    /// 只装配子账户身份与归属，不承担风控或余额校验。
    pub fn new(
        account_id: AccountId,
        master_account_id: AccountId,
        display_name: Option<String>,
    ) -> Self {
        Self { account_id, master_account_id, display_name }
    }

    /// 返回子账户身份。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回主账户归属身份。
    pub fn master_account_id(&self) -> &AccountId {
        &self.master_account_id
    }

    /// 返回展示名。
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// 判断子账户是否归某主账户所有。
    pub fn belongs_to(&self, master_account: &MasterAccount) -> bool {
        self.master_account_id() == master_account.account_id()
    }
}

/// 保证金汇总快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarginSummary {
    account_value: Decimal,
    total_position_notional: Decimal,
    total_raw_usd: Decimal,
    total_margin_used: Decimal,
}

impl MarginSummary {
    /// 只装配已经被 adapter 解析好的保证金汇总事实。
    pub fn new(
        account_value: Decimal,
        total_position_notional: Decimal,
        total_raw_usd: Decimal,
        total_margin_used: Decimal,
    ) -> Self {
        Self { account_value, total_position_notional, total_raw_usd, total_margin_used }
    }

    /// 账户总权益。
    pub fn account_value(&self) -> Decimal {
        self.account_value
    }

    /// 总仓位名义价值。
    pub fn total_position_notional(&self) -> Decimal {
        self.total_position_notional
    }

    /// 原始 USD 价值汇总。
    pub fn total_raw_usd(&self) -> Decimal {
        self.total_raw_usd
    }

    /// 总已用保证金。
    pub fn total_margin_used(&self) -> Decimal {
        self.total_margin_used
    }
}

/// 单币种现货余额快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBalance {
    asset_id: AssetId,
    total: Decimal,
    hold: Decimal,
    entry_notional: Option<Decimal>,
    identifier: Option<String>,
}

impl SpotBalance {
    /// 只装配单个币种余额事实。
    pub fn new(
        asset_id: AssetId,
        total: Decimal,
        hold: Decimal,
        entry_notional: Option<Decimal>,
        identifier: Option<String>,
    ) -> Self {
        Self { asset_id, total, hold, entry_notional, identifier }
    }

    /// 返回余额归属币种。
    pub fn asset_id(&self) -> &AssetId {
        &self.asset_id
    }

    /// 返回该币种总余额。
    pub fn total(&self) -> Decimal {
        self.total
    }

    /// 返回该币种占用余额。
    pub fn hold(&self) -> Decimal {
        self.hold
    }

    /// 返回该币种可用余额。
    pub fn available(&self) -> Decimal {
        self.total - self.hold
    }

    /// 返回该币种入场名义值。
    pub fn entry_notional(&self) -> Option<Decimal> {
        self.entry_notional
    }

    /// 返回上游标识信息。
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }
}

/// 子账户现货清算状态快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotClearinghouseState {
    account_id: AccountId,
    balances: Vec<SpotBalance>,
}

impl SpotClearinghouseState {
    /// 只装配某子账户的现货余额快照。
    pub fn new(account_id: AccountId, balances: Vec<SpotBalance>) -> Self {
        Self { account_id, balances }
    }

    /// 返回该现货快照所属子账户。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回余额集合的只读切片。
    pub fn balances(&self) -> &[SpotBalance] {
        &self.balances
    }

    /// 返回指定币种的余额总额；未找到时返回零。
    pub fn total_balance_of(&self, asset_id: &AssetId) -> Decimal {
        self.balance_of(asset_id).map(SpotBalance::total).unwrap_or(Decimal::ZERO)
    }

    /// 返回指定币种的可用余额；未找到时返回零。
    pub fn available_balance_of(&self, asset_id: &AssetId) -> Decimal {
        self.balance_of(asset_id).map(SpotBalance::available).unwrap_or(Decimal::ZERO)
    }

    /// 按币种定位余额快照。
    pub fn balance_of(&self, asset_id: &AssetId) -> Option<&SpotBalance> {
        self.balances.iter().find(|balance| balance.asset_id() == asset_id)
    }
}

/// 单个 perp 仓位快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpPosition {
    asset_id: PerpAssetId,
    size: Decimal,
    side: PositionSide,
    entry_price: Decimal,
    margin_mode: MarginMode,
    leverage: u32,
    unrealized_pnl: Option<Decimal>,
    realized_pnl: Option<Decimal>,
}

impl PerpPosition {
    /// 只装配单仓位事实，不承担风控推导。
    pub fn new(
        asset_id: PerpAssetId,
        size: Decimal,
        side: PositionSide,
        entry_price: Decimal,
        margin_mode: MarginMode,
        leverage: u32,
        unrealized_pnl: Option<Decimal>,
        realized_pnl: Option<Decimal>,
    ) -> Self {
        Self {
            asset_id,
            size,
            side,
            entry_price,
            margin_mode,
            leverage,
            unrealized_pnl,
            realized_pnl,
        }
    }

    /// 返回 perp 资产标识。
    pub fn asset_id(&self) -> &PerpAssetId {
        &self.asset_id
    }

    /// 返回仓位数量。
    pub fn size(&self) -> Decimal {
        self.size
    }

    /// 返回仓位方向。
    pub fn side(&self) -> PositionSide {
        self.side
    }

    /// 返回开仓均价。
    pub fn entry_price(&self) -> Decimal {
        self.entry_price
    }

    /// 返回保证金模式。
    pub fn margin_mode(&self) -> MarginMode {
        self.margin_mode
    }

    /// 返回杠杆档位。
    pub fn leverage(&self) -> u32 {
        self.leverage
    }

    /// 返回未实现盈亏。
    pub fn unrealized_pnl(&self) -> Option<Decimal> {
        self.unrealized_pnl
    }

    /// 返回已实现盈亏。
    pub fn realized_pnl(&self) -> Option<Decimal> {
        self.realized_pnl
    }

    /// 判断该仓位是否仍然开着。
    pub fn is_open(&self) -> bool {
        !self.size.is_zero()
    }
}

/// 子账户永续合约风险账户快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpClearinghouseState {
    account_id: AccountId,
    positions: Vec<PerpPosition>,
    margin_summary: MarginSummary,
    cross_margin_summary: MarginSummary,
    withdrawable: Decimal,
    risk_state: RiskState,
}

impl PerpClearinghouseState {
    /// 只装配某子账户的 perp 风险快照。
    pub fn new(
        account_id: AccountId,
        positions: Vec<PerpPosition>,
        margin_summary: MarginSummary,
        cross_margin_summary: MarginSummary,
        withdrawable: Decimal,
        risk_state: RiskState,
    ) -> Self {
        Self {
            account_id,
            positions,
            margin_summary,
            cross_margin_summary,
            withdrawable,
            risk_state,
        }
    }

    /// 返回该 perp 快照所属子账户。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回仓位集合的只读切片。
    pub fn positions(&self) -> &[PerpPosition] {
        &self.positions
    }

    /// 返回保证金汇总。
    pub fn margin_summary(&self) -> &MarginSummary {
        &self.margin_summary
    }

    /// 返回 cross 保证金汇总。
    pub fn cross_margin_summary(&self) -> &MarginSummary {
        &self.cross_margin_summary
    }

    /// 返回可提现额度。
    pub fn withdrawable(&self) -> Decimal {
        self.withdrawable
    }

    /// 返回风险状态。
    pub fn risk_state(&self) -> RiskState {
        self.risk_state
    }

    /// 判断该子账户是否存在未平仓仓位。
    pub fn has_open_positions(&self) -> bool {
        self.positions.iter().any(PerpPosition::is_open)
    }

    /// 按 perp 资产查找仓位。
    pub fn position_of(&self, asset_id: &PerpAssetId) -> Option<&PerpPosition> {
        self.positions.iter().find(|position| position.asset_id() == asset_id)
    }
}

/// 子账户完整业务快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubAccountSnapshot {
    profile: SubAccountProfile,
    spot_state: SpotClearinghouseState,
    perp_state: PerpClearinghouseState,
}

impl SubAccountSnapshot {
    /// 组装同一子账户在同一时点的现货与 perp 快照。
    pub fn try_new(
        profile: SubAccountProfile,
        spot_state: SpotClearinghouseState,
        perp_state: PerpClearinghouseState,
    ) -> Result<Self, SubAccountSnapshotError> {
        if profile.account_id() != spot_state.account_id() {
            return Err(SubAccountSnapshotError::SpotAccountMismatch {
                profile_account_id: profile.account_id().clone(),
                spot_account_id: spot_state.account_id().clone(),
            });
        }

        if profile.account_id() != perp_state.account_id() {
            return Err(SubAccountSnapshotError::PerpAccountMismatch {
                profile_account_id: profile.account_id().clone(),
                perp_account_id: perp_state.account_id().clone(),
            });
        }

        Ok(Self { profile, spot_state, perp_state })
    }

    /// 返回子账户身份对象。
    pub fn profile(&self) -> &SubAccountProfile {
        &self.profile
    }

    /// 返回现货状态快照。
    pub fn spot_state(&self) -> &SpotClearinghouseState {
        &self.spot_state
    }

    /// 返回 perp 状态快照。
    pub fn perp_state(&self) -> &PerpClearinghouseState {
        &self.perp_state
    }

    /// 返回该完整快照对应的子账户。
    pub fn account_id(&self) -> &AccountId {
        self.profile.account_id()
    }
}

/// 子账户完整快照装配失败。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SubAccountSnapshotError {
    #[error("spot account id mismatch: profile={profile_account_id}, spot={spot_account_id}")]
    SpotAccountMismatch { profile_account_id: AccountId, spot_account_id: AccountId },
    #[error("perp account id mismatch: profile={profile_account_id}, perp={perp_account_id}")]
    PerpAccountMismatch { profile_account_id: AccountId, perp_account_id: AccountId },
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::{
        AccountId, AssetId, MarginMode, MarginSummary, MasterAccount, PerpAssetId,
        PerpClearinghouseState, PerpPosition, PositionSide, RiskState, SpotBalance,
        SpotClearinghouseState, SubAccountProfile, SubAccountSnapshot, SubAccountSnapshotError,
    };

    fn sample_margin_summary() -> MarginSummary {
        MarginSummary::new(dec!(1000), dec!(500), dec!(900), dec!(100))
    }

    fn sample_profile() -> SubAccountProfile {
        SubAccountProfile::new(
            AccountId::new("sub-1"),
            AccountId::new("master-1"),
            Some("maker-sub".to_string()),
        )
    }

    #[test]
    fn spot_balance_available_is_total_minus_hold() {
        let balance =
            SpotBalance::new(AssetId::new("USDC"), dec!(10.5), dec!(3.25), Some(dec!(10.5)), None);

        assert_eq!(balance.available(), dec!(7.25));
    }

    #[test]
    fn spot_state_returns_zero_for_missing_asset() {
        let state = SpotClearinghouseState::new(
            AccountId::new("sub-1"),
            vec![SpotBalance::new(AssetId::new("USDC"), dec!(10), dec!(2), None, None)],
        );

        assert_eq!(state.total_balance_of(&AssetId::new("BTC")), dec!(0));
        assert_eq!(state.available_balance_of(&AssetId::new("BTC")), dec!(0));
    }

    #[test]
    fn perp_state_reports_open_positions_and_finds_position_by_asset() {
        let btc = PerpPosition::new(
            PerpAssetId::new("BTC-PERP"),
            dec!(1.5),
            PositionSide::Long,
            dec!(64000),
            MarginMode::Cross,
            5,
            Some(dec!(120)),
            Some(dec!(12)),
        );
        let eth = PerpPosition::new(
            PerpAssetId::new("ETH-PERP"),
            dec!(0),
            PositionSide::Short,
            dec!(3200),
            MarginMode::Isolated,
            3,
            Some(dec!(0)),
            Some(dec!(0)),
        );
        let state = PerpClearinghouseState::new(
            AccountId::new("sub-1"),
            vec![btc.clone(), eth],
            sample_margin_summary(),
            sample_margin_summary(),
            dec!(500),
            RiskState::Healthy,
        );

        assert!(state.has_open_positions());
        assert_eq!(state.position_of(&PerpAssetId::new("BTC-PERP")), Some(&btc));
        assert_eq!(state.position_of(&PerpAssetId::new("SOL-PERP")), None);
    }

    #[test]
    fn sub_account_snapshot_accepts_consistent_account_id() {
        let profile = sample_profile();
        let spot_state = SpotClearinghouseState::new(profile.account_id().clone(), vec![]);
        let perp_state = PerpClearinghouseState::new(
            profile.account_id().clone(),
            vec![],
            sample_margin_summary(),
            sample_margin_summary(),
            dec!(0),
            RiskState::Unknown,
        );
        let snapshot =
            SubAccountSnapshot::try_new(profile.clone(), spot_state, perp_state).unwrap();

        assert_eq!(snapshot.account_id(), profile.account_id());
        assert_eq!(snapshot.profile().display_name(), Some("maker-sub"));
    }

    #[test]
    fn sub_account_profile_belongs_to_master_account() {
        let master = MasterAccount::new(AccountId::new("master-1"));
        let profile = sample_profile();

        assert!(profile.belongs_to(&master));
        assert!(master.owns(&profile));
    }

    #[test]
    fn sub_account_snapshot_rejects_mismatched_spot_account() {
        let profile = sample_profile();
        let spot_state = SpotClearinghouseState::new(AccountId::new("sub-2"), vec![]);
        let perp_state = PerpClearinghouseState::new(
            profile.account_id().clone(),
            vec![],
            sample_margin_summary(),
            sample_margin_summary(),
            dec!(0),
            RiskState::Unknown,
        );

        let error = SubAccountSnapshot::try_new(profile, spot_state, perp_state).unwrap_err();

        assert_eq!(
            error,
            SubAccountSnapshotError::SpotAccountMismatch {
                profile_account_id: AccountId::new("sub-1"),
                spot_account_id: AccountId::new("sub-2"),
            }
        );
    }
}
