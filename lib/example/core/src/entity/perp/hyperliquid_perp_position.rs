use common_entity::{Entity, EntityError, EntityFieldChange};

const HYPERLIQUID_PERP_POSITION_ENTITY_TYPE: u8 = 11;

/// Hyperliquid perp 单向净仓位方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpPositionSide {
    /// 无持仓。
    Flat,
    /// 多头净仓位。
    Long,
    /// 空头净仓位。
    Short,
}

impl HyperliquidPerpPositionSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Long => "long",
            Self::Short => "short",
        }
    }
}

/// 当前仓位在一次资金费结算中的收付方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperliquidPerpFundingDirection {
    /// 当前仓位需要支付资金费。
    Pay,
    /// 当前仓位会收到资金费。
    Receive,
}

/// Hyperliquid perp 账户在单个合约上的 Cross 净仓位。
///
/// `version == 0` 可表示 adapter 加载到的未创建空仓位槽位；结算后若产生非空仓位，
/// use case 会发出 create event。构造器假设输入来自已校验命令或事件回放。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpPosition {
    /// 本系统稳定仓位 ID，建议由 account + asset/symbol 生成。
    pub position_id: String,
    /// 仓位所属账户 ID。
    pub account_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 单向净仓位方向。
    pub side: HyperliquidPerpPositionSide,
    /// 当前净仓位数量。
    pub qty: u64,
    /// 当前仓位均价；空仓时为 0。
    pub entry_price: u64,
    /// Cross 保证金计算使用的杠杆。
    pub leverage: u64,
    /// 当前仓位占用保证金。
    pub margin: u64,
    /// 累计已实现 PnL，允许为负。
    pub realized_pnl: i64,
    /// 当前仓位实体版本。
    pub version: u64,
}

impl HyperliquidPerpPosition {
    /// 从已经校验过的业务事实或回放事件构造 Hyperliquid perp 仓位。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        position_id: String,
        account_id: String,
        asset: u32,
        symbol: String,
        side: HyperliquidPerpPositionSide,
        qty: u64,
        entry_price: u64,
        leverage: u64,
        margin: u64,
        realized_pnl: i64,
        version: u64,
    ) -> Self {
        Self {
            position_id,
            account_id,
            asset,
            symbol,
            side,
            qty,
            entry_price,
            leverage,
            margin,
            realized_pnl,
            version,
        }
    }

    /// 返回尚未创建的空仓位槽位。
    pub fn empty_slot(
        position_id: String,
        account_id: String,
        asset: u32,
        symbol: String,
        leverage: u64,
    ) -> Self {
        Self::new(
            position_id,
            account_id,
            asset,
            symbol,
            HyperliquidPerpPositionSide::Flat,
            0,
            0,
            leverage,
            0,
            0,
            0,
        )
    }

    /// 返回仓位是否属于指定账户。
    pub fn belongs_to_account(&self, account_id: &str) -> bool {
        self.account_id == account_id
    }

    /// 返回仓位是否交易指定 Hyperliquid asset。
    pub fn trades_asset(&self, asset: u32) -> bool {
        self.asset == asset
    }

    /// 返回仓位是否交易指定展示合约。
    pub fn trades_symbol(&self, symbol: &str) -> bool {
        self.symbol == symbol
    }

    /// 返回仓位是否为空。
    pub fn is_flat(&self) -> bool {
        self.side == HyperliquidPerpPositionSide::Flat && self.qty == 0
    }

    /// 返回仓位状态是否和数量、均价、保证金一致。
    pub fn has_consistent_state(&self) -> bool {
        match self.side {
            HyperliquidPerpPositionSide::Flat => {
                self.qty == 0 && self.entry_price == 0 && self.margin == 0
            }
            HyperliquidPerpPositionSide::Long | HyperliquidPerpPositionSide::Short => {
                self.qty > 0 && self.entry_price > 0
            }
        }
    }

    /// 返回当前仓位名义价值；乘法溢出时返回 `None`。
    pub fn notional(&self) -> Option<u64> {
        self.qty.checked_mul(self.entry_price)
    }

    /// 返回当前仓位是否应参与资金费结算。
    pub fn is_funding_eligible(&self) -> bool {
        !self.is_flat() && self.has_consistent_state()
    }

    /// 返回按 oracle 价格计算的资金费名义价值；乘法溢出时返回 `None`。
    pub fn funding_notional(&self, oracle_price: u64) -> Option<u64> {
        self.qty.checked_mul(oracle_price)
    }

    /// 返回当前仓位在指定资金费率下是付款方还是收款方。
    pub fn funding_direction(
        &self,
        funding_rate_e8: i64,
    ) -> Option<HyperliquidPerpFundingDirection> {
        if self.is_flat() || funding_rate_e8 == 0 {
            return None;
        }

        match (self.side, funding_rate_e8.is_positive()) {
            (HyperliquidPerpPositionSide::Long, true)
            | (HyperliquidPerpPositionSide::Short, false) => {
                Some(HyperliquidPerpFundingDirection::Pay)
            }
            (HyperliquidPerpPositionSide::Long, false)
            | (HyperliquidPerpPositionSide::Short, true) => {
                Some(HyperliquidPerpFundingDirection::Receive)
            }
            (HyperliquidPerpPositionSide::Flat, _) => None,
        }
    }

    /// 返回当前仓位在指定 oracle 价格和资金费率下的资金费绝对金额。
    pub fn funding_fee(&self, oracle_price: u64, funding_rate_e8: i64) -> Option<u64> {
        if funding_rate_e8 == 0 {
            return Some(0);
        }
        let notional = self.funding_notional(oracle_price)? as u128;
        let rate = funding_rate_e8.unsigned_abs() as u128;
        let fee = notional.checked_mul(rate)?.checked_div(100_000_000)?;
        u64::try_from(fee).ok()
    }

    /// 返回 `ceil(qty * entry_price / leverage)`；空仓返回 0。
    pub fn required_margin(&self) -> Option<u64> {
        required_position_margin(self.qty, self.entry_price, self.leverage)
    }

    /// 应用已计算好的仓位字段和版本。
    #[allow(clippy::too_many_arguments)]
    pub fn apply_after(
        &mut self,
        side: HyperliquidPerpPositionSide,
        qty: u64,
        entry_price: u64,
        margin: u64,
        realized_pnl: i64,
        version: u64,
    ) {
        self.side = side;
        self.qty = qty;
        self.entry_price = entry_price;
        self.margin = margin;
        self.realized_pnl = realized_pnl;
        self.version = version;
    }
}

impl Entity for HyperliquidPerpPosition {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.position_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_POSITION_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("position_id", "", self.position_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("entry_price", "", self.entry_price.to_string()),
            EntityFieldChange::new("leverage", "", self.leverage.to_string()),
            EntityFieldChange::new("margin", "", self.margin.to_string()),
            EntityFieldChange::new("realized_pnl", "", self.realized_pnl.to_string()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();

        push_change(&mut changes, "account_id", &self.account_id, &other.account_id);
        push_change(&mut changes, "asset", self.asset.to_string(), other.asset.to_string());
        push_change(&mut changes, "symbol", &self.symbol, &other.symbol);
        push_change(&mut changes, "side", self.side.as_str(), other.side.as_str());
        push_change(&mut changes, "qty", self.qty.to_string(), other.qty.to_string());
        push_change(
            &mut changes,
            "entry_price",
            self.entry_price.to_string(),
            other.entry_price.to_string(),
        );
        push_change(
            &mut changes,
            "leverage",
            self.leverage.to_string(),
            other.leverage.to_string(),
        );
        push_change(&mut changes, "margin", self.margin.to_string(), other.margin.to_string());
        push_change(
            &mut changes,
            "realized_pnl",
            self.realized_pnl.to_string(),
            other.realized_pnl.to_string(),
        );

        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "position_id" | "account_id" | "symbol" | "side" => 0,
            "asset" | "qty" | "entry_price" | "leverage" | "margin" | "realized_pnl" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.position_id))
    }
}

/// 返回 `ceil(qty * price / leverage)`，任一算术步骤溢出或杠杆为零时返回 `None`。
pub fn required_position_margin(qty: u64, price: u64, leverage: u64) -> Option<u64> {
    if qty == 0 {
        return Some(0);
    }
    if leverage == 0 {
        return None;
    }
    let notional = qty.checked_mul(price)?;
    let quotient = notional / leverage;
    let remainder = notional % leverage;
    if remainder == 0 { Some(quotient) } else { quotient.checked_add(1) }
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

    #[test]
    fn empty_slot_carries_leverage_for_future_position_creation() {
        let position = HyperliquidPerpPosition::empty_slot(
            "buyer:BTC-PERP".to_string(),
            "buyer".to_string(),
            0,
            "BTC-PERP".to_string(),
            10,
        );

        assert!(position.belongs_to_account("buyer"));
        assert!(position.trades_asset(0));
        assert!(position.trades_symbol("BTC-PERP"));
        assert!(position.is_flat());
        assert!(!position.is_funding_eligible());
        assert_eq!(position.required_margin(), Some(0));
        assert_eq!(position.version, 0);
    }

    #[test]
    fn required_margin_rounds_up() {
        assert_eq!(required_position_margin(3, 101, 5), Some(61));
        assert_eq!(required_position_margin(10, 20, 5), Some(40));
        assert_eq!(required_position_margin(0, 20, 0), Some(0));
        assert_eq!(required_position_margin(1, 1, 0), None);
    }

    #[test]
    fn funding_helpers_follow_hyperliquid_direction_rule() {
        let long = HyperliquidPerpPosition::new(
            "long-1".to_string(),
            "trader-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Long,
            2,
            50_000,
            10,
            10_000,
            0,
            3,
        );
        let short = HyperliquidPerpPosition::new(
            "short-1".to_string(),
            "trader-2".to_string(),
            0,
            "BTC-PERP".to_string(),
            HyperliquidPerpPositionSide::Short,
            2,
            50_000,
            10,
            10_000,
            0,
            3,
        );

        assert!(long.is_funding_eligible());
        assert_eq!(long.funding_notional(60_000), Some(120_000));
        assert_eq!(long.funding_direction(10_000), Some(HyperliquidPerpFundingDirection::Pay));
        assert_eq!(short.funding_direction(10_000), Some(HyperliquidPerpFundingDirection::Receive));
        assert_eq!(long.funding_direction(-10_000), Some(HyperliquidPerpFundingDirection::Receive));
        assert_eq!(long.funding_fee(60_000, 10_000), Some(12));
        assert_eq!(long.funding_fee(60_000, 0), Some(0));
    }
}
