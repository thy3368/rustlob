#!/bin/bash

# 用户路由测试脚本
# 测试 Pingora Gateway 的用户路由功能

GATEWAY_URL="http://localhost:8080"

echo "======================================"
echo "Pingora Gateway 用户路由测试"
echo "======================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试 1: 使用 JSON 请求体中的 user_id
echo -e "${YELLOW}测试 1: JSON 请求体中提取 user_id${NC}"
echo "发送请求: user_id=alice"
curl -s -X POST "${GATEWAY_URL}/api/spot/v2/" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "alice",
    "symbol": "BTCUSDT",
    "side": "buy",
    "price": 50000,
    "quantity": 1.5
  }' | jq .

echo ""
echo "发送请求: user_id=bob"
curl -s -X POST "${GATEWAY_URL}/api/spot/v2/" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "bob",
    "symbol": "ETHUSDT",
    "side": "sell",
    "price": 3000,
    "quantity": 2.0
  }' | jq .

echo ""
echo "======================================"
echo ""

# 测试 2: 使用查询参数中的 user_id
echo -e "${YELLOW}测试 2: 查询参数中提取 user_id${NC}"
echo "发送请求: ?user_id=alice"
curl -s -X POST "${GATEWAY_URL}/api/spot/v2/?user_id=alice&symbol=BTCUSDT" \
  -H "Content-Type: application/json" \
  -d '{"side": "buy", "price": 50000}' | jq .

echo ""
echo "发送请求: ?user_id=bob"
curl -s -X POST "${GATEWAY_URL}/api/spot/v2/?user_id=bob&symbol=ETHUSDT" \
  -H "Content-Type: application/json" \
  -d '{"side": "sell", "price": 3000}' | jq .

echo ""
echo "======================================"
echo ""

# 测试 3: 使用 HTTP 请求头中的 user_id
echo -e "${YELLOW}测试 3: HTTP 请求头中提取 user_id${NC}"
echo "发送请求: X-User-Id: alice"
curl -s -X POST "${GATEWAY_URL}/api/spot/user/data" \
  -H "Content-Type: application/json" \
  -H "X-User-Id: alice" \
  -d '{"action": "get_balance"}' | jq .

echo ""
echo "发送请求: X-User-Id: bob"
curl -s -X POST "${GATEWAY_URL}/api/spot/user/data" \
  -H "Content-Type: application/json" \
  -H "X-User-Id: bob" \
  -d '{"action": "get_balance"}' | jq .

echo ""
echo "======================================"
echo ""

# 测试 4: 不同用户 ID 字段名
echo -e "${YELLOW}测试 4: 不同的用户 ID 字段名${NC}"

echo "发送请求: trader_id"
curl -s -X POST "${GATEWAY_URL}/api/spot/v2/" \
  -H "Content-Type: application/json" \
  -d '{
    "trader_id": "alice",
    "symbol": "BTCUSDT",
    "price": 50000
  }' | jq .

echo ""
echo "发送请求: uid (数字)"
curl -s -X POST "${GATEWAY_URL}/api/spot/v2/" \
  -H "Content-Type: application/json" \
  -d '{
    "uid": 12345,
    "symbol": "ETHUSDT",
    "price": 3000
  }' | jq .

echo ""
echo "======================================"
echo ""

# 测试 5: 轮询负载均衡（user_1 有 2 个后端）
echo -e "${YELLOW}测试 5: 轮询负载均衡 (user_1)${NC}"
echo "连续发送 5 个请求，观察负载均衡效果"
for i in {1..5}; do
  echo "请求 $i:"
  curl -s -X POST "${GATEWAY_URL}/api/spot/v2/" \
    -H "Content-Type: application/json" \
    -d "{\"user_id\": \"user_1\", \"request_id\": $i}" | jq .
  sleep 0.5
done

echo ""
echo "======================================"
echo ""

# 测试 6: 未配置用户（使用默认后端）
echo -e "${YELLOW}测试 6: 未配置用户 (使用默认后端)${NC}"
echo "发送请求: user_id=unknown_user"
curl -s -X POST "${GATEWAY_URL}/api/spot/v2/" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "unknown_user",
    "symbol": "BTCUSDT",
    "price": 50000
  }' | jq .

echo ""
echo "======================================"
echo ""

# 测试 7: 非路由路径（标准代理）
echo -e "${YELLOW}测试 7: 非用户路由路径${NC}"
echo "发送请求到 /api/spot/order/ (标准代理)"
curl -s -X POST "${GATEWAY_URL}/api/spot/order/" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "alice",
    "symbol": "BTCUSDT",
    "side": "buy",
    "price": 50000
  }' | jq .

echo ""
echo "======================================"
echo ""

# 测试 8: 健康检查
echo -e "${YELLOW}测试 8: 健康检查${NC}"
echo "GET /api/spot/health"
curl -s -X GET "${GATEWAY_URL}/api/spot/health" | jq .

echo ""
echo "======================================"
echo -e "${GREEN}测试完成！${NC}"
echo "======================================"
