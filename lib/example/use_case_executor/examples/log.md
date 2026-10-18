
https://mp.weixin.qq.com/s/Fk0NDOj2cq8wNVaEYnHESQ

这些理论的核心基石都是 确定性复制状态机（Deterministic Replicated State Machine, RSM）。

图片
状态机的计算模型极其纯粹：

1. 它只消费输入事件（Events In）；
2. 状态的演进必须是完全确定性（Deterministic）的；
3. 根据输入与当前状态，生成一个或多个输出事件（Events Out）。
   结合虚拟同步思想，当我们在分布式系统中引入一个全局全序消息日志（Totally Ordered Sequence Log）时，奇妙的物理特性便涌现了：


写个文件 