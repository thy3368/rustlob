use common_entity::{Entity, EntityError, EntityFieldChange};

use crate::entity::HyperliquidPerpMarginMode;

const HYPERLIQUID_PERP_LEVERAGE_SETTING_ENTITY_TYPE: u8 = 15;

/// Hyperliquid perp 账户在单个 asset + 保证金模式下的杠杆配置快照。
///
/// 该实体表达 `updateLeverage` 这类配置事实，而不是仓位事实。
/// `setting_id` 固定由 `{account_id}:{asset}:{margin_mode}` 生成。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpLeverageSetting {
    /// 稳定配置主键：`{account_id}:{asset}:{margin_mode}`。
    pub setting_id: String,
    /// 配置所属账户 ID。
    pub account_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 当前配置对应的保证金模式。
    pub margin_mode: HyperliquidPerpMarginMode,
    /// 当前生效杠杆。
    pub leverage: u64,
    /// 当前配置实体版本。
    pub version: u64,
}

impl HyperliquidPerpLeverageSetting {
    /// 从已校验命令或事件回放事实构造杠杆配置快照。
    pub fn new(
        account_id: String,
        asset: u32,
        margin_mode: HyperliquidPerpMarginMode,
        leverage: u64,
        version: u64,
    ) -> Self {
        Self {
            setting_id: Self::compose_setting_id(account_id.as_str(), asset, margin_mode),
            account_id,
            asset,
            margin_mode,
            leverage,
            version,
        }
    }

    /// 生成稳定配置主键。
    pub fn compose_setting_id(
        account_id: &str,
        asset: u32,
        margin_mode: HyperliquidPerpMarginMode,
    ) -> String {
        format!("{}:{}:{}", account_id, asset, margin_mode.as_str())
    }

    /// 返回该配置是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回该配置是否覆盖指定 asset。
    pub fn trades_asset(&self, asset: u32) -> bool {
        self.asset == asset
    }

    /// 返回该配置是否匹配指定保证金模式。
    pub fn matches_margin_mode(&self, margin_mode: HyperliquidPerpMarginMode) -> bool {
        self.margin_mode == margin_mode
    }

    /// 应用新的杠杆值和下一个版本。
    pub fn apply_new_leverage(&mut self, new_leverage: u64, next_version: u64) {
        self.leverage = new_leverage;
        self.version = next_version;
    }
}

impl Entity for HyperliquidPerpLeverageSetting {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.setting_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_LEVERAGE_SETTING_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("margin_mode", "", self.margin_mode.as_str()),
            EntityFieldChange::new("leverage", "", self.leverage.to_string()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "asset", self.asset.to_string(), other.asset.to_string());
        push_change(
            &mut changes,
            "margin_mode",
            self.margin_mode.as_str(),
            other.margin_mode.as_str(),
        );
        push_change(
            &mut changes,
            "leverage",
            self.leverage.to_string(),
            other.leverage.to_string(),
        );
        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "account_id" | "margin_mode" => 0,
            "asset" | "leverage" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.setting_id))
    }
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn push_change(
    changes: &mut Vec<EntityFieldChange>,
    field_name: &'static str,
    old_value: impl Into<String>,
    new_value: impl Into<String>,
) {
    let old_value = old_value.into();
    let new_value = new_value.into();
    if old_value != new_value {
        changes.push(EntityFieldChange::new(field_name, old_value, new_value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cross_setting() -> HyperliquidPerpLeverageSetting {
        HyperliquidPerpLeverageSetting::new(
            "trader-1".to_string(),
            7,
            HyperliquidPerpMarginMode::Cross,
            5,
            3,
        )
    }

    #[test]
    fn setting_id_is_stable_for_account_asset_and_margin_mode() {
        let setting = cross_setting();

        assert_eq!(setting.setting_id, "trader-1:7:cross");
        assert_eq!(
            HyperliquidPerpLeverageSetting::compose_setting_id(
                "trader-1",
                7,
                HyperliquidPerpMarginMode::Cross,
            ),
            "trader-1:7:cross"
        );
    }

    #[test]
    fn apply_new_leverage_updates_value_and_version() {
        let mut setting = cross_setting();

        setting.apply_new_leverage(10, 4);

        assert_eq!(setting.leverage, 10);
        assert_eq!(setting.version, 4);
    }

    #[test]
    fn diff_projects_leverage_change() {
        let before = cross_setting();
        let mut after = before.clone();
        after.apply_new_leverage(8, 4);

        let changes = before.diff(&after);

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].field_name.as_ref(), "leverage");
        assert_eq!(changes[0].old_value, "5");
        assert_eq!(changes[0].new_value, "8");
    }
}
