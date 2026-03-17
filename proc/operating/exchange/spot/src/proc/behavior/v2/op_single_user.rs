//! 单用户多订单处理器（使用泛型算子trait）
//!
//! 场景：一个用户的多笔订单（如100笔limit buy单）
//!
//! 设计：
//! - 使用泛型算子trait，通过输入输出类型区分不同算子
//! - 一个struct可实现多个算子
//! - 每个算子职责单一、可测试、可复用

use std::collections::HashMap;

use base_types::account::balance_soa::BalanceSoa;
use diff::diff::changelog_entry_base::{ChangeLogEntrySoa, FieldChange};

use crate::proc::behavior::v2::new_order_cmd_base::{NewOrderCmdSoa, OrderSideTag};

// ============================================================================
// 纯函数式算子
// ============================================================================

/// 算子：计算冻结金额
pub fn calculate_freeze_amounts(orders: &NewOrderCmdSoa) -> Result<Vec<(u32, i64)>, ProcessError> {
    let mut amounts = Vec::with_capacity(orders.len());
    for i in 0..orders.len() {
        let (asset_id, amount) = if orders.sides[i] == OrderSideTag::Buy as u8 {
            (1u32, (orders.prices[i] as i128 * orders.quantities[i] as i128 / 100_000_000) as i64)
        } else {
            (2u32, orders.quantities[i] as i64)
        };
        amounts.push((asset_id, amount));
    }
    Ok(amounts)
}

// ============================================================================
// 辅助函数（需要额外参数的操作）
// ============================================================================

/// 构建余额索引（需要account_id参数）
pub fn build_balance_index(
    balance_soa: &BalanceSoa,
    account_id: u64,
) -> Result<HashMap<u32, usize>, ProcessError> {
    let mut indices = HashMap::new();
    for i in 0..balance_soa.len() {
        if balance_soa.account_ids[i] == account_id {
            indices.insert(balance_soa.asset_ids[i], i);
        }
    }
    if indices.is_empty() {
        return Err(ProcessError::BalanceNotFound);
    }
    Ok(indices)
}

/// 冻结余额（需要多个参数，使用普通函数）
pub fn freeze_balances(
    balance_soa: &mut BalanceSoa,
    balance_indices: &HashMap<u32, usize>,
    freeze_amounts: &[(u32, i64)],
    account_id: u64,
    mut sequence_counter: u64,
) -> Result<(ChangeLogEntrySoa, u64), ProcessError> {
    let now = current_timestamp();

    let mut total_by_asset: HashMap<u32, i64> = HashMap::new();
    for &(asset_id, amount) in freeze_amounts {
        *total_by_asset.entry(asset_id).or_insert(0) += amount;
    }

    let mut changelog = ChangeLogEntrySoa::with_capacity(total_by_asset.len());

    for (&asset_id, &total_amount) in &total_by_asset {
        let balance_idx = *balance_indices.get(&asset_id).ok_or(ProcessError::BalanceNotFound)?;

        let old_available = balance_soa.availables[balance_idx];
        let old_frozen = balance_soa.frozens[balance_idx];

        balance_soa
            .batch_freeze(&[balance_idx], &[total_amount], now)
            .map_err(|_| ProcessError::InsufficientBalance(0))?;

        let entity_id = balance_entity_id(account_id, asset_id);
        let mut field_changes = Vec::new();
        field_changes.push(FieldChange::new(
            field_name("available"),
            &old_available.to_le_bytes(),
            &balance_soa.availables[balance_idx].to_le_bytes(),
            1,
        ));
        field_changes.push(FieldChange::new(
            field_name("frozen"),
            &old_frozen.to_le_bytes(),
            &balance_soa.frozens[balance_idx].to_le_bytes(),
            1,
        ));

        sequence_counter += 1;
        changelog.push(now, sequence_counter, entity_id, 1, 1, field_changes);
    }

    Ok((changelog, sequence_counter))
}

/// 生成订单日志（需要sequence_counter参数）
pub fn generate_order_log(
    orders: &NewOrderCmdSoa,
    mut sequence_counter: u64,
) -> (ChangeLogEntrySoa, u64) {
    let mut changelog = ChangeLogEntrySoa::with_capacity(orders.len());
    let now = current_timestamp();

    for i in 0..orders.len() {
        let order_id = now + i as u64;
        let entity_id = order_entity_id(order_id);

        let mut field_changes = Vec::new();
        field_changes.push(FieldChange::new(field_name("order_id"), &[], &order_id.to_le_bytes(), 1));
        field_changes.push(FieldChange::new(field_name("symbol"), &[], &orders.symbols[i], 4));
        field_changes.push(FieldChange::new(field_name("side"), &[], &[orders.sides[i]], 1));
        field_changes.push(FieldChange::new(field_name("quantity"), &[], &orders.quantities[i].to_le_bytes(), 1));
        field_changes.push(FieldChange::new(field_name("price"), &[], &orders.prices[i].to_le_bytes(), 1));

        sequence_counter += 1;
        changelog.push(now, sequence_counter, entity_id, 2, 0, field_changes);
    }

    (changelog, sequence_counter)
}

// ============================================================================
// 辅助函数
// ============================================================================

fn balance_entity_id(account_id: u64, asset_id: u32) -> [u8; 32] {
    let mut id = [0u8; 32];
    let s = format!("balance_{}_{}", account_id, asset_id);
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    id[..len].copy_from_slice(&bytes[..len]);
    id
}

fn order_entity_id(order_id: u64) -> [u8; 32] {
    let mut id = [0u8; 32];
    let s = format!("order_{}", order_id);
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    id[..len].copy_from_slice(&bytes[..len]);
    id
}

fn field_name(name: &str) -> [u8; 32] {
    let mut field = [0u8; 32];
    let bytes = name.as_bytes();
    let len = bytes.len().min(32);
    field[..len].copy_from_slice(&bytes[..len]);
    field
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64
}

// ============================================================================
// 单用户处理器
// ============================================================================

/// 单用户订单处理器
pub struct SingleUserProcessor {
    account_id: u64,
    sequence_counter: u64,
}

impl SingleUserProcessor {
    pub fn new(account_id: u64) -> Self {
        Self {
            account_id,
            sequence_counter: 0,
        }
    }

    /// 批量处理单用户的多笔订单
    pub fn process(
        &mut self,
        orders: &NewOrderCmdSoa,
        balance_soa: &mut BalanceSoa,
    ) -> Result<(ChangeLogEntrySoa, ChangeLogEntrySoa), ProcessError> {
        // 验证
        // PortableSimdValidator::verify(orders).map_err(|e| match e {
        //     crate::proc::behavior::v2::verify_portable_simd::ProcessError::InvalidQuantity(idx) => ProcessError::InvalidQuantity(idx),
        //     crate::proc::behavior::v2::verify_portable_simd::ProcessError::InvalidPrice(idx) => ProcessError::InvalidPrice(idx),
        //     crate::proc::behavior::v2::verify_portable_simd::ProcessError::InvalidSymbol(idx) => ProcessError::InvalidSymbol(idx),
        // })?;

        // 算子1: 计算冻结金额（使用泛型算子）
        let freeze_amounts = self.ops.apply_soa(orders)?;

        // 算子2: 构建余额索引
        let balance_indices = build_balance_index(balance_soa, self.account_id)?;

        // 算子3: 冻结余额
        let (balance_log, seq) = freeze_balances(
            balance_soa,
            &balance_indices,
            &freeze_amounts,
            self.account_id,
            self.sequence_counter,
        )?;
        self.sequence_counter = seq;

        // 算子4: 生成订单日志
        let (order_log, seq) = generate_order_log(orders, self.sequence_counter);
        self.sequence_counter = seq;

        Ok((balance_log, order_log))
    }
}

// ============================================================================
// 错误定义
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessError {
    InvalidQuantity(usize),
    InvalidPrice(usize),
    InvalidSymbol(usize),
    InsufficientBalance(usize),
    BalanceNotFound,
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::InvalidQuantity(idx) => write!(f, "Invalid quantity at order index {}", idx),
            ProcessError::InvalidPrice(idx) => write!(f, "Invalid price at order index {}", idx),
            ProcessError::InvalidSymbol(idx) => write!(f, "Invalid symbol at order index {}", idx),
            ProcessError::InsufficientBalance(idx) => write!(f, "Insufficient balance at order index {}", idx),
            ProcessError::BalanceNotFound => write!(f, "Balance not found"),
        }
    }
}

impl std::error::Error for ProcessError {}
