#!/bin/bash

# HTTP API 测试脚本
# 用法: ./test_http.sh

BASE_URL="http://localhost:8080"

echo "========================================="
echo "订单匹配服务 HTTP API 测试"
echo "========================================="
echo ""

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 1. 健康检查
echo -e "${BLUE}[1] 健康检查${NC}"
curl -s -X GET "${BASE_URL}/health" | jq .
echo ""
echo ""

# 2. 查询初始市场深度
echo -e "${BLUE}[2] 查询初始市场深度${NC}"
curl -s -X GET "${BASE_URL}/api/market/depth" | jq .
echo ""
echo ""

# 3. 下第一个卖单（价格10100，数量100）
echo -e "${BLUE}[3] 下卖单: SELLER1, price=10100, qty=100${NC}"
curl -s -X POST "${BASE_URL}/api/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "trader_id": "SELLER1",
    "side": "sell",
    "price": 10100,
    "quantity": 100
  }' | jq .
echo ""
echo ""

# 4. 下第二个卖单（价格10200，数量50）
echo -e "${BLUE}[4] 下卖单: SELLER2, price=10200, qty=50${NC}"
curl -s -X POST "${BASE_URL}/api/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "trader_id": "SELLER2",
    "side": "sell",
    "price": 10200,
    "quantity": 50
  }' | jq .
echo ""
echo ""

# 5. 查询市场深度（应该有卖单）
echo -e "${BLUE}[5] 查询市场深度（有卖单）${NC}"
curl -s -X GET "${BASE_URL}/api/market/depth" | jq .
echo ""
echo ""

# 6. 下买单，部分成交（价格10150，数量60）
echo -e "${BLUE}[6] 下买单: BUYER1, price=10150, qty=60 (应该部分成交)${NC}"
curl -s -X POST "${BASE_URL}/api/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "trader_id": "BUYER1",
    "side": "buy",
    "price": 10150,
    "quantity": 60
  }' | jq .
echo ""
echo ""

# 7. 下买单，完全成交（价格10300，数量200）
echo -e "${BLUE}[7] 下买单: BUYER2, price=10300, qty=200 (应该完全成交)${NC}"
curl -s -X POST "${BASE_URL}/api/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "trader_id": "BUYER2",
    "side": "buy",
    "price": 10300,
    "quantity": 200
  }' | jq .
echo ""
echo ""

# 8. 再次查询市场深度
echo -e "${BLUE}[8] 查询市场深度（成交后）${NC}"
curl -s -X GET "${BASE_URL}/api/market/depth" | jq .
echo ""
echo ""

# 9. 下订单并获取订单ID，然后取消
echo -e "${BLUE}[9] 下买单并获取订单ID${NC}"
ORDER_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/orders" \
  -H "Content-Type: application/json" \
  -d '{
    "trader_id": "BUYER3",
    "side": "buy",
    "price": 9900,
    "quantity": 100
  }')
echo "$ORDER_RESPONSE" | jq .

ORDER_ID=$(echo "$ORDER_RESPONSE" | jq -r '.order_id')
echo ""
echo ""

# 10. 取消订单
if [ "$ORDER_ID" != "0" ] && [ "$ORDER_ID" != "null" ]; then
  echo -e "${BLUE}[10] 取消订单: order_id=$ORDER_ID${NC}"
  curl -s -X POST "${BASE_URL}/api/orders/cancel" \
    -H "Content-Type: application/json" \
    -d "{
      \"order_id\": $ORDER_ID
    }" | jq .
  echo ""
  echo ""
else
  echo -e "${YELLOW}[10] 跳过取消订单测试（订单完全成交）${NC}"
  echo ""
fi

# 11. 最终市场深度
echo -e "${BLUE}[11] 最终市场深度${NC}"
curl -s -X GET "${BASE_URL}/api/market/depth" | jq .
echo ""
echo ""

echo -e "${GREEN}=========================================${NC}"
echo -e "${GREEN}测试完成！${NC}"
echo -e "${GREEN}=========================================${NC}"
