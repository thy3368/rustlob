use std::collections::HashMap;

use crate::types::{Block, TransactionAction};

#[derive(Debug, Clone)]
pub struct BlockAnalysis {
    pub total_txs: usize,
    pub success_txs: usize,
    pub error_txs: usize,
    pub tx_type_distribution: HashMap<String, usize>,
    pub error_distribution: HashMap<String, usize>,
    pub unique_users: usize,
    pub top_users: Vec<(String, usize)>,
    pub asset_distribution: HashMap<u32, usize>,
    pub tps: f64,
    pub effective_tps: f64,
}

impl Default for BlockAnalysis {
    fn default() -> Self {
        Self {
            total_txs: 0,
            success_txs: 0,
            error_txs: 0,
            tx_type_distribution: HashMap::new(),
            error_distribution: HashMap::new(),
            unique_users: 0,
            top_users: Vec::new(),
            asset_distribution: HashMap::new(),
            tps: 0.0,
            effective_tps: 0.0,
        }
    }
}

pub fn analyze_block(block: &Block) -> BlockAnalysis {
    let mut analysis = BlockAnalysis::default();

    analysis.total_txs = block.transactions.len();

    let mut user_tx_count: HashMap<String, usize> = HashMap::new();
    let mut tx_types: HashMap<String, usize> = HashMap::new();
    let mut errors: HashMap<String, usize> = HashMap::new();
    let mut assets: HashMap<u32, usize> = HashMap::new();

    for tx in &block.transactions {
        if tx.error.is_none() {
            analysis.success_txs += 1;
        } else {
            analysis.error_txs += 1;
            if let Some(err) = &tx.error {
                *errors.entry(err.clone()).or_insert(0) += 1;
            }
        }

        *user_tx_count.entry(tx.user.clone()).or_insert(0) += 1;

        let tx_type = match &tx.action {
            TransactionAction::Order(_) => "Order",
            TransactionAction::Cancel(_) => "Cancel",
            TransactionAction::CancelByCloid(_) => "CancelByCloid",
            TransactionAction::Noop => "Noop",
            TransactionAction::BatchModify(_) => "BatchModify",
            TransactionAction::Modify(_) => "Modify",
            TransactionAction::ScheduleCancel(_) => "ScheduleCancel",
            TransactionAction::UpdateLeverage(_) => "UpdateLeverage",
        };
        *tx_types.entry(tx_type.to_string()).or_insert(0) += 1;

        if let TransactionAction::Order(action) = &tx.action {
            for order in &action.orders {
                *assets.entry(order.asset).or_insert(0) += 1;
            }
        }
    }

    let mut top_users: Vec<_> = user_tx_count.into_iter().collect();
    top_users.sort_by(|a, b| b.1.cmp(&a.1));
    analysis.unique_users = top_users.len();
    top_users.truncate(10);
    analysis.top_users = top_users;

    analysis.tx_type_distribution = tx_types;
    analysis.error_distribution = errors;
    analysis.asset_distribution = assets;

    analysis.tps = analysis.total_txs as f64;
    analysis.effective_tps = analysis.success_txs as f64;

    analysis
}
