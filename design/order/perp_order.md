Hyperliquid Perp 的订单可以分成 **3 层**：底层只有 2 种 → 前端/SDK 封装出一堆 → 再叠加 flags / grouping。下面按“穷举”来列。

---

## 一、底层 Wire 层（真正发到链上）
Perp 和 Spot 一样，订单类型只有 2 个分支：

1. `limit`
    - `tif: "Gtc"` 好挂单，直到成交/撤单
    - `tif: "Alo"` Post-Only，会吃单就拒
    - `tif: "Ioc"` 立即成交，剩余取消（**“市价单”就是它 + 激进价**）
2. `trigger`
    - `isMarket: true` 触发后市价（底层 IOC limit）
    - `isMarket: false` 触发后限价
    - `tpsl: "tp" | "sl"`
    - 触发价看 **mark price**

> 没有 FOK，没有真正的 `market` 类型，没有原生 trailing stop。

---

## 二、用户/前端层（Perp 全支持）
| 名字 | 底层实现 | Perp 支持 |
|---|---|---|
| Limit | `limit/Gtc` | ✅ |
| Limit ALO / Post-Only | `limit/Alo` | ✅ |
| Limit IOC | `limit/Ioc` | ✅ |
| Market（伪市价） | `limit/Ioc` + best±滑点 | ✅ |
| Chase Order | ALO limit，浏览器里跟 best bid/ask 重定价 | ✅（前端特性） |
| Stop Market | `trigger/isMarket=true` | ✅ |
| Stop Limit | `trigger/isMarket=false` | ✅ |
| Take Profit Market | `trigger/tp/isMarket=true` | ✅ |
| Take Profit Limit | `trigger/tp/isMarket=false` | ✅ |
| Stop Loss Market | `trigger/sl/isMarket=true` | ✅ |
| Stop Loss Limit | `trigger/sl/isMarket=false` | ✅ |
| Scale Order | N 个 `limit` 铺价格区间 | ✅ |
| TWAP | 定时下 IOC/limit 子单，30s 间隔，滑点≤3% | ✅ |
| Bracket / Grouped TP-SL | 1 entry + N tp/sl trigger，原子提交 | ✅ |
| Position-level TP/SL | 对当前仓位挂 trigger | ✅ |

---

## 三、Order flags / 修饰符（Perp 特有概念）
Perp 比 Spot 多的是**仓位/保证金语义**，不是新订单类型：

- `reduceOnly: true`：只能减仓，不能开反向仓
- `cross` / `isolated`：仓位级保证金模式（不是订单字段，是账户/仓位状态）
- 杠杆：下单前通过 `updateLeverage` 设置，不是订单本体
- `client order id (cloid)`：追踪用

---

## 四、Perp 有、Spot 没的东西
- 强平单（系统下，不是用户订单）
- Backstop / HLP 接管（清算流程）
- ADL 减仓（系统行为）
- 资金费、维持保证金、破产价、mark price 触发
- TP/SL 常配合“开仓单”做 bracket（Spot 也能做但语义是买币/卖币）

---

## 五、Perp 不支持（别等）
- FOK
- 原生 Market（必须 IOC 模拟）
- 原生 Trailing Stop（只能自己写 bot 改 triggerPx）
- OCO 作为独立订单类型（用 bracket / tp+sl 近似）
- 条件单触发用 last price（Perp TP/SL 用 mark price）

---

## 六、一句话穷举
> Perp 订单 = **Limit(Gtc/Alo/Ioc)** + **Trigger(tp/sl, market/limit)**  
> 再叠加 **Scale / TWAP / Bracket / Chase / Position TP-SL**  
> 再叠加 **reduceOnly / cross-iso / leverage**  
> 系统侧还有 **强平 / Backstop / ADL**（不是你下的单）。

如果你要，我可以下一版直接给：

1. **Perp 订单类型 → JSON wire 示例全集**（limit / ioc / alo / stop / tp / sl / bracket / twap / scale）
2. **Rust 枚举映射表**：`MyOrderKind → hyperliquid::OrderType`
3. **Perp vs Spot 订单能力对照表（含哪些 flag 在 perp 才有意义）**

要哪个？