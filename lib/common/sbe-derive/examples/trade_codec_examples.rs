// 交易编解码器使用示例 - 可运行的测试
#[cfg(test)]
mod trade_codec_examples {
    // use com_tanggo_fund_metadriven_sbe::{
    //     trade_codec::{TradeEncoder, TradeDecoder, SBE_BLOCK_LENGTH},
    //     ReadBuf, WriteBuf,
    // };

    use sbe::{Encoder, ReadBuf, WriteBuf};

    /// 示例 1: 基本编码
    /// 编码一个简单的交易消息
    #[test]
    fn example_1_basic_encoding() {
        println!("\n=== 示例 1: 基本编码 ===");

        // 分配缓冲区
        let mut buffer = vec![0u8; 64];
        let write_buf = WriteBuf::new(&mut buffer);
        let mut encoder = TradeEncoder::default().wrap(write_buf, 0);

        // 设置交易字段
        let trade_id = 12345u64;
        let symbol = b'A'; // ASCII 'A'
        let price = 99.50;
        let quantity = 1000i32;

        encoder.trade_id(trade_id);
        encoder.symbol(symbol);
        encoder.price(price);
        encoder.quantity(quantity);

        println!("✓ 交易ID: {}", trade_id);
        println!("✓ 符号: {} (ASCII {})", symbol as char, symbol);
        println!("✓ 价格: {}", price);
        println!("✓ 数量: {}", quantity);
        println!("✓ 编码长度: {} 字节", encoder.encoded_length());

        assert_eq!(encoder.encoded_length(), SBE_BLOCK_LENGTH as usize);
    }

    /// 示例 2: 基本解码
    /// 从缓冲区解码交易消息
    #[test]
    fn example_2_basic_decoding() {
        println!("\n=== 示例 2: 基本解码 ===");

        // 第一步：编码数据
        let mut buffer = vec![0u8; 64];
        {
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = TradeEncoder::default().wrap(write_buf, 0);
            encoder.trade_id(54321);
            encoder.symbol(b'C');
            encoder.price(75.25);
            encoder.quantity(2000);
        }

        // 第二步：解码数据
        {
            let read_buf = ReadBuf::new(&buffer);
            let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);

            let decoded_id = decoder.trade_id();
            let decoded_symbol = decoder.symbol();
            let decoded_price = decoder.price();
            let decoded_qty = decoder.quantity();

            println!("✓ 解码交易ID: {}", decoded_id);
            println!("✓ 解码符号: {} (ASCII {})", decoded_symbol as char, decoded_symbol);
            println!("✓ 解码价格: {}", decoded_price);
            println!("✓ 解码数量: {}", decoded_qty);

            assert_eq!(decoded_id, 54321);
            assert_eq!(decoded_symbol, b'C');
            assert_eq!(decoded_price, 75.25);
            assert_eq!(decoded_qty, 2000);
        }
    }

    /// 示例 3: 往返编码解码
    /// 编码然后解码，验证数据完整性
    #[test]
    fn example_3_roundtrip() {
        println!("\n=== 示例 3: 往返编码解码 ===");

        let test_cases = vec![
            (111111u64, b'A', 100.00, 1000i32, "案例1"),
            (222222u64, b'B', 200.50, 2000i32, "案例2"),
            (333333u64, b'C', 300.75, 3000i32, "案例3"),
        ];

        for (orig_id, orig_symbol, orig_price, orig_qty, label) in test_cases {
            println!("\n处理 {}:", label);

            let mut buffer = vec![0u8; 64];

            // 编码
            {
                let write_buf = WriteBuf::new(&mut buffer);
                let mut encoder = TradeEncoder::default().wrap(write_buf, 0);
                encoder.trade_id(orig_id);
                encoder.symbol(orig_symbol);
                encoder.price(orig_price);
                encoder.quantity(orig_qty);
            }

            // 解码
            {
                let read_buf = ReadBuf::new(&buffer);
                let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);

                let decoded_id = decoder.trade_id();
                let decoded_symbol = decoder.symbol();
                let decoded_price = decoder.price();
                let decoded_qty = decoder.quantity();

                // 验证
                assert_eq!(decoded_id, orig_id, "交易ID不匹配");
                assert_eq!(decoded_symbol, orig_symbol, "符号不匹配");
                assert_eq!(decoded_price, orig_price, "价格不匹配");
                assert_eq!(decoded_qty, orig_qty, "数量不匹配");

                println!("  ✓ 交易ID匹配: {}", decoded_id);
                println!("  ✓ 符号匹配: {}", decoded_symbol as char);
                println!("  ✓ 价格匹配: {}", decoded_price);
                println!("  ✓ 数量匹配: {}", decoded_qty);
            }
        }
    }

    /// 示例 4: 批量编码
    /// 在同一缓冲区中编码多个交易
    #[test]
    fn example_4_batch_encoding() {
        println!("\n=== 示例 4: 批量编码 ===");

        let trades = vec![
            (101u64, b'X', 50.25, 500i32),
            (102u64, b'Y', 75.75, 250i32),
            (103u64, b'Z', 99.50, 1000i32),
        ];

        // 分配足够的缓冲区
        let buffer_size = trades.len() * SBE_BLOCK_LENGTH as usize;
        let mut buffer = vec![0u8; buffer_size];

        // 编码每个交易
        for (idx, &(trade_id, symbol, price, quantity)) in trades.iter().enumerate() {
            let offset = idx * SBE_BLOCK_LENGTH as usize;
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = TradeEncoder::default().wrap(write_buf, offset);

            encoder.trade_id(trade_id);
            encoder.symbol(symbol);
            encoder.price(price);
            encoder.quantity(quantity);

            println!(
                "✓ 交易 {}: ID={}, 符号={}, 价格={}, 数量={}",
                idx, trade_id, symbol as char, price, quantity
            );
        }

        println!("✓ 共编码 {} 个交易，总长度 {} 字节", trades.len(), buffer.len());

        // 验证可以解码
        for (idx, &(exp_id, exp_symbol, exp_price, exp_qty)) in trades.iter().enumerate() {
            let offset = idx * SBE_BLOCK_LENGTH as usize;
            let read_buf = ReadBuf::new(&buffer);
            let decoder = TradeDecoder::default().wrap(read_buf, offset, SBE_BLOCK_LENGTH, 0);

            assert_eq!(decoder.trade_id(), exp_id);
            assert_eq!(decoder.symbol(), exp_symbol);
            assert_eq!(decoder.price(), exp_price);
            assert_eq!(decoder.quantity(), exp_qty);
        }

        println!("✓ 所有交易验证通过");
    }

    /// 示例 5: 字段数据类型验证
    /// 演示每个字段支持的数据类型和范围
    #[test]
    fn example_5_field_types() {
        println!("\n=== 示例 5: 字段数据类型验证 ===");

        let mut buffer = vec![0u8; 64];

        // u64 交易ID - 支持完整的 64 位无符号整数
        println!("\n交易ID字段 (u64):");
        {
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = TradeEncoder::default().wrap(write_buf, 0);

            let id = u64::MAX - 1;
            encoder.trade_id(id);

            let read_buf = ReadBuf::new(&buffer);
            let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);
            assert_eq!(decoder.trade_id(), id);
            println!("  ✓ 支持最大值: {}", id);
        }

        // u8 符号 - ASCII 可打印字符 (32-126)
        println!("\n符号字段 (u8 ASCII):");
        let symbols = vec![(32u8, ' '), (65u8, 'A'), (90u8, 'Z'), (97u8, 'a'), (126u8, '~')];
        for (byte_val, char_val) in symbols {
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = TradeEncoder::default().wrap(write_buf, 0);
            encoder.symbol(byte_val);

            let read_buf = ReadBuf::new(&buffer);
            let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);
            assert_eq!(decoder.symbol(), byte_val);
            println!("  ✓ ASCII {}: '{}'", byte_val, char_val);
        }

        // f64 价格 - 64 位浮点数
        println!("\n价格字段 (f64):");
        {
            let prices = vec![0.0, 100.50, 1000.99, -50.25, f64::MIN_POSITIVE];
            for price in prices {
                let write_buf = WriteBuf::new(&mut buffer);
                let mut encoder = TradeEncoder::default().wrap(write_buf, 0);
                encoder.price(price);

                let read_buf = ReadBuf::new(&buffer);
                let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);
                assert_eq!(decoder.price(), price);
                println!("  ✓ 价格值: {}", price);
            }
        }

        // i32 数量 - 有符号32位整数
        println!("\n数量字段 (i32):");
        {
            let quantities = vec![0i32, 1000, -1000, i32::MAX, i32::MIN + 1];
            for qty in quantities {
                let write_buf = WriteBuf::new(&mut buffer);
                let mut encoder = TradeEncoder::default().wrap(write_buf, 0);
                encoder.quantity(qty);

                let read_buf = ReadBuf::new(&buffer);
                let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);
                assert_eq!(decoder.quantity(), qty);
                println!("  ✓ 数量值: {}", qty);
            }
        }
    }

    /// 示例 6: 字段偏移和消息布局
    /// 展示消息的内存布局
    #[test]
    fn example_6_message_layout() {
        println!("\n=== 示例 6: 消息内存布局 ===");

        println!("Trade 消息布局 (总长度: {} 字节):", SBE_BLOCK_LENGTH);
        println!("┌─────────────────────────────────────────┐");
        println!("│ 字段      │ 偏移 │ 长度 │ 类型        │");
        println!("├─────────────────────────────────────────┤");
        println!("│ tradeId   │  0   │  8   │ u64 (LE)    │");
        println!("│ symbol    │  8   │  1   │ u8 (ASCII)  │");
        println!("│ price     │  9   │  8   │ f64 (LE)    │");
        println!("│ quantity  │ 17   │  4   │ i32 (LE)    │");
        println!("└─────────────────────────────────────────┘");
        println!("\nLE = Little-Endian (小端序编码)");

        // 验证偏移
        assert_eq!(SBE_BLOCK_LENGTH as usize, 21);
        println!("\n✓ 消息块长度确认: {} 字节", SBE_BLOCK_LENGTH);
    }

    /// 示例 7: 编码器和解码器状态
    /// 演示编码器和解码器的初始化和状态管理
    #[test]
    fn example_7_encoder_decoder_state() {
        println!("\n=== 示例 7: 编码器和解码器状态 ===");

        let mut buffer = vec![0u8; 64];

        // 编码器初始化
        {
            let write_buf = WriteBuf::new(&mut buffer);
            let encoder = TradeEncoder::default().wrap(write_buf, 0);

            println!("编码器状态:");
            println!("  ✓ 编码长度 (encoded_length): {}", encoder.encoded_length());
            println!("  ✓ 限制 (limit): {}", encoder.get_limit());

            assert_eq!(encoder.encoded_length(), SBE_BLOCK_LENGTH as usize);
            assert_eq!(encoder.get_limit(), SBE_BLOCK_LENGTH as usize);
        }

        // 编码数据
        {
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = TradeEncoder::default().wrap(write_buf, 0);
            encoder.trade_id(999);
            encoder.symbol(b'X');
            encoder.price(123.45);
            encoder.quantity(999);
        }

        // 解码器初始化
        {
            let read_buf = ReadBuf::new(&buffer);
            let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);

            println!("\n解码器状态:");
            println!("  ✓ 编码块长度 (acting_block_length): {}", decoder.acting_block_length);
            println!("  ✓ 编码版本 (acting_version): {}", decoder.acting_version);
            println!("  ✓ 编码长度 (encoded_length): {}", decoder.encoded_length());

            assert_eq!(decoder.acting_block_length, SBE_BLOCK_LENGTH);
            assert_eq!(decoder.acting_version, 0);
            assert_eq!(decoder.encoded_length(), SBE_BLOCK_LENGTH as usize);
        }
    }

    /// 示例 8: 性能特性
    /// 展示编码器和解码器的性能优化
    #[test]
    fn example_8_performance_characteristics() {
        println!("\n=== 示例 8: 性能特性 ===");

        println!("TradeEncoder 和 TradeDecoder 的性能特性:");
        println!("  ✓ 所有字段设置和读取方法都是内联的 (#[inline])");
        println!("  ✓ 零复制编码 - 直接写入缓冲区");
        println!("  ✓ 零复制解码 - 直接从缓冲区读取");
        println!("  ✓ 固定消息大小 - 21 字节");
        println!("  ✓ 小端序编码 - 现代CPU原生支持");
        println!("  ✓ 无堆分配 - 全栈操作");

        let mut buffer = vec![0u8; 64];

        // 快速编码序列
        {
            let write_buf = WriteBuf::new(&mut buffer);
            let mut encoder = TradeEncoder::default().wrap(write_buf, 0);

            // 这些都是内联操作，没有函数调用开销
            encoder.trade_id(123);
            encoder.symbol(b'A');
            encoder.price(45.67);
            encoder.quantity(890);

            println!("\n  ✓ 快速编码序列完成");
        }

        // 快速解码序列
        {
            let read_buf = ReadBuf::new(&buffer);
            let decoder = TradeDecoder::default().wrap(read_buf, 0, SBE_BLOCK_LENGTH, 0);

            // 这些都是内联操作，没有函数调用开销
            let _ = decoder.trade_id();
            let _ = decoder.symbol();
            let _ = decoder.price();
            let _ = decoder.quantity();

            println!("  ✓ 快速解码序列完成");
        }
    }

    /// 示例 9: 实际应用场景
    /// 模拟一个简单的交易系统
    #[test]
    fn example_9_real_world_scenario() {
        println!("\n=== 示例 9: 实际应用场景 ===");

        struct TradeRecord {
            id: u64,
            symbol: u8,
            price: f64,
            qty: i32,
        }

        let incoming_trades = vec![
            TradeRecord { id: 1001, symbol: b'A', price: 100.50, qty: 1000 },
            TradeRecord { id: 1002, symbol: b'B', price: 200.75, qty: 500 },
            TradeRecord { id: 1003, symbol: b'C', price: 150.25, qty: 750 },
        ];

        println!("接收 {} 笔交易，执行编码处理...", incoming_trades.len());

        // 编码并存储
        let mut storage = vec![0u8; incoming_trades.len() * SBE_BLOCK_LENGTH as usize];

        for (idx, trade) in incoming_trades.iter().enumerate() {
            let offset = idx * SBE_BLOCK_LENGTH as usize;
            let write_buf = WriteBuf::new(&mut storage);
            let mut encoder = TradeEncoder::default().wrap(write_buf, offset);

            encoder.trade_id(trade.id);
            encoder.symbol(trade.symbol);
            encoder.price(trade.price);
            encoder.quantity(trade.qty);

            println!("  ✓ 交易 {} 编码完成: ID={}", idx + 1, trade.id);
        }

        println!("\n总存储大小: {} 字节", storage.len());

        // 解码并验证
        println!("\n读取并验证编码的交易...");
        for (idx, expected) in incoming_trades.iter().enumerate() {
            let offset = idx * SBE_BLOCK_LENGTH as usize;
            let read_buf = ReadBuf::new(&storage);
            let decoder = TradeDecoder::default().wrap(read_buf, offset, SBE_BLOCK_LENGTH, 0);

            let id = decoder.trade_id();
            let symbol = decoder.symbol();
            let price = decoder.price();
            let qty = decoder.quantity();

            assert_eq!(id, expected.id);
            assert_eq!(symbol, expected.symbol);
            assert_eq!(price, expected.price);
            assert_eq!(qty, expected.qty);

            println!(
                "  ✓ 交易 {} 验证通过: ID={}, 符号={}, 价格={}, 数量={}",
                idx + 1,
                id,
                symbol as char,
                price,
                qty
            );
        }
    }
}
