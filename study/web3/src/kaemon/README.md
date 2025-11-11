# Kameo Actor Framework Examples

这是一个使用 [Kameo](https://github.com/tqwewe/kameo) Actor 框架的简单示例。

## 什么是 Kameo？

Kameo 是一个轻量级、高性能的 Rust Actor 框架，提供：
- 类型安全的消息传递
- 异步消息处理
- 简单易用的 API
- 零成本抽象

## 示例说明

### Counter Actor (计数器 Actor)

这是一个最简单的 Actor 示例，展示了：
- 如何定义 Actor
- 如何定义和处理消息
- 如何与 Actor 通信

**核心概念：**

1. **Actor 定义**：使用 `#[derive(Actor)]` 宏
2. **消息定义**：实现 `Message<T>` trait
3. **消息处理**：在 `handle` 方法中处理消息
4. **消息发送**：使用 `ask().send().await` 发送消息并等待回复

## 运行示例

```bash
# 运行 Counter 示例
cargo run --bin kameo_simple
```

## 代码结构

```
src/kaemon/
├── counter.rs         # Counter Actor 模块（可重用）
├── kameo_simple.rs    # 独立示例（包含完整代码）
├── mod.rs            # 模块定义
└── README.md         # 本文档
```

## 示例输出

```
=== Kameo Counter Example ===

✓ Counter actor spawned
After increment: 1
After increment: 2
After increment: 3

Current count: 3
✓ Counter reset
Count after reset: 0

=== Example completed ===
```

## 扩展示例

你可以基于这个简单示例扩展：
- 添加更多消息类型
- 实现 Actor 间通信
- 使用 Actor 生命周期钩子
- 实现监督策略
