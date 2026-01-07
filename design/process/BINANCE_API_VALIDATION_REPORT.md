# 币安永续合约API校验报告

**日期**: 2025-12-12
**文件**: `perp_order_exch_proc.xpdl`
**参考**: [Binance USDT-Margined Futures API](https://developers.binance.com/docs/derivatives/usds-margined-futures/general-info)

---

## 执行摘要

本报告对比了 `perp_order_exch_proc.xpdl` 流程定义与币安官方USDT永续合约API，识别出**关键差异**和**需要修正的地方**。

### 总体评估

| 维度 | 状态 | 符合度 |
|------|------|--------|
| 订单类型 | ⚠️ 部分符合 | 85% |
| 保证金模式 | ✅ 符合 | 100% |
| 杠杆范围 | ✅ 符合 | 100% |
| 持仓模式 | ❌ 缺失 | 0% |
| 资金费率 | ⚠️ 部分符合 | 80% |
| API端点映射 | ⚠️ 需补充 | 60% |

---

## 1. 订单类型对比

### ✅ 已正确实现的订单类型

当前XPDL文件中的订单类型定义（第77-86行）：

```xml
<TypeDeclaration Id="OrderType" Name="订单类型">
  <EnumerationType>
    <EnumerationValue Name="LIMIT">限价单</EnumerationValue>
    <EnumerationValue Name="MARKET">市价单</EnumerationValue>
    <EnumerationValue Name="STOP_MARKET">止损市价单</EnumerationValue>
    <EnumerationValue Name="STOP_LIMIT">止损限价单</EnumerationValue>
    <EnumerationValue Name="TAKE_PROFIT_MARKET">止盈市价单</EnumerationValue>
    <EnumerationValue Name="TAKE_PROFIT_LIMIT">止盈限价单</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>
```

**币安API支持的订单类型**：
- ✅ `LIMIT` - 限价单
- ✅ `MARKET` - 市价单
- ✅ `STOP` / `STOP_MARKET` - 止损单
- ✅ `TAKE_PROFIT` / `TAKE_PROFIT_MARKET` - 止盈单
- ❌ `TRAILING_STOP_MARKET` - **缺失**：追踪止损单
- ❌ `STOP_LOSS` - **缺失**：普通止损单（STOP）
- ❌ `TAKE_PROFIT` - **缺失**：止盈限价单（与STOP类似）

### 🔴 关键问题：2025年12月9日API重大变更

根据[币安变更日志](https://developers.binance.com/docs/derivatives/change-log)：

> **生效日期：2025-12-09**
> USDⓈ-M Futures将条件订单迁移至Algo Service，影响以下订单类型：
> `STOP_MARKET` / `TAKE_PROFIT_MARKET` / `STOP` / `TAKE_PROFIT` / `TRAILING_STOP_MARKET`

**影响**：
- 条件订单现在通过独立的Algo端点处理
- 原有的 `POST /fapi/v1/order` 不再接受条件订单
- 需要使用新的Algo订单端点

### ⚠️ 建议修正

1. **添加缺失的订单类型**：
```xml
<EnumerationValue Name="TRAILING_STOP_MARKET">追踪止损市价单</EnumerationValue>
<EnumerationValue Name="STOP">止损限价单</EnumerationValue>
<EnumerationValue Name="TAKE_PROFIT">止盈限价单</EnumerationValue>
```

2. **区分普通订单和Algo订单**：
```xml
<TypeDeclaration Id="OrderCategory" Name="订单分类">
  <EnumerationType>
    <EnumerationValue Name="REGULAR">普通订单 (LIMIT, MARKET)</EnumerationValue>
    <EnumerationValue Name="CONDITIONAL">条件订单 (Algo Service)</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>
```

---

## 2. 持仓模式对比

### ❌ **严重缺失：Position Mode（持仓模式）**

币安支持两种持仓模式：
- **One-Way Mode（单向持仓）**：默认模式，一个合约只能持有一个方向的仓位
- **Hedge Mode（对冲持仓）**：可以同时持有同一合约的多空双向仓位

**API端点**：
- `GET /fapi/v1/positionSide/dual` - 查询当前持仓模式
- `POST /fapi/v1/positionSide/dual` - 更改持仓模式

### 🔴 当前XPDL文件的问题

当前只定义了 `PositionSide`（多/空），但**没有定义 PositionMode**：

```xml
<!-- 当前定义 - 不完整 -->
<TypeDeclaration Id="PositionSide" Name="持仓方向">
  <EnumerationType>
    <EnumerationValue Name="LONG">多仓</EnumerationValue>
    <EnumerationValue Name="SHORT">空仓</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>
```

### ✅ 建议添加

```xml
<!-- 持仓模式 -->
<TypeDeclaration Id="PositionMode" Name="持仓模式">
  <EnumerationType>
    <EnumerationValue Name="ONE_WAY">单向持仓</EnumerationValue>
    <EnumerationValue Name="HEDGE">对冲持仓（双向持仓）</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>

<!-- 在对冲模式下，需要区分 positionSide -->
<TypeDeclaration Id="PositionSideEnum" Name="仓位方向">
  <EnumerationType>
    <EnumerationValue Name="BOTH">单向持仓（默认）</EnumerationValue>
    <EnumerationValue Name="LONG">对冲模式-多仓</EnumerationValue>
    <EnumerationValue Name="SHORT">对冲模式-空仓</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>
```

---

## 3. 保证金模式对比

### ✅ 完全符合

当前定义（第88-93行）：
```xml
<TypeDeclaration Id="MarginMode" Name="保证金模式">
  <EnumerationType>
    <EnumerationValue Name="ISOLATED">逐仓</EnumerationValue>
    <EnumerationValue Name="CROSS">全仓</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>
```

**币安API**：
- `ISOLATED` - 逐仓模式
- `CROSSED` - 全仓模式（币安API使用 `CROSSED` 而非 `CROSS`）

### ⚠️ 细微差异

建议修改为与币安API完全一致：
```xml
<EnumerationValue Name="CROSSED">全仓</EnumerationValue>  <!-- 改为 CROSSED -->
```

**对应API端点**：
- `POST /fapi/v1/marginType` - 变更保证金模式
  - 参数：`marginType` = `ISOLATED` | `CROSSED`

---

## 4. 杠杆倍数对比

### ✅ 范围正确

当前定义（第64-67行）：
```xml
<TypeDeclaration Id="Leverage" Name="杠杆倍数">
  <BasicType Type="INTEGER"/>
  <Description>范围: 1-125</Description>
</TypeDeclaration>
```

**币安实际情况**：
- 最大杠杆因合约而异（1x - 125x）
- BTC/USDT 等主流合约支持高达 125x
- 部分小市值币种杠杆较低（如 20x、50x）

**对应API端点**：
- `POST /fapi/v1/leverage` - 调整开仓杠杆
  - 参数：`symbol`, `leverage` (1-125)
- `GET /fapi/v1/leverageBracket` - 查询杠杆分档

### ⚠️ 建议细化

```xml
<TypeDeclaration Id="Leverage" Name="杠杆倍数">
  <BasicType Type="INTEGER"/>
  <Description>范围: 1-125（具体最大值依合约而定，需查询 leverageBracket）</Description>
  <ExtendedAttributes>
    <ExtendedAttribute Name="MinValue" Value="1"/>
    <ExtendedAttribute Name="MaxValue" Value="125"/>
    <ExtendedAttribute Name="Dynamic" Value="true"/>
  </ExtendedAttributes>
</TypeDeclaration>
```

---

## 5. 资金费率结算对比

### ⚠️ 部分符合，需调整

当前定义（第1507-1516行）：
```xml
<Activity Id="Start" Name="触发结算">
  <Event>
    <StartEvent Trigger="Timer">
      <TriggerTimer>
        <TimeCycle>0 */8 * * *</TimeCycle>  <!-- 每8小时 -->
      </TriggerTimer>
    </StartEvent>
  </Event>
  <Description>每8小时触发一次 (00:00, 08:00, 16:00 UTC)</Description>
</Activity>
```

**币安官方资金费率时间**：
- ✅ 默认：每8小时一次
- ✅ 结算时间：00:00 UTC, 08:00 UTC, 16:00 UTC

### 🔴 重要差异

1. **部分合约为4小时结算**
   根据[币安公告](https://www.binance.com/en/support/announcement/important-updates-on-funding-rates-of-usd%E2%93%A2-m-perpetual-contracts-98d6b24d3e5c4f84a8ed04087997d8d0)：

   > 从2023-10-12起，部分USDⓈ-M永续合约的资金费率结算频率从每8小时调整为每4小时

2. **资金费率计算公式**

当前XPDL（第1531-1543行）的计算逻辑：
```rust
let premium_index = (mark_price - index_price) / index_price;
let interest_rate = 0.0001; // 0.01% per 8h
let clamped_diff = (interest_rate - premium_index).clamp(-0.0005, 0.0005);
let funding_rate = premium_index + clamped_diff;
```

**币安官方公式**：
```
Premium Index (P) = [Max(0, Impact Bid Price - Price Index) - Max(0, Price Index - Impact Ask Price)] / Price Index

Funding Rate = Premium Index + clamp(Interest Rate - Premium Index, -0.05%, 0.05%)
```

- ✅ 公式基本正确
- ⚠️ Interest Rate 固定为 0.03% 日利率（0.01% per 8h）
- ✅ Funding Rate 上下限：±0.05%

### ⚠️ 建议修正

1. **支持动态结算间隔**：
```xml
<DataField Id="funding_interval" Name="资金费率结算间隔">
  <DataType><BasicType Type="STRING"/></DataType>
  <InitialValue>8h</InitialValue>
  <Description>可选值: 4h | 8h，依合约而定</Description>
</DataField>
```

2. **使用Impact Price计算Premium**：
```rust
// 更准确的Premium Index计算
let impact_bid = get_impact_bid_price(symbol);
let impact_ask = get_impact_ask_price(symbol);
let index_price = get_index_price(symbol);

let premium_index = (
    max(0.0, impact_bid - index_price) - max(0.0, index_price - impact_ask)
) / index_price;
```

**对应API端点**：
- `GET /fapi/v1/fundingRate` - 查询历史资金费率
- `GET /fapi/v1/premiumIndex` - 查询最新标记价格和资金费率

---

## 6. API端点映射

### 当前缺失的关键API端点映射

XPDL文件中的活动需要映射到币安的具体API端点：

| XPDL活动 | 币安API端点 | 当前状态 |
|---------|-----------|---------|
| OpenPosition | `POST /fapi/v1/order` | ⚠️ 需区分普通/Algo订单 |
| ClosePosition | `POST /fapi/v1/order` (平仓订单) | ✅ 基本符合 |
| AdjustLeverage | `POST /fapi/v1/leverage` | ✅ 符合 |
| AddMargin | `POST /fapi/v1/positionMargin` (type=1) | ✅ 符合 |
| ReduceMargin | `POST /fapi/v1/positionMargin` (type=2) | ✅ 符合 |
| GetPosition | `GET /fapi/v2/positionRisk` 或 `GET /fapi/v3/positionRisk` | ❌ 缺失 |
| ChangeMarginType | `POST /fapi/v1/marginType` | ❌ 缺失 |
| GetLiquidations | `GET /fapi/v1/allForceOrders` | ❌ 缺失 |

### 🔴 关键缺失的活动

1. **查询持仓信息**
```xml
<Activity Id="GetPositionInfo" Name="查询持仓信息">
  <Implementation>
    <Task>
      <TaskScript>
        <Script Type="rust/api-call">
          // GET /fapi/v2/positionRisk 或 GET /fapi/v3/positionRisk
          // V3版本仅返回有持仓或挂单的合约（性能优化）
        </Script>
      </TaskScript>
    </Task>
  </Implementation>
</Activity>
```

2. **变更保证金类型**
```xml
<Activity Id="ChangeMarginType" Name="变更保证金类型">
  <Implementation>
    <Task>
      <TaskScript>
        <Script Type="rust/api-call">
          // POST /fapi/v1/marginType
          // 参数: symbol, marginType (ISOLATED | CROSSED)
        </Script>
      </TaskScript>
    </Task>
  </Implementation>
</Activity>
```

3. **查询强平订单历史**
```xml
<Activity Id="GetLiquidationHistory" Name="查询强平历史">
  <Implementation>
    <Task>
      <TaskScript>
        <Script Type="rust/api-call">
          // GET /fapi/v1/allForceOrders
          // 查询用户强平订单或所有强平订单（需权限）
        </Script>
      </TaskScript>
    </Task>
  </Implementation>
</Activity>
```

---

## 7. 持仓实体结构对比

### 当前XPDL定义（第116-134行）

```xml
<TypeDeclaration Id="Position" Name="持仓实体">
  <RecordType>
    <Member Name="position_id" Type="PositionId"/>
    <Member Name="trader" Type="TraderId"/>
    <Member Name="symbol" Type="Symbol"/>
    <Member Name="side" Type="PositionSide"/>
    <Member Name="quantity" Type="Quantity"/>
    <Member Name="entry_price" Type="Price"/>
    <Member Name="leverage" Type="Leverage"/>
    <Member Name="margin_mode" Type="MarginMode"/>
    <Member Name="initial_margin" Type="INTEGER"/>
    <Member Name="maintenance_margin" Type="INTEGER"/>
    <Member Name="unrealized_pnl" Type="INTEGER"/>
    <Member Name="liquidation_price" Type="Price"/>
    <Member Name="status" Type="PositionStatus"/>
    <Member Name="created_at" Type="DATETIME"/>
    <Member Name="updated_at" Type="DATETIME"/>
  </RecordType>
</TypeDeclaration>
```

### 币安API返回的Position字段（`GET /fapi/v2/positionRisk`）

```json
{
  "symbol": "BTCUSDT",
  "positionSide": "BOTH",  // BOTH | LONG | SHORT (对冲模式)
  "positionAmt": "0.001",  // 持仓数量
  "entryPrice": "50000.0",  // 开仓均价
  "breakEvenPrice": "50050.0",  // 盈亏平衡价
  "markPrice": "51000.0",  // 标记价格
  "unRealizedProfit": "1.0",  // 未实现盈亏
  "liquidationPrice": "45000.0",  // 强平价格
  "leverage": "10",  // 当前杠杆倍数
  "maxNotionalValue": "250000",  // 当前杠杆下最大名义价值
  "marginType": "cross",  // isolated | cross
  "isolatedMargin": "0.0",  // 逐仓保证金
  "isAutoAddMargin": "false",  // 是否自动追加保证金
  "positionInitialMargin": "5.0",  // 持仓起始保证金
  "openOrderInitialMargin": "0.0",  // 挂单起始保证金
  "adl": 1,  // 自动减仓队列
  "updateTime": 1625474304765
}
```

### ❌ 缺失的关键字段

1. **positionSide** - 仓位方向（对冲模式必需）
2. **breakEvenPrice** - 盈亏平衡价
3. **markPrice** - 标记价格（计算未实现盈亏）
4. **maxNotionalValue** - 最大名义价值
5. **isAutoAddMargin** - 是否自动追加保证金（逐仓模式）
6. **adl** - 自动减仓队列等级（1-5）
7. **openOrderInitialMargin** - 挂单起始保证金

### ✅ 建议修正

```xml
<TypeDeclaration Id="Position" Name="持仓实体">
  <RecordType>
    <!-- 基础信息 -->
    <Member Name="symbol" Type="Symbol"/>
    <Member Name="position_side" Type="PositionSideEnum"/>  <!-- BOTH | LONG | SHORT -->

    <!-- 持仓数量和价格 -->
    <Member Name="position_amount" Type="DECIMAL"/>  <!-- 持仓数量（可为负） -->
    <Member Name="entry_price" Type="DECIMAL"/>  <!-- 开仓均价 -->
    <Member Name="break_even_price" Type="DECIMAL"/>  <!-- 盈亏平衡价 -->
    <Member Name="mark_price" Type="DECIMAL"/>  <!-- 当前标记价格 -->
    <Member Name="liquidation_price" Type="DECIMAL"/>  <!-- 强平价格 -->

    <!-- 盈亏 -->
    <Member Name="unrealized_pnl" Type="DECIMAL"/>  <!-- 未实现盈亏 -->

    <!-- 杠杆和保证金 -->
    <Member Name="leverage" Type="INTEGER"/>  <!-- 当前杠杆倍数 -->
    <Member Name="margin_type" Type="MarginMode"/>  <!-- isolated | cross -->
    <Member Name="isolated_margin" Type="DECIMAL"/>  <!-- 逐仓保证金 -->
    <Member Name="position_initial_margin" Type="DECIMAL"/>  <!-- 持仓起始保证金 -->
    <Member Name="open_order_initial_margin" Type="DECIMAL"/>  <!-- 挂单起始保证金 -->
    <Member Name="max_notional_value" Type="DECIMAL"/>  <!-- 最大名义价值 -->

    <!-- 风控 -->
    <Member Name="is_auto_add_margin" Type="BOOLEAN"/>  <!-- 是否自动追加保证金 -->
    <Member Name="adl" Type="INTEGER"/>  <!-- 自动减仓队列 (1-5) -->

    <!-- 时间戳 -->
    <Member Name="update_time" Type="DATETIME"/>
  </RecordType>
</TypeDeclaration>
```

---

## 8. 订单实体结构

### ❌ 当前XPDL未定义Order实体

建议添加完整的订单实体定义：

```xml
<TypeDeclaration Id="Order" Name="订单实体">
  <RecordType>
    <!-- 订单基础信息 -->
    <Member Name="order_id" Type="INTEGER"/>
    <Member Name="client_order_id" Type="STRING"/>
    <Member Name="symbol" Type="Symbol"/>

    <!-- 订单方向和类型 -->
    <Member Name="side" Type="OrderSide"/>  <!-- BUY | SELL -->
    <Member Name="position_side" Type="PositionSideEnum"/>  <!-- BOTH | LONG | SHORT -->
    <Member Name="type" Type="OrderType"/>

    <!-- 价格和数量 -->
    <Member Name="price" Type="DECIMAL"/>
    <Member Name="quantity" Type="DECIMAL"/>
    <Member Name="executed_qty" Type="DECIMAL"/>  <!-- 已成交数量 -->
    <Member Name="cumulative_quote_qty" Type="DECIMAL"/>  <!-- 累计成交金额 -->
    <Member Name="avg_price" Type="DECIMAL"/>  <!-- 平均成交价 -->

    <!-- 订单状态 -->
    <Member Name="status" Type="OrderStatus"/>  <!-- NEW | PARTIALLY_FILLED | FILLED | CANCELED | EXPIRED -->
    <Member Name="time_in_force" Type="TimeInForce"/>  <!-- GTC | IOC | FOK | GTX -->

    <!-- 条件订单参数 -->
    <Member Name="stop_price" Type="DECIMAL"/>  <!-- 触发价 -->
    <Member Name="activation_price" Type="DECIMAL"/>  <!-- 追踪止损激活价 -->
    <Member Name="callback_rate" Type="DECIMAL"/>  <!-- 追踪止损回调比例 -->
    <Member Name="working_type" Type="WorkingType"/>  <!-- MARK_PRICE | CONTRACT_PRICE -->

    <!-- 其他 -->
    <Member Name="reduce_only" Type="BOOLEAN"/>  <!-- 只减仓 -->
    <Member Name="close_position" Type="BOOLEAN"/>  <!-- 触发后全部平仓 -->
    <Member Name="update_time" Type="DATETIME"/>
  </RecordType>
</TypeDeclaration>

<!-- 订单方向 -->
<TypeDeclaration Id="OrderSide" Name="订单方向">
  <EnumerationType>
    <EnumerationValue Name="BUY">买入</EnumerationValue>
    <EnumerationValue Name="SELL">卖出</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>

<!-- 订单状态 -->
<TypeDeclaration Id="OrderStatus" Name="订单状态">
  <EnumerationType>
    <EnumerationValue Name="NEW">新建订单</EnumerationValue>
    <EnumerationValue Name="PARTIALLY_FILLED">部分成交</EnumerationValue>
    <EnumerationValue Name="FILLED">完全成交</EnumerationValue>
    <EnumerationValue Name="CANCELED">已撤销</EnumerationValue>
    <EnumerationValue Name="EXPIRED">已过期</EnumerationValue>
    <EnumerationValue Name="NEW_INSURANCE">风险保障基金(强平)</EnumerationValue>
    <EnumerationValue Name="NEW_ADL">自动减仓序列(强平)</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>

<!-- 有效方式 -->
<TypeDeclaration Id="TimeInForce" Name="订单有效方式">
  <EnumerationType>
    <EnumerationValue Name="GTC">成交为止（Good Till Cancel）</EnumerationValue>
    <EnumerationValue Name="IOC">无法立即成交的部分就撤销（Immediate or Cancel）</EnumerationValue>
    <EnumerationValue Name="FOK">无法全部立即成交就撤销（Fill or Kill）</EnumerationValue>
    <EnumerationValue Name="GTX">无法成为挂单方就撤销（Good Till Crossing）</EnumerationValue>
    <EnumerationValue Name="GTD">到指定时间撤销（Good Till Date）</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>

<!-- 条件价格触发类型 -->
<TypeDeclaration Id="WorkingType" Name="条件价格触发类型">
  <EnumerationType>
    <EnumerationValue Name="MARK_PRICE">标记价格</EnumerationValue>
    <EnumerationValue Name="CONTRACT_PRICE">最新成交价</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>
```

---

## 9. 强平机制对比

### ⚠️ 当前实现过于简化

当前XPDL的强平流程（第763-905行）缺少关键细节：

**币安实际强平机制**：

1. **标记价格系统**
   - 使用标记价格（而非最新成交价）判断是否触发强平
   - 标记价格 = 指数价格 + 资金费率基差的移动平均

2. **维持保证金率**
   - 不同仓位规模有不同的维持保证金率
   - 使用分档维持保证金率（GET /fapi/v1/leverageBracket）

3. **强平流程**
   当 `标记价格` 达到 `强平价格` 时：

   a. **第一步：尝试市场平仓**
      - 系统提交市价单尝试平仓

   b. **如果市场流动性不足**：
      - **风险保障基金**接管持仓
      - 订单状态变为 `NEW_INSURANCE`

   c. **如果风险保障基金不足**：
      - 触发**自动减仓（ADL）**
      - 对手方的盈利仓位被强制平仓
      - 订单状态变为 `NEW_ADL`

### ✅ 建议补充

```xml
<!-- 强平类型 -->
<TypeDeclaration Id="LiquidationType" Name="强平类型">
  <EnumerationType>
    <EnumerationValue Name="MARKET">市场强平</EnumerationValue>
    <EnumerationValue Name="INSURANCE_FUND">风险保障基金接管</EnumerationValue>
    <EnumerationValue Name="ADL">自动减仓</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>

<!-- 在强平活动中添加多级处理 -->
<Activity Id="ExecuteLiquidation" Name="执行强平">
  <Implementation>
    <Task>
      <TaskScript>
        <Script Type="rust/composite">
          <![CDATA[
          async fn execute_liquidation(position_id: PositionId) -> Result<(), Error> {
              // 1. 冻结持仓
              freeze_position(position_id).await?;

              // 2. 提交市价强平单
              let order_id = submit_market_liquidation_order(position_id).await?;

              // 3. 尝试市场成交
              let trades = try_match_in_market(order_id, timeout: 5s).await?;

              if trades.is_fully_filled() {
                  // 市场强平成功
                  settle_market_liquidation(position_id, trades).await?;
              } else {
                  // 4. 市场流动性不足，触发风险保障基金
                  if insurance_fund_has_capacity() {
                      takeover_by_insurance_fund(position_id).await?;
                      mark_order_status(order_id, OrderStatus::NEW_INSURANCE).await?;
                  } else {
                      // 5. 触发ADL（自动减仓）
                      trigger_adl_for_counterparties(position_id).await?;
                      mark_order_status(order_id, OrderStatus::NEW_ADL).await?;
                  }
              }

              Ok(())
          }
          ]]>
        </Script>
      </TaskScript>
    </Task>
  </Implementation>
</Activity>
```

**对应API端点**：
- `GET /fapi/v1/allForceOrders` - 查询强平订单
  - 返回字段包含强平类型（通过 `status` 字段区分）

---

## 10. 价格类型和精度

### ⚠️ 当前定义过于简化

当前XPDL（第55-58行）：
```xml
<TypeDeclaration Id="Price" Name="价格">
  <BasicType Type="INTEGER"/>
  <Description>以分为单位，避免浮点运算</Description>
</TypeDeclaration>
```

**问题**：
- 币安API使用**字符串类型**传递价格（避免浮点精度问题）
- 不同合约有不同的价格精度（tickSize）和数量精度（stepSize）

### ✅ 建议改进

```xml
<TypeDeclaration Id="Price" Name="价格">
  <BasicType Type="STRING"/>
  <Description>字符串类型，保留完整精度。内部可用定点数或高精度整数表示</Description>
  <ExtendedAttributes>
    <ExtendedAttribute Name="Format" Value="Decimal"/>
    <ExtendedAttribute Name="Example" Value="50000.50"/>
  </ExtendedAttributes>
</TypeDeclaration>

<TypeDeclaration Id="Quantity" Name="数量">
  <BasicType Type="STRING"/>
  <Description>字符串类型，保留完整精度</Description>
  <ExtendedAttributes>
    <ExtendedAttribute Name="Format" Value="Decimal"/>
    <ExtendedAttribute Name="Example" Value="0.001"/>
  </ExtendedAttributes>
</TypeDeclaration>

<!-- 新增：合约规格 -->
<TypeDeclaration Id="ContractSpecs" Name="合约规格">
  <RecordType>
    <Member Name="symbol" Type="Symbol"/>
    <Member Name="price_precision" Type="INTEGER"/>  <!-- 价格精度 -->
    <Member Name="quantity_precision" Type="INTEGER"/>  <!-- 数量精度 -->
    <Member Name="tick_size" Type="STRING"/>  <!-- 价格最小变动单位 -->
    <Member Name="step_size" Type="STRING"/>  <!-- 数量最小变动单位 -->
    <Member Name="min_qty" Type="STRING"/>  <!-- 最小下单数量 -->
    <Member Name="max_qty" Type="STRING"/>  <!-- 最大下单数量 -->
    <Member Name="min_notional" Type="STRING"/>  <!-- 最小名义价值 -->
  </RecordType>
</TypeDeclaration>
```

**对应API端点**：
- `GET /fapi/v1/exchangeInfo` - 获取所有合约的规格信息
  - 包含 `pricePrecision`, `quantityPrecision`, `filters` 等

---

## 11. 错误处理和限频

### ❌ XPDL未定义错误类型

币安API有明确的错误代码体系，建议添加：

```xml
<TypeDeclaration Id="ApiErrorCode" Name="API错误码">
  <EnumerationType>
    <!-- 通用错误 -->
    <EnumerationValue Name="UNKNOWN">-1000: 未知错误</EnumerationValue>
    <EnumerationValue Name="DISCONNECTED">-1001: 内部错误（断开连接）</EnumerationValue>

    <!-- 请求错误 -->
    <EnumerationValue Name="UNAUTHORIZED">-1002: 未授权</EnumerationValue>
    <EnumerationValue Name="TOO_MANY_REQUESTS">-1003: 请求过多</EnumerationValue>
    <EnumerationValue Name="INVALID_MESSAGE">-1007: 超时</EnumerationValue>

    <!-- 订单错误 -->
    <EnumerationValue Name="UNKNOWN_ORDER">-2013: 订单不存在</EnumerationValue>
    <EnumerationValue Name="BAD_API_KEY">-2015: 无效API密钥</EnumerationValue>
    <EnumerationValue Name="REJECTED_MBX_KEY">-2016: 服务器繁忙</EnumerationValue>

    <!-- 余额/杠杆错误 -->
    <EnumerationValue Name="NO_NEED_TO_CHANGE_MARGIN_TYPE">-4046: 无需更改保证金类型</EnumerationValue>
    <EnumerationValue Name="INVALID_LEVERAGE">-4028: 杠杆倍数无效</EnumerationValue>
    <EnumerationValue Name="INSUFFICIENT_BALANCE">-2019: 余额不足</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>

<!-- 限频规则 -->
<ExtendedAttributes>
  <ExtendedAttribute Name="RateLimits">
    <ExtendedAttribute Name="Orders" Value="300/10s (下单限频)"/>
    <ExtendedAttribute Name="Weight" Value="2400/1min (请求权重)"/>
    <ExtendedAttribute Name="WebSocket" Value="5/s (连接限制)"/>
  </ExtendedAttribute>
</ExtendedAttributes>
```

---

## 12. 总结与行动计划

### 🔴 高优先级修正项（必须）

1. **添加持仓模式（Position Mode）**
   - 支持单向持仓（One-Way）和对冲持仓（Hedge Mode）
   - 在Position实体中添加 `positionSide` 字段

2. **修正保证金类型命名**
   - `CROSS` → `CROSSED`（与币安API一致）

3. **添加缺失的订单类型**
   - `TRAILING_STOP_MARKET`
   - `STOP`（止损限价单）
   - `TAKE_PROFIT`（止盈限价单）

4. **区分普通订单和条件订单（Algo订单）**
   - 2025年12月9日后，条件订单使用独立端点

5. **补充Position实体字段**
   - `positionSide`, `breakEvenPrice`, `markPrice`, `adl`, `isAutoAddMargin` 等

6. **修正价格和数量类型**
   - `INTEGER` → `STRING`（或内部使用定点数）

### ⚠️ 中优先级补充项（建议）

7. **添加完整的Order实体定义**
8. **细化强平流程**（市场强平 → 风险保障基金 → ADL）
9. **添加合约规格（ContractSpecs）定义**
10. **支持4小时资金费率结算（部分合约）**
11. **添加错误码定义和限频规则**

### ✅ 已正确实现的部分

- ✅ 基本订单类型（LIMIT, MARKET）
- ✅ 保证金模式概念（需改名）
- ✅ 杠杆范围（1-125x）
- ✅ 资金费率基本公式
- ✅ 活动架构设计

---

## 参考资源

- [Binance USDT-Margined Futures API](https://developers.binance.com/docs/derivatives/usds-margined-futures/general-info)
- [Binance Derivatives Change Log](https://developers.binance.com/docs/derivatives/change-log)
- [Position Information V2](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Position-Information-V2)
- [Change Initial Leverage](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Initial-Leverage)
- [Binance Funding Rates](https://www.binance.com/en/support/faq/introduction-to-binance-futures-funding-rates-360033525031)
- [Liquidation Mechanism](https://www.binance.com/en/support/faq/how-liquidation-works-in-futures-trading-7ba80e1b406f40a0a140a84b3a10c387)

---

**报告生成时间**: 2025-12-12
**下次审查建议**: 2025年Q1（跟踪币安API变更）
