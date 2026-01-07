#!/bin/bash
# WebSocket 服务快速测试脚本

set -e

echo "======================================"
echo "  WebSocket 订单匹配服务 快速测试"
echo "======================================"
echo ""

# 检查服务是否运行
echo "[1] 检查服务健康状态..."
HEALTH=$(curl -s http://localhost:9090/health)
echo "✓ 服务状态: $HEALTH"
echo ""

# 创建测试 WebSocket 客户端（使用 websocat）
if ! command -v websocat &> /dev/null; then
    echo "⚠️  未找到 websocat，请安装:"
    echo "   brew install websocat  (macOS)"
    echo "   cargo install websocat  (通用)"
    echo ""
    echo "或者运行 Rust 客户端示例:"
    echo "   cargo run --example ws_client --release"
    exit 0
fi

echo "[2] 通过 websocat 连接到 WebSocket..."
echo ""
echo "发送测试消息（手动测试）："
echo ""
echo '{"type":"ping"}'
echo '{"type":"limit_order","trader_id":"alice","side":"sell","price":50100,"quantity":5}'
echo '{"type":"limit_order","trader_id":"bob","side":"buy","price":50100,"quantity":2}'
echo ""
echo "运行: websocat ws://localhost:9090/ws"
echo ""

echo "======================================"
echo "  建议运行以下测试:"
echo "======================================"
echo ""
echo "1. 运行示例客户端:"
echo "   cargo run --example ws_client --release"
echo ""
echo "2. 运行性能基准测试:"
echo "   cargo run --example ws_benchmark --release"
echo ""
echo "3. 查看 WEBSOCKET.md 文档获取更多信息"
echo ""
