use clap::Parser;
use hyperliquid_analyzer::{analyze_block, format_block_report, HyperliquidClient};

#[derive(Parser, Debug)]
#[command(name = "hl_analyzer")]
#[command(about = "Hyperliquid 区块分析器", long_about = None)]
struct Cli {
    /// 区块高度，或 "latest" 获取最新区块
    #[arg(value_name = "HEIGHT")]
    height: String,

    /// 显示所有交易详情
    #[arg(short, long)]
    verbose: bool,

    /// 限制显示的交易数量
    #[arg(short, long, default_value = "20")]
    limit: usize,

    /// 只显示特定类型的交易
    #[arg(short = 'f', long)]
    filter: Option<String>,

    /// 只显示失败的交易
    #[arg(long)]
    show_failed: bool,

    /// 只显示成功的交易
    #[arg(long)]
    show_success: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let height = if cli.height == "latest" {
        None
    } else {
        Some(cli.height.parse::<u64>()?)
    };

    let client = HyperliquidClient::new()?;

    println!("正在获取区块数据...");

    let block = if let Some(h) = height {
        client.fetch_block(h).await?
    } else {
        client.fetch_latest_block().await?
    };

    let analysis = analyze_block(&block);

    let report = format_block_report(&block, &analysis);
    println!("{}", report);

    Ok(())
}
