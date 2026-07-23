use std::fmt;

use thiserror::Error;

use super::{PerpClearinghouseState, SpotClearinghouseState};

/// Hyperliquid 账户体系里的统一账户标识。
///
/// 该值对象不承载 wire 层编号格式，只表达核心领域里的稳定身份。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountId(String);

impl AccountId {
    /// 基于已知业务事实构造账户标识。
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 返回账户标识的只读字符串视图。
    pub fn as_str(&self) -> &str {
        self.0.as_str()
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

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Hyperliquid 现货资产标识。
///
/// 该值对象只保留“资产身份”语义，不把外部接口字段格式带入核心。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssetId(String);

impl AssetId {
    /// 基于已知业务事实构造现货资产标识。
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 返回资产标识的只读字符串视图。
    pub fn as_str(&self) -> &str {
        self.0.as_str()
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

impl fmt::Display for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Hyperliquid perp 合约资产标识。
///
/// 与 `AssetId` 分离，避免把现货与 perp 资产空间混成同一语义。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PerpAssetId(String);

impl PerpAssetId {
    /// 基于已知业务事实构造 perp 资产标识。
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// 返回 perp 资产标识的只读字符串视图。
    pub fn as_str(&self) -> &str {
        self.0.as_str()
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

impl fmt::Display for PerpAssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 主账户主档，表达主账户身份与其拥有的一组子账户。
///
/// 该对象属于 `Party/Place/Thing`，主要承载所有权和归属查询语义。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterAccount {
    account_id: AccountId,
    sub_account_ids: Vec<AccountId>,
}

impl MasterAccount {
    /// 从已校验事实装配主账户主档。
    pub fn new(account_id: AccountId, sub_account_ids: Vec<AccountId>) -> Self {
        Self { account_id, sub_account_ids }
    }

    /// 返回主账户标识。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回主账户当前拥有的子账户列表。
    pub fn sub_account_ids(&self) -> &[AccountId] {
        &self.sub_account_ids
    }

    /// 返回指定子账户是否归属于该主账户。
    pub fn owns(&self, account_id: &AccountId) -> bool {
        self.sub_account_ids.iter().any(|candidate| candidate == account_id)
    }
}

/// 子账户主档，表达子账户身份、归属主账户与展示名。
///
/// 该对象属于 `Party/Place/Thing`，主要承载归属与展示层需要的稳定业务事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubAccountProfile {
    account_id: AccountId,
    master_account_id: AccountId,
    display_name: String,
}

impl SubAccountProfile {
    /// 从已校验事实装配子账户主档。
    pub fn new(
        account_id: AccountId,
        master_account_id: AccountId,
        display_name: impl Into<String>,
    ) -> Self {
        Self { account_id, master_account_id, display_name: display_name.into() }
    }

    /// 返回子账户标识。
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// 返回归属的主账户标识。
    pub fn master_account_id(&self) -> &AccountId {
        &self.master_account_id
    }

    /// 返回子账户展示名。
    pub fn display_name(&self) -> &str {
        self.display_name.as_str()
    }

    /// 返回该子账户是否归属于指定主账户。
    pub fn belongs_to(&self, master_account: &MasterAccount) -> bool {
        self.master_account_id == master_account.account_id && master_account.owns(&self.account_id)
    }
}

/// 子账户完整快照。
///
/// 该聚合快照把主档、现货清算状态和 perp 清算状态绑定在一起，
/// 用于后续 mapper 或 read path 做一致性装配。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubAccountSnapshot {
    profile: SubAccountProfile,
    spot: SpotClearinghouseState,
    perp: PerpClearinghouseState,
}

impl SubAccountSnapshot {
    /// 装配同一子账户的完整快照。
    ///
    /// 若 `profile`、`spot`、`perp` 的 `account_id` 不一致，则返回明确领域错误。
    pub fn try_new(
        profile: SubAccountProfile,
        spot: SpotClearinghouseState,
        perp: PerpClearinghouseState,
    ) -> Result<Self, SubAccountSnapshotError> {
        if profile.account_id() != spot.account_id() {
            return Err(SubAccountSnapshotError::SpotAccountMismatch {
                profile_account_id: profile.account_id().clone(),
                spot_account_id: spot.account_id().clone(),
            });
        }

        if profile.account_id() != perp.account_id() {
            return Err(SubAccountSnapshotError::PerpAccountMismatch {
                profile_account_id: profile.account_id().clone(),
                perp_account_id: perp.account_id().clone(),
            });
        }

        Ok(Self { profile, spot, perp })
    }

    /// 返回子账户主档。
    pub fn profile(&self) -> &SubAccountProfile {
        &self.profile
    }

    /// 返回现货清算状态快照。
    pub fn spot(&self) -> &SpotClearinghouseState {
        &self.spot
    }

    /// 返回 perp 清算状态快照。
    pub fn perp(&self) -> &PerpClearinghouseState {
        &self.perp
    }

    /// 返回当前聚合快照对应的子账户标识。
    pub fn account_id(&self) -> &AccountId {
        self.profile.account_id()
    }
}

/// 子账户完整快照的装配错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SubAccountSnapshotError {
    /// 现货清算状态不属于该子账户。
    #[error(
        "spot clearinghouse state account_id mismatch: profile={profile_account_id}, spot={spot_account_id}"
    )]
    SpotAccountMismatch { profile_account_id: AccountId, spot_account_id: AccountId },
    /// perp 清算状态不属于该子账户。
    #[error(
        "perp clearinghouse state account_id mismatch: profile={profile_account_id}, perp={perp_account_id}"
    )]
    PerpAccountMismatch { profile_account_id: AccountId, perp_account_id: AccountId },
}

#[cfg(test)]
mod tests {
    use decimal::Decimal;

    use super::*;
    use crate::entity::hyperliquid_account::{
        MarginSummary, PerpClearinghouseState, PerpPositionRiskSnapshot, RiskState,
        SpotClearinghouseState,
    };
    use crate::entity::{Balance, HyperliquidPerpMarginMode, HyperliquidPerpPosition};

    fn dec(units: i64) -> Decimal {
        Decimal::from_raw(units * 100_000_000)
    }

    fn sample_spot(account_id: &str) -> SpotClearinghouseState {
        SpotClearinghouseState::new(
            AccountId::from(account_id),
            vec![Balance::new_with_snapshot_facts(
                account_id.to_string(),
                "USDC".to_string(),
                10,
                2,
                Some(12),
                Some("usdc-spot".to_owned()),
                1,
            )],
        )
    }

    fn sample_perp(account_id: &str) -> PerpClearinghouseState {
        let position = HyperliquidPerpPosition::new(
            format!("{account_id}-BTC-PERP"),
            account_id.to_owned(),
            0,
            "BTC-PERP".to_owned(),
            1,
            100_000,
            3,
            HyperliquidPerpMarginMode::Cross,
            50,
            1,
        );
        PerpClearinghouseState::new(
            AccountId::from(account_id),
            vec![PerpPositionRiskSnapshot {
                position,
                mark_price: dec(100_000),
                position_value: dec(100_000),
                unrealized_pnl: dec(250),
                margin_used: dec(10_000),
                liquidation_price: Some(dec(90_000)),
                return_on_equity: dec(0),
            }],
            MarginSummary::new(dec(20_000), dec(8_000), dec(100_000), dec(19_750)),
            MarginSummary::new(dec(18_000), dec(7_000), dec(100_000), dec(17_750)),
            Some(dec(1_500)),
            dec(5_000),
            RiskState::Normal,
        )
    }

    #[test]
    fn master_account_and_profile_form_ownership_relation() {
        let master = MasterAccount::new(
            AccountId::from("master-1"),
            vec![AccountId::from("sub-1"), AccountId::from("sub-2")],
        );
        let profile =
            SubAccountProfile::new(AccountId::from("sub-1"), AccountId::from("master-1"), "alpha");

        assert!(master.owns(&AccountId::from("sub-1")));
        assert!(!master.owns(&AccountId::from("sub-9")));
        assert!(profile.belongs_to(&master));
    }

    #[test]
    fn sub_account_snapshot_try_new_accepts_consistent_account_ids() {
        let profile =
            SubAccountProfile::new(AccountId::from("sub-1"), AccountId::from("master-1"), "alpha");

        let snapshot =
            SubAccountSnapshot::try_new(profile, sample_spot("sub-1"), sample_perp("sub-1"))
                .expect("consistent account ids should assemble");

        assert_eq!(snapshot.account_id(), &AccountId::from("sub-1"));
        assert_eq!(snapshot.spot().available_balance_of(&AssetId::from("USDC")), 10);
    }

    #[test]
    fn sub_account_snapshot_try_new_rejects_spot_account_mismatch() {
        let profile =
            SubAccountProfile::new(AccountId::from("sub-1"), AccountId::from("master-1"), "alpha");

        let error =
            SubAccountSnapshot::try_new(profile, sample_spot("sub-2"), sample_perp("sub-1"))
                .expect_err("spot account mismatch should fail");

        assert_eq!(
            error,
            SubAccountSnapshotError::SpotAccountMismatch {
                profile_account_id: AccountId::from("sub-1"),
                spot_account_id: AccountId::from("sub-2"),
            }
        );
    }

    #[test]
    fn sub_account_snapshot_try_new_rejects_perp_account_mismatch() {
        let profile =
            SubAccountProfile::new(AccountId::from("sub-1"), AccountId::from("master-1"), "alpha");

        let error =
            SubAccountSnapshot::try_new(profile, sample_spot("sub-1"), sample_perp("sub-2"))
                .expect_err("perp account mismatch should fail");

        assert_eq!(
            error,
            SubAccountSnapshotError::PerpAccountMismatch {
                profile_account_id: AccountId::from("sub-1"),
                perp_account_id: AccountId::from("sub-2"),
            }
        );
    }
}
