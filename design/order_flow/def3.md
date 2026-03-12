# Order Flow 指标清单

**来源**: Optimus Futures (Best Order Flow Indicators / Order Flow Trading) + City Traders Imperium (Order Flow Trading Analysis)

---

## 一、核心工具类指标

| 指标 | 说明 | 数据来源 |
|------|------|----------|
| DOM (Depth of Market) | 实时买卖限价单深度，Level 2 数据 | 交易所 Order Book |
| Footprint Chart (Cluster Chart) | 每根K线内部按价格分解的买卖成交量 | 逐笔成交 |
| Volume Profile | 按价格水平分布的成交量直方图 | 历史成交 |
| Heat Map | 流动性热力图，大额挂单可视化 | DOM 快照流 |
| Time & Sales (Tape) | 逐笔成交记录：价格、数量、时间、方向 | 逐笔成交 |

## 二、可计算的量化指标

| 指标 | 计算方式 | 用途 |
|------|----------|------|
| Delta | 主动买量 − 主动卖量 | 多空力量对比 |
| Cumulative Delta | Delta 的累计值 | 趋势中的买卖压力趋势 |
| Delta Divergence | 价格创新高/低但 Delta 减弱 | 反转预警 |
| POC (Point of Control) | 某时段内成交量最大的价格 | 价值中枢 / 磁吸位 |
| HVN (High Volume Node) | 成交量密集区 | 支撑/阻力区，价格移动慢 |
| LVN (Low Volume Node) | 成交量稀疏区 | 价格快速穿越区 |
| VWAP | Σ(价格×成交量) / Σ成交量 | 机构基准价，均值回归 |
| Volume Imbalance | 某价位买卖量比值（如 buy/sell > 3:1） | 失衡检测，突破确认 |
| Absorption | 大量对手方订单被吸收（如800买 vs 500卖） | 假突破识别 |
| Volume Climax | 成交量尖峰 + Delta 反转 | 力竭信号 |
| Iceberg Order Detection | 同一价位反复出现相同大小的限价单 | 机构大单识别 |
| Power Trade (大单扫描) | 短时间窗口内的大额成交聚集 | 短期动量方向 |
| Order Flow Rate | 单位时间内市价单到达速率 | 市场活跃度/情绪变化 |
| Bid/Ask Ratio | 买方限价单总量 / 卖方限价单总量 | 短期方向偏向 |

## 三、宏观/持仓类指标

| 指标 | 说明 | 用途 |
|------|------|------|
| COT Net Position | CFTC 报告中各类交易者净多/净空头寸 | 机构宏观方向 |
| COT Position Change | 周度持仓变化量 | 动量确认/反转预警 |
| Open Interest | 未平仓合约总量 | 资金流入/流出 |

## 四、衍生信号/模式

| 模式 | 描述 |
|------|------|
| Stop Hunt / Liquidity Sweep | 价格突破关键高低点触发止损后快速反转 |
| Bull/Bear Trap | 假突破诱多/诱空 |
| Bid Pulling / Offer Pulling | 限价单撤离，支撑/阻力消失 |
| Spoofing Detection | 大额挂单频繁出现又撤销（虚假流动性） |

## 五、RustLOB 可实现性分析

### 可直接从撮合引擎计算（数据已有）

- **Delta / Cumulative Delta** — 每笔成交记录 aggressor side，直接聚合
- **Volume Profile (POC / HVN / LVN)** — 按价格桶聚合历史成交量
- **VWAP** — 逐笔成交的价格×数量累计
- **Volume Imbalance** — 按价位统计买卖量比
- **Absorption** — 检测某价位大量对手方成交被吸收
- **Footprint Chart 数据** — 按K线时间窗 + 价格桶聚合买卖量
- **Power Trade** — 滑动时间窗口内大额成交检测
- **Order Flow Rate** — 市价单到达速率统计
- **Time & Sales** — 逐笔成交流直接输出

### 可从 Order Book 快照计算（数据已有）

- **DOM 深度** — Order Book 本身
- **Bid/Ask Ratio** — 买卖盘总量比
- **Heat Map** — DOM 快照序列的流动性分布
- **Iceberg Detection** — 同价位限价单反复刷新检测
- **Bid/Offer Pulling** — DOM 快照差分，检测限价单撤离
- **Spoofing Detection** — 大额挂单出现后短时间内撤销

### 需要外部数据源

- **COT 报告** — CFTC 周度数据，需额外接入
- **Open Interest** — 合约市场数据，现货无此概念（衍生品阶段需要）
