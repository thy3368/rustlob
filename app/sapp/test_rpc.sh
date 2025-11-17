#!/bin/bash
# LOB JSON-RPC API 测试脚本

API_URL="http://127.0.0.1:3030"

echo "==================================================================="
echo "  LOB Matching Service - JSON-RPC API 测试"
echo "==================================================================="
echo ""

# 1. 健康检查
echo "1. 健康检查..."
curl -s -X POST $API_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"health","params":{},"id":1}' | jq '.'
echo ""

# 2. 提交卖单
echo "2. 提交卖单 (SELLER01, SELL, price=10000, quantity=100)..."
curl -s -X POST $API_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"place_limit_order","params":{"trader_id":"SELLER01","side":"SELL","price":10000,"quantity":100},"id":2}' | jq '.'
echo ""

# 3. 提交买单（部分匹配）
echo "3. 提交买单 (BUYER001, BUY, price=10000, quantity=50)..."
curl -s -X POST $API_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"place_limit_order","params":{"trader_id":"BUYER001","side":"BUY","price":10000,"quantity":50},"id":3}' | jq '.'
echo ""

# 4. 查看订单簿状态
echo "4. 查看订单簿状态..."
curl -s -X POST $API_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"get_book_status","params":{},"id":4}' | jq '.'
echo ""

# 5. 提交买单（完全匹配）
echo "5. 提交买单 (BUYER002, BUY, price=10000, quantity=50)..."
curl -s -X POST $API_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"place_limit_order","params":{"trader_id":"BUYER002","side":"BUY","price":10000,"quantity":50},"id":5}' | jq '.'
echo ""

# 6. 最终订单簿状态
echo "6. 最终订单簿状态..."
curl -s -X POST $API_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"get_book_status","params":{},"id":6}' | jq '.'
echo ""

echo "==================================================================="
echo "  测试完成!"
echo "==================================================================="
