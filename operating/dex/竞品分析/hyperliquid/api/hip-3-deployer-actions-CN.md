# HIP-3 部署者操作

部署和操作构建者部署的永续 DEX 的 API 使用以下 L1 操作类型。

## 核心操作类型

```typescript
type PerpDeployAction =
  | { type: "perpDeploy", registerAsset2: RegisterAsset2 }
  | { type: "perpDeploy", registerAsset: RegisterAsset }
  | { type: "perpDeploy", setOracle: SetOracle }
  | { type: "perpDeploy", setFundingMultipliers: SetFundingMultipliers }
  | { type: "perpDeploy", setFundingInterestRates: SetFundingInterestRates }
  | { type: "perpDeploy", haltTrading: { coin: string, isHalted: boolean } }
  | { type: "perpDeploy", setMarginTableIds: SetMarginTableIds }
  | { type: "perpDeploy", setFeeRecipient: { dex: string, feeRecipient: address } }
  | { type: "perpDeploy", setOpenInterestCaps: SetOpenInterestCaps }
  | { type: "perpDeploy", setSubDeployers: { dex: string, subDeployers: Array<SubDeployerInput> } }
  | { type: "perpDeploy", setMarginModes: SetMarginModes }
  | { type: "perpDeploy", setFeeScale: SetFeeScale }
  | { type: "perpDeploy", setGrowthModes: SetGrowthModes }
  | { type: "perpDeploy", setPerpAnnotation: SetPerpAnnotation }
```

> 所有元组列表应在签名前按字典序排序。

## 资产注册

`RegisterAsset2` 同时初始化新的 DEX 并注册资产。`RegisterAsset` 可以重复调用以向现有 DEX 添加更多资产。

```typescript
type RegisterAsset2 = {
  maxGas?: number;
  assetRequest: RegisterAssetRequest2;
  dex: string;         // 2-4 个字符
  schema?: PerpDexSchemaInput;
}

type RegisterAssetRequest2 = {
  coin: string;
  szDecimals: number;
  oraclePx: string;
  marginTableId: number;
  marginMode: "strictIsolated" | "noCross" | "normal";
}
```

`maxGas` 控制部署成本：
- 省略 → 使用当前拍卖价格
- `0` → 预留部署（即使拍卖结束后也以当前拍卖价格部署；允许 7 个预留部署）

## 预言机更新

```typescript
type SetOracle = {
  dex: string;
  oraclePxs: Array<[string, string]>;        // 按键排序
  markPxs: Array<Array<[string, string]>>;   // 内部列表按键排序
  externalPerpPxs: Array<[string, string]>;  // 按键排序，必须包含所有资产
}
```

关键约束：
- `setOracle` 调用之间最少 2.5 秒
- 过期标记价格在 10 秒后回退到本地标记价格 — 但部署者应每 3 秒调用 `setOracle` 无论如何
- 所有价格限制在当天开始值的 10 倍
- 标记价格变动限制在之前值的 1%

## DEX 模式

```typescript
type PerpDexSchemaInput = {
  fullName: string;
  collateralToken: int;
  oracleUpdater?: string;  // 如果省略，默认为部署者
}
```

## 资金费率

```typescript
// 乘数：0 到 10 之间，缩放资金费率
type SetFundingMultipliers = Array<[string, string]>;

// 8 小时利率：-0.01 到 0.01 之间
type SetFundingInterestRates = Array<[string, string]>;
```

## 保证金表

```typescript
type RawMarginTable = {
  description: string;
  marginTiers: Array<RawMarginTier>;  // 最大长度 3，按 increasing lowerBound / decreasing maxLeverage 排序
}

type RawMarginTier = {
  lowerBound: int;      // 应用 maxLeverage 的名义阈值
  maxLeverage: number;  // 范围 [1, 50]
}
```

## 持仓上限

```typescript
// 名义上限 — 必须 >= max(1_000_000, 当前 OI 的一半)
type SetOpenInterestCaps = Array<[string, number]>;
```

两种上限类型适用于构建者部署的市场：
- **名义** — 每个资产和整个 DEX 强制执行
- **数量** — 仅每个资产，目前每个资产固定为 10 亿

合理的默认值是设置 `szDecimals`，使最小数量增量在初始标记价格时为 $1–10。

## 费用规模

```typescript
type SetFeeScale = {
  dex: string;
  scale: string;  // 十进制字符串 "0.0" 到 "3.0"
}
```

费用分配逻辑（其中 `x` = 正常用户费率，`y` = 对齐报价抵押品调整后的费率）：

| 条件 | 协议获得 | 部署者获得 |
|---|---|---|
| `x > 0`, `scale < 1` | `y` | `scale * x` |
| `x > 0`, `scale > 1` | `y * scale` | `x * scale` |
| `x < 0`, `scale < 1` | `y / (1 + scale)` | `y * scale / (1 + scale)` |
| `x < 0`, `scale > 1` | `y / 2` | `y / 2` |

主网上每 30 天限制一次更改。

## 其他操作

```typescript
// 子部署者权限
type SubDeployerInput = {
  variant: string;   // 例如 "haltTrading" 或 "setOracle"
  user: string;
  allowed: boolean;
}

// 保证金模式 — 每个代币每 30 天只能更改一次
type SetMarginModes = Array<[string, "strictIsolated" | "noCross"]>;

// 增长模式 — 每个代币每 30 天只能更改一次
type SetGrowthModes = Array<[string, bool]>;

// 注释
type SetPerpAnnotation = {
  coin: string;
  category: string;      // <= 15 个字符
  description: string;   // <= 400 个字符
  displayName: string | null;
  keywords: Array<string>;
}
```

## 参考

- [永续部署拍卖状态端点](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint/perpetuals#retrieve-information-about-the-perp-deploy-auction)
- [HIP-3 部署者操作文档](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/hip-3-deployer-actions)
