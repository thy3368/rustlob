# Hyperliquid 区块分析器

用于下载和分析 Hyperliquid L1 区块链的单个区块数据的 Rust 工具。

## 功能特性

- 下载指定高度的区块数据
- 解析区块头、交易、订单等所有数据
- 实时终端输出完整的区块信息明细
- 提供可复用的 Library API

## 安装

```bash
cd study/hyperliquid_analyzer
cargo build --release
```

## 使用方法

### 基本分析

```bash
cargo run --bin hl_analyzer -- 932387680
```

### 分析最新区块

```bash
cargo run --bin hl_analyzer -- latest
```

### 命令行选项

```bash
hl_analyzer <HEIGHT> [OPTIONS]

参数:
  <HEIGHT>  区块高度，或 "latest" 获取最新区块

选项:
  -v, --verbose              显示所有交易详情
  -l, --limit <N>            限制显示交易数量 [默认: 20]
  -f, --filter <TYPE>        只显示特定类型交易
      --show-failed          只显示失败交易
      --show-success         只显示成功交易
  -h, --help                 显示帮助信息
```

## 作为库使用

```rust
use hyperliquid_analyzer::{HyperliquidClient, analyze_block, format_block_report};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::new()?;
    let block = client.fetch_block(932387680).await?;
    let analysis = analyze_block(&block);
    let report = format_block_report(&block, &analysis);
    println!("{}", report);
    Ok(())
}
```

## 架构

- `types.rs` - 数据类型定义
- `client.rs` - HTTP 客户端
- `analyzer.rs` - 统计分析引擎
- `reporter.rs` - 终端输出格式化
- `lib.rs` - 库入口
- `bin/hl_analyzer.rs` - CLI 工具

## 技术栈

- `tokio` - 异步运行时
- `reqwest` - HTTP 客户端
- `serde` - 序列化/反序列化
- `clap` - CLI 参数解析
- `colored` - 彩色终端输出
- `thiserror` - 错误处理

## 许可证

MIT
