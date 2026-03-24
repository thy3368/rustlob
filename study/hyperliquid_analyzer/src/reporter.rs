use crate::analyzer::BlockAnalysis;
use crate::types::{Block, TransactionData};
use colored::*;

pub fn format_block_report(block: &Block, analysis: &BlockAnalysis) -> String {
    let mut output = String::new();

    output.push_str(&format_header(block));
    output.push_str(&format_statistics(analysis));
    output.push_str(&format_tx_type_distribution(analysis));
    output.push_str(&format_transaction_list(block, 20));

    if analysis.error_txs > 0 {
        output.push_str(&format_error_analysis(analysis));
    }

    output.push_str(&format_user_analysis(analysis));

    if !analysis.asset_distribution.is_empty() {
        output.push_str(&format_asset_distribution(analysis));
    }

    output.push_str(&format_performance_metrics(analysis));

    output
}

fn format_header(block: &Block) -> String {
    format!(
        "\n{}\n{}\n{}\n\n{}\n{}\n  区块高度:     {}\n  区块哈希:     {}\n  时间戳:       {}\n  提议者:       {}\n  总交易数:     {}\n\n",
        "╔══════════════════════════════════════════════════════════╗".bright_blue(),
        format!("║        区块 #{} 完整分析                          ║", block.header.height).bright_blue(),
        "╚══════════════════════════════════════════════════════════╝".bright_blue(),
        "📦 区块头信息".bright_cyan().bold(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black(),
        format!("{}", block.header.height).yellow(),
        truncate_hash(&block.header.hash).yellow(),
        format_timestamp(block.header.time),
        truncate_hash(&block.header.proposer).yellow(),
        format!("{}", block.transactions.len()).yellow(),
    )
}

fn format_statistics(analysis: &BlockAnalysis) -> String {
    let success_rate = if analysis.total_txs > 0 {
        (analysis.success_txs as f64 / analysis.total_txs as f64) * 100.0
    } else {
        0.0
    };
    let error_rate = if analysis.total_txs > 0 {
        (analysis.error_txs as f64 / analysis.total_txs as f64) * 100.0
    } else {
        0.0
    };

    format!(
        "{}\n{}\n  总交易数:     {:>8}\n  成功交易:     {:>8} ({})\n  失败交易:     {:>8} ({})\n\n",
        "📊 交易统计".bright_cyan().bold(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black(),
        format!("{}", analysis.total_txs).white(),
        format!("{}", analysis.success_txs).green(),
        format!("{:.1}%", success_rate).green(),
        format!("{}", analysis.error_txs).red(),
        format!("{:.1}%", error_rate).red(),
    )
}

fn format_tx_type_distribution(analysis: &BlockAnalysis) -> String {
    let mut output = format!(
        "{}\n{}\n",
        "📈 交易类型分布".bright_cyan().bold(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black(),
    );

    let mut types: Vec<_> = analysis.tx_type_distribution.iter().collect();
    types.sort_by(|a, b| b.1.cmp(a.1));

    for (tx_type, count) in types.iter().take(10) {
        let percentage = if analysis.total_txs > 0 {
            (**count as f64 / analysis.total_txs as f64) * 100.0
        } else {
            0.0
        };
        let bar = create_bar(percentage, 20);

        output.push_str(&format!(
            "  {:<18} {:>6} ({:>5.1}%) {}\n",
            tx_type.bright_white(),
            format!("{}", count).yellow(),
            percentage,
            bar.bright_blue(),
        ));
    }

    output.push_str("\n");
    output
}

fn format_transaction_list(block: &Block, limit: usize) -> String {
    let mut output = format!(
        "{}\n{}\n{:<6} {:<20} {:<20} {:<10} {}\n{}\n",
        format!("📝 交易明细 (前 {} 笔)", limit).bright_cyan().bold(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black(),
        "#", "类型", "用户地址", "状态", "详情",
        "──────────────────────────────────────────────────────────────".bright_black(),
    );

    for (i, tx) in block.transactions.iter().take(limit).enumerate() {
        let tx_type = match &tx.data {
            TransactionData::Order(_) => "Order",
            TransactionData::Cancel(_) => "Cancel",
            TransactionData::CancelByCloid(_) => "CancelByCloid",
            TransactionData::Noop(_) => "Noop",
            TransactionData::BatchModify(_) => "BatchModify",
            TransactionData::Unknown { tx_type, .. } => tx_type.as_str(),
        };

        let status = match tx.status {
            crate::types::TxStatus::Success => "✓".green(),
            crate::types::TxStatus::Error => "✗".red(),
        };

        let details = format_tx_details(&tx.data);

        output.push_str(&format!(
            "{:<6} {:<20} {:<20} {:<10} {}\n",
            format!("{}", i + 1).bright_black(),
            tx_type.bright_white(),
            truncate_hash(&tx.user_address).yellow(),
            status,
            details,
        ));
    }

    output.push_str("\n");
    output
}

fn format_error_analysis(analysis: &BlockAnalysis) -> String {
    let mut output = format!(
        "{}\n{}\n  总失败数: {}\n\n  失败原因分布:\n",
        "❌ 失败交易分析".bright_cyan().bold(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black(),
        format!("{}", analysis.error_txs).red(),
    );

    let mut errors: Vec<_> = analysis.error_distribution.iter().collect();
    errors.sort_by(|a, b| b.1.cmp(a.1));

    for (error, count) in errors.iter().take(10) {
        let percentage = if analysis.error_txs > 0 {
            (**count as f64 / analysis.error_txs as f64) * 100.0
        } else {
            0.0
        };

        output.push_str(&format!(
            "    {:<30} {:>6} ({:>5.1}%)\n",
            error.bright_white(),
            format!("{}", count).red(),
            percentage,
        ));
    }

    output.push_str("\n");
    output
}

fn format_user_analysis(analysis: &BlockAnalysis) -> String {
    let mut output = format!(
        "{}\n{}\n  独立用户数:   {}\n\n  Top 10 活跃用户:\n",
        "👥 活跃用户分析".bright_cyan().bold(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black(),
        format!("{}", analysis.unique_users).yellow(),
    );

    for (i, (user, count)) in analysis.top_users.iter().enumerate() {
        let percentage = if analysis.total_txs > 0 {
            (*count as f64 / analysis.total_txs as f64) * 100.0
        } else {
            0.0
        };

        output.push_str(&format!(
            "    #{:<3} {}    {:>6} txs ({:>5.1}%)\n",
            format!("{}", i + 1).bright_black(),
            truncate_hash(user).yellow(),
            format!("{}", count).white(),
            percentage,
        ));
    }

    output.push_str("\n");
    output
}

fn format_asset_distribution(analysis: &BlockAnalysis) -> String {
    let mut output = format!(
        "{}\n{}\n",
        "📊 交易对分析".bright_cyan().bold(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black(),
    );

    let mut assets: Vec<_> = analysis.asset_distribution.iter().collect();
    assets.sort_by(|a, b| b.1.cmp(a.1));

    let total_order_txs: usize = assets.iter().map(|(_, count)| *count).sum();

    for (asset, count) in assets.iter().take(10) {
        let percentage = if total_order_txs > 0 {
            (**count as f64 / total_order_txs as f64) * 100.0
        } else {
            0.0
        };

        output.push_str(&format!(
            "  Asset {:<6}    {:>6} txs ({:>5.1}%)\n",
            format!("{}", asset).bright_white(),
            format!("{}", count).yellow(),
            percentage,
        ));
    }

    output.push_str("\n");
    output
}

fn format_performance_metrics(analysis: &BlockAnalysis) -> String {
    format!(
        "{}\n{}\n  TPS:          {}\n  有效 TPS:     {}\n\n",
        "⏱️  性能指标".bright_cyan().bold(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black(),
        format!("{:.0} tx/s", analysis.tps).yellow(),
        format!("{:.0} tx/s", analysis.effective_tps).green(),
    )
}

fn truncate_hash(hash: &str) -> String {
    if hash.len() > 16 {
        format!("{}...{}", &hash[..8], &hash[hash.len() - 4..])
    } else {
        hash.to_string()
    }
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::{Duration, UNIX_EPOCH};
    let d = UNIX_EPOCH + Duration::from_nanos(timestamp);
    format!("{:?}", d).yellow().to_string()
}

fn create_bar(percentage: f64, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f64) as usize;
    "█".repeat(filled) + &"░".repeat(width.saturating_sub(filled))
}

fn format_tx_details(data: &TransactionData) -> String {
    match data {
        TransactionData::Order(order) => {
            let side = if order.is_buy { "Buy" } else { "Sell" };
            format!(
                "Asset {} {} {} @ {}",
                order.asset, side, order.sz, order.limit_px
            )
        }
        TransactionData::Cancel(cancel) => {
            format!("Asset {} OID {}", cancel.asset, cancel.oid)
        }
        TransactionData::CancelByCloid(cancel) => {
            format!("Asset {} CLOID {}", cancel.asset, cancel.cloid)
        }
        TransactionData::Noop(_) => "No operation".to_string(),
        TransactionData::BatchModify(batch) => {
            format!("{} orders", batch.modifies.len())
        }
        TransactionData::Unknown { tx_type, .. } => {
            format!("Type: {}", tx_type)
        }
    }
}
