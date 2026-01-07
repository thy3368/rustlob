#!/usr/bin/env python3
import asyncio
import websockets
import time
import statistics
import json

async def connect_and_send():
    try:
        async with websockets.connect('ws://localhost:8080') as websocket:
            # 发送消息
            msg = json.dumps({"text": "Hello World"})
            await websocket.send(msg)

            # 接收响应
            response = await websocket.recv()
            return True
    except Exception as e:
        print(f"Error: {e}")
        return False

async def test_single_connection(num_runs):
    latencies = []

    for _ in range(num_runs):
        start_time = time.time()
        success = await connect_and_send()
        end_time = time.time()

        if success:
            latency = (end_time - start_time) * 1000  # 毫秒
            latencies.append(latency)

    if latencies:
        avg_latency = statistics.mean(latencies)
        p95_latency = sorted(latencies)[int(len(latencies) * 0.95)]
        p99_latency = sorted(latencies)[int(len(latencies) * 0.99)]

        print(f"Single connection - Runs: {len(latencies)}, Avg: {avg_latency:.2f}ms, P95: {p95_latency:.2f}ms, P99: {p99_latency:.2f}ms")

    return latencies

async def test_multiple_connections(num_connections):
    tasks = []

    for i in range(num_connections):
        task = asyncio.create_task(connect_and_send())
        tasks.append(task)

    results = await asyncio.gather(*tasks)
    successful_connections = sum(1 for result in results if result)

    print(f"Multiple connections - Attempted: {num_connections}, Successful: {successful_connections}")

    return successful_connections

async def main():
    print("WebSocket Performance Test")
    print("=" * 50)

    # 测试单连接延迟
    print("\n1. Single Connection Latency Test")
    print("-" * 30)
    latencies = await test_single_connection(100)

    # 测试多个连接
    print("\n2. Multiple Connections Test")
    print("-" * 30)
    for num_conns in [10, 100, 200, 500, 1000]:
        print(f"\nTesting {num_conns} connections...")
        successful = await test_multiple_connections(num_conns)
        await asyncio.sleep(1)  # 等待连接清理

    print("\n" + "=" * 50)
    print("Test completed!")

if __name__ == "__main__":
    asyncio.run(main())