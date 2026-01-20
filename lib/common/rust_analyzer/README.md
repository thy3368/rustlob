# Rust 代码优化分析工具

一个强大的 Rust 代码静态分析工具，用于检测性能优化机会和潜在的性能问题。

## 功能特性

### 🔍 静态代码分析
- **AST 分析**: 使用 `syn` 解析 Rust 代码，检测常见性能反模式
- **正则模式匹配**: 识别内存分配、克隆操作、对齐问题等
- **函数分析**: 检测内联候选、递归函数、热路径分配

### 🔬 LLVM 深度分析
- **IR 生成**: 自动生成不同优化级别的 LLVM IR
- **向量化分析**: 检测循环向量化机会和障碍
- **内联分析**: 评估函数内联决策
- **循环优化**: 分析循环展开、交换、融合等优化

### 📊 优化评分系统
- **多维度评分**: 向量化、内存、内联、缓存对齐
- **性能预估**: 估算优化后的加速比
- **优先级排序**: 按影响程度排序问题

### 📈 多格式报告
- **终端输出**: 彩色交互式报告
- **JSON/YAML**: 机器可读格式
- **HTML**: 精美的可视化报告

## 安装

```bash
cd /Users/hongyaotang/src/rustlob/opt/rust_analyzer
cargo build --release
```

## 使用方法

### 基础分析

分析当前项目：
```bash
cargo run -- analyze
```

分析指定项目：
```bash
cargo run -- analyze --path /path/to/your/project
```

### 深度 LLVM 分析

```bash
cargo run -- analyze --deep --path /path/to/project
```

### 生成报告

JSON 格式：
```bash
cargo run -- analyze --output json --output-file report.json
```

HTML 格式：
```bash
cargo run -- analyze --output html --output-file report.html
```

### LLVM IR 分析

生成并分析 LLVM IR：
```bash
cargo run -- llvm-analyze --path /path/to/project --opt-level 3
```

比较不同优化级别：
```bash
cargo run -- compare --path /path/to/project
```

## 检测的优化机会

### 内存管理
- ✅ 热路径中的堆分配
- ✅ 不必要的 `.clone()` 调用
- ✅ 未使用对象池的重复分配
- ✅ 字符串拼接在循环中

### 向量化
- ✅ 可向量化的循环
- ✅ SIMD 优化机会
- ✅ 内存访问模式分析
- ✅ 向量化障碍识别

### 内联优化
- ✅ 内联候选函数
- ✅ 小函数未添加 `#[inline]`
- ✅ 过大函数阻止内联

### 缓存对齐
- ✅ 未对齐的关键数据结构
- ✅ False sharing 风险
- ✅ 缓存行大小优化

### 并发优化
- ✅ 过度使用 Mutex/RwLock
- ✅ 无锁数据结构建议
- ✅ 原子操作优化

### 算法优化
- ✅ 递归函数检测
- ✅ 分支预测优化
- ✅ 循环优化建议

## 评分系统

评分范围: 0-100
- **A (90-100)**: 优秀 - 代码优化良好
- **B (80-89)**: 良好 - 有少量改进空间
- **C (70-79)**: 中等 - 有明显优化机会
- **D (60-69)**: 及格 - 需要重点优化
- **F (<60)**: 需改进 - 存在严重性能问题

## 输出示例

```
╔══════════════════════════════════════════════════════════╗
║       Rust 代码优化潜力分析报告                         ║
╚══════════════════════════════════════════════════════════╝

📊 基本统计:
  • 分析文件数: 45
  • 总代码行数: 12,350
  • 发现问题数: 23
  • 总函数数量: 156

🎯 优化分数:
  • 总体评分: 78.5/100 (C (中等))
  • 向量化得分: 82.3/100
  • 内存管理得分: 71.2/100
  • 内联优化得分: 85.1/100
  • 缓存对齐得分: 75.8/100

  • 预估加速潜力: 1.21x
  • 优化潜力空间: 21.5%

🔬 LLVM深度分析:
  • 向量化循环: 34/45 (75.6%)
  • 内联函数: 142/156 (91.0%)
  • 循环展开: 28

💡 优化建议:
  1. 考虑使用对象池或预分配内存减少堆分配
  2. 使用迭代器方法或显式SIMD指令优化循环
  3. 为小函数添加 #[inline] 属性
  4. 使用 #[repr(align(64))] 对齐关键数据结构
```

## 技术栈

- **syn**: Rust 代码解析
- **regex**: 模式匹配
- **serde**: 序列化支持
- **colored**: 终端彩色输出
- **clap**: CLI 参数解析

## 配置要求

- Rust 1.70+
- LLVM 工具链 (可选，用于深度分析)
- Cargo

## 架构设计

遵循 Clean Architecture 原则：

```
src/
├── main.rs           # CLI 入口
├── analyzer.rs       # 核心分析引擎
├── llvm_analyzer.rs  # LLVM IR 分析
├── patterns.rs       # 模式检测器
├── scorer.rs         # 评分系统
├── reporter.rs       # 报告生成器
└── optimizer.rs      # 优化建议
```

## 性能

- 分析 1000 个文件约需 5-10 秒
- LLVM 深度分析额外需要编译时间
- 支持并行分析（使用 rayon）

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 参考文档

基于 `/Users/hongyaotang/src/rustlob/opt/优化.md` 中的优化指南实现。
