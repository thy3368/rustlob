
https://mp.weixin.qq.com/s/Fk0NDOj2cq8wNVaEYnHESQ

这些理论的核心基石都是 确定性复制状态机（Deterministic Replicated State Machine, RSM）。

图片
状态机的计算模型极其纯粹：

1. 它只消费输入事件（Events In）；
2. 状态的演进必须是完全确定性（Deterministic）的；
3. 根据输入与当前状态，生成一个或多个输出事件（Events Out）。
   结合虚拟同步思想，当我们在分布式系统中引入一个全局全序消息日志（Totally Ordered Sequence Log）时，奇妙的物理特性便涌现了：

1. 消除时钟漂移（Clock Drift）
   在分布式系统中，物理时钟同步（NTP、PTP）极其脆弱且昂贵。但在全序日志模型中，消息在日志中的序号（Sequence Number）与日志自带的时间戳就是系统唯一的“逻辑时间”。任何节点只要处理了相同序号的消息，它们在时间维度上就是绝对对齐的，完全不需要原子钟或复杂的时钟协商。


结合 /Users/hongyaotang/src/rustlob/lib/example/use_case_executor/examples/place_only_spot_order_v2.rs 实例 

写个文件   介绍 确定性复制状态机 领域框架



command/use_case/event/entity  

ts/lsn 在 command/event/entity  的因果关系。在领域层 保证复放绝对数据一致
