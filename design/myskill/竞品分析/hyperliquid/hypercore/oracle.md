# Oracle

Validators publish spot oracle prices for each perp asset every 3 seconds. These prices determine funding rates and contribute to the mark price, which influences margining, liquidations, and TP/SL order triggers.

Spot oracle prices are calculated as "the weighted median of Binance, OKX, Bybit, Kraken, Kucoin, Gate IO, MEXC, and Hyperliquid spot mid prices" with weights of 3, 2, 2, 1, 1, 1, 1, 1 respectively. Assets with primary spot liquidity on Hyperliquid (like HYPE) exclude external sources until sufficient liquidity develops. Assets with primary liquidity elsewhere (like BTC) exclude Hyperliquid spot prices from the oracle calculation.

The clearinghouse applies a final weighted median across all validator submissions, with validators weighted according to their stake.
