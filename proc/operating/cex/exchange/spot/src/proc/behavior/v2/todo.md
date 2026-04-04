
entity.status=entity_change_log

Balance.change


entity_change_log

仓储：


快照仓储（单人[客户端]/分组[服务端]/全量）
事件仓储（单人[客户端]/分组[服务端]/全量）

由服务端分发出来。

客户端
服务端；


command----->changelog-------快照-------

网络中： 快照/change_log两种数据


客户端：
command_repo

changelog_repo[client/多entity]<-------changelog_repo[服务端]
快照_repo[每entity]



✅ 写个example 将 NewOrderCmds 通过simd 转成 ChangeLogEntries
   - 已完成：soa_conversion_example.rs
   - 包含 SIMD 优化的时间戳/序列号生成
   - 包含批量转换、过滤、统计分析
   - 包含完整的测试和性能基准测试


======================================================================================
command/query/result/changelogentry[使用基础类型定义，方便 编解码时0copy 0alloc]

使用基础类型，可以嵌套，退entiy a包含entity b/[b] 




Encoder
Decoder

decoder作为行为的command





高优先级（影响正确性和安全性）：
1. 添加对齐检查（4.1）
2. 修复溢出风险（4.2）
3. 添加字节序处理（4.3）

中优先级（显著性能提升）：
1. 添加容量预分配（2.1）
2. 添加迭代器接口（3.1）
3. 缓存切片引用（1.2）



