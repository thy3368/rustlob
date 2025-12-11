# task

- 分析 加密交易所 如币安/ok的 流程域 流程组 流程 任务(command)； 任务由不同角色触发生成entitychangelog并驱动流程的状态 ;写到 /Users/hongyaotang/src/rustlob/app/design/process.md


- 现货对应了 现货账户 杠杆账户（全仓，逐仓） 什么意思 分析一下/Users/hongyaotang/src/rustlob/app/design/process/spot_account.md
- 生成现货流程的任务方法文档/Users/hongyaotang/src/rustlob/app/design/process/spot_api.md 以 方便开发者看  根据/Users/hongyaotang/src/rustlob/lib/core/exchange/lob/src/lob/domain/service/handler.rs的定义
- 分析头部交易所的order id生成机制 设计orderid生成方案到 /Users/hongyaotang/src/rustlob/app/design/process/story/orderid.md
- 分析头部交易所cex, 纽交所，大综期货等交易系统 评估时延 设计交易指令支持的网络的协议方案到 /Users/hongyaotang/src/rustlob/app/design/process/story/command_proto.md
- 竞品分析头部交易所cex, 纽交所，大综期货等交易系统 评估超低时延 cex需要的硬件 /Users/hongyaotang/src/rustlob/app/design/process/story/hard.md
- 竞品分析头部交易所cex, 纽交所，大综期货等交易系统 评估超低时延 cex需要的操作系统 /Users/hongyaotang/src/rustlob/app/design/process/story/os.md
- 分析 tps/qps cqrs 关系 写文档/Users/hongyaotang/src/rustlob/app/design/process/story/cqrs.md
- 借鉴币安的行情，定义行情的读模型，标注读模型的用途如app k线等 使用场景 如散户 量化机构 写文档/Users/hongyaotang/src/rustlob/app/design/process/story/mkdata.md

- 翻译 https://www.coinapi.io/blog/level-1-vs-level-2-vs-level-3-market-data-how-to-read-the-crypto-order-book  写文档/Users/hongyaotang/src/rustlob/app/design/process/story/mkdata123.md
- 写个教程给开发者 “介绍   bdd用于流程建模   ddd用于实体建模 异同”
