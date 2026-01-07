#!/usr/bin/env python3
"""
测试 SBE 接口的 Python 脚本
发送 SBE 编码的交易请求并验证响应
"""

import socket
import struct

def create_sbe_trade_message(trade_id: int, symbol: int, price: float, quantity: int) -> bytes:
    """
    创建 SBE 编码的交易消息

    SBE 消息布局 (21字节):
    ┌─────────────────────────────────────────┐
    │ 字段      │ 偏移 │ 长度 │ 类型        │
    ├─────────────────────────────────────────┤
    │ tradeId   │  0   │  8   │ u64 (LE)    │
    │ symbol    │  8   │  1   │ u8 (ASCII)  │
    │ price     │  9   │  8   │ f64 (LE)    │
    │ quantity  │ 17   │  4   │ i32 (LE)    │
    └─────────────────────────────────────────┘
    """
    buffer = bytearray(21)

    # tradeId (u64, little-endian)
    struct.pack_into('<Q', buffer, 0, trade_id)

    # symbol (u8)
    buffer[8] = symbol

    # price (f64, little-endian)
    struct.pack_into('<d', buffer, 9, price)

    # quantity (i32, little-endian)
    struct.pack_into('<i', buffer, 17, quantity)

    return bytes(buffer)

def parse_sbe_trade_message(data: bytes):
    """解析 SBE 编码的交易消息"""
    trade_id = struct.unpack_from('<Q', data, 0)[0]
    symbol = data[8]
    price = struct.unpack_from('<d', data, 9)[0]
    quantity = struct.unpack_from('<i', data, 17)[0]
    return trade_id, chr(symbol), price, quantity

def test_sbe_api():
    """测试 SBE API 接口"""
    server_address = ('localhost', 3000)
    endpoint = '/api/trade/sbe'

    # 创建测试数据
    test_trade_id = 12345
    test_symbol = ord(b'A')
    test_price = 99.50
    test_quantity = 1000

    # 创建 SBE 编码的请求
    request_data = create_sbe_trade_message(
        test_trade_id,
        test_symbol,
        test_price,
        test_quantity
    )

    # 创建 HTTP POST 请求
    http_request = (
        f"POST {endpoint} HTTP/1.1\r\n"
        f"Host: {server_address[0]}:{server_address[1]}\r\n"
        f"Content-Type: application/octet-stream\r\n"
        f"Content-Length: {len(request_data)}\r\n"
        "\r\n"
    ).encode('ascii') + request_data

    try:
        # 发送请求
        with socket.create_connection(server_address, timeout=5) as sock:
            sock.sendall(http_request)

            # 接收响应
            response = b''
            while True:
                chunk = sock.recv(1024)
                if not chunk:
                    break
                response += chunk

        # 解析响应
        headers_end = response.index(b'\r\n\r\n') + 4
        headers = response[:headers_end].decode('ascii')
        body = response[headers_end:]

        # 检查状态码
        if '200 OK' not in headers:
            print(f"❌ 请求失败，状态码: {headers.split()[1]}")
            return False

        print(f"✅ 请求成功，状态码: 200 OK")
        print(f"📦 响应头信息:\n{headers}")
        print(f"💾 响应体长度: {len(body)} 字节")

        # 解析 SBE 响应
        if len(body) >= 21:
            trade_id, symbol, price, quantity = parse_sbe_trade_message(body)
            print(f"\n📈 解析到的交易信息:")
            print(f"   交易ID: {trade_id}")
            print(f"   符号: {symbol}")
            print(f"   价格: {price:.2f}")
            print(f"   数量: {quantity}")

            # 验证响应（根据服务器端的处理逻辑）
            expected_price = test_price * 1.01
            expected_quantity = test_quantity * 2

            price_match = abs(price - expected_price) < 0.0001
            quantity_match = quantity == expected_quantity

            print(f"\n✅ 验证结果:")
            print(f"   价格验证: {'通过' if price_match else '失败'} (期望: {expected_price:.2f}, 实际: {price:.2f})")
            print(f"   数量验证: {'通过' if quantity_match else '失败'} (期望: {expected_quantity}, 实际: {quantity})")

            return price_match and quantity_match

        else:
            print(f"❌ 响应体长度不足，无法解析 SBE 消息")
            return False

    except Exception as e:
        print(f"❌ 测试失败: {e}")
        return False

if __name__ == "__main__":
    print("🚀 测试 SBE 接口\n")
    success = test_sbe_api()
    print("\n" + "="*50)
    if success:
        print("✅ 所有测试通过！")
    else:
        print("❌ 测试失败！")