use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff, FourColorArchetype};

const CEX_OPTION_INSTRUMENT_ENTITY_TYPE: u8 = 40;

/// CEX option 合约类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CexOptionType {
    /// 看涨期权。
    Call,
    /// 看跌期权。
    Put,
}

impl CexOptionType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Call => "call",
            Self::Put => "put",
        }
    }
}

/// CEX option 合约挂牌状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CexOptionInstrumentStatus {
    /// 可交易。
    Trading,
    /// 暂停交易。
    Halted,
    /// 已到期。
    Expired,
    /// 已下架。
    Delisted,
}

impl CexOptionInstrumentStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trading => "trading",
            Self::Halted => "halted",
            Self::Expired => "expired",
            Self::Delisted => "delisted",
        }
    }

    pub const fn is_tradable(self) -> bool {
        matches!(self, Self::Trading)
    }
}

/// CEX option 合约规格快照。
///
/// 该实体保存挂牌规则中的稳定事实：标的资产、报价资产、结算资产、到期时间、行权价和
/// Call/Put 类型。订单只引用 `instrument_id`，不复制行权价。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CexOptionInstrument {
    /// 交易所风格合约 ID，例如 `BTC-20260828-100000-PUT`。
    pub instrument_id: String,
    /// 标的资产，例如 `BTC`。
    pub underlying_asset: String,
    /// 权利金报价资产，例如 `USDT`。
    pub quote_asset: String,
    /// 交割或结算资产，例如 `USDT`。
    pub settle_asset: String,
    /// 到期时间，Unix 毫秒。
    pub expiry_time: u64,
    /// 行权价。该价格属于合约规格，不属于订单权利金报价。
    pub strike_price: u64,
    /// Call / Put 类型。
    pub option_type: CexOptionType,
    /// 挂牌状态。
    pub status: CexOptionInstrumentStatus,
    /// 当前合约规格实体版本。
    pub version: u64,
}

impl CexOptionInstrument {
    /// 从已经校验过的业务事实或回放事件构造 CEX option 合约规格快照。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: String,
        underlying_asset: String,
        quote_asset: String,
        settle_asset: String,
        expiry_time: u64,
        strike_price: u64,
        option_type: CexOptionType,
        status: CexOptionInstrumentStatus,
    ) -> Self {
        Self {
            instrument_id,
            underlying_asset,
            quote_asset,
            settle_asset,
            expiry_time,
            strike_price,
            option_type,
            status,
            version: 1,
        }
    }

    /// 返回该合约当前是否允许普通下单。
    pub fn is_tradable(&self) -> bool {
        self.status.is_tradable()
    }
}

impl FieldDiff for CexOptionInstrument {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("instrument_id", "", self.instrument_id.clone()),
            EntityFieldChange::new("underlying_asset", "", self.underlying_asset.clone()),
            EntityFieldChange::new("quote_asset", "", self.quote_asset.clone()),
            EntityFieldChange::new("settle_asset", "", self.settle_asset.clone()),
            EntityFieldChange::new("expiry_time", "", self.expiry_time.to_string()),
            EntityFieldChange::new("strike_price", "", self.strike_price.to_string()),
            EntityFieldChange::new("option_type", "", self.option_type.as_str()),
            EntityFieldChange::new("status", "", self.status.as_str()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        push_change(
            &mut changes,
            "underlying_asset",
            &self.underlying_asset,
            &other.underlying_asset,
        );
        push_change(&mut changes, "quote_asset", &self.quote_asset, &other.quote_asset);
        push_change(&mut changes, "settle_asset", &self.settle_asset, &other.settle_asset);
        push_change(
            &mut changes,
            "expiry_time",
            self.expiry_time.to_string(),
            other.expiry_time.to_string(),
        );
        push_change(
            &mut changes,
            "strike_price",
            self.strike_price.to_string(),
            other.strike_price.to_string(),
        );
        push_change(
            &mut changes,
            "option_type",
            self.option_type.as_str(),
            other.option_type.as_str(),
        );
        push_change(&mut changes, "status", self.status.as_str(), other.status.as_str());
        changes
    }
}

impl Entity for CexOptionInstrument {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.instrument_id.clone()
    }

    fn entity_type() -> u8 {
        CEX_OPTION_INSTRUMENT_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::Description
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "expiry_time" | "strike_price" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.instrument_id))
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
