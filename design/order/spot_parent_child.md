有，但**不是传统 CEX 那种「父单成交后自动下子单」的主子单树**，而是 **TPSL Bracket（括号单）**。

---

## 一、Hyperliquid 的"主子单"是什么？

叫 **`Grouped Order` / `Bracket Order`**，本质：

> **一笔 entry 单 + 一笔或多笔 TP/SL 单，打包成一个 action 原子提交。**

子单（TP/SL）**不会立刻下到订单簿**，而是：
- 等父单成交 → 子单才激活
- 父单没成交 → 子单不存在
- 父单被撤 → 子单自动取消

---

## 二、API 层面怎么表达

`/exchange` action 类型：

```json
{
  "type": "order",
  "orders": [
    { "asset": 10000, "isBuy": true, "sz": "10", "limitPx": "0.50", "orderType": { "limit": { "tif": "Gtc" } } }
  ],
  "grouping": "normalTpsl",
  "tpsl": [
    { "asset": 10000, "isBuy": false, "sz": "10", "triggerPx": "0.70", "orderType": { "trigger": { "isMarket": true, "tpsl": "tp" } }, "reduceOnly": true },
    { "asset": 10000, "isBuy": false, "sz": "10", "triggerPx": "0.40", "orderType": { "trigger": { "trigger": { "isMarket": true, "tpsl": "sl" } }, "reduceOnly": true }
  ]
}
```

> 一个 `orders[0]` 是父单，`tpsl[]` 是子单。  
> `grouping: "normalTpsl"` 告诉系统：这是一组 bracket。

---

## 三、Rust SDK 写法

官方 SDK 里用 `bulk_order_grouped`：

```rust
use hyperliquid::{
    types::exchange::request::{Limit, OrderRequest, OrderType, Trigger},
    Exchange,
};

let entry = OrderRequest {
    asset: 10000,
    is_buy: true,
    reduce_only: false,
    limit_px: "0.50".into(),
    sz: "10".into(),
    order_type: OrderType::Limit(Limit { tif: "Gtc".into() }),
    cloid: None,
};

let tp = OrderRequest {
    asset: 10000,
    is_buy: false,
    reduce_only: true,
    limit_px: "0.70".into(),
    sz: "10".into(),
    order_type: OrderType::Trigger(Trigger {
        is_market: true,
        trigger_px: "0.70".into(),
        tpsl: "tp".into(),
    }),
    cloid: None,
};

let sl = OrderRequest {
    asset: 10000,
    is_buy: false,
    reduce_only: true,
    limit_px: "0.40".into(),
    sz: "10".into(),
    order_type: OrderType::Trigger(Trigger {
        is_market: true,
        trigger_px: "0.40".into(),
        tpsl: "sl".into(),
    }),
    cloid: None,
};

// 原子提交：entry + tp + sl
let resp = exch
    .bulk_order_grouped(
        wallet.clone(),
        vec![entry],
        vec![tp, sl],
        hyperliquid::types::exchange::request::Grouping::NormalTpsl,
        None,
    )
    .await
    .unwrap();
```

---

## 四、和传统 CEX 主子单的区别

| 维度 | Binance / OKX OCO / 主子单 | Hyperliquid Bracket |
|---|---|---|
| 父单类型 | 市价/限价/条件 | 限价（spot 暂不支持父单是 trigger） |
| 子单激活时机 | 父成交后 | 父成交后 |
| 原子性 | 部分支持 | ✅ 完全原子（一个 block） |
| 子单数量 | 通常 1 TP + 1 SL | 多个 TP/SL 都行 |
| 父撤 → 子撤 | ✅ | ✅ |
| 子成交 → 其他子撤 | ✅ | ✅ |
| 部分成交 | 按剩余仓位比例缩子单 | 按剩余仓位比例缩子单 |

---

## 五、关键行为

### 1. 部分成交
父单买 10 个，只成交 6 个：
- TP/SL 子单自动缩到 6 个
- 不是"全部取消"也不是"保持原样"

### 2. 一个子成交，另一个自动取消
- TP 触发成交了 → SL 自动撤
- SL 触发成交了 → TP 自动撤

### 3. 父单取消 → 子单全部取消
- 手动撤父单 → 系统自动撤所有关联的 TP/SL

### 4. 子单必须 `reduce_only: true`
- Spot 里 = 只能卖你持有的币 / 用你有的 USDC 买回
- Perp 里 = 只能减仓位

---

## 六、Spot 的限制

- **父单不能是 trigger**（spot 的 trigger 只能做减仓）
- 所以 spot bracket 的玩法：
    - 父单 = limit GTC 买（建仓）
    - 子单 = TP + SL（都是 trigger，reduce_only=true）
- 如果你想"先卖后买回"，反过来：父单 = limit sell，子单 = 低处买回的 trigger

---

## 七、一句话

> Hyperliquid 有主子单，叫 **Grouped / Bracket Order**。  
> 父单成交 → 子单激活；父撤 → 子撤；子成交 → 兄弟撤。  
> 不是 OCO（二选一），是 **TPSL bracket（多子单）**，原子提交，链上执行。

要不要我给你一个 **Spot 完整 bracket 例子：买 PURR + 挂 TP + 挂 SL，一次提交，含自动拉 spotMeta 算 asset id**？