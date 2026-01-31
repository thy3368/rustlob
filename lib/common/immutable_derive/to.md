
# 体系化的减少数据竞争 clearn架构下的无状态的编程

# 体系主的无状态编程

## 行为类 #[immutable]
- adapter
- service
- repo
handle 实现  sync+send

## 数据类

### 不可变 #[immutable]
- command/query/result/entitychangelog


### 可变

- entity（state）内存化状态 数据库化状态
- 内存态 状态变更 在需要arc+mutex;无锁对列等
- 数据库态  需要select for update, 即数据库锁实现数据竞争。



以 /Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/spot_trade_v2.rs 和/Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/behavior/v2/spot_trade_behavior_v2.rs 中的 #[immutable]为例 并讲 /Users/hongyaotang/src/rustlob/lib/common/immutable_derive/src/lib.rs 原理和用法
