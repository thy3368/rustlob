# Asset IDs

Asset IDs are the integer representation for sending orders and cancels via actions. See the [exchange endpoint documentation](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint) for more details.

## Perps

Perpetual endpoints expect an integer for `asset`, which is the index of the coin found in the `meta` info response. For example, `BTC = 0` on mainnet.

Builder-deployed perps use the formula `100000 + perp_dex_index * 10000 + index_in_meta`. For instance, `test:ABC` on testnet has `perp_dex_index = 1`, `index_in_meta = 0`, resulting in `asset = 110000`. Builder-deployed perps always follow the naming format `{dex}:{coin}`.

## Spot

Spot endpoints expect `10000 + spotInfo["index"]` where `spotInfo` is the corresponding object in `spotMeta` containing the desired quote and base tokens. For example, when submitting an order for `PURR/USDC`, use asset `10000` since its spot info index is `0`.

Spot ID differs from token ID, and mainnet and testnet have different asset IDs. For HYPE as an example:

- Mainnet token ID: 150
- Mainnet spot ID: 107
- Testnet token ID: 1105
- Testnet spot ID: 1035

## Outcomes

Outcomes share most implementation details with spot trading, with each outcome side represented by a different token. However, the API representation differs from both spot and perps.

Outcome assets derive from an `outcome` id plus a binary `side`, found in the `outcomeMeta` info response.

For an outcome with id `outcome` and side `side`:

```
encoding = 10 * outcome + side
```

Only sides `0` and `1` are valid.

Example:

- outcome `1`, side `0` → encoding `10`

The same `encoding` appears in three representations:

- Outcome spot coin: `#<encoding>`
- Outcome token name: `+<encoding>`
- Outcome asset ID: `100_000_000 + encoding`

Example:

- `#10` = outcome `1`, side `0`
- `+10` = the corresponding token name
- `100000010` = asset ID
