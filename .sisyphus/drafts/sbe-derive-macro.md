# Draft: SBE 宏需求

## 用户需求
- 实现类似 serde 的 derive 宏用于 SBE (Simple Binary Encoding)
- 完全替换现有手写实现

## 需要支持的 SBE 特性
- [x] 基本类型字段 (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64)
- [x] 定长数组 (fixed string)
- [x] 变长数据 (var-data)
- [x] SBE 消息头 (block_length, template_id, schema_id, version)
- [x] 版本字段 (version >= X 时存在)
- [x] 嵌套消息
- [x] 常量字段

## 期望的使用方式 (类似 serde)
```rust
#[sbe(message)]
pub struct Trade {
    #[sbe(field = "u64")]
    pub trade_id: u64,
    
    #[sbe(field = "u8", primitive_type = "char")]
    pub symbol: [u8; 10],
    
    #[sbe(field = "f64")]
    pub price: f64,
    
    #[sbe(field = "i32")]
    pub quantity: i32,
}

#[sbe(message)]
pub struct Order {
    #[sbe(field = "u64")]
    pub order_id: u64,
    
    #[sbe(field = "message")]
    pub trade: Trade,
}
```

## 待确认问题
1. 
2. 
3. 
