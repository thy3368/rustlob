use std::cmp::Ordering;

use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff};

use crate::HyperliquidPerpOrderSide;
use crate::entity::{
    HyperliquidPerpPosition, HyperliquidPerpPositionError, HyperliquidPerpPositionTradeOutcome,
    SettlementKind, SettlementTransferLeg, SettlementTransferPurpose, SettlementTransferVoucher,
};

const HYPERLIQUID_PERP_TRADE_ENTITY_TYPE: u8 = 10;

/// 已完成撮合的一笔 Hyperliquid perp 成交事实。
///
/// 该实体只记录订单撮合结果，不表达仓位、保证金、手续费、PnL 或资金费。
/// 构造器假定输入已经由撮合 use case 校验。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpTrade {
    /// 本系统稳定成交 ID。
    pub trade_id: String,
    /// 一次撮合批次 ID。
    pub match_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 主动吃单订单 ID。
    pub taker_order_id: String,
    /// 被动挂单订单 ID。
    pub maker_order_id: String,
    /// taker 所属账户 ID。
    pub taker_account_id: String,
    /// maker 所属账户 ID。
    pub maker_account_id: String,
    /// taker 买卖方向。
    pub taker_side: HyperliquidPerpOrderSide,
    /// 成交价格，取 maker 限价。
    pub price: u64,
    /// 成交数量。
    pub qty: u64,
    /// 成交发生时间，单位毫秒。
    pub executed_at_ms: u64,
}

/// 一笔 Hyperliquid perp 成交同时落到 taker / maker 两边仓位后的业务结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpTradePositionSettlement {
    /// taker 账户结算后的仓位事实。
    pub taker_position_after: HyperliquidPerpPosition,
    /// maker 账户结算后的仓位事实。
    pub maker_position_after: HyperliquidPerpPosition,
    /// taker 账户本次成交的 PnL 与保证金变化。
    pub taker_outcome: HyperliquidPerpPositionTradeOutcome,
    /// maker 账户本次成交的 PnL 与保证金变化。
    pub maker_outcome: HyperliquidPerpPositionTradeOutcome,
}

impl HyperliquidPerpTrade {
    /// 从已经校验过的撮合事实构造成交实体。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trade_id: String,
        match_id: String,
        asset: u32,
        symbol: String,
        taker_order_id: String,
        maker_order_id: String,
        taker_account_id: String,
        maker_account_id: String,
        taker_side: HyperliquidPerpOrderSide,
        price: u64,
        qty: u64,
        executed_at_ms: u64,
    ) -> Self {
        Self {
            trade_id,
            match_id,
            asset,
            symbol,
            taker_order_id,
            maker_order_id,
            taker_account_id,
            maker_account_id,
            taker_side,
            price,
            qty,
            executed_at_ms,
        }
    }

    /// 返回成交 quote 名义价值；乘法溢出时返回 `None`。
    pub fn notional_quote(&self) -> Option<u64> {
        self.price.checked_mul(self.qty)
    }

    fn signed_size_delta_for_account(
        &self,
        account_id: &str,
    ) -> Result<i64, HyperliquidPerpPositionError> {
        if account_id == self.taker_account_id {
            return signed_trade_qty(
                self.qty,
                match self.taker_side {
                    HyperliquidPerpOrderSide::Buy => 1,
                    HyperliquidPerpOrderSide::Sell => -1,
                },
            );
        }
        if account_id == self.maker_account_id {
            return signed_trade_qty(
                self.qty,
                match self.taker_side {
                    HyperliquidPerpOrderSide::Buy => -1,
                    HyperliquidPerpOrderSide::Sell => 1,
                },
            );
        }
        Err(HyperliquidPerpPositionError::InconsistentState)
    }

    /// 可 BDD 规格化的聚合根行为：将一笔成交一次性落到 taker / maker 两边仓位。
    ///
    /// 该方法只派生成交后的仓位事实和单边 outcome；余额、手续费、凭证和持久化仍由清结算 use case 编排。
    pub fn settle_into_positions(
        &self,
        taker_position: &HyperliquidPerpPosition,
        maker_position: &HyperliquidPerpPosition,
    ) -> Result<HyperliquidPerpTradePositionSettlement, HyperliquidPerpPositionError> {
        if !taker_position.belongs_to_account(self.taker_account_id.as_str())
            || !maker_position.belongs_to_account(self.maker_account_id.as_str())
        {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }

        let (taker_position_after, taker_outcome) =
            self.settle_one_account_position(taker_position)?;
        let (maker_position_after, maker_outcome) =
            self.settle_one_account_position(maker_position)?;

        Ok(HyperliquidPerpTradePositionSettlement {
            taker_position_after,
            maker_position_after,
            taker_outcome,
            maker_outcome,
        })
    }

    fn settle_one_account_position(
        &self,
        position: &HyperliquidPerpPosition,
    ) -> Result<
        (HyperliquidPerpPosition, HyperliquidPerpPositionTradeOutcome),
        HyperliquidPerpPositionError,
    > {
        let signed_size_delta = self.signed_size_delta_for_account(&position.account_id)?;
        if !position.trades_asset(self.asset) || !position.trades_symbol(self.symbol.as_str()) {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }
        if !position.has_consistent_state() {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }
        if position.is_flat() {
            return HyperliquidPerpPosition::open_position(
                position.position_key.clone(),
                position.account_id.clone(),
                position.perp_asset_id,
                position.coin.clone(),
                signed_size_delta,
                self.price,
                position.leverage_value,
                position.margin_mode,
            );
        }

        let mut next_position = position.clone();
        if position.signed_size.signum() == signed_size_delta.signum() {
            let outcome = next_position.increase_position(signed_size_delta, self.price)?;
            return Ok((next_position, outcome));
        }

        match signed_size_delta.unsigned_abs().cmp(&position.qty()) {
            Ordering::Less => {
                let outcome = next_position.reduce_position(signed_size_delta, self.price)?;
                Ok((next_position, outcome))
            }
            Ordering::Equal => {
                let outcome = next_position.close_position(signed_size_delta, self.price)?;
                Ok((next_position, outcome))
            }
            Ordering::Greater => self.flip_position(position, signed_size_delta),
        }
    }

    fn flip_position(
        &self,
        position: &HyperliquidPerpPosition,
        signed_size_delta: i64,
    ) -> Result<
        (HyperliquidPerpPosition, HyperliquidPerpPositionTradeOutcome),
        HyperliquidPerpPositionError,
    > {
        let close_delta = signed_trade_qty(position.qty(), signed_size_delta.signum())?;
        let remaining_delta = signed_size_delta
            .checked_sub(close_delta)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let mut next_position = position.clone();
        let close_outcome = next_position.close_position(close_delta, self.price)?;
        let (opened_position, open_outcome) = HyperliquidPerpPosition::open_position(
            next_position.position_key.clone(),
            next_position.account_id.clone(),
            next_position.perp_asset_id,
            next_position.coin.clone(),
            remaining_delta,
            self.price,
            next_position.leverage_value,
            next_position.margin_mode,
        )?;

        next_position.signed_size = opened_position.signed_size;
        next_position.entry_price = opened_position.entry_price;
        next_position.cumulative_realized_pnl = opened_position.cumulative_realized_pnl;

        let required_margin_delta = close_outcome
            .required_margin_delta
            .checked_add(open_outcome.required_margin_delta)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        Ok((
            next_position,
            HyperliquidPerpPositionTradeOutcome {
                realized_pnl_delta: close_outcome.realized_pnl_delta,
                required_margin_delta,
            },
        ))
    }

    /// 从永续成交事实与 settlement outcome 派生清结算转账凭证。
    ///
    /// 该凭证只承接“账户间转账”语义：
    /// - 一正一负对冲的 realized pnl，生成 1 条账户间 PnL transfer leg
    /// - taker / maker fee 各自向 `fee_account_id` 支付手续费
    /// - 同账户内 `available / frozen` 的保证金重分类，不在这里建模成 transfer leg
    ///
    /// fee 与 realized pnl 来自 settlement outcome，不属于原始撮合事实，因此由调用方传入。
    /// 若 realized pnl 正负绝对值不一致，或 `i64` 绝对值转换失败，则返回 `None`。
    #[allow(clippy::too_many_arguments)]
    pub fn derive_perp_settlement_transfer_voucher(
        &self,
        voucher_id: String,
        settlement_id: String,
        margin_asset_id: &str,
        fee_account_id: String,
        taker_fee: u64,
        maker_fee: u64,
        taker_realized_pnl: i64,
        maker_realized_pnl: i64,
    ) -> Option<SettlementTransferVoucher> {
        let mut legs = Vec::new();

        match (taker_realized_pnl.cmp(&0), maker_realized_pnl.cmp(&0)) {
            (Ordering::Greater, Ordering::Less) => {
                let profit = u64::try_from(taker_realized_pnl).ok()?;
                let loss = abs_i64_to_u64(maker_realized_pnl)?;
                if profit != loss {
                    return None;
                }
                legs.push(SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        self.maker_account_id,
                        self.taker_account_id
                    ),
                    self.maker_account_id.clone(),
                    self.taker_account_id.clone(),
                    margin_asset_id.to_string(),
                    profit,
                    SettlementTransferPurpose::PerpRealizedPnlTransfer,
                    format!(
                        "balance-ledger:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        self.maker_account_id,
                        self.taker_account_id
                    ),
                ));
            }
            (Ordering::Less, Ordering::Greater) => {
                let profit = u64::try_from(maker_realized_pnl).ok()?;
                let loss = abs_i64_to_u64(taker_realized_pnl)?;
                if profit != loss {
                    return None;
                }
                legs.push(SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        self.taker_account_id,
                        self.maker_account_id
                    ),
                    self.taker_account_id.clone(),
                    self.maker_account_id.clone(),
                    margin_asset_id.to_string(),
                    profit,
                    SettlementTransferPurpose::PerpRealizedPnlTransfer,
                    format!(
                        "balance-ledger:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        self.taker_account_id,
                        self.maker_account_id
                    ),
                ));
            }
            _ => {}
        }

        if taker_fee > 0 {
            legs.push(SettlementTransferLeg::new(
                format!(
                    "settlement-leg:{}:{}:taker",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str()
                ),
                self.taker_account_id.clone(),
                fee_account_id.clone(),
                margin_asset_id.to_string(),
                taker_fee,
                SettlementTransferPurpose::TradingFee,
                format!(
                    "balance-ledger:{}:{}:taker",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str()
                ),
            ));
        }

        if maker_fee > 0 {
            legs.push(SettlementTransferLeg::new(
                format!(
                    "settlement-leg:{}:{}:maker",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str()
                ),
                self.maker_account_id.clone(),
                fee_account_id.clone(),
                margin_asset_id.to_string(),
                maker_fee,
                SettlementTransferPurpose::TradingFee,
                format!(
                    "balance-ledger:{}:{}:maker",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str()
                ),
            ));
        }

        Some(SettlementTransferVoucher::new(
            voucher_id,
            SettlementKind::Perp,
            settlement_id,
            self.trade_id.clone(),
            Some(self.match_id.clone()),
            fee_account_id,
            legs,
        ))
    }
}

impl FieldDiff for HyperliquidPerpTrade {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("match_id", "", self.match_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("taker_order_id", "", self.taker_order_id.clone()),
            EntityFieldChange::new("maker_order_id", "", self.maker_order_id.clone()),
            EntityFieldChange::new("taker_account_id", "", self.taker_account_id.clone()),
            EntityFieldChange::new("maker_account_id", "", self.maker_account_id.clone()),
            EntityFieldChange::new("taker_side", "", self.taker_side.as_str()),
            EntityFieldChange::new("price", "", self.price.to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("executed_at_ms", "", self.executed_at_ms.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Entity for HyperliquidPerpTrade {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.trade_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_TRADE_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "trade_id" | "match_id" | "symbol" | "taker_order_id" | "maker_order_id"
            | "taker_account_id" | "maker_account_id" | "taker_side" => 0,
            "asset" | "price" | "qty" | "executed_at_ms" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.trade_id))
    }
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn abs_i64_to_u64(value: i64) -> Option<u64> {
    value.checked_abs().and_then(|abs| u64::try_from(abs).ok())
}

fn signed_trade_qty(qty: u64, sign: i64) -> Result<i64, HyperliquidPerpPositionError> {
    let qty = i64::try_from(qty).map_err(|_| HyperliquidPerpPositionError::ArithmeticOverflow)?;
    if sign.is_negative() {
        qty.checked_neg().ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)
    } else {
        Ok(qty)
    }
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn trade() -> HyperliquidPerpTrade {
        HyperliquidPerpTrade::new(
            "match-1-1".to_string(),
            "match-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            "taker-1".to_string(),
            "maker-1".to_string(),
            "winner".to_string(),
            "loser".to_string(),
            HyperliquidPerpOrderSide::Buy,
            100,
            2,
            1_717_171_717_000,
        )
    }

    fn sell_trade() -> HyperliquidPerpTrade {
        let mut trade = trade();
        trade.taker_side = HyperliquidPerpOrderSide::Sell;
        trade
    }

    fn empty_position(account_id: &str) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::empty_slot(
            format!("{account_id}:0:BTC-PERP"),
            account_id.to_string(),
            0,
            "BTC-PERP".to_string(),
            10,
        )
    }

    fn position(account_id: &str, signed_size: i64, entry_price: u64) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            format!("{account_id}:0:BTC-PERP"),
            account_id.to_string(),
            0,
            "BTC-PERP".to_string(),
            signed_size,
            entry_price,
            10,
            crate::entity::HyperliquidPerpMarginMode::Cross,
            0,
            3,
        )
    }

    fn bad_market_position(account_id: &str) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::empty_slot(
            format!("{account_id}:1:ETH-PERP"),
            account_id.to_string(),
            1,
            "ETH-PERP".to_string(),
            10,
        )
    }

    #[test]
    fn create_event_contains_trade_fields() {
        let trade = trade();

        let event = trade.track_create_event().unwrap();

        assert!(event.is_created());
        assert_eq!(trade.notional_quote(), Some(200));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("trade_id")
                && change.new_value_bytes() == b"match-1-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("executed_at_ms")
                && change.new_value_bytes() == b"1717171717000"
        }));
    }

    #[test]
    fn signed_size_delta_follows_taker_and_maker_roles() {
        let buy = trade();
        assert_eq!(buy.signed_size_delta_for_account("winner"), Ok(2));
        assert_eq!(buy.signed_size_delta_for_account("loser"), Ok(-2));

        let sell = sell_trade();
        assert_eq!(sell.signed_size_delta_for_account("winner"), Ok(-2));
        assert_eq!(sell.signed_size_delta_for_account("loser"), Ok(2));
        assert_eq!(
            sell.signed_size_delta_for_account("stranger"),
            Err(HyperliquidPerpPositionError::InconsistentState)
        );
    }

    #[test]
    fn settle_into_positions_opens_both_empty_slots() {
        let settlement = trade()
            .settle_into_positions(&empty_position("winner"), &empty_position("loser"))
            .unwrap();

        assert!(settlement.taker_position_after.is_long());
        assert_eq!(settlement.taker_position_after.qty(), 2);
        assert_eq!(settlement.taker_position_after.entry_price, 100);
        assert_eq!(settlement.taker_position_after.version, 1);
        assert_eq!(settlement.taker_outcome.realized_pnl_delta, 0);
        assert_eq!(settlement.taker_outcome.required_margin_delta, 20);
        assert!(settlement.maker_position_after.is_short());
        assert_eq!(settlement.maker_position_after.qty(), 2);
        assert_eq!(settlement.maker_position_after.entry_price, 100);
        assert_eq!(settlement.maker_position_after.version, 1);
        assert_eq!(settlement.maker_outcome.realized_pnl_delta, 0);
        assert_eq!(settlement.maker_outcome.required_margin_delta, 20);
    }

    #[test]
    fn settle_into_positions_increases_taker_and_opens_maker() {
        let settlement = trade()
            .settle_into_positions(&position("winner", 2, 80), &empty_position("loser"))
            .unwrap();

        assert_eq!(settlement.taker_position_after.signed_size, 4);
        assert_eq!(settlement.taker_position_after.entry_price, 90);
        assert_eq!(settlement.taker_position_after.version, 4);
        assert_eq!(settlement.taker_outcome.realized_pnl_delta, 0);
        assert_eq!(settlement.taker_outcome.required_margin_delta, 20);
        assert!(settlement.maker_position_after.is_short());
        assert_eq!(settlement.maker_position_after.qty(), 2);
        assert_eq!(settlement.maker_outcome.required_margin_delta, 20);
    }

    #[test]
    fn settle_into_positions_reduces_both_sides_partially() {
        let settlement = sell_trade()
            .settle_into_positions(&position("winner", 5, 80), &position("loser", -5, 120))
            .unwrap();

        assert_eq!(settlement.taker_position_after.signed_size, 3);
        assert_eq!(settlement.taker_position_after.entry_price, 80);
        assert_eq!(settlement.taker_position_after.cumulative_realized_pnl, 40);
        assert_eq!(settlement.taker_outcome.realized_pnl_delta, 40);
        assert_eq!(settlement.taker_outcome.required_margin_delta, -16);
        assert_eq!(settlement.maker_position_after.signed_size, -3);
        assert_eq!(settlement.maker_position_after.entry_price, 120);
        assert_eq!(settlement.maker_position_after.cumulative_realized_pnl, 40);
        assert_eq!(settlement.maker_outcome.realized_pnl_delta, 40);
        assert_eq!(settlement.maker_outcome.required_margin_delta, -24);
    }

    #[test]
    fn settle_into_positions_closes_both_sides_exactly() {
        let settlement = sell_trade()
            .settle_into_positions(&position("winner", 2, 80), &position("loser", -2, 120))
            .unwrap();

        assert!(settlement.taker_position_after.is_flat());
        assert_eq!(settlement.taker_position_after.entry_price, 0);
        assert_eq!(settlement.taker_position_after.version, 4);
        assert_eq!(settlement.taker_position_after.cumulative_realized_pnl, 40);
        assert_eq!(settlement.taker_outcome.realized_pnl_delta, 40);
        assert_eq!(settlement.taker_outcome.required_margin_delta, -16);
        assert!(settlement.maker_position_after.is_flat());
        assert_eq!(settlement.maker_position_after.cumulative_realized_pnl, 40);
        assert_eq!(settlement.maker_outcome.realized_pnl_delta, 40);
        assert_eq!(settlement.maker_outcome.required_margin_delta, -24);
    }

    #[test]
    fn settle_into_positions_closes_then_opens_when_trade_flips_side() {
        let mut trade = sell_trade();
        trade.qty = 5;
        trade.price = 90;
        let mut taker_position = position("winner", 2, 100);
        taker_position.cumulative_realized_pnl = 25;
        let mut maker_position = position("loser", -2, 80);
        maker_position.cumulative_realized_pnl = 25;

        let settlement = trade.settle_into_positions(&taker_position, &maker_position).unwrap();

        assert!(settlement.taker_position_after.is_short());
        assert_eq!(settlement.taker_position_after.qty(), 3);
        assert_eq!(settlement.taker_position_after.entry_price, 90);
        assert_eq!(settlement.taker_position_after.version, 4);
        assert_eq!(settlement.taker_position_after.cumulative_realized_pnl, 0);
        assert_eq!(settlement.taker_outcome.realized_pnl_delta, -20);
        assert_eq!(settlement.taker_outcome.required_margin_delta, 7);
        assert!(settlement.maker_position_after.is_long());
        assert_eq!(settlement.maker_position_after.qty(), 3);
        assert_eq!(settlement.maker_position_after.entry_price, 90);
        assert_eq!(settlement.maker_position_after.cumulative_realized_pnl, 0);
        assert_eq!(settlement.maker_outcome.realized_pnl_delta, -20);
        assert_eq!(settlement.maker_outcome.required_margin_delta, 11);
    }

    #[test]
    fn settle_into_positions_rejects_swapped_or_mismatched_positions() {
        assert_eq!(
            trade().settle_into_positions(&empty_position("loser"), &empty_position("winner")),
            Err(HyperliquidPerpPositionError::InconsistentState)
        );
        assert_eq!(
            trade().settle_into_positions(&empty_position("winner"), &empty_position("stranger")),
            Err(HyperliquidPerpPositionError::InconsistentState)
        );
        assert_eq!(
            trade().settle_into_positions(&bad_market_position("winner"), &empty_position("loser")),
            Err(HyperliquidPerpPositionError::InconsistentState)
        );
    }

    #[test]
    fn settle_into_positions_rejects_inconsistent_position_state() {
        let mut inconsistent = empty_position("winner");
        inconsistent.entry_price = 100;

        assert_eq!(
            trade().settle_into_positions(&inconsistent, &empty_position("loser")),
            Err(HyperliquidPerpPositionError::InconsistentState)
        );
    }

    #[test]
    fn derive_perp_settlement_transfer_voucher_creates_pnl_and_fee_legs() {
        let voucher = trade()
            .derive_perp_settlement_transfer_voucher(
                "voucher-1".to_string(),
                "settle-1".to_string(),
                "USDC",
                "fee-account".to_string(),
                3,
                2,
                25,
                -25,
            )
            .unwrap();

        let pnl_legs =
            voucher.transfers_for_purpose(SettlementTransferPurpose::PerpRealizedPnlTransfer);
        let fee_legs = voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee);

        assert_eq!(voucher.settlement_kind(), SettlementKind::Perp);
        assert_eq!(pnl_legs.len(), 1);
        assert_eq!(pnl_legs[0].from_account_id(), "loser");
        assert_eq!(pnl_legs[0].to_account_id(), "winner");
        assert_eq!(pnl_legs[0].amount(), 25);
        assert_eq!(fee_legs.len(), 2);
        assert_eq!(voucher.fee_amount_paid_by("winner"), Some(3));
        assert_eq!(voucher.fee_amount_paid_by("loser"), Some(2));
    }

    #[test]
    fn derive_perp_settlement_transfer_voucher_skips_zero_pnl_and_fee() {
        let voucher = trade()
            .derive_perp_settlement_transfer_voucher(
                "voucher-1".to_string(),
                "settle-1".to_string(),
                "USDC",
                "fee-account".to_string(),
                0,
                0,
                0,
                0,
            )
            .unwrap();

        assert!(
            voucher
                .transfers_for_purpose(SettlementTransferPurpose::PerpRealizedPnlTransfer)
                .is_empty()
        );
        assert!(voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee).is_empty());
    }

    #[test]
    fn derive_perp_settlement_transfer_voucher_returns_none_for_bad_pnl_pair() {
        assert_eq!(
            trade().derive_perp_settlement_transfer_voucher(
                "voucher-1".to_string(),
                "settle-1".to_string(),
                "USDC",
                "fee-account".to_string(),
                0,
                0,
                25,
                -24,
            ),
            None
        );
        assert_eq!(
            trade().derive_perp_settlement_transfer_voucher(
                "voucher-1".to_string(),
                "settle-1".to_string(),
                "USDC",
                "fee-account".to_string(),
                0,
                0,
                25,
                i64::MIN,
            ),
            None
        );
    }
}
