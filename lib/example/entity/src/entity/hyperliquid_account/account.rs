use std::fmt;

use thiserror::Error;

use super::{PerpClearinghouseState, SpotClearinghouseState};




/// 主账户主档，表达主账户身份与其拥有的一组子账户。
///
/// 该对象属于 `Party/Place/Thing`，主要承载所有权和归属查询语义。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterAccount {
    account_id: u64,
    sub_account_ids: Vec<u64>,
}

impl MasterAccount {
    /// 从已校验事实装配主账户主档。
    pub fn new(account_id: u64, sub_account_ids: Vec<u64>) -> Self {
        Self { account_id, sub_account_ids }
    }

    /// 返回主账户标识。
    pub fn account_id(&self) -> &u64 {
        &self.account_id
    }

    /// 返回主账户当前拥有的子账户列表。
    pub fn sub_account_ids(&self) -> &[u64] {
        &self.sub_account_ids
    }

    /// 返回指定子账户是否归属于该主账户。
    pub fn owns(&self, account_id: &u64) -> bool {
        self.sub_account_ids.iter().any(|candidate| candidate == account_id)
    }
}

/// 子账户主档，表达子账户身份、归属主账户与展示名。
///
/// 该对象属于 `Party/Place/Thing`，主要承载归属与展示层需要的稳定业务事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubAccountProfile {
    account_id: u64,
    master_account_id: u64,
    display_name: String,
}

impl SubAccountProfile {
    /// 从已校验事实装配子账户主档。
    pub fn new(
        account_id: u64,
        master_account_id: u64,
        display_name: impl Into<String>,
    ) -> Self {
        Self { account_id, master_account_id, display_name: display_name.into() }
    }

    /// 返回子账户标识。
    pub fn account_id(&self) -> &u64 {
        &self.account_id
    }

    /// 返回归属的主账户标识。
    pub fn master_account_id(&self) -> &u64 {
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
    pub fn account_id(&self) -> &u64 {
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
    SpotAccountMismatch { profile_account_id: u64, spot_account_id: u64 },
    /// perp 清算状态不属于该子账户。
    #[error(
        "perp clearinghouse state account_id mismatch: profile={profile_account_id}, perp={perp_account_id}"
    )]
    PerpAccountMismatch { profile_account_id: u64, perp_account_id: u64 },
}

