# lob

- 初始化， 从磁盘读数据 EntityReplayableEvent和快照，完成内存初始化
- 读，根据条件查询对手单
- 写，写EntityReplayableEvent或快照 并重放到内存。
- 关机，停机前的操作


- raft 容灾 放到哪一层？是lob这层还是他的上一层。
- 
