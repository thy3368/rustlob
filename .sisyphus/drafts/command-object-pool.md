# Draft: Command 对象池设计

## 需求确认

### 用户目标
为 NewOrderCmd 等 command 设计内存池或对象池，减少短生命周期对象频繁内存操作，提高 cache 友好性。

### 关键约束
1. **处理模式**: 需要支持三种场景
   - 单线程 Actor 模型（每个 Stage 独立线程）
   - 多线程共享处理（多个线程从共享队列获取）
   - 批处理模式（一次处理一批 command）

2. **不可变性约束**: `#[immutable]` 宏使 Command 完全不可变
   - 字段不能修改
   - 需要采用 **Builder 池化** 策略
   - 池化可重用的 Builder，而非 Command 本身

3. **池化粒度**: 池化所有 Command 类型
   - NewOrderCmd
   - CancelOrderCmd
   - QueryOrderCmd
   - 其他所有 command 类型

### 技术发现

#### Command 结构特征
从代码分析发现：
- **结构体较大**: 15-25 个字段
- **包含堆分配**: String、Option<T> 等类型
- **生命周期短**: HTTP 请求 → 处理 → 生成事件 → 丢弃
- **高频创建**: 每个订单请求都创建新对象

#### 当前使用模式
- 位置: `proc/operating/exchange/spot/src/proc/v2/actor/spot_trade_acquiring_stage.rs`
- 处理流程: HTTP → AcquiringStage → handle_new_order → 生成事件
- 无明显的对象池实现

#### 性能目标（从 CLAUDE.md）
- Rust 目标延迟: 零分配操作 < 50ns
- 线程本地池: 获取/归还 < 50ns
- 无锁全局池: 获取/归还 < 200ns

## 设计方案概要

### 方案 A: 线程本地对象池（ThreadLocalPool）
- **适用**: Actor 模型（每个 Stage 独立线程）
- **优势**: 零锁竞争，延迟 < 50ns
- **实现**: thread_local! + RefCell

### 方案 B: 无锁全局池（LockFreePool）
- **适用**: 多线程共享处理
- **优势**: 跨线程共享，延迟 < 200ns
- **实现**: crossbeam::ArrayQueue

### 方案 C: Arena 分配器（ArenaPool）
- **适用**: 批处理模式
- **优势**: 批量分配/释放，内存连续
- **实现**: typed-arena 或自定义 arena

### Builder 池化策略
由于 Command 不可变，采用以下策略：
1. 池化 Builder 而非 Command
2. Builder 可重用，构建出不可变的 Command
3. Builder 归还到池中供下次使用
4. 可选：池化 String 缓冲区减少堆分配

## 待研究问题

1. **#[immutable] 宏实现**: 需要查看宏的具体实现，确认是否有内部可变性
2. **现有池实现**: 检查项目中是否已有对象池基础设施
3. **Command 创建热点**: 确认哪些 command 最频繁创建
4. **内存分配分析**: 使用 profiler 确认当前内存分配瓶颈

## 下一步

等待 Prometheus 启动研究任务，然后生成完整的工作计划。

## 用户确认的需求

### 1. 处理模式（已确认）
**选择：三个方案都要**
- 方案 A：线程本地对象池（ThreadLocalPool）- 单线程 Actor 模型
- 方案 B：无锁全局池（LockFreePool）- 多线程共享处理
- 方案 C：Arena 分配器（ArenaPool）- 批处理模式

**理由**：项目使用多种部署架构（单机单线程、单机多线程、分布式），需要支持所有场景。

### 2. 不可变性约束（已确认）
**选择：完全不可变**
- `#[immutable]` 宏使 Command 字段完全不可变
- 无法实现传统的 reset() 方法
- **必须采用 Builder 池化策略**：
  - 池化可重用的 Builder
  - Builder 构建出不可变的 Command
  - Builder 归还到池中供下次使用

### 3. 池化粒度（已确认）
**选择：池化所有 Command 类型**
- NewOrderCmd（现货、期货、期权）
- CancelOrderCmd
- QueryOrderCmd
- 其他所有 command 类型

**设计要求**：
- 通用的 `CommandPool<T>` trait
- 支持所有 Command 类型的池化
- 可选：池化内部 String 字段（symbol, client_order_id）

## 研究任务进行中

### 任务 1: 性能瓶颈分析
- **状态**: 运行中
- **任务ID**: bg_3ca2fb62
- **目标**: 识别内存分配热点，量化优化潜力

### 任务 2: #[immutable] 宏和使用模式研究
- **状态**: 运行中
- **任务ID**: bg_333e5ff2
## 用户补充：短生命周期对象分类（关键发现）

### 对象分类

**类别 1: 不可变对象**
- **类型**: Command / CommandResult / ChangeLogEntry
- **特征**: `#[immutable]` 宏标记，字段完全不可变
- **池化策略**: Builder 池化
  - 池化可重用的 Builder
  - Builder 构建出不可变的对象
  - Builder 归还到池中供下次使用

**类别 2: 可变对象**
- **类型**: Entity（如 SpotOrder）
- **特征**: 可变字段，支持状态修改
- **池化策略**: 传统对象池
  - 对象可以通过 reset() 方法重置状态
  - 对象直接归还到池中重用
  - 无需 Builder 中间层

### 设计影响

这个分类意味着我们需要设计**两套不同的池化方案**：

1. **CommandPool<T>**: 针对不可变 Command 类型
   - 池化 CommandBuilder<T>
   - Builder.build() → Command
   - Builder.reset() → 清空字段

2. **EntityPool<T>**: 针对可变 Entity 类型
   - 池化 Entity 对象本身
   - Entity.reset() → 重置状态
   - 直接重用对象

### 优先级调整

基于用户需求，优先级应该是：

1. **高优先级**: Command/CommandResult/ChangeLogEntry（不可变）
   - 用户明确提到 "为 NewOrderCmd 等 command 设计"
   - 这是主要需求

2. **中优先级**: Entity（可变）
   - 用户补充提到的第二类对象
   - 可以复用相同的池基础设施

### 待确认问题

1. ChangeLogEntry 是否也需要池化？（用户提到了但未明确）
2. CommandResult 的使用频率如何？
3. SpotOrder 等 Entity 的生命周期模式是什么？
## 用户补充：短生命周期对象分类（关键发现）
### 对象分类
**类别 1: 不可变对象**
- **类型**: Command / CommandResult / ChangeLogEntry
- **特征**: `#[immutable]` 宏标记，字段完全不可变
- **池化策略**: Builder 池化
  - 池化可重用的 Builder
  - Builder 构建出不可变的对象
  - Builder 归还到池中供下次使用
**类别 2: 可变对象**
- **类型**: Entity（如 SpotOrder）
- **特征**: 可变字段，支持状态修改
- **池化策略**: 传统对象池
  - 对象可以通过 reset() 方法重置状态
  - 对象直接归还到池中重用
  - 无需 Builder 中间层
### 设计影响
这个分类意味着我们需要设计**两套不同的池化方案**：
1. **CommandPool<T>**: 针对不可变 Command 类型
   - 池化 CommandBuilder<T>
   - Builder.build() → Command
   - Builder.reset() → 清空字段
2. **EntityPool<T>**: 针对可变 Entity 类型
   - 池化 Entity 对象本身
   - Entity.reset() → 重置状态
   - 直接重用对象
### 优先级调整
基于用户需求，优先级应该是：
1. **高优先级**: Command/CommandResult/ChangeLogEntry（不可变）
   - 用户明确提到 "为 NewOrderCmd 等 command 设计"
   - 这是主要需求
2. **中优先级**: Entity（可变）
   - 用户补充提到的第二类对象
   - 可以复用相同的池基础设施
### 待确认问题
1. ChangeLogEntry 是否也需要池化？（用户提到了但未明确）
2. CommandResult 的使用频率如何？
## 用户新问题：栈分配 vs 堆分配 vs 对象池（架构决策）
### 问题
如果短生命周期对象全部都做栈分配而非堆分配，需要做什么？优缺点是什么？能带来多少性能提升？
### 技术背景
这是一个**替代方案**或**补充方案**的探讨：
- **当前方案**：对象池（堆分配 + 重用）
- **新方向**：栈分配（完全避免堆分配）
- **混合方案**：小对象栈分配，大对象池化
### 关键约束（Rust 特定）
1. **栈空间有限**：默认 2-8MB，大对象会栈溢出
2. **生命周期约束**：栈对象不能跨函数返回（除非 move）
3. **异步问题**：async 函数的栈帧会被捕获到 Future 中
4. **跨线程传递**：栈对象不能直接跨线程传递
5. **String 仍需堆分配**：即使结构体在栈上，String 内容仍在堆上
### 待分析内容
1. Rust 中实现栈分配的技术手段
2. 栈分配的约束和限制
3. 性能提升量化分析（理论 vs 实际）
4. 与对象池方案的对比
5. 混合策略的可能性
6. Command 对象的实际大小估算
## 用户新问题：栈分配 vs 堆分配 vs 对象池（架构决策）
### 问题
如果短生命周期对象全部都做栈分配而非堆分配，需要做什么？优缺点是什么？能带来多少性能提升？
### 技术背景
这是一个**替代方案**或**补充方案**的探讨：
- **当前方案**：对象池（堆分配 + 重用）
- **新方向**：栈分配（完全避免堆分配）
- **混合方案**：小对象栈分配，大对象池化
### 关键约束（Rust 特定）
1. **栈空间有限**：默认 2-8MB，大对象会栈溢出
2. **生命周期约束**：栈对象不能跨函数返回（除非 move）
3. **异步问题**：async 函数的栈帧会被捕获到 Future 中
4. **跨线程传递**：栈对象不能直接跨线程传递
5. **String 仍需堆分配**：即使结构体在栈上，String 内容仍在堆上
### 待分析内容
1. Rust 中实现栈分配的技术手段
2. 栈分配的约束和限制
3. 性能提升量化分析（理论 vs 实际）
4. 与对象池方案的对比
5. 混合策略的可能性
6. Command 对象的实际大小估算
## 用户架构决策：两类对象池（2026-03-03）

### 池化架构分类

用户提出将对象池分成两个独立的类别，这是一个**架构级别的改进**：

#### 1. ImmutablePool（不可变对象池）

**适用对象**：
- Command（NewOrderCmd, CancelOrderCmd, QueryOrderCmd 等）
- CommandResult（所有 Command 的响应对象）
- ChangeLogEntry（订单/成交/余额变更日志）

**池化策略**：
- 池化 **Builder** 而非对象本身
- Builder 可重用，构建出不可变对象
- Builder.reset() 清空所有字段
- Builder.build() 生成不可变对象

**实现方式**：
```rust
// 通用不可变对象池
pub struct ImmutablePool<B: BuilderTrait> {
    builders: Vec<B>,  // 池化 Builder
}

impl<B: BuilderTrait> ImmutablePool<B> {
    pub fn acquire(&self) -> PoolGuard<B> {
        let mut builder = self.builders.pop().unwrap_or_default();
        builder.reset();  // 清空字段
        PoolGuard::new(builder, |b| self.builders.push(b))
    }
}

// 使用示例
let mut builder = CMD_BUILDER_POOL.acquire();
builder.symbol("BTCUSDT").price(50000.0);
let cmd = builder.build();  // 构建不可变 Command
// builder 自动归还到池中
```

#### 2. MutablePool（可变对象池）

**适用对象**：
- Entity（SpotOrder, SpotTrade, Balance 等）
- 任何可变状态的领域对象

**池化策略**：
- 直接池化 **对象本身**
- 对象通过 reset() 方法重置状态
- 对象可以被修改和重用

**实现方式**：
```rust
// 通用可变对象池
pub struct MutablePool<T: Resettable> {
    objects: Vec<T>,  // 池化对象本身
}

impl<T: Resettable> MutablePool<T> {
    pub fn acquire(&self) -> PoolGuard<T> {
        let mut obj = self.objects.pop().unwrap_or_default();
        obj.reset();  // 重置状态
        PoolGuard::new(obj, |o| self.objects.push(o))
    }
}

// 使用示例
let mut order = ORDER_POOL.acquire();
order.set_symbol("BTCUSDT");
order.set_price(50000.0);
// order 自动归还到池中
```

### 架构优势

这个分类带来以下优势：

1. **语义清晰**：名称直接反映对象的可变性特征
2. **类型安全**：编译期保证不可变对象不会被错误修改
3. **通用性强**：可以应用于任何不可变/可变对象
4. **易于扩展**：新增对象类型时，只需选择对应的池类型
5. **性能优化**：两类池可以采用不同的优化策略

### 设计影响

基于这个分类，我们需要设计：

1. **ImmutablePool<B>**：
   - 三种实现：ThreadLocalImmutablePool, LockFreeImmutablePool, ArenaImmutablePool
   - 泛型参数 B 是 Builder 类型
   - 要求 B 实现 BuilderTrait（reset() + build()）

2. **MutablePool<T>**：
   - 三种实现：ThreadLocalMutablePool, LockFreeMutablePool, ArenaMutablePool
   - 泛型参数 T 是对象类型
   - 要求 T 实现 Resettable trait（reset()）

3. **统一的池基础设施**：
   - PoolGuard<T>：RAII 自动归还
   - PoolStats：统计信息（获取/归还/分配次数）
   - PoolConfig：容量配置、预热策略

### 命名约定

- **ImmutablePool**: 不可变对象池（池化 Builder）
- **MutablePool**: 可变对象池（池化对象本身）
- **ThreadLocal{Immutable|Mutable}Pool**: 线程本地池
- **LockFree{Immutable|Mutable}Pool**: 无锁全局池
- **Arena{Immutable|Mutable}Pool**: Arena 分配器

### 最终池化范围（已确认）

**ImmutablePool 对象**：
1. Command（NewOrderCmd, CancelOrderCmd, QueryOrderCmd 等）
2. CommandResult（所有 Command 的响应对象）
3. ChangeLogEntry（订单/成交/余额变更日志）

**MutablePool 对象**：
1. Entity（SpotOrder, SpotTrade, Balance 等）
2. Entity.reset() 策略：完全重置所有字段到默认值
