# HIP-3 Deployer Actions

The API for deploying and operating builder-deployed perpetual DEXs uses the following L1 action types.

## Core Action Type

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

> All lists of tuples should be lexicographically sorted before signing.

## Asset Registration

`RegisterAsset2` initializes a new DEX and registers an asset simultaneously. `RegisterAsset` can be called repeatedly to add more assets to an existing DEX.

```typescript
type RegisterAsset2 = {
  maxGas?: number;
  assetRequest: RegisterAssetRequest2;
  dex: string;         // 2-4 characters
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

`maxGas` controls deployment cost:
- Omitted → uses current auction price
- `0` → reserve deployment (deploys at current auction price even after auction ends; 7 reserve deployments allowed)

## Oracle Updates

```typescript
type SetOracle = {
  dex: string;
  oraclePxs: Array<[string, string]>;        // sorted by key
  markPxs: Array<Array<[string, string]>>;   // inner lists sorted by key
  externalPerpPxs: Array<[string, string]>;  // sorted by key, must include all assets
}
```

Key constraints:
- Minimum 2.5 seconds between `setOracle` calls
- Stale mark prices fall back to local mark price after 10 seconds — but deployers should call `setOracle` every 3 seconds regardless
- All prices clamped to 10x the start-of-day value
- Mark price moves clamped to 1% from previous value

## DEX Schema

```typescript
type PerpDexSchemaInput = {
  fullName: string;
  collateralToken: int;
  oracleUpdater?: string;  // defaults to deployer if omitted
}
```

## Funding

```typescript
// Multipliers: between 0 and 10, scale the funding rate
type SetFundingMultipliers = Array<[string, string]>;

// 8-hour interest rates: between -0.01 and 0.01
type SetFundingInterestRates = Array<[string, string]>;
```

## Margin Tables

```typescript
type RawMarginTable = {
  description: string;
  marginTiers: Array<RawMarginTier>;  // max length 3, sorted by increasing lowerBound / decreasing maxLeverage
}

type RawMarginTier = {
  lowerBound: int;      // notional threshold above which maxLeverage applies
  maxLeverage: number;  // range [1, 50]
}
```

## Open Interest Caps

```typescript
// Notional caps — must be >= max(1_000_000, half of current OI)
type SetOpenInterestCaps = Array<[string, number]>;
```

Two cap types apply to builder-deployed markets:
- **Notional** — enforced per-asset and across the entire DEX
- **Size** — per-asset only, currently fixed at 1B per asset

A reasonable default is to set `szDecimals` so the minimum size increment is $1–10 at the initial mark price.

## Fee Scale

```typescript
type SetFeeScale = {
  dex: string;
  scale: string;  // decimal string "0.0" to "3.0"
}
```

Fee distribution logic (where `x` = normal user rate, `y` = rate after aligned quote collateral adjustment):

| Condition | Protocol gets | Deployer gets |
|---|---|---|
| `x > 0`, `scale < 1` | `y` | `scale * x` |
| `x > 0`, `scale > 1` | `y * scale` | `x * scale` |
| `x < 0`, `scale < 1` | `y / (1 + scale)` | `y * scale / (1 + scale)` |
| `x < 0`, `scale > 1` | `y / 2` | `y / 2` |

Rate-limited to one change per 30 days on mainnet.

## Other Actions

```typescript
// Sub-deployer permissions
type SubDeployerInput = {
  variant: string;   // e.g. "haltTrading" or "setOracle"
  user: string;
  allowed: boolean;
}

// Margin modes — can only change once per 30 days per coin
type SetMarginModes = Array<[string, "strictIsolated" | "noCross"]>;

// Growth modes — can only change once per 30 days per coin
type SetGrowthModes = Array<[string, bool]>;

// Annotations
type SetPerpAnnotation = {
  coin: string;
  category: string;      // <= 15 characters
  description: string;   // <= 400 characters
  displayName: string | null;
  keywords: Array<string>;
}
```

## References

- [Perp deploy auction status endpoint](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint/perpetuals#retrieve-information-about-the-perp-deploy-auction)
- [HIP-3 deployer actions docs](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/hip-3-deployer-actions)
