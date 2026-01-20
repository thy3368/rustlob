#!/bin/bash
# 测试脚本 - 分析示例项目

set -e

echo "🧪 测试 Rust 优化分析工具..."
echo

# 构建工具
echo "📦 构建分析工具..."
cargo build --release
echo

# 测试基础分析
echo "🔍 测试基础分析..."
cargo run --release -- analyze --path ../../study/web3
echo

# 测试 JSON 输出
echo "📄 测试 JSON 输出..."
cargo run --release -- analyze --path ../../study/web3 --output json --output-file test_report.json
echo

# 测试 HTML 输出
echo "🌐 测试 HTML 输出..."
cargo run --release -- analyze --path ../../study/web3 --output html --output-file test_report.html
echo

echo "✅ 测试完成！"
echo "📊 查看报告:"
echo "  - JSON: test_report.json"
echo "  - HTML: test_report.html"
