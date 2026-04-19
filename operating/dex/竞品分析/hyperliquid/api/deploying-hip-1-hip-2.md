# Deploying HIP-1 and HIP-2 assets

The API for deploying HIP-1 and HIP-2 assets involves a five-step process. You send the first five variants of the enum in the specified order, with the last two being optional.

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

UserGenesis can be called multiple times. It takes a token identifier, user addresses with genesis amounts in wei, existing token holders with genesis amounts, and optional blacklist status updates.

## Genesis

Genesis denotes the initial token creation with a maximum supply. It includes a checksum ensuring all UserGenesis calls succeeded and an optional flag to set hyperliquidity balance to zero.

## RegisterSpot

This action registers a trading pair using base and quote token indices. It also deploys pairs between existing assets, which follow an independent Dutch auction whose status is available via the `spotPairDeployAuctionStatus` info request.

## RegisterHyperliquidity

This configures hyperliquidity with a starting price, order size, number of orders, and optional seeded levels. If "noHyperliquidity" was set to true, the order count must be zero.

## SetDeployerTradingFeeShare

This optional action can be performed anytime after RegisterToken2. The fee share defaults to 100% but can be resent multiple times as long as it doesn't increase. The share range is "0%" to "100%".
