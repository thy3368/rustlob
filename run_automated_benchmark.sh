#!/bin/bash

# 自动化基准测试脚本
# 定期运行 WebSocket 服务器性能测试并更新报告

# 设置颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 配置
TEST_INTERVAL=3600  # 每小时运行一次测试（秒）
LOG_FILE="benchmark_history.log"
REPORT_FILE="websocket_benchmark_report.md"

echo -e "${GREEN}=== WebSocket 服务器自动化基准测试 ===${NC}"
echo -e "${YELLOW}测试间隔: ${TEST_INTERVAL}秒 (1小时)${NC}"
echo -e "${YELLOW}日志文件: ${LOG_FILE}${NC}"
echo -e "${YELLOW}报告文件: ${REPORT_FILE}${NC}"
echo "----------------------------------------"

# 清理函数
cleanup() {
    echo -e "${YELLOW}正在停止服务器...${NC}"
    pkill -f websocket_axum 2>/dev/null || true
    pkill -f websocket_sockudo 2>/dev/null || true
    wait 2>/dev/null
    echo -e "${GREEN}✅ 服务器已停止${NC}"
}

# 运行基准测试
run_benchmark() {
    echo -e "\n${YELLOW}[$(date '+%Y-%m-%d %H:%M:%S')] 正在启动服务器...${NC}"

    # 停止之前的服务器实例
    pkill -f websocket_axum 2>/dev/null || true
    pkill -f websocket_sockudo 2>/dev/null || true
    wait 2>/dev/null

    # 启动服务器
    cargo run --release -p websocket_axum > axum_server.log 2>&1 &
    AXUM_PID=$!
    cargo run --release -p websocket_sockudo > sockudo_server.log 2>&1 &
    SOCKUDO_PID=$!

    # 等待服务器启动
    sleep 5

    # 检查服务器是否正在运行
    if ! kill -0 $AXUM_PID 2>/dev/null; then
        echo -e "${RED}❌ Axum 服务器启动失败${NC}"
        echo "查看日志: cat axum_server.log"
        kill -9 $SOCKUDO_PID 2>/dev/null
        return 1
    fi

    if ! kill -0 $SOCKUDO_PID 2>/dev/null; then
        echo -e "${RED}❌ Sockudo 服务器启动失败${NC}"
        echo "查看日志: cat sockudo_server.log"
        kill -9 $AXUM_PID 2>/dev/null
        return 1
    fi

    echo -e "${GREEN}✅ 服务器启动成功${NC}"
    echo "Axum PID: $AXUM_PID"
    echo "Sockudo PID: $SOCKUDO_PID"

    # 运行基准测试
    echo -e "${YELLOW}正在运行基准测试...${NC}"
    BENCH_OUTPUT=$(cargo bench -p websocket_axum --bench comparison_benchmark 2>&1)
    BENCH_STATUS=$?

    if [ $BENCH_STATUS -ne 0 ]; then
        echo -e "${RED}❌ 基准测试失败${NC}"
        echo "$BENCH_OUTPUT"
        cleanup
        return 1
    fi

    echo -e "${GREEN}✅ 基准测试完成${NC}"

    # 停止服务器
    cleanup

    # 保存结果到日志
    echo -e "\n=== 测试结果 ($(date '+%Y-%m-%d %H:%M:%S')) ===" >> $LOG_FILE
    echo "$BENCH_OUTPUT" >> $LOG_FILE

    return 0
}

# 主循环
while true; do
    run_benchmark

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}[$(date '+%Y-%m-%d %H:%M:%S')] 测试成功，等待下一次运行...${NC}"
    else
        echo -e "${RED}[$(date '+%Y-%m-%d %H:%M:%S')] 测试失败，等待下一次运行...${NC}"
    fi

    # 等待指定间隔
    sleep $TEST_INTERVAL
done