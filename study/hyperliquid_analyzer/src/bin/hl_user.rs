use clap::Parser;
use colored::*;
use hyperliquid_analyzer::HyperliquidClient;
use std::time::{Duration, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(name = "hl_user")]
#[command(about = "Hyperliquid 用户信息查询工具", long_about = None)]
struct Cli {
    #[arg(value_name = "ADDRESS")]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let address = cli.address;

    if !address.starts_with("0x") || address.len() < 40 || address.len() > 42 {
        eprintln!("错误: 地址格式不正确，需要 40 位十六进制地址 (0x...)");
        std::process::exit(1);
    }

    let client = HyperliquidClient::new()?;

    println!("正在查询用户信息: {}", address.yellow());
    println!();

    // 用户详情 (Explorer API - optional)
    println!("{}", "📋 用户详情 (Explorer)".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black());
    // Note: userDetails via Info API returns 422 for unregistered users
    // Using explorer endpoint for transaction history instead
    println!("  (使用 explorer 端点查看交易历史)");
    println!();

    // 查询合约持仓
    println!("{}", "📈 合约持仓 (Perpetual)".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black());
    match client.fetch_clearinghouse_state(&address).await {
        Ok(state) => {
            println!("  可提取金额:   {}", state.withdrawable.green());

            println!("\n  账户概览:");
            println!("    账户价值:   ${}", state.cross_margin_summary.account_value.yellow());
            println!("    总持仓:     ${}", state.cross_margin_summary.total_ntl_pos.yellow());
            println!("    已用保证金: ${}", state.cross_margin_summary.total_margin_used.red());

            if state.asset_positions.is_empty() {
                println!("\n  当前无持仓");
            } else {
                println!("\n  持仓明细:");
                for pos in &state.asset_positions {
                    let p = &pos.position;
                    let side = if p.side == "A" { "做多".green() } else { "做空".red() };
                    println!(
                        "    {} {} @ ${} {} 杠杆: {}x",
                        p.sz, p.coin.cyan(), p.entry_px, side, p.leverage.value
                    );
                    if let Some(pnl) = &p.unrealized_pnl {
                        let pnl_val: f64 = pnl.parse().unwrap_or(0.0);
                        let pnl_str = if pnl_val >= 0.0 {
                            format!("+${}", pnl_val).green()
                        } else {
                            format!("-${}", pnl_val.abs()).red()
                        };
                        println!("      未实现盈亏: {}", pnl_str);
                    }
                }
            }
        }
        Err(e) => {
            println!("  查询失败: {}", e);
        }
    }
    println!();

    // 查询现货持仓
    println!("{}", "💰 现货余额".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black());
    match client.fetch_spot_state(&address).await {
        Ok(state) => {
            if state.balances.is_empty() {
                println!("  无现货余额");
            } else {
                for balance in &state.balances {
                    if balance.total != "0" {
                        println!("  {}: {}", balance.coin.cyan(), balance.total.yellow());
                    }
                }
            }
        }
        Err(e) => {
            println!("  查询失败: {}", e);
        }
    }
    println!();

    // 查询未完成订单
    println!("{}", "📝 未完成订单".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black());
    match client.fetch_open_orders(&address).await {
        Ok(orders) => {
            if orders.is_empty() {
                println!("  无未完成订单");
            } else {
                for order in &orders {
                    let o = &order.order;
                    let side = if o.side == "A" { "买入".green() } else { "卖出".red() };
                    println!(
                        "  {} {} @ ${} {} ID: {}",
                        o.sz, o.coin.cyan(), o.limit_px, side, o.oid
                    );
                }
            }
        }
        Err(e) => {
            println!("  查询失败: {}", e);
        }
    }
    println!();

    // 查询最近成交
    println!("{}", "📜 最近成交记录".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black());
    match client.fetch_user_fills(&address).await {
        Ok(fills) => {
            if fills.fills.is_empty() {
                println!("  无成交记录");
            } else {
                for fill in fills.fills.iter().take(10) {
                    let side = if fill.side == "A" { "买入".green() } else { "卖出".red() };
                    let datetime = UNIX_EPOCH + Duration::from_millis(fill.time);
                    let time_str = format!("{:?}", datetime);
                    println!(
                        "  {} {} @ ${} {} 手续费: {} - {}",
                        fill.sz, fill.coin.cyan(), fill.px, side, fill.fee, time_str
                    );
                }
            }
        }
        Err(e) => {
            println!("  查询失败: {}", e);
        }
    }

    Ok(())
}
