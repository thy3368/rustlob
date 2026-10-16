# lob

## 单机

- 初始化， 从磁盘读数据 EntityReplayableEvent和快照，完成内存初始化
- 读，根据条件查询对手单
- 写，通过EntityReplayableEvent重放到刷新内存。
- 关机，停机前的操作
- 单机容灾 就是 通过 EntityReplayableEvent和快照实现。

## 集群

- 集群容灾：raft 容灾

