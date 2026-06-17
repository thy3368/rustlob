# Websocket

WebSocket endpoints support real-time data streaming on Hyperliquid:

- Mainnet: `wss://api.hyperliquid.xyz/ws`
- Testnet: `wss://api.hyperliquid-testnet.xyz/ws`

### Connecting

Connect to the appropriate URL, then send subscription messages to receive updates.

```
$ wscat -c wss://api.hyperliquid.xyz/ws
Connected (press CTRL+C to quit)
> { "method": "subscribe", "subscription": { "type": "trades", "coin": "SOL" } }
< {"channel":"subscriptionResponse","data":{"method":"subscribe","subscription":{"type":"trades","coin":"SOL"}}}
```

Automated users should handle server-side disconnects and reconnect gracefully — "missed data during the reconnect will be present in the snapshot ack on reconnect."

Python SDK references:
- [Types](https://github.com/hyperliquid-dex/hyperliquid-python-sdk/blob/master/hyperliquid/utils/types.py)
- [WebSocket manager](https://github.com/hyperliquid-dex/hyperliquid-python-sdk/blob/master/hyperliquid/websocket_manager.py)
