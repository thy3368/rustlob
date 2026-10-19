use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel,
    EntityUseCaseApiSurface, FieldDiff, FinancialClassification, FourColorArchetype,
};
use serde::{Deserialize, Serialize};

use crate::BalanceLedgerReason;
use crate::account::balance_ledger_entry_v2::{BalanceLedgerEntryV2, BalanceLedgerEntryV2Error};
use crate::support::{concat2, derived_entity_id};

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

impl FieldDiff for SettlementTransferLeg {
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
        Vec::with_capacity(0)
    }
}

impl common_entity::Shard for SettlementTransferLeg {
    fn partition(&self) -> u8 {
        todo!()
    }
}

impl Entity for SettlementTransferLeg {
    fn entity_id(&self) -> u64 {
        common_entity::stable_string_entity_id(&self.leg_id)
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

    fn lsn(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        if field_name == "amount" { 1 } else { 0 }
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
    voucher_id: u64,
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
        settlement_kind: SettlementKind,
        settlement_id: String,
        trade_id: String,
        match_id: Option<String>,
        fee_account_id: String,
        legs: Vec<SettlementTransferLeg>,
    ) -> Self {
        Self {
            voucher_id: derived_entity_id(
                SETTLEMENT_TRANSFER_VOUCHER_ENTITY_TYPE,
                &[
                    settlement_kind.as_str().as_bytes(),
                    settlement_id.as_bytes(),
                    trade_id.as_bytes(),
                ],
            ),
            settlement_kind,
            settlement_id,
            trade_id,
            match_id,
            fee_account_id,
            legs,
        }
    }

    /// 返回凭证 ID。
    pub const fn voucher_id(&self) -> u64 {
        self.voucher_id
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

    /// [Internal] 向已构造凭证追加一条已校验资金腿。
    pub(crate) fn push_leg(&mut self, leg: SettlementTransferLeg) {
        self.legs.push(leg);
    }

    /// 从结算转账凭证纯派生下游余额流水单据。
    ///
    /// 每条转账腿派生两条流水：付款方 debit，收款方 credit。
    /// 该方法只生成流水事实，不应用余额、不持久化、不发布事件。
    pub fn derive_balance_ledger_entries(
        &self,
        balance_entity_id_for: impl Fn(&str, &str) -> String,
    ) -> Result<Vec<BalanceLedgerEntryV2>, BalanceLedgerEntryV2Error> {
        let mut entries = Vec::with_capacity(self.legs.len().saturating_mul(2));

        for leg in &self.legs {
            let reason = self.balance_ledger_reason_for(leg);
            let debit_entry_id = concat2(leg.balance_ledger_entry_id(), ":debit");
            let credit_entry_id = concat2(leg.balance_ledger_entry_id(), ":credit");
            let debit_balance_entity_id =
                balance_entity_id_for(leg.from_account_id(), leg.asset_id());
            let credit_balance_entity_id =
                balance_entity_id_for(leg.to_account_id(), leg.asset_id());

            let debit = match self.debit_operation_for(leg.purpose()) {
                SettlementDebitOperation::DebitFrozen => BalanceLedgerEntryV2::debit_frozen(
                    debit_entry_id,
                    leg.from_account_id().to_string(),
                    leg.asset_id().to_string(),
                    debit_balance_entity_id,
                    leg.amount(),
                    reason.clone(),
                ),
                SettlementDebitOperation::DebitAvailable => BalanceLedgerEntryV2::debit_available(
                    debit_entry_id,
                    leg.from_account_id().to_string(),
                    leg.asset_id().to_string(),
                    debit_balance_entity_id,
                    leg.amount(),
                    reason.clone(),
                ),
            }?;
            let credit = BalanceLedgerEntryV2::credit_available(
                credit_entry_id,
                leg.to_account_id().to_string(),
                leg.asset_id().to_string(),
                credit_balance_entity_id,
                leg.amount(),
                reason,
            )?;

            entries.push(debit);
            entries.push(credit);
        }

        Ok(entries)
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

    fn balance_ledger_reason_for(&self, leg: &SettlementTransferLeg) -> BalanceLedgerReason {
        match self.settlement_kind {
            SettlementKind::Spot => BalanceLedgerReason::SettleSpotTrade {
                trade_id: self.trade_id.clone(),
                match_id: self.match_id.clone().unwrap_or_default(),
                settlement_batch_id: self.settlement_id.clone(),
                purpose: leg.purpose(),
            },
            SettlementKind::Perp => BalanceLedgerReason::SettlePerpTrade {
                trade_id: self.trade_id.clone(),
                match_id: self.match_id.clone().unwrap_or_default(),
                settlement_id: self.settlement_id.clone(),
                purpose: leg.purpose(),
            },
        }
    }

    fn debit_operation_for(&self, purpose: SettlementTransferPurpose) -> SettlementDebitOperation {
        match self.settlement_kind {
            SettlementKind::Spot => match purpose {
                SettlementTransferPurpose::SpotBuyerPayQuote
                | SettlementTransferPurpose::SpotBuyerReceiveBase
                | SettlementTransferPurpose::SpotSellerReceiveQuote
                | SettlementTransferPurpose::SpotSellerDeliverBase => {
                    SettlementDebitOperation::DebitFrozen
                }
                SettlementTransferPurpose::TradingFee
                | SettlementTransferPurpose::PerpMarginTransfer
                | SettlementTransferPurpose::PerpRealizedPnlTransfer => {
                    SettlementDebitOperation::DebitAvailable
                }
            },
            SettlementKind::Perp => SettlementDebitOperation::DebitAvailable,
        }
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
enum SettlementDebitOperation {
    DebitAvailable,
    DebitFrozen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferDirection {
    Sent,
    Received,
}

impl FieldDiff for SettlementTransferVoucher {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let mut changes = vec![
            EntityFieldChange::new("voucher_id", "", self.voucher_id.to_string()),
            EntityFieldChange::new("settlement_kind", "", self.settlement_kind.as_str()),
            EntityFieldChange::new("settlement_id", "", self.settlement_id.clone()),
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("match_id", "", self.match_id.clone().unwrap_or_default()),
            EntityFieldChange::new("fee_account_id", "", self.fee_account_id.clone()),
            EntityFieldChange::new("leg_count", "", self.legs.len().to_string()),
        ];

        for (index, leg) in self.legs.iter().enumerate() {
            let prefix = concat2("leg_", index.to_string().as_str());
            changes.push(EntityFieldChange::new(
                concat2(prefix.as_str(), "_leg_id"),
                "",
                leg.leg_id.clone(),
            ));
            changes.push(EntityFieldChange::new(
                concat2(prefix.as_str(), "_from_account_id"),
                "",
                leg.from_account_id.clone(),
            ));
            changes.push(EntityFieldChange::new(
                concat2(prefix.as_str(), "_to_account_id"),
                "",
                leg.to_account_id.clone(),
            ));
            changes.push(EntityFieldChange::new(
                concat2(prefix.as_str(), "_asset_id"),
                "",
                leg.asset_id.clone(),
            ));
            changes.push(EntityFieldChange::new(
                concat2(prefix.as_str(), "_amount"),
                "",
                leg.amount.to_string(),
            ));
            changes.push(EntityFieldChange::new(
                concat2(prefix.as_str(), "_purpose"),
                "",
                leg.purpose.as_str(),
            ));
            changes.push(EntityFieldChange::new(
                concat2(prefix.as_str(), "_balance_ledger_entry_id"),
                "",
                leg.balance_ledger_entry_id.clone(),
            ));
        }

        changes
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::with_capacity(0)
    }
}

impl common_entity::Shard for SettlementTransferVoucher {
    fn partition(&self) -> u8 {
        self.voucher_id() as u8
    }
}

impl Entity for SettlementTransferVoucher {
    fn entity_id(&self) -> u64 {
        self.voucher_id
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

    fn lsn(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        if matches!(field_name, "voucher_id" | "leg_count")
            || (field_name.starts_with("leg_") && field_name.ends_with("_amount"))
        {
            1
        } else {
            0
        }
    }
}
