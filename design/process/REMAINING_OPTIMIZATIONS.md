# 剩余优化建议清单

**日期**: 2025-12-12
**状态**: 高优先级项目已完成 ✅

---

## ✅ 已完成的高优先级项目

1. ✅ 添加持仓模式（Position Mode: ONE_WAY | HEDGE）
2. ✅ 修正保证金类型命名（CROSS → CROSSED）
3. ✅ 添加缺失的订单类型（TRAILING_STOP_MARKET等）
4. ✅ 添加订单分类（普通订单 vs Algo订单）
5. ✅ 补充Position实体的所有币安API字段
6. ✅ 修正价格和数量类型（INTEGER → STRING）
7. ✅ 添加Order实体完整定义
8. ✅ 添加合约规格（ContractSpecs）定义

---

## 🔶 中优先级优化建议

### 1. 细化强平流程（三级机制）⭐

**优先级**: 中-高
**影响范围**: `LiquidationProcess` 流程

**当前问题**:
强平流程过于简化，未体现币安的三级强平机制。

**币安实际流程**:
```
标记价格达到强平价
  → 1️⃣ 市场强平（提交市价单）
      ✅ 成功 → 结束
      ❌ 流动性不足
        → 2️⃣ 风险保障基金接管
            ✅ 基金充足 → 订单状态=NEW_INSURANCE
            ❌ 基金不足
              → 3️⃣ 自动减仓（ADL）
                  → 对手方盈利仓位强制平仓
                  → 订单状态=NEW_ADL
```

**建议修改**:
在 `LiquidationProcess` 的 `ExecuteLiquidation` 活动中添加三级处理逻辑：

```xml
<Activity Id="ExecuteLiquidation" Name="执行三级强平">
  <Implementation>
    <Task>
      <TaskScript>
        <Script Type="rust/composite">
          <![CDATA[
          async fn execute_three_tier_liquidation(position_id: PositionId) -> Result<LiquidationResult, Error> {
              // 1️⃣ 第一级：市场强平
              let market_result = try_market_liquidation(position_id, timeout: 5s).await;

              if market_result.is_fully_filled() {
                  return Ok(LiquidationResult::Market(market_result));
              }

              // 2️⃣ 第二级：风险保障基金接管
              if insurance_fund_has_capacity().await? {
                  let insurance_result = insurance_fund_takeover(position_id).await?;
                  mark_order_status(order_id, OrderStatus::NEW_INSURANCE).await?;
                  return Ok(LiquidationResult::InsuranceFund(insurance_result));
              }

              // 3️⃣ 第三级：自动减仓（ADL）
              let adl_result = trigger_auto_deleveraging(position_id).await?;
              mark_order_status(order_id, OrderStatus::NEW_ADL).await?;
              Ok(LiquidationResult::ADL(adl_result))
          }
          ]]>
        </Script>
      </TaskScript>
    </Task>
  </Implementation>
</Activity>
```

**对应API**:
- `GET /fapi/v1/allForceOrders` - 查询强平订单（通过status字段区分类型）

---

### 2. 支持动态资金费率结算间隔 ⭐

**优先级**: 中
**影响范围**: `FundingRateSettlementProcess` 流程

**当前问题**:
硬编码为8小时结算，但部分合约已改为4小时。

**币安实际情况**:
- 默认：8小时（00:00, 08:00, 16:00 UTC）
- 部分合约：4小时（00:00, 04:00, 08:00, 12:00, 16:00, 20:00 UTC）
- 可通过 `GET /fapi/v1/exchangeInfo` 查询每个合约的结算间隔

**建议修改**:

1. 在 `ContractSpecs` 中添加字段：
```xml
<Member Name="funding_interval_hours" Type="INTEGER"/>
<Description>资金费率结算间隔（小时）：4 | 8</Description>
```

2. 在流程中使用动态Cron表达式：
```xml
<Activity Id="Start" Name="触发结算">
  <Event>
    <StartEvent Trigger="Timer">
      <TriggerTimer>
        <TimeCycle>DYNAMIC</TimeCycle>
        <Description>
          根据合约配置动态调整：
          - 8小时: 0 */8 * * *
          - 4小时: 0 */4 * * *
        </Description>
      </TriggerTimer>
    </StartEvent>
  </Event>
</Activity>
```

3. 在计算逻辑中考虑间隔：
```rust
// Interest Rate根据间隔调整
let interest_rate = match funding_interval_hours {
    4 => 0.01 / 2.0,  // 0.005% per 4h
    8 => 0.01,        // 0.01% per 8h
    _ => 0.01,
};
```

---

### 3. 添加API错误码定义 🔧

**优先级**: 中
**影响范围**: TypeDeclarations

**建议添加**:
```xml
<!-- API错误码 -->
<TypeDeclaration Id="ApiErrorCode" Name="API错误码">
  <EnumerationType>
    <!-- 通用错误 1xxx -->
    <EnumerationValue Name="UNKNOWN">-1000: 未知错误</EnumerationValue>
    <EnumerationValue Name="DISCONNECTED">-1001: 内部错误</EnumerationValue>
    <EnumerationValue Name="UNAUTHORIZED">-1002: 未授权</EnumerationValue>
    <EnumerationValue Name="TOO_MANY_REQUESTS">-1003: 请求过多（触发限频）</EnumerationValue>
    <EnumerationValue Name="UNEXPECTED_RESP">-1006: 服务器响应异常</EnumerationValue>
    <EnumerationValue Name="TIMEOUT">-1007: 超时</EnumerationValue>

    <!-- 请求格式错误 -10xx -->
    <EnumerationValue Name="INVALID_MESSAGE">-1013: 非法参数</EnumerationValue>
    <EnumerationValue Name="UNKNOWN_ORDER_COMPOSITION">-1014: 不支持的订单组合</EnumerationValue>
    <EnumerationValue Name="TOO_MANY_ORDERS">-1015: 订单过多</EnumerationValue>

    <!-- 订单相关错误 -20xx -->
    <EnumerationValue Name="UNKNOWN_ORDER">-2013: 订单不存在</EnumerationValue>
    <EnumerationValue Name="BAD_API_KEY">-2015: 无效的API密钥</EnumerationValue>
    <EnumerationValue Name="NO_TRADING_WINDOW">-2016: 当前时间不在交易窗口</EnumerationValue>
    <EnumerationValue Name="BALANCE_NOT_SUFFICIENT">-2019: 余额不足</EnumerationValue>

    <!-- 合约特定错误 -40xx -->
    <EnumerationValue Name="INVALID_LEVERAGE">-4028: 杠杆倍数无效</EnumerationValue>
    <EnumerationValue Name="INVALID_ORDER_STATUS">-4044: 订单状态不允许此操作</EnumerationValue>
    <EnumerationValue Name="NO_NEED_TO_CHANGE_MARGIN_TYPE">-4046: 无需更改保证金类型</EnumerationValue>
    <EnumerationValue Name="POSITION_SIDE_NOT_MATCH">-4059: 持仓方向不匹配</EnumerationValue>
    <EnumerationValue Name="REDUCE_ONLY_REJECT">-4061: 只减仓订单被拒绝</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>

<!-- API错误响应实体 -->
<TypeDeclaration Id="ApiError" Name="API错误响应">
  <RecordType>
    <Member Name="code" Type="INTEGER"/>
    <Description>错误代码</Description>
    <Member Name="msg" Type="STRING"/>
    <Description>错误消息</Description>
  </RecordType>
  <Description>
    币安API统一错误响应格式：
    {"code": -1003, "msg": "Too many requests."}
  </Description>
</TypeDeclaration>
```

---

### 4. 添加限频规则文档 📊

**优先级**: 中-低
**影响范围**: ExtendedAttributes

**建议在ExtendedAttributes中添加**:
```xml
<ExtendedAttribute Name="BinanceAPILimits">
  <ExtendedAttribute Name="OrderRateLimit" Value="300 orders/10s per account"/>
  <ExtendedAttribute Name="RequestWeight" Value="2400 weight/1min per IP"/>
  <ExtendedAttribute Name="RawRequests" Value="6000 requests/5min per IP"/>

  <!-- 具体端点权重 -->
  <ExtendedAttribute Name="POST_order_weight" Value="0 or 1 (depending on conditions)"/>
  <ExtendedAttribute Name="GET_positionRisk_weight" Value="5"/>
  <ExtendedAttribute Name="POST_leverage_weight" Value="1"/>
  <ExtendedAttribute Name="POST_marginType_weight" Value="1"/>
  <ExtendedAttribute Name="POST_positionMargin_weight" Value="1"/>

  <!-- WebSocket限制 -->
  <ExtendedAttribute Name="WebSocketConnections" Value="5 connections/s per IP"/>
  <ExtendedAttribute Name="WebSocketSubscriptions" Value="200 streams per connection"/>
</ExtendedAttribute>

<ExtendedAttribute Name="RateLimitHeaders">
  <ExtendedAttribute Name="X-MBX-USED-WEIGHT-1M" Value="当前1分钟内已使用的权重"/>
  <ExtendedAttribute Name="X-MBX-ORDER-COUNT-10S" Value="当前10秒内下单次数"/>
  <ExtendedAttribute Name="Retry-After" Value="触发限频后的恢复时间（秒）"/>
</ExtendedAttribute>
```

---

### 5. 更新流程活动中的数据类型引用 🔄

**优先级**: 中
**影响范围**: 所有Activities

**需要更新的地方**:

#### a) 开仓活动 - 更新position_side引用
```xml
<!-- 当前 -->
<DataField Id="position_side" Name="持仓方向">
  <DataType><BasicType Type="STRING"/></DataType>
  <Description>LONG | SHORT</Description>
</DataField>

<!-- 应改为 -->
<DataField Id="position_side" Name="仓位方向">
  <DataType><BasicType Type="STRING"/></DataType>
  <Description>BOTH | LONG | SHORT（取决于持仓模式）</Description>
</DataField>
```

#### b) 添加持仓模式字段
```xml
<DataField Id="position_mode" Name="持仓模式">
  <DataType><BasicType Type="STRING"/></DataType>
  <InitialValue>ONE_WAY</InitialValue>
  <Description>ONE_WAY | HEDGE</Description>
</DataField>
```

#### c) 更新订单side字段
```xml
<DataField Id="order_side" Name="订单方向">
  <DataType><BasicType Type="STRING"/></DataType>
  <Description>BUY | SELL</Description>
</DataField>
```

---

### 6. 添加WebSocket数据流定义 📡

**优先级**: 低
**影响范围**: 新增部分

**建议添加**（如果需要实时数据）:
```xml
<!-- WebSocket订阅类型 -->
<TypeDeclaration Id="WebSocketStreamType" Name="WebSocket数据流类型">
  <EnumerationType>
    <EnumerationValue Name="TRADE">实时成交</EnumerationValue>
    <EnumerationValue Name="KLINE">K线数据</EnumerationValue>
    <EnumerationValue Name="MARK_PRICE">标记价格</EnumerationValue>
    <EnumerationValue Name="LIQUIDATION">强平订单</EnumerationValue>
    <EnumerationValue Name="BOOK_TICKER">最优挂单</EnumerationValue>
    <EnumerationValue Name="USER_DATA">用户数据流（订单更新、账户更新）</EnumerationValue>
  </EnumerationType>
</TypeDeclaration>
```

---

### 7. 添加认证相关定义 🔐

**优先级**: 低
**影响范围**: 新增部分

**建议添加**:
```xml
<!-- API认证配置 -->
<TypeDeclaration Id="ApiCredentials" Name="API认证凭据">
  <RecordType>
    <Member Name="api_key" Type="STRING"/>
    <Description>API Key（用于HTTP Header: X-MBX-APIKEY）</Description>
    <Member Name="secret_key" Type="STRING"/>
    <Description>Secret Key（用于HMAC SHA256签名）</Description>
    <Member Name="timestamp" Type="INTEGER"/>
    <Description>请求时间戳（毫秒）</Description>
    <Member Name="recv_window" Type="INTEGER"/>
    <Description>接收窗口（默认5000ms，最大60000ms）</Description>
  </RecordType>
  <Description>
    签名算法：
    signature = HMAC_SHA256(secret_key, query_string)
    例如：symbol=BTCUSDT&amp;side=BUY&amp;type=LIMIT&amp;quantity=1&amp;price=50000&amp;timestamp=1609459200000
  </Description>
</TypeDeclaration>
```

---

## 📋 优化优先级建议

### 高优先级（建议立即完成）
- 无（已全部完成 ✅）

### 中优先级（根据需求选择）
1. ⭐ **细化强平流程** - 如果需要完整模拟币安强平机制
2. ⭐ **动态资金费率间隔** - 如果支持多种合约
3. 🔧 **API错误码** - 增强错误处理能力
4. 📊 **限频规则** - 防止触发API限制

### 低优先级（可选）
5. 🔄 **更新流程数据类型** - 代码清理工作
6. 📡 **WebSocket定义** - 如果需要实时数据流
7. 🔐 **认证定义** - 如果需要完整的API集成文档

---

## 🎯 建议下一步

**如果你的目标是**：

### 1️⃣ 完整模拟币安交易所
→ 建议完成**中优先级**的所有项目（1-4）

### 2️⃣ 作为业务流程参考文档
→ 当前已足够，可选择性添加错误码和限频规则

### 3️⃣ 用于实际开发指导
→ 建议添加：
- 强平流程细化
- API错误码定义
- 限频规则文档
- 认证相关定义

---

## 📚 参考资料

- [Binance API Documentation](https://developers.binance.com/docs/derivatives/usds-margined-futures/general-info)
- [Error Codes](https://binance-docs.github.io/apidocs/futures/en/#error-codes)
- [Rate Limits](https://binance-docs.github.io/apidocs/futures/en/#limits)
- [Liquidation Mechanism](https://www.binance.com/en/support/faq/how-liquidation-works-in-futures-trading-7ba80e1b406f40a0a140a84b3a10c387)
- [Funding Rates](https://www.binance.com/en/support/faq/introduction-to-binance-futures-funding-rates-360033525031)

---

**文档生成时间**: 2025-12-12
**状态**: 高优先级项目✅ | 中优先级待定⏳
