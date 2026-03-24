use hyperliquid_analyzer::{analyze_block, format_block_report, HyperliquidClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = HyperliquidClient::new()?;

    // 获取指定区块
    println!("正在获取区块 932387680...");
    let block = client.fetch_block(932387680).await?;

    // 分析区块
    let analysis = analyze_block(&block);

    // 输出报告
    let report = format_block_report(&block, &analysis);
    println!("{}", report);

    Ok(())
}
