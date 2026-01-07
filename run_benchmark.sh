#!/bin/bash

# 基准测试脚本 - 启动两个 WebSocket 服务器并运行比较测试

# 设置颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== 启动 WebSocket 服务器进行基准测试 ===${NC}"

# 检查是否已安装所需工具
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo 未找到，请确保已安装 Rust${NC}"
    exit 1
fi

# 编译发布版本
echo -e "${YELLOW}编译发布版本...${NC}"
cargo build --release -p websocket_axum -p websocket_sockudo

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ 编译失败${NC}"
    exit 1
fi

# 启动 Axum 服务器（后台运行）
echo -e "${YELLOW}启动 Axum 服务器（端口 8080）...${NC}"
cargo run --release -p websocket_axum > axum_server.log 2>&1 &
AXUM_PID=$!

# 启动 Sockudo 服务器（后台运行）
echo -e "${YELLOW}启动 Sockudo 服务器（端口 8081）...${NC}"
cargo run --release -p websocket_sockudo > sockudo_server.log 2>&1 &
SOCKUDO_PID=$!

# 等待服务器启动
echo -e "${YELLOW}等待服务器启动...${NC}"
sleep 3

# 检查服务器是否正在运行
if ! kill -0 $AXUM_PID 2>/dev/null; then
    echo -e "${RED}❌ Axum 服务器启动失败${NC}"
    echo "查看日志：cat axum_server.log"
    kill -9 $SOCKUDO_PID 2>/dev/null
    exit 1
fi

if ! kill -0 $SOCKUDO_PID 2>/dev/null; then
    echo -e "${RED}❌ Sockudo 服务器启动失败${NC}"
    echo "查看日志：cat sockudo_server.log"
    kill -9 $AXUM_PID 2>/dev/null
    exit 1
fi

echo -e "${GREEN}✅ 服务器启动成功！${NC}"
echo "Axum 服务器 PID: $AXUM_PID"
echo "Sockudo 服务器 PID: $SOCKUDO_PID"

# 运行基准测试
echo -e "${YELLOW}运行基准测试...${NC}"
cargo bench -p websocket_axum --bench comparison_benchmark

# 停止服务器
echo -e "${YELLOW}停止服务器...${NC}"
kill -9 $AXUM_PID $SOCKUDO_PID 2>/dev/null
wait $AXUM_PID $SOCKUDO_PID 2>/dev/null

echo -e "${GREEN}✅ 基准测试完成！${NC}"

# 显示日志信息
echo -e "${YELLOW}服务器日志：${NC}"
echo "Axum 服务器日志（最后 10 行）："
tail -10 axum_server.log
echo ""
echo "Sockudo 服务器日志（最后 10 行）："
tail -10 sockudo_server.log