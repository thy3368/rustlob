use std::cmp::Ordering;

use common_entity::{Entity, EntityFieldChange, FieldDiff};

use crate::OrderSide;
use crate::entity::{
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
    /// 主动吃单订单 Hyperliquid perp `oid`。
    pub taker_order_id: u64,
    /// 被动挂单订单 Hyperliquid perp `oid`。
    pub maker_order_id: u64,
    /// taker 所属账户 ID。//todo delete
    pub taker_account_id: String,
    /// maker 所属账户 ID。//todo delete
    pub maker_account_id: String,
    /// taker 买卖方向。
    pub taker_side: OrderSide,
    /// 成交价格，取 maker 限价。
    pub price: u64,
    /// 成交数量。
    pub qty: u64,

    // pub closed_pnl: Usdc, // 本次平仓实现盈亏（费前，开仓为 0）
    // pub fee: Usdc,        // 正=用户付平台，负=maker 返佣
    //
    // // ── 上下文 ──
    // pub margin_mode: MarginMode,
    // pub liquidation: Option<LiqMarker>, // 强平标记

    // /// 订单创建时间，单位为 Unix 纳秒。
    // pub created_at: u64,
    // /// 订单最后更新时间，单位为 Unix 纳秒。
    // pub updated_at: u64,
    /// 成交发生时间，单位毫秒。
    pub created_at: u64,
}

/// 一笔 Hyperliquid perp 成交同时落到 taker / maker 两边仓位后的业务结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpTradePositionSettlement {
    /// taker 账户结算后的仓位事实。
    pub taker_position_after: PerpPosition,
    /// maker 账户结算后的仓位事实。
    pub maker_position_after: PerpPosition,
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
        taker_order_id: u64,
        maker_order_id: u64,
        taker_account_id: String,
        maker_account_id: String,
        taker_side: OrderSide,
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
            created_at: executed_at_ms,
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
                    OrderSide::Buy => 1,
                    OrderSide::Sell => -1,
                },
            );
        }
        if account_id == self.maker_account_id {
            return signed_trade_qty(
                self.qty,
                match self.taker_side {
                    OrderSide::Buy => -1,
                    OrderSide::Sell => 1,
                },
            );
        }
        Err(HyperliquidPerpPositionError::InconsistentState)
    }

    /// 可 BDD 规格化的聚合根行为：将一笔成交一次性落到 taker / maker 两边仓位。
    ///
    /// 该方法只派生成交后的仓位事实和单边 outcome；余额、手续费、凭证和持久化仍由清结算 use case
    /// 编排。
    pub fn settle_into_positions(
        &self,
        taker_position: &PerpPosition,
        maker_position: &PerpPosition,
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
        position: &PerpPosition,
    ) -> Result<(PerpPosition, HyperliquidPerpPositionTradeOutcome), HyperliquidPerpPositionError>
    {
        let signed_size_delta = self.signed_size_delta_for_account(&position.account_id)?;
        if !position.trades_asset(self.asset) || !position.trades_symbol(self.symbol.as_str()) {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }
        if !position.has_consistent_state() {
            return Err(HyperliquidPerpPositionError::InconsistentState);
        }
        if position.is_flat() {
            return PerpPosition::open_position(
                position.position_key.clone(),
                position.account_id.clone(),
                position.perp_asset_id,
                position.coin.clone(),
                signed_size_delta,
                self.price,
                position.leverage,
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
        position: &PerpPosition,
        signed_size_delta: i64,
    ) -> Result<(PerpPosition, HyperliquidPerpPositionTradeOutcome), HyperliquidPerpPositionError>
    {
        let close_delta = signed_trade_qty(position.qty(), signed_size_delta.signum())?;
        let remaining_delta = signed_size_delta
            .checked_sub(close_delta)
            .ok_or(HyperliquidPerpPositionError::ArithmeticOverflow)?;
        let mut next_position = position.clone();
        let close_outcome = next_position.close_position(close_delta, self.price)?;
        let (opened_position, open_outcome) = PerpPosition::open_position(
            next_position.position_key.clone(),
            next_position.account_id.clone(),
            next_position.perp_asset_id,
            next_position.coin.clone(),
            remaining_delta,
            self.price,
            next_position.leverage,
            next_position.margin_mode,
        )?;

        next_position.signed_size = opened_position.signed_size;
        next_position.entry_px = opened_position.entry_px;
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
        settlement_id: String,
        margin_asset_id: &str,
        fee_account_id: String,
        taker_fee: u64,
        maker_fee: u64,
        taker_realized_pnl: i64,
        maker_realized_pnl: i64,
    ) -> Option<SettlementTransferVoucher> {
        let mut legs = Vec::with_capacity(0);

        match (taker_realized_pnl.cmp(&0), maker_realized_pnl.cmp(&0)) {
            (Ordering::Greater, Ordering::Less) => {
                let profit = u64::try_from(taker_realized_pnl).ok()?;
                let loss = abs_i64_to_u64(maker_realized_pnl)?;
                if profit != loss {
                    return None;
                }
                legs.push(SettlementTransferLeg::new(
                    perp_transfer_id(
                        "settlement-leg:",
                        settlement_id.as_str(),
                        SettlementTransferPurpose::PerpRealizedPnlTransfer,
                        self.maker_account_id.as_str(),
                        self.taker_account_id.as_str(),
                    ),
                    self.maker_account_id.clone(),
                    self.taker_account_id.clone(),
                    margin_asset_id.to_string(),
                    profit,
                    SettlementTransferPurpose::PerpRealizedPnlTransfer,
                    perp_transfer_id(
                        "balance-ledger:",
                        settlement_id.as_str(),
                        SettlementTransferPurpose::PerpRealizedPnlTransfer,
                        self.maker_account_id.as_str(),
                        self.taker_account_id.as_str(),
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
                    perp_transfer_id(
                        "settlement-leg:",
                        settlement_id.as_str(),
                        SettlementTransferPurpose::PerpRealizedPnlTransfer,
                        self.taker_account_id.as_str(),
                        self.maker_account_id.as_str(),
                    ),
                    self.taker_account_id.clone(),
                    self.maker_account_id.clone(),
                    margin_asset_id.to_string(),
                    profit,
                    SettlementTransferPurpose::PerpRealizedPnlTransfer,
                    perp_transfer_id(
                        "balance-ledger:",
                        settlement_id.as_str(),
                        SettlementTransferPurpose::PerpRealizedPnlTransfer,
                        self.taker_account_id.as_str(),
                        self.maker_account_id.as_str(),
                    ),
                ));
            }
            _ => {}
        }

        if taker_fee > 0 {
            legs.push(SettlementTransferLeg::new(
                perp_role_id(
                    "settlement-leg:",
                    settlement_id.as_str(),
                    SettlementTransferPurpose::TradingFee,
                    "taker",
                ),
                self.taker_account_id.clone(),
                fee_account_id.clone(),
                margin_asset_id.to_string(),
                taker_fee,
                SettlementTransferPurpose::TradingFee,
                perp_role_id(
                    "balance-ledger:",
                    settlement_id.as_str(),
                    SettlementTransferPurpose::TradingFee,
                    "taker",
                ),
            ));
        }

        if maker_fee > 0 {
            legs.push(SettlementTransferLeg::new(
                perp_role_id(
                    "settlement-leg:",
                    settlement_id.as_str(),
                    SettlementTransferPurpose::TradingFee,
                    "maker",
                ),
                self.maker_account_id.clone(),
                fee_account_id.clone(),
                margin_asset_id.to_string(),
                maker_fee,
                SettlementTransferPurpose::TradingFee,
                perp_role_id(
                    "balance-ledger:",
                    settlement_id.as_str(),
                    SettlementTransferPurpose::TradingFee,
                    "maker",
                ),
            ));
        }

        Some(SettlementTransferVoucher::new(
            SettlementKind::Perp,
            settlement_id,
            self.trade_id.clone(),
            Some(self.match_id.clone()),
            fee_account_id,
            legs,
        ))
    }
}

fn perp_transfer_id(
    prefix: &str,
    settlement_id: &str,
    purpose: SettlementTransferPurpose,
    from_account_id: &str,
    to_account_id: &str,
) -> String {
    let mut out = String::with_capacity(128);
    out.push_str(prefix);
    out.push_str(settlement_id);
    out.push(':');
    out.push_str(purpose.as_str());
    out.push(':');
    out.push_str(from_account_id);
    out.push(':');
    out.push_str(to_account_id);
    out
}

fn perp_role_id(
    prefix: &str,
    settlement_id: &str,
    purpose: SettlementTransferPurpose,
    role: &str,
) -> String {
    let mut out = String::with_capacity(96);
    out.push_str(prefix);
    out.push_str(settlement_id);
    out.push(':');
    out.push_str(purpose.as_str());
    out.push(':');
    out.push_str(role);
    out
}

impl FieldDiff for HyperliquidPerpTrade {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("match_id", "", self.match_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("taker_order_id", "", self.taker_order_id.to_string()),
            EntityFieldChange::new("maker_order_id", "", self.maker_order_id.to_string()),
            EntityFieldChange::new("taker_account_id", "", self.taker_account_id.clone()),
            EntityFieldChange::new("maker_account_id", "", self.maker_account_id.clone()),
            EntityFieldChange::new("taker_side", "", self.taker_side.as_str()),
            EntityFieldChange::new("price", "", self.price.to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("executed_at_ms", "", self.created_at.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::with_capacity(0)
    }
}

impl common_entity::Shard for HyperliquidPerpTrade {
    fn partition(&self) -> u8 {
        todo!()
    }
}

impl Entity for HyperliquidPerpTrade {
    fn entity_id(&self) -> u64 {
        common_entity::stable_string_entity_id(&self.trade_id)
    }

    fn is_derived() -> bool {
        false
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_TRADE_ENTITY_TYPE
    }

    fn lsn(&self) -> u64 {
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

