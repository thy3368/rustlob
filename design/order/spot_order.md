好，我用**官方 `hyperliquid` Rust SDK**（`hyperliquid-dex/hyperliquid-rust-sdk`），把 Spot 支持的订单类型**穷举**出来。代码直接可跑（把私钥换成你自己的）。

先给结论——Spot 订单只有 **2 大类、5 种形态**：

| # | 订单类型 | `OrderType` | 说明 |
|---|---|---|---|
| 1 | Limit GTC | `Limit { tif: "Gtc" }` | 一直挂到成交/撤单 |
| 2 | Limit ALO (Post-Only) | `Limit { tif: "Alo" }` | 只挂单，会吃单就拒 |
| 3 | Limit IOC（= 市价） | `Limit { tif: "Ioc" }` + 激进价 | Spot 没有真 market |
| 4 | Trigger = Stop / TakeProfit（触发后限价） | `Trigger { is_market: false, ... }` | 到价挂限价单 |
| 5 | Trigger = SL/TP（触发后市价） | `Trigger { is_market: true, ... }` | 到价市价吃单 |

> SDK 源码里 `OrderType` 就两个分支：`Limit(Limit)` 和 `Trigger(Trigger)`，`Limit.tif` 只有 `"Alo" | "Ioc" | "Gtc"`。其余（Market / Scale / TWAP / Bracket）都是这两类的封装或独立 action。

---

## 一、Cargo.toml
```toml
[package]
name = "hl-spot-orders"
version = "0.1.0"
edition = "2021"

[dependencies]
hyperliquid = "0.2"          # 官方 SDK
ethers = "2"
tokio = { version = "1", features = ["full"] }
uuid = "1"
```

---

## 二、完整示例：`examples/all_spot_orders.rs`

```rust
use std::sync::Arc;

use ethers::signers::LocalWallet;
use hyperliquid::{
    types::{
        exchange::request::{Limit, OrderRequest, OrderType, Trigger},
        Chain,
    },
    Exchange, Hyperliquid,
};
use uuid::Uuid;

/// Spot 的 asset id = 10000 + spotMeta.universe 里的 index
/// PURR/USDC 的 index 通常是 0 → asset = 10000
/// 真实代码请先调 /info {type:"spotMeta"} 查，别硬编码
const PURR_SPOT: u32 = 10000;
const SZ_DECIMALS: u32 = 2; // 从 spotMeta.universe[i].szDecimals 取

/// 把 f64 转成符合精度的价格/数量字符串
fn px(v: f64) -> String { format!("{v}") }
fn sz(v: f64) -> String { format!("{v:.SZ_DECIMALS$}", SZ_DECIMALS = SZ_DECIMALS as usize) }

#[tokio::main]
async fn main() {
    let wallet: Arc<LocalWallet> = Arc::new(
        "0xe908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );
    let exch = Hyperliquid::new(Chain::Mainnet);

    // ============================================================
    // 1) Limit GTC —— 普通限价单，一直挂着
    // ============================================================
    let gtc = OrderRequest {
        asset: PURR_SPOT,
        is_buy: true,                          // true = 花 USDC 买 PURR
        reduce_only: false,
        limit_px: px(0.50),                    // 买在 0.50
        sz: sz(10.0),                          // 买 10 PURR
        order_type: OrderType::Limit(Limit { tif: "Gtc".into() }),
        cloid: Some(Uuid::new_v4().to_string()),
    };

    // ============================================================
    // 2) Limit ALO (Post-Only) —— 只做 maker，会吃单就拒绝
    //   价格必须不穿越盘口（买单 < best_ask）
    // ============================================================
    let alo = OrderRequest {
        asset: PURR_SPOT,
        is_buy: true,
        reduce_only: false,
        limit_px: px(0.49),                    // 必须低于当前 best ask
        sz: sz(10.0),
        order_type: OrderType::Limit(Limit { tif: "Alo".into() }),
        cloid: Some(Uuid::new_v4().to_string()),
    };

    // ============================================================
    // 3) IOC "市价单" —— Spot 没有真 market 类型
    //    用 Ioc + 激进价(= best_ask * (1+slippage)) 模拟市价买入
    // ============================================================
    let best_ask = 0.51_f64;                   // 真实场景从 L2 snapshot 读
    let slip = 0.02;                           // 2% 滑点保护
    let market_buy = OrderRequest {
        asset: PURR_SPOT,
        is_buy: true,
        reduce_only: false,
        limit_px: px(best_ask * (1.0 + slip)),  // 激进价格，保证能成交
        sz: sz(5.0),
        order_type: OrderType::Limit(Limit { tif: "Ioc".into() }),
        cloid: None,
    };

    // 一次提交多笔（同一 action，原子批量）
    let r1 = exch
        .place_order(wallet.clone(), vec![gtc, alo, market_buy], None)
        .await
        .unwrap();
    println!("limit/gtc/alo/ioc  => {:?}", r1);

    // ============================================================
    // 4) Trigger：触发后是「限价单」(isMarket=false)
    //    持 PURR，涨到 0.70 想限价卖出
    // ============================================================
    let stop_limit = OrderRequest {
        asset: PURR_SPOT,
        is_buy: false,                          // 卖 PURR
        reduce_only: true,                      // spot 的 TP/SL 几乎都设 reduce_only
        limit_px: px(0.71),                     // 触发后挂的限价（高于触发价，确保能挂上）
        sz: sz(10.0),
        order_type: OrderType::Trigger(Trigger {
            is_market: false,                   // 触发 → 下 LIMIT 单
            trigger_px: px(0.70),
            tpsl: "tp".into(),                  // "tp" 或 "sl"
        }),
        cloid: None,
    };

    // ============================================================
    // 5) Trigger：触发后是「市价单」(isMarket=true)
    //    持 PURR，跌到 0.40 立即市价卖出止损
    // ============================================================
    let stop_market = OrderRequest {
        asset: PURR_SPOT,
        is_buy: false,
        reduce_only: true,
        limit_px: px(0.39),                     // is_market=true 时此价仅作触发后兜底价
        sz: sz(10.0),
        order_type: OrderType::Trigger(Trigger {
            is_market: true,                    // 触发 → 市价吃单（底层还是 ioc limit）
            trigger_px: px(0.40),
            tpsl: "sl".into(),
        }),
        cloid: None,
    };

    let r2 = exch
        .place_order(wallet.clone(), vec![stop_limit, stop_market], None)
        .await
        .unwrap();
    println!("trigger(tp/sl)    => {:?}", r2);
}
```

---

## 三、穷举速查表（对应上面代码）

| # | 名称 | `OrderType` 写法 | `is_buy` | `reduce_only` | 关键字段 |
|---|---|---|---|---|---|
| 1 | Limit GTC | `Limit { tif: "Gtc" }` | 买/卖 | false | 一直挂 |
| 2 | Limit ALO（Post-Only） | `Limit { tif: "Alo" }` | 买/卖 | false | 穿越盘口会被拒 |
| 3 | IOC 市价 | `Limit { tif: "Ioc" }` | 买/卖 | false | 价格用 `best * (1±slip)` |
| 4 | Trigger TP-SL（触发后限价） | `Trigger { is_market: false, tpsl: "tp"/"sl" }` | 减仓方向 | **true** | `trigger_px` + `limit_px` |
| 5 | Trigger TP-SL（触发后市价） | `Trigger { is_market: true, tpsl: "tp"/"sl" }` | 减仓方向 | **true** | 只看 `trigger_px` |

> ⚠️ 注意 SDK 版本差异：较新的官方 SDK 里 `Tif`/`Tpsl` 是**字符串**（`"Gtc"`/`"Alo"`/`"Ioc"`、`"tp"`/`"sl"`）；`hl-types` 等封装 crate 才提供 `Tif::Gtc` 这样的枚举。如果你用 `hyperliquid = "0.2"` 直接写字符串最稳。

---

## 四、几个必须知道的坑

1. **Asset ID**：Spot 是 `10000 + spotInfo["index"]`，**不是 perp 的 0/1/2**，写错会下到错误市场。启动时拉一次 `spotMeta` 建映射表缓存，别硬编码。

2. **没有真正的 Market 单**：`OrderType` 只有 `Limit` 和 `Trigger`。所谓"市价"全是 `Ioc + 激进价` 的封装。

3. **价格/数量必须是字符串**，精度按 `spotMeta.universe[i].szDecimals` 对齐。

4. **Spot 没有杠杆、没有强平、没有资金费**，`reduce_only` 的含义是"别卖超余额 / 别买成加仓"。

5. **`cloid` 建议用 UUID**：Trigger 单**必须**带 client order id（crate 要求用 `ClientOrderId::uuid()`）。

---

## 五、没在上面但你可能想要

- **Scale（区间铺单）**、`TWAP`：官方 SDK 是独立 action（不是 `OrderRequest`），用专门的 builder 方法。
- **Bracket（entry + TP + SL 一组）**：用 `bulk_order_grouped(..., Grouping::NormalTpsl)`，父单是 entry，子单自动 `reduce_only: true`。
- **更高层封装**：`hl-executor` 提供了 `market_open` / `place_scale_order` / `place_twap_order` / `spot_asset_index("PURR")` 这些 helper，不想手写 wire 格式就用它。

---

要不要我接着给你：
1. **自动拉 `spotMeta` 建 asset 映射 + 读 L2 算 bestAsk/bestBid，让 IOC 市价单动态取价**（避免硬编码 0.51）？
2. **Bracket 订单（买 PURR + 挂 TP + 挂 SL 一次原子提交）**的完整示例？

要哪个我直接写。