炸烈方案

-----------------------------------------

定序器集群（只排号）
↓ Condensed Input Log
外置状态机（风控/撮合/行情）

定序器集群+外置状态机 架构 外置状态机怎么容灾（撮合）； 撮合分片（按symbol) 多副本

定序器集群+外置状态机 架构 外置状态机怎么容灾（撮合）； 撮合分片（按symbol) 每个分片 Active/Active，event_log
存kv，通过持久化时event_log去重。

订阅event_log 回放实体，从产market数据

本质上两个全序 command_log, replyable_event_log, mi等去重。 基它派出快照 都可重放，也解决锁的的问题，不需要锁。

三大特点。

command_log, replyable_event_log,

aa

全无锁

===================

# 基于分片的定序器， 全序command,event日志, 分片内 active+active容灾

## 分片

订单/撮合 按asset分片
行情 按asset分片
资产 按用户分片

## 每个分片

一个定序器+双活状态机（active+active)

## 每个分片的数据

command_replayable_log 按 Aeron

entity_replayable_event 存分布式kv（实现多副本）每个分片内 使用 entity_replayable_event 做mi等去重

entity 存数据库 （再主备） 通过重放event生成

状态机（use case) 输入 一个 command_replayable_log 输出一组 entity_replayable_event

## 关键领域数据

command_replayable_log{
topic,
shard,
lsn,
logic_ts
}

entity_replayable_event{
topic,
shard,
lsn,
entity_id,
logic_ts
}

entity{
topic,
shard,
lsn,
entity_id,
logic_ts
}

状态机
use_case{

topic,
shard,
} 状态机只处理本分片的

## 中间件选型

定序器 用 Aeron
kv 用 TiKV

## 恢复 可以按分片进行恢复。

entity_replayable_event 唯一事实源，重建entity和lob

## 优缺点





