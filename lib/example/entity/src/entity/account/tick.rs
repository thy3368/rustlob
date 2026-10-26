use rust_decimal::Decimal;

struct AssetConfig {
    coin: u64,
    price_tick_size: Decimal, // 价格最小步长
    size_tick_size: Decimal,  // 数量最小步长
                              // ...
}
