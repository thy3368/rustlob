# WebSocket

WebSocket 端点支持 Hyperliquid 上的实时数据流：

- 主网：`wss://api.hyperliquid.xyz/ws`
- 测试网：`wss://api.hyperliquid-testnet.xyz/ws`

### 连接

连接到适当的 URL，然后发送订阅消息以接收更新。

```
$ wscat -c wss://api.hyperliquid.xyz/ws
Connected (press CTRL+C to quit)
> { "method": "subscribe", "subscription": { "type": "trades", "coin": "SOL" } }
< {"channel":"subscriptionResponse","data":{"method":"subscribe","subscription":{"type":"trades","coin":"SOL"}}}
```

自动用户应优雅地处理服务器端断开连接和重新连接 — "重新连接期间丢失的数据将存在于重新连接时的快照确认中。"

Python SDK 参考：
- [类型](https://github.com/hyperliquid-dex/hyperliquid-python-sdk/blob/master/hyperliquid/utils/types.py)
- [WebSocket 管理器](https://github.com/hyperliquid-dex/hyperliquid-python-sdk/blob/master/hyperliquid/websocket_manager.py)
