# 基准测试文档中心

欢迎来到 OrderDelta 性能优化和 Criterion 基准测试学习中心！

## 📚 文档导航

### 🎓 学习 Criterion

如果你是 Criterion 新手，按以下顺序学习：

1. **[Criterion 速查表](./CRITERION_CHEATSHEET.md)** ⭐ 推荐首先阅读
   - 快速参考，5 分钟掌握核心 API
   - 常用命令和模式
   - 常见陷阱和解决方案

2. **[Criterion 完整教程](./CRITERION_TUTORIAL.md)**
   - 从零开始的详细教程
   - 基础概念深入讲解
   - 实战案例和最佳实践

3. **[注释版基准测试代码](./orderbook_delta_allocation_annotated.rs)**
   - 逐行注释的实战代码
   - 解释每个决策的原因
   - 可以直接运行和修改

### 🚀 性能优化

了解 OrderDelta 的优化过程：

1. **[性能分析](./PERFORMANCE_ANALYSIS.md)**
   - 为什么 `collect()` 最快？
   - 编译器优化原理
   - 缓存行分析

2. **[优化方案](./ORDERDELTA_OPTIMIZATION.md)**
   - 5 种优化方案详解
   - 内存布局分析
   - 实施路径建议

3. **[优化结果](./OPTIMIZATION_RESULTS.md)**
   - 基准测试结果对比
   - 实际收益计算
   - 经验总结

### 🔧 实用工具

1. **[内存布局分析工具](../examples/memory_layout_analysis.rs)**
   ```bash
   cargo run --example memory_layout_analysis
   ```

2. **[基准测试](./orderbook_delta_allocation.rs)**
   ```bash
   cargo bench --bench orderbook_delta_allocation
   ```

3. **[优化版本对比](./orderdelta_optimized.rs)**
   ```bash
   cargo bench --bench orderdelta_optimized
   ```

---

## 🎯 快速开始

### 第一次使用 Criterion？

```bash
# 1. 查看速查表（5 分钟）
cat benches/CRITERION_CHEATSHEET.md

# 2. 运行示例基准测试
cargo bench --bench orderbook_delta_allocation

# 3. 查看 HTML 报告
open target/criterion/report/index.html

# 4. 阅读注释版代码
cat benches/orderbook_delta_allocation_annotated.rs
```

### 想要优化性能？

```bash
# 1. 分析当前内存布局
cargo run --example memory_layout_analysis

# 2. 阅读优化方案
cat benches/ORDERDELTA_OPTIMIZATION.md

# 3. 运行优化对比测试
cargo bench --bench orderdelta_optimized

# 4. 查看优化结果
cat benches/OPTIMIZATION_RESULTS.md
```

---

## 📊 基准测试结果摘要

### OrderDelta 分配性能（100 个实例）

| 方法 | 时间 | 相对性能 |
|------|------|---------|
| `collect()` | 141.71 ns | 🥇 最快 (基准) |
| `Vec::with_capacity + push` | 179.66 ns | 1.27x 慢 |
| `[T; 100]` 栈数组 | 296.02 ns | 2.09x 慢 |
| `Vec::new + push` | 396.19 ns | 2.80x 慢 |

### OrderDelta 优化效果

| 版本 | 大小 | 分配时间 | 改进 |
|------|------|---------|------|
| V1 (原始) | 56 字节 | 142 ns | 基准 |
| V2 (NonZeroU64) | 48 字节 | 128 ns | -14.3% 内存, -10.3% 时间 |
| V3 (位域压缩) | 48 字节 | 115 ns | -14.3% 内存, -19.5% 时间 |

---

## 🎓 学习路径

### 初学者路径

1. 阅读 [速查表](./CRITERION_CHEATSHEET.md)（15 分钟）
2. 运行 `cargo bench --bench orderbook_delta_allocation`
3. 查看 HTML 报告理解输出
4. 阅读 [注释版代码](./orderbook_delta_allocation_annotated.rs)
5. 修改代码，添加自己的测试

### 进阶路径

1. 完整阅读 [Criterion 教程](./CRITERION_TUTORIAL.md)
2. 学习 [性能分析](./PERFORMANCE_ANALYSIS.md)
3. 研究 [优化方案](./ORDERDELTA_OPTIMIZATION.md)
4. 运行优化对比测试
5. 应用到自己的项目

### 专家路径

1. 深入研究编译器优化
2. 使用 `cargo-asm` 查看汇编代码
3. 使用 profiler 分析性能瓶颈
4. 实施高级优化技术
5. 贡献优化经验

---

## 💡 关键要点

### Criterion 使用

- ✅ **始终使用 `black_box`** 防止编译器优化
- ✅ **使用 `iter_batched`** 分离设置和测量
- ✅ **参数化测试** 找出性能特征
- ✅ **保存基线** 用于回归检测
- ✅ **查看 HTML 报告** 理解结果

### 性能优化

- 🎯 **测量驱动**: 先测量，再优化
- 🎯 **热点聚焦**: 优化瓶颈，不是全面优化
- 🎯 **内存布局**: 字段顺序影响性能
- 🎯 **编译器友好**: 让编译器看到完整数据流
- 🎯 **缓存局部性**: 顺序访问优于随机访问

---

## 🔗 外部资源

### Criterion

- [Criterion.rs 官方文档](https://bheisler.github.io/criterion.rs/book/)
- [Criterion GitHub](https://github.com/bheisler/criterion.rs)

### Rust 性能

- [Rust 性能手册](https://nnethercote.github.io/perf-book/)
- [Jon Gjengset 的性能视频](https://www.youtube.com/c/JonGjengset)
- [Benchmarking 最佳实践](https://easyperf.net/blog/)

### 工具

- [cargo-asm](https://github.com/gnzlbg/cargo-asm) - 查看汇编代码
- [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) - 生成火焰图
- [perf](https://perf.wiki.kernel.org/) - Linux 性能分析工具

---

## 📝 常用命令

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准测试
cargo bench --bench orderbook_delta_allocation

# 只运行匹配的测试
cargo bench -- collect

# 保存基线
cargo bench -- --save-baseline my-baseline

# 对比基线
cargo bench -- --baseline my-baseline

# 查看 HTML 报告
open target/criterion/report/index.html

# 运行内存分析
cargo run --example memory_layout_analysis

# 查看汇编代码（需要安装 cargo-asm）
cargo asm --bench orderbook_delta_allocation create_orderbook_delta
```

---

## 🤝 贡献

发现问题或有改进建议？欢迎：

1. 提交 Issue
2. 发起 Pull Request
3. 分享你的优化经验

---

## 📄 文档列表

### 教程和指南

- [CRITERION_CHEATSHEET.md](./CRITERION_CHEATSHEET.md) - Criterion 速查表
- [CRITERION_TUTORIAL.md](./CRITERION_TUTORIAL.md) - Criterion 完整教程

### 性能分析

- [PERFORMANCE_ANALYSIS.md](./PERFORMANCE_ANALYSIS.md) - 性能深度分析
- [ORDERDELTA_OPTIMIZATION.md](./ORDERDELTA_OPTIMIZATION.md) - 优化方案详解
- [OPTIMIZATION_RESULTS.md](./OPTIMIZATION_RESULTS.md) - 优化结果总结

### 代码示例

- [orderbook_delta_allocation.rs](./orderbook_delta_allocation.rs) - 基础基准测试
- [orderbook_delta_allocation_annotated.rs](./orderbook_delta_allocation_annotated.rs) - 注释版教学代码
- [orderdelta_optimized.rs](./orderdelta_optimized.rs) - 优化版本对比
- [../examples/memory_layout_analysis.rs](../examples/memory_layout_analysis.rs) - 内存布局分析工具

---

## 📊 文档结构

```
benches/
├── README.md                                    # 本文件 - 文档导航
├── CRITERION_CHEATSHEET.md                      # 速查表（推荐首先阅读）
├── CRITERION_TUTORIAL.md                        # 完整教程
├── PERFORMANCE_ANALYSIS.md                      # 性能分析
├── ORDERDELTA_OPTIMIZATION.md                   # 优化方案
├── OPTIMIZATION_RESULTS.md                      # 优化结果
├── orderbook_delta_allocation.rs                # 基础基准测试
├── orderbook_delta_allocation_annotated.rs      # 注释版教学代码
└── orderdelta_optimized.rs                      # 优化版本对比

examples/
└── memory_layout_analysis.rs                    # 内存布局分析工具
```

---

## 🎯 下一步

1. **新手**: 从 [速查表](./CRITERION_CHEATSHEET.md) 开始
2. **学习**: 阅读 [完整教程](./CRITERION_TUTORIAL.md)
3. **实践**: 运行基准测试，修改代码
4. **优化**: 应用到自己的项目
5. **分享**: 贡献你的经验

---

**最后更新**: 2025-12-10
**维护者**: Claude Code
**版本**: 1.0.0

祝你基准测试愉快！🚀
