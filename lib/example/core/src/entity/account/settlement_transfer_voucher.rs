use std::cmp::Ordering;

use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel,
    EntityUseCaseApiSurface, FinancialClassification, FourColorArchetype,
};
use serde::{Deserialize, Serialize};

use crate::{HyperliquidPerpTrade, SpotTrade};

const SETTLEMENT_TRANSFER_VOUCHER_ENTITY_TYPE: u8 = 21;
const SETTLEMENT_TRANSFER_LEG_ENTITY_TYPE: u8 = 22;

/// 统一结算转账凭证的业务类型。
///
/// 该分类只区分凭证服务的是现货还是永续结算；具体资金腿语义由
/// [`SettlementTransferPurpose`] 继续细分。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettlementKind {
    /// 现货成交清结算。
    Spot,
    /// 永续成交或结算清算。
    Perp,
}

impl SettlementKind {
    /// 返回稳定的业务标签。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Spot => "spot",
            Self::Perp => "perp",
        }
    }
}

/// 结算转账腿的业务用途。
///
/// 这些用途只表达“这条腿为什么存在”，不直接驱动余额变化。
/// 真正的余额扣增仍由 `BalanceLedgerEntryV2` 落账。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettlementTransferPurpose {
    /// 现货买方收到 base。
    SpotBuyerReceiveBase,
    /// 现货买方支付 quote。
    SpotBuyerPayQuote,
    /// 现货卖方交付 base。
    SpotSellerDeliverBase,
    /// 现货卖方收到 quote。
    SpotSellerReceiveQuote,
    /// 永续保证金划转。
    PerpMarginTransfer,
    /// 永续已实现盈亏划转。
    PerpRealizedPnlTransfer,
    /// 交易手续费划转。
    TradingFee,
}

impl SettlementTransferPurpose {
    /// 返回稳定的业务标签。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpotBuyerReceiveBase => "spot_buyer_receive_base",
            Self::SpotBuyerPayQuote => "spot_buyer_pay_quote",
            Self::SpotSellerDeliverBase => "spot_seller_deliver_base",
            Self::SpotSellerReceiveQuote => "spot_seller_receive_quote",
            Self::PerpMarginTransfer => "perp_margin_transfer",
            Self::PerpRealizedPnlTransfer => "perp_realized_pnl_transfer",
            Self::TradingFee => "trading_fee",
        }
    }

    /// 返回该用途是否属于手续费腿。
    pub const fn is_fee(self) -> bool {
        matches!(self, Self::TradingFee)
    }
}

/// 统一结算转账凭证中的一条资金腿。
///
/// 一条腿对应一次独立的账户间资产移动，并绑定到底层一条余额流水。
/// 在聚合边界上，它只作为 `SettlementTransferVoucher` 的聚合成员存在。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementTransferLeg {
    /// 腿 ID。
    leg_id: String,
    /// 付款账户 ID。
    from_account_id: String,
    /// 收款账户 ID。
    to_account_id: String,
    /// 资产 ID。
    asset_id: String,
    /// 划转数量。
    amount: u64,
    /// 该腿的业务用途。
    purpose: SettlementTransferPurpose,
    /// 对应底层余额流水 ID。
    balance_ledger_entry_id: String,
}

impl SettlementTransferLeg {
    /// 从已经校验过的结算事实构造一条资金腿。
    pub(crate) fn new(
        leg_id: String,
        from_account_id: String,
        to_account_id: String,
        asset_id: String,
        amount: u64,
        purpose: SettlementTransferPurpose,
        balance_ledger_entry_id: String,
    ) -> Self {
        Self {
            leg_id,
            from_account_id,
            to_account_id,
            asset_id,
            amount,
            purpose,
            balance_ledger_entry_id,
        }
    }

    /// 返回腿 ID。
    pub fn leg_id(&self) -> &str {
        self.leg_id.as_str()
    }

    /// 返回付款账户 ID。
    pub fn from_account_id(&self) -> &str {
        self.from_account_id.as_str()
    }

    /// 返回收款账户 ID。
    pub fn to_account_id(&self) -> &str {
        self.to_account_id.as_str()
    }

    /// 返回资产 ID。
    pub fn asset_id(&self) -> &str {
        self.asset_id.as_str()
    }

    /// 返回划转数量。
    pub const fn amount(&self) -> u64 {
        self.amount
    }

    /// 返回业务用途。
    pub const fn purpose(&self) -> SettlementTransferPurpose {
        self.purpose
    }

    /// 返回底层余额流水 ID。
    pub fn balance_ledger_entry_id(&self) -> &str {
        self.balance_ledger_entry_id.as_str()
    }

    /// 返回该腿是否涉及指定账户。
    pub fn references_account(&self, account_id: &str) -> bool {
        self.from_account_id == account_id || self.to_account_id == account_id
    }

    /// 返回该腿是否引用指定底层余额流水。
    pub fn references_ledger_entry(&self, entry_id: &str) -> bool {
        self.balance_ledger_entry_id == entry_id
    }

    /// 返回该腿是否为手续费腿。
    pub fn is_fee_leg(&self) -> bool {
        self.purpose.is_fee()
    }
}

impl Entity for SettlementTransferLeg {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.leg_id.clone()
    }

    fn entity_type() -> u8 {
        SETTLEMENT_TRANSFER_LEG_ENTITY_TYPE
    }

    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateMember
    }

    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::AppendOnlyRecord
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("leg_id", "", self.leg_id.clone()),
            EntityFieldChange::new("from_account_id", "", self.from_account_id.clone()),
            EntityFieldChange::new("to_account_id", "", self.to_account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("amount", "", self.amount.to_string()),
            EntityFieldChange::new("purpose", "", self.purpose.as_str()),
            EntityFieldChange::new(
                "balance_ledger_entry_id",
                "",
                self.balance_ledger_entry_id.clone(),
            ),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        if field_name == "amount" { 1 } else { 0 }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.leg_id))
    }
}

/// 一条面向 use case 暴露的稳定转账结论。
///
/// 它只暴露稳定业务事实：谁向谁转了什么资产、多少数量；
/// 不把底层 `SettlementTransferLeg` 的内部 ID 与落账材料暴露给 use case。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettlementTransferSummary<'a> {
    from_account_id: &'a str,
    to_account_id: &'a str,
    asset_id: &'a str,
    amount: u64,
}

impl<'a> SettlementTransferSummary<'a> {
    fn from_leg(leg: &'a SettlementTransferLeg) -> Self {
        Self {
            from_account_id: leg.from_account_id(),
            to_account_id: leg.to_account_id(),
            asset_id: leg.asset_id(),
            amount: leg.amount(),
        }
    }

    /// 返回付款账户 ID。
    pub fn from_account_id(&self) -> &str {
        self.from_account_id
    }

    /// 返回收款账户 ID。
    pub fn to_account_id(&self) -> &str {
        self.to_account_id
    }

    /// 返回该业务结论对应的资产 ID。
    pub fn asset_id(&self) -> &str {
        self.asset_id
    }

    /// 返回该业务结论对应的数量。
    pub const fn amount(&self) -> u64 {
        self.amount
    }
}

/// 一张统一收拢多腿结算转账事实的审计凭证。
///
/// `SettlementTransferVoucher` 是 `trade settlement` 上层的 Moment-Interval 审计事实，
/// 用来描述一次结算到底发生了哪些 principal token 与 fee token 划转。
/// 它本身不直接修改余额；真实余额变化仍由 `BalanceLedgerEntryV2` 独立落账。
///
/// 该实体属于 `MomentInterval + AppendOnlyRecord + AggregateRoot`，
/// spot/perp 共用同一凭证壳体，内部 `legs` 作为聚合成员存在。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementTransferVoucher {
    /// 凭证 ID。
    voucher_id: String,
    /// 结算业务类型。
    settlement_kind: SettlementKind,
    /// 上层结算记录 ID。
    settlement_id: String,
    /// 关联成交 ID。
    trade_id: String,
    /// 关联撮合批次 ID。
    match_id: Option<String>,
    /// 平台手续费账户 ID。
    fee_account_id: String,
    /// 该次结算下所有资金腿。
    legs: Vec<SettlementTransferLeg>,
}

impl SettlementTransferVoucher {
    /// [General] 从已经校验过的结算事实构造统一转账凭证。
    ///
    /// 构造器只负责装配事实，不直接校验余额是否足够，也不直接修改余额。
    pub(crate) fn new(
        voucher_id: String,
        settlement_kind: SettlementKind,
        settlement_id: String,
        trade_id: String,
        match_id: Option<String>,
        fee_account_id: String,
        legs: Vec<SettlementTransferLeg>,
    ) -> Self {
        Self {
            voucher_id,
            settlement_kind,
            settlement_id,
            trade_id,
            match_id,
            fee_account_id,
            legs,
        }
    }

    /// 返回凭证 ID。
    pub fn voucher_id(&self) -> &str {
        self.voucher_id.as_str()
    }

    /// 返回结算业务类型。
    pub const fn settlement_kind(&self) -> SettlementKind {
        self.settlement_kind
    }

    /// 返回上层结算记录 ID。
    pub fn settlement_id(&self) -> &str {
        self.settlement_id.as_str()
    }

    /// 返回关联成交 ID。
    pub fn trade_id(&self) -> &str {
        self.trade_id.as_str()
    }

    /// 返回关联撮合批次 ID。
    pub fn match_id(&self) -> Option<&str> {
        self.match_id.as_deref()
    }

    /// 返回手续费账户 ID。
    pub fn fee_account_id(&self) -> &str {
        self.fee_account_id.as_str()
    }

    // ===== 面向 spot settlement use case 的业务方法 =====

    /// [UseCase-facing] 从一笔现货成交构造 principal-only settlement voucher。
    ///
    /// 当前只生成 4 条 principal legs，不包含任何手续费腿。
    /// 若 `price * qty` 溢出，则返回 `None`。
    pub fn build_spot_principal_voucher(
        voucher_id: String,
        settlement_id: String,
        trade: &SpotTrade,
        base_asset_id: &str,
        quote_asset_id: &str,
        fee_account_id: String,
    ) -> Option<Self> {
        let quote_amount = trade.notional_quote()?;
        let buyer_account_id = trade.buyer_account_id();
        let seller_account_id = trade.seller_account_id();

        Some(Self::new(
            voucher_id,
            SettlementKind::Spot,
            settlement_id.clone(),
            trade.trade_id.clone(),
            Some(trade.match_id.clone()),
            fee_account_id,
            vec![
                SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotBuyerReceiveBase.as_str()
                    ),
                    seller_account_id.to_string(),
                    buyer_account_id.to_string(),
                    base_asset_id.to_string(),
                    trade.qty,
                    SettlementTransferPurpose::SpotBuyerReceiveBase,
                    format!(
                        "balance-ledger:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotBuyerReceiveBase.as_str()
                    ),
                ),
                SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotBuyerPayQuote.as_str()
                    ),
                    buyer_account_id.to_string(),
                    seller_account_id.to_string(),
                    quote_asset_id.to_string(),
                    quote_amount,
                    SettlementTransferPurpose::SpotBuyerPayQuote,
                    format!(
                        "balance-ledger:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotBuyerPayQuote.as_str()
                    ),
                ),
                SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotSellerReceiveQuote.as_str()
                    ),
                    buyer_account_id.to_string(),
                    seller_account_id.to_string(),
                    quote_asset_id.to_string(),
                    quote_amount,
                    SettlementTransferPurpose::SpotSellerReceiveQuote,
                    format!(
                        "balance-ledger:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotSellerReceiveQuote.as_str()
                    ),
                ),
                SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotSellerDeliverBase.as_str()
                    ),
                    seller_account_id.to_string(),
                    buyer_account_id.to_string(),
                    base_asset_id.to_string(),
                    trade.qty,
                    SettlementTransferPurpose::SpotSellerDeliverBase,
                    format!(
                        "balance-ledger:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotSellerDeliverBase.as_str()
                    ),
                ),
            ],
        ))
    }

    /// [UseCase-facing] 从一笔现货成交构造同时包含 principal 与 fee 的 settlement voucher。
    ///
    /// fee 统一以 quote 资产计价，且买卖双方各自向 `fee_account_id` 支付自己真实角色下的 fee。
    /// 若 quote notional 溢出则返回 `None`。
    pub fn build_spot_voucher_with_fees(
        voucher_id: String,
        settlement_id: String,
        trade: &SpotTrade,
        base_asset_id: &str,
        quote_asset_id: &str,
        fee_account_id: String,
        buyer_fee: u64,
        seller_fee: u64,
    ) -> Option<Self> {
        let mut voucher = Self::build_spot_principal_voucher(
            voucher_id,
            settlement_id.clone(),
            trade,
            base_asset_id,
            quote_asset_id,
            fee_account_id.clone(),
        )?;

        if buyer_fee > 0 {
            voucher.legs.push(SettlementTransferLeg::new(
                format!(
                    "settlement-leg:{}:{}:{}",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str(),
                    trade.buyer_fee_role().as_str()
                ),
                trade.buyer_account_id().to_string(),
                fee_account_id.clone(),
                quote_asset_id.to_string(),
                buyer_fee,
                SettlementTransferPurpose::TradingFee,
                format!(
                    "balance-ledger:{}:{}:{}",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str(),
                    trade.buyer_fee_role().as_str()
                ),
            ));
        }

        if seller_fee > 0 {
            voucher.legs.push(SettlementTransferLeg::new(
                format!(
                    "settlement-leg:{}:{}:{}",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str(),
                    trade.seller_fee_role().as_str()
                ),
                trade.seller_account_id().to_string(),
                fee_account_id.clone(),
                quote_asset_id.to_string(),
                seller_fee,
                SettlementTransferPurpose::TradingFee,
                format!(
                    "balance-ledger:{}:{}:{}",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str(),
                    trade.seller_fee_role().as_str()
                ),
            ));
        }

        Some(voucher)
    }

    /// [UseCase-facing] 从一笔永续成交构造清结算转账凭证。
    ///
    /// 该凭证只承接“账户间转账”语义：
    /// - 一正一负对冲的 realized pnl，生成 1 条账户间 PnL transfer leg
    /// - taker / maker fee 各自向 `fee_account_id` 支付手续费
    /// - 同账户内 `available / frozen` 的保证金重分类，不在这里建模成 transfer leg
    ///
    /// 若 realized pnl 正负绝对值不一致，或 `i64` 绝对值转换失败，则返回 `None`。
    pub fn build_perp_voucher(
        voucher_id: String,
        settlement_id: String,
        trade: &HyperliquidPerpTrade,
        margin_asset_id: &str,
        fee_account_id: String,
        taker_fee: u64,
        maker_fee: u64,
        taker_realized_pnl: i64,
        maker_realized_pnl: i64,
    ) -> Option<Self> {
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
                        trade.maker_account_id,
                        trade.taker_account_id
                    ),
                    trade.maker_account_id.clone(),
                    trade.taker_account_id.clone(),
                    margin_asset_id.to_string(),
                    profit,
                    SettlementTransferPurpose::PerpRealizedPnlTransfer,
                    format!(
                        "balance-ledger:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        trade.maker_account_id,
                        trade.taker_account_id
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
                        trade.taker_account_id,
                        trade.maker_account_id
                    ),
                    trade.taker_account_id.clone(),
                    trade.maker_account_id.clone(),
                    margin_asset_id.to_string(),
                    profit,
                    SettlementTransferPurpose::PerpRealizedPnlTransfer,
                    format!(
                        "balance-ledger:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        trade.taker_account_id,
                        trade.maker_account_id
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
                trade.taker_account_id.clone(),
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
                trade.maker_account_id.clone(),
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

        Some(Self::new(
            voucher_id,
            SettlementKind::Perp,
            settlement_id,
            trade.trade_id.clone(),
            Some(trade.match_id.clone()),
            fee_account_id,
            legs,
        ))
    }

    // ===== 通用业务查询 / 不变量方法 =====

    /// [General] 返回该凭证是否引用指定账户。
    pub fn references_account(&self, account_id: &str) -> bool {
        self.legs.iter().any(|leg| leg.references_account(account_id))
    }

    /// [General] 返回指定业务用途下的稳定转账结论。
    ///
    /// 返回值只包含业务方通常稳定依赖的付款方、收款方、资产与数量，
    /// 不返回底层 `leg_id`、`balance_ledger_entry_id` 等聚合内部材料。
    pub fn transfers_for_purpose(
        &self,
        purpose: SettlementTransferPurpose,
    ) -> Vec<SettlementTransferSummary<'_>> {
        self.legs
            .iter()
            .filter(|leg| leg.purpose() == purpose)
            .map(SettlementTransferSummary::from_leg)
            .collect()
    }

    /// [General] 汇总指定账户在某一业务用途下支付的数量。
    pub fn amount_sent_by_for_purpose(
        &self,
        account_id: &str,
        purpose: SettlementTransferPurpose,
    ) -> Option<u64> {
        self.sum_amount_by(account_id, purpose, TransferDirection::Sent)
    }

    /// [General] 汇总指定账户在某一业务用途下收到的数量。
    pub fn amount_received_by_for_purpose(
        &self,
        account_id: &str,
        purpose: SettlementTransferPurpose,
    ) -> Option<u64> {
        self.sum_amount_by(account_id, purpose, TransferDirection::Received)
    }

    /// [General] 汇总指定账户支付的手续费数量。
    ///
    /// 仅统计 `TradingFee` 且 `from_account_id` 命中该账户的腿。
    /// 若没有任何手续费腿则返回 `None`；若求和溢出也返回 `None`。
    pub fn fee_amount_paid_by(&self, account_id: &str) -> Option<u64> {
        self.amount_sent_by_for_purpose(account_id, SettlementTransferPurpose::TradingFee)
    }

    /// [General] 返回该凭证是否引用指定底层余额流水。
    pub fn references_ledger_entry(&self, entry_id: &str) -> bool {
        self.legs.iter().any(|leg| leg.references_ledger_entry(entry_id))
    }

    fn sum_amount_by(
        &self,
        account_id: &str,
        purpose: SettlementTransferPurpose,
        direction: TransferDirection,
    ) -> Option<u64> {
        let mut total = 0_u64;
        let mut matched = false;

        for leg in self.legs.iter().filter(|leg| leg.purpose() == purpose) {
            let account_matches = match direction {
                TransferDirection::Sent => leg.from_account_id() == account_id,
                TransferDirection::Received => leg.to_account_id() == account_id,
            };

            if account_matches {
                matched = true;
                total = total.checked_add(leg.amount())?;
            }
        }

        if matched { Some(total) } else { None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferDirection {
    Sent,
    Received,
}

impl Entity for SettlementTransferVoucher {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.voucher_id.clone()
    }

    fn entity_type() -> u8 {
        SETTLEMENT_TRANSFER_VOUCHER_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::AppendOnlyRecord
    }

    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification {
        FinancialClassification::AccountingVoucher
    }

    fn use_case_api_surface() -> EntityUseCaseApiSurface {
        EntityUseCaseApiSurface::MinimalBusinessApi
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let mut changes = vec![
            EntityFieldChange::new("voucher_id", "", self.voucher_id.clone()),
            EntityFieldChange::new("settlement_kind", "", self.settlement_kind.as_str()),
            EntityFieldChange::new("settlement_id", "", self.settlement_id.clone()),
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("match_id", "", self.match_id.clone().unwrap_or_default()),
            EntityFieldChange::new("fee_account_id", "", self.fee_account_id.clone()),
            EntityFieldChange::new("leg_count", "", self.legs.len().to_string()),
        ];

        for (index, leg) in self.legs.iter().enumerate() {
            let prefix = format!("leg_{index}");
            changes.push(EntityFieldChange::new(
                format!("{prefix}_leg_id"),
                "",
                leg.leg_id.clone(),
            ));
            changes.push(EntityFieldChange::new(
                format!("{prefix}_from_account_id"),
                "",
                leg.from_account_id.clone(),
            ));
            changes.push(EntityFieldChange::new(
                format!("{prefix}_to_account_id"),
                "",
                leg.to_account_id.clone(),
            ));
            changes.push(EntityFieldChange::new(
                format!("{prefix}_asset_id"),
                "",
                leg.asset_id.clone(),
            ));
            changes.push(EntityFieldChange::new(
                format!("{prefix}_amount"),
                "",
                leg.amount.to_string(),
            ));
            changes.push(EntityFieldChange::new(
                format!("{prefix}_purpose"),
                "",
                leg.purpose.as_str(),
            ));
            changes.push(EntityFieldChange::new(
                format!("{prefix}_balance_ledger_entry_id"),
                "",
                leg.balance_ledger_entry_id.clone(),
            ));
        }

        changes
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        if matches!(field_name, "leg_count")
            || (field_name.starts_with("leg_") && field_name.ends_with("_amount"))
        {
            1
        } else {
            0
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.voucher_id))
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

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn perp_trade() -> HyperliquidPerpTrade {
        HyperliquidPerpTrade::new(
            "trade-perp-1".to_string(),
            "match-perp-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            "taker-order-1".to_string(),
            "maker-order-1".to_string(),
            "winner".to_string(),
            "loser".to_string(),
            crate::HyperliquidPerpOrderSide::Buy,
            100,
            1,
            1_717_171_717_000,
        )
    }

    fn spot_voucher() -> SettlementTransferVoucher {
        SettlementTransferVoucher::new(
            "voucher-spot-1".to_string(),
            SettlementKind::Spot,
            "settlement-spot-1".to_string(),
            "trade-spot-1".to_string(),
            Some("match-spot-1".to_string()),
            "fee-account".to_string(),
            vec![
                SettlementTransferLeg::new(
                    "leg-1".to_string(),
                    "seller".to_string(),
                    "buyer".to_string(),
                    "BTC".to_string(),
                    2,
                    SettlementTransferPurpose::SpotBuyerReceiveBase,
                    "ledger-1".to_string(),
                ),
                SettlementTransferLeg::new(
                    "leg-2".to_string(),
                    "buyer".to_string(),
                    "seller".to_string(),
                    "USDT".to_string(),
                    200,
                    SettlementTransferPurpose::SpotSellerReceiveQuote,
                    "ledger-2".to_string(),
                ),
                SettlementTransferLeg::new(
                    "leg-3".to_string(),
                    "buyer".to_string(),
                    "fee-account".to_string(),
                    "USDT".to_string(),
                    3,
                    SettlementTransferPurpose::TradingFee,
                    "ledger-3".to_string(),
                ),
                SettlementTransferLeg::new(
                    "leg-4".to_string(),
                    "seller".to_string(),
                    "fee-account".to_string(),
                    "USDT".to_string(),
                    2,
                    SettlementTransferPurpose::TradingFee,
                    "ledger-4".to_string(),
                ),
            ],
        )
    }

    fn perp_voucher() -> SettlementTransferVoucher {
        SettlementTransferVoucher::build_perp_voucher(
            "voucher-perp-1".to_string(),
            "settlement-perp-1".to_string(),
            &perp_trade(),
            "USDC",
            "fee-account".to_string(),
            1,
            0,
            25,
            -25,
        )
        .unwrap()
    }

    #[test]
    fn constructor_stores_spot_voucher_facts() {
        let voucher = spot_voucher();
        let buyer_receive_base =
            voucher.transfers_for_purpose(SettlementTransferPurpose::SpotBuyerReceiveBase);

        assert_eq!(SettlementTransferVoucher::aggregate_role(), AggregateRole::AggregateRoot);
        assert_eq!(
            SettlementTransferVoucher::financial_classification(),
            FinancialClassification::AccountingVoucher
        );
        assert_eq!(
            SettlementTransferVoucher::use_case_api_surface(),
            EntityUseCaseApiSurface::MinimalBusinessApi
        );
        assert_eq!(SettlementTransferLeg::aggregate_role(), AggregateRole::AggregateMember);
        assert_eq!(voucher.voucher_id(), "voucher-spot-1");
        assert_eq!(voucher.settlement_kind(), SettlementKind::Spot);
        assert_eq!(voucher.trade_id(), "trade-spot-1");
        assert_eq!(voucher.match_id(), Some("match-spot-1"));
        assert_eq!(voucher.fee_account_id(), "fee-account");
        assert!(voucher.references_account("buyer"));
        assert_eq!(buyer_receive_base.len(), 1);
        assert_eq!(buyer_receive_base[0].from_account_id(), "seller");
        assert_eq!(buyer_receive_base[0].to_account_id(), "buyer");
        assert_eq!(buyer_receive_base[0].asset_id(), "BTC");
        assert_eq!(buyer_receive_base[0].amount(), 2);
        assert_eq!(voucher.fee_amount_paid_by("buyer"), Some(3));
    }

    #[test]
    fn build_spot_principal_voucher_creates_expected_spot_settlement_facts() {
        let trade = SpotTrade::new(
            "trade-1".to_string(),
            "match-1".to_string(),
            10_001,
            "BTCUSDT".to_string(),
            "taker-1".to_string(),
            "maker-1".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            crate::SpotOrderSide::Buy,
            100,
            2,
        );

        let voucher = SettlementTransferVoucher::build_spot_principal_voucher(
            "voucher-1".to_string(),
            "settle-1-1".to_string(),
            &trade,
            "BTC",
            "USDT",
            String::new(),
        )
        .unwrap();

        let buyer_receive_base =
            voucher.transfers_for_purpose(SettlementTransferPurpose::SpotBuyerReceiveBase);
        let buyer_pay_quote =
            voucher.transfers_for_purpose(SettlementTransferPurpose::SpotBuyerPayQuote);
        let seller_receive_quote =
            voucher.transfers_for_purpose(SettlementTransferPurpose::SpotSellerReceiveQuote);
        let seller_deliver_base =
            voucher.transfers_for_purpose(SettlementTransferPurpose::SpotSellerDeliverBase);

        assert_eq!(voucher.voucher_id(), "voucher-1");
        assert_eq!(voucher.settlement_id(), "settle-1-1");
        assert_eq!(voucher.trade_id(), "trade-1");
        assert_eq!(voucher.match_id(), Some("match-1"));
        assert_eq!(voucher.settlement_kind(), SettlementKind::Spot);
        assert_eq!(buyer_receive_base.len(), 1);
        assert_eq!(buyer_receive_base[0].asset_id(), "BTC");
        assert_eq!(buyer_receive_base[0].amount(), 2);
        assert_eq!(buyer_pay_quote.len(), 1);
        assert_eq!(buyer_pay_quote[0].asset_id(), "USDT");
        assert_eq!(buyer_pay_quote[0].amount(), 200);
        assert_eq!(seller_receive_quote.len(), 1);
        assert_eq!(seller_receive_quote[0].asset_id(), "USDT");
        assert_eq!(seller_receive_quote[0].amount(), 200);
        assert_eq!(seller_deliver_base.len(), 1);
        assert_eq!(seller_deliver_base[0].asset_id(), "BTC");
        assert_eq!(seller_deliver_base[0].amount(), 2);
    }

    #[test]
    fn build_spot_principal_voucher_returns_none_when_notional_overflows() {
        let trade = SpotTrade::new(
            "trade-overflow".to_string(),
            "match-overflow".to_string(),
            10_001,
            "BTCUSDT".to_string(),
            "taker-overflow".to_string(),
            "maker-overflow".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            crate::SpotOrderSide::Buy,
            u64::MAX,
            2,
        );

        assert_eq!(
            SettlementTransferVoucher::build_spot_principal_voucher(
                "voucher-overflow".to_string(),
                "settle-overflow-1".to_string(),
                &trade,
                "BTC",
                "USDT",
                String::new(),
            ),
            None
        );
    }

    #[test]
    fn build_spot_voucher_with_fees_creates_principal_and_fee_legs() {
        let trade = SpotTrade::new(
            "trade-fee-1".to_string(),
            "match-fee-1".to_string(),
            10_001,
            "BTCUSDT".to_string(),
            "taker-fee-1".to_string(),
            "maker-fee-1".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            crate::SpotOrderSide::Buy,
            100,
            2,
        );

        let voucher = SettlementTransferVoucher::build_spot_voucher_with_fees(
            "voucher-fee-1".to_string(),
            "settle-fee-1".to_string(),
            &trade,
            "BTC",
            "USDT",
            "fee-account".to_string(),
            1,
            2,
        )
        .unwrap();

        let fee_legs = voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee);
        assert_eq!(voucher.settlement_kind(), SettlementKind::Spot);
        assert_eq!(voucher.fee_account_id(), "fee-account");
        assert_eq!(fee_legs.len(), 2);
        assert_eq!(voucher.fee_amount_paid_by("buyer"), Some(1));
        assert_eq!(voucher.fee_amount_paid_by("seller"), Some(2));
        assert_eq!(fee_legs[0].to_account_id(), "fee-account");
        assert_eq!(fee_legs[0].asset_id(), "USDT");
    }

    #[test]
    fn constructor_stores_perp_voucher_facts() {
        let voucher = perp_voucher();

        assert_eq!(voucher.settlement_kind(), SettlementKind::Perp);
        assert_eq!(voucher.match_id(), Some("match-perp-1"));
        assert!(voucher.references_account("winner"));
        assert_eq!(voucher.fee_amount_paid_by("winner"), Some(1));
        assert_eq!(
            voucher.transfers_for_purpose(SettlementTransferPurpose::PerpRealizedPnlTransfer).len(),
            1
        );
    }

    #[test]
    fn build_perp_voucher_with_only_fee_creates_only_fee_legs() {
        let trade = perp_trade();
        let voucher = SettlementTransferVoucher::build_perp_voucher(
            "voucher-perp-fee".to_string(),
            "settlement-perp-fee".to_string(),
            &trade,
            "USDC",
            "fee-account".to_string(),
            3,
            2,
            0,
            0,
        )
        .unwrap();

        let pnl_legs =
            voucher.transfers_for_purpose(SettlementTransferPurpose::PerpRealizedPnlTransfer);
        let fee_legs = voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee);

        assert_eq!(voucher.trade_id(), "trade-perp-1");
        assert_eq!(voucher.match_id(), Some("match-perp-1"));
        assert_eq!(voucher.settlement_kind(), SettlementKind::Perp);
        assert_eq!(voucher.fee_account_id(), "fee-account");
        assert!(pnl_legs.is_empty());
        assert_eq!(fee_legs.len(), 2);
        assert_eq!(voucher.fee_amount_paid_by("winner"), Some(3));
        assert_eq!(voucher.fee_amount_paid_by("loser"), Some(2));
    }

    #[test]
    fn build_perp_voucher_with_only_realized_pnl_creates_pnl_transfer_leg() {
        let trade = perp_trade();
        let voucher = SettlementTransferVoucher::build_perp_voucher(
            "voucher-perp-pnl".to_string(),
            "settlement-perp-pnl".to_string(),
            &trade,
            "USDC",
            "fee-account".to_string(),
            0,
            0,
            25,
            -25,
        )
        .unwrap();

        let pnl_legs =
            voucher.transfers_for_purpose(SettlementTransferPurpose::PerpRealizedPnlTransfer);
        let fee_legs = voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee);

        assert_eq!(pnl_legs.len(), 1);
        assert_eq!(pnl_legs[0].from_account_id(), "loser");
        assert_eq!(pnl_legs[0].to_account_id(), "winner");
        assert_eq!(pnl_legs[0].asset_id(), "USDC");
        assert_eq!(pnl_legs[0].amount(), 25);
        assert!(fee_legs.is_empty());
    }

    #[test]
    fn build_perp_voucher_with_fee_and_realized_pnl_keeps_all_legs() {
        let trade = perp_trade();
        let voucher = SettlementTransferVoucher::build_perp_voucher(
            "voucher-perp-all".to_string(),
            "settlement-perp-all".to_string(),
            &trade,
            "USDC",
            "fee-account".to_string(),
            4,
            1,
            25,
            -25,
        )
        .unwrap();

        let pnl_legs =
            voucher.transfers_for_purpose(SettlementTransferPurpose::PerpRealizedPnlTransfer);
        let fee_legs = voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee);

        assert_eq!(pnl_legs.len(), 1);
        assert_eq!(fee_legs.len(), 2);
        assert_eq!(voucher.fee_amount_paid_by("winner"), Some(4));
        assert_eq!(voucher.fee_amount_paid_by("loser"), Some(1));
    }

    #[test]
    fn build_perp_voucher_skips_zero_fee_and_zero_realized_pnl_legs() {
        let trade = perp_trade();
        let voucher = SettlementTransferVoucher::build_perp_voucher(
            "voucher-perp-empty".to_string(),
            "settlement-perp-empty".to_string(),
            &trade,
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
        assert_eq!(voucher.fee_amount_paid_by("winner"), None);
        assert_eq!(voucher.fee_amount_paid_by("loser"), None);
    }

    #[test]
    fn business_queries_return_semantic_results_without_exposing_raw_legs() {
        let voucher = spot_voucher();
        let trade = SpotTrade::new(
            "trade-semantic-1".to_string(),
            "match-semantic-1".to_string(),
            10_002,
            "BTCUSDT".to_string(),
            "taker-semantic-1".to_string(),
            "maker-semantic-1".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            crate::SpotOrderSide::Buy,
            100,
            2,
        );
        let principal_voucher = SettlementTransferVoucher::build_spot_principal_voucher(
            "voucher-semantic-1".to_string(),
            "settle-semantic-1".to_string(),
            &trade,
            "BTC",
            "USDT",
            "fee-account".to_string(),
        )
        .unwrap();

        assert!(voucher.references_account("buyer"));
        assert!(voucher.references_account("fee-account"));
        assert!(!voucher.references_account("outsider"));

        assert_eq!(
            principal_voucher.amount_received_by_for_purpose(
                "buyer",
                SettlementTransferPurpose::SpotBuyerReceiveBase
            ),
            Some(2)
        );
        assert_eq!(
            principal_voucher
                .amount_sent_by_for_purpose("buyer", SettlementTransferPurpose::SpotBuyerPayQuote),
            Some(200)
        );
        assert_eq!(
            principal_voucher.amount_received_by_for_purpose(
                "seller",
                SettlementTransferPurpose::SpotSellerReceiveQuote
            ),
            Some(200)
        );
        assert_eq!(
            principal_voucher.amount_sent_by_for_purpose(
                "seller",
                SettlementTransferPurpose::SpotSellerDeliverBase
            ),
            Some(2)
        );
        assert_eq!(voucher.fee_amount_paid_by("buyer"), Some(3));
        assert_eq!(voucher.fee_amount_paid_by("seller"), Some(2));
        assert_eq!(voucher.fee_amount_paid_by("outsider"), None);

        assert!(voucher.references_ledger_entry("ledger-3"));
        assert!(!voucher.references_ledger_entry("ledger-missing"));
    }

    #[test]
    fn create_event_contains_voucher_summary_and_leg_details() {
        let event = spot_voucher().track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("voucher_id")
                && change.new_value_bytes() == b"voucher-spot-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("settlement_kind")
                && change.new_value_bytes() == b"spot"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("trade_id")
                && change.new_value_bytes() == b"trade-spot-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("fee_account_id")
                && change.new_value_bytes() == b"fee-account"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("leg_count") && change.new_value_bytes() == b"4"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("leg_0_purpose")
                && change.new_value_bytes() == b"spot_buyer_receive_base"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("leg_0_balance_ledger_entry_id")
                && change.new_value_bytes() == b"ledger-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("leg_2_purpose")
                && change.new_value_bytes() == b"trading_fee"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("leg_2_amount")
                && change.new_value_bytes() == b"3"
        }));
    }
}
