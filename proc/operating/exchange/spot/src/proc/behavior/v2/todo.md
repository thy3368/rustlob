
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





