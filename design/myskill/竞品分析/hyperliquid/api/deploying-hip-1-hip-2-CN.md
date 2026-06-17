# 部署 HIP-1 和 HIP-2 资产

部署 HIP-1 和 HIP-2 资产的 API 涉及五个步骤的过程。您按指定顺序发送枚举的前五个变体，最后两个是可选的。

```typescript
type SpotDeployAction =
  | {
      type: "spotDeploy";
      registerToken2: RegisterToken2;
    }
  | {
      type: "spotDeploy";
      userGenesis: UserGenesis;
    }
  | {
      type: "spotDeploy";
      genesis: Genesis;
    }
  | {
      type: "spotDeploy";
      registerSpot: RegisterSpot;
    }
  | {
      type: "spotDeploy";
      registerHyperliquidity: RegisterHyperliquidity;
    }
  | {
      type: "spotDeploy";
      setDeployerTradingFeeShare: SetDeployerTradingFeeShare;
    }
  | {
      type: "spotDeploy";
      enableQuoteToken: { token: number };
    }
  | {
      type: "spotDeploy";
      enableAlignedQuoteToken: { token: number };
    };

type RegisterToken2 = {
  spec: TokenSpec;
  maxGas: number;
  fullName?: string;
}

type TokenSpec = {
  name: string,
  szDecimals: number,
  weiDecimals: number,
}

type UserGenesis = {
  token: number;
  userAndWei: Array<[string, string]>;
  existingTokenAndWei: Array<[number, string]>;
  blacklistUsers?: Array<[string, boolean]>;
}

type Genesis = {
  token: number;
  maxSupply: string;
  noHyperliquidity?: boolean;
}

type RegisterSpot = {
  tokens: [number, number];
}

type RegisterHyperliquidity = {
  spot: number;
  startPx: string;
  orderSz: string;
  nOrders: number;
  nSeededLevels?: number;
}

type SetDeployerTradingFeeShare {
  token: number;
  share: string;
}
```

## UserGenesis

UserGenesis 可以被调用多次。它接受代币标识符、带有 wei 单位创世金额的用户地址、带有创世金额的现有代币持有者，以及可选的黑名单状态更新。

## Genesis

Genesis 表示具有最大供应量的初始代币创建。它包括一个校验和，确保所有 UserGenesis 调用成功，以及一个可选标志将 hyperliquidity 余额设置为零。

## RegisterSpot

此操作使用基础代币和报价代币索引注册交易对。它还部署现有资产之间的交易对，这些遵循独立的荷兰拍卖，其状态可通过 `spotPairDeployAuctionStatus` info 请求获取。

## RegisterHyperliquidity

这将配置 hyperliquidity，包括起始价格、订单大小、订单数量和可选的种子档位。如果 "noHyperliquidity" 设置为 true，订单数量必须为零。

## SetDeployerTradingFeeShare

此可选操作可以在 RegisterToken2 之后的任何时间执行。费用份额默认为 100%，但只要不增加就可以多次重新发送。份额范围是 "0%" 到 "100%".
