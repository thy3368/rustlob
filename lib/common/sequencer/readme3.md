Lighter 作为一个高性能的去中心化永续合约交易平台，其核心优势在于通过订单簿模式和链下匹配-链上结算的混合架构，实现了低延迟、高吞吐量的交易体验。下面这份伪代码将详细展示其排序器与撮合引擎如何协同工作，特别是如何处理市价单、限价单并进行保证金检查。

🔄 系统核心处理流程

为了直观展示订单从提交到成交的全过程，下图描绘了Lighter系统中排序器与撮合引擎的协同工作流程，特别是对市价单、限价单的不同处理路径以及关键的保证金检查环节。
flowchart TD
A[用户提交订单] --> B{订单类型判断}

    B -- 市价单 --> C[市价单通道]
    B -- 限价单 --> D[限价单通道]
    
    C --> E[保证金检查]
    D --> E
    
    E --> F{保证金是否充足?}
    F -- 不足 --> G[拒绝订单并通知用户]
    F -- 充足 --> H
    
    subgraph H [排序器]
        H1[市价单: 直接送入撮合引擎]
        H2[限价单: 按“价格-时间优先”<br>规则插入订单簿]
    end
    
    H1 --> I[撮合引擎]
    H2 --> J[订单簿队列]
    J --> K{订单是否可匹配?}
    K -- 是 --> I
    K -- 否 --> L[继续等待]
    L --> J
    
    I --> M[执行成交并更新仓位]
    M --> N[生成交易证明]
    N --> O[周期性地将状态根提交至链上]


💡 排序器模块与保证金检查伪代码

排序器是交易进入系统的第一道关口，负责接收、验证订单，并执行关键的风险控制——保证金检查。

算法: LighterSequencer.processOrder
输入: 新订单 order
输出: 订单接受结果 (成功/失败及原因)

// 1. 基础格式验证
如果 order 格式无效 (如价格非正数、数量非正整数等):
记录日志("订单格式错误", order.id)
返回 FAIL("订单格式无效")

// 2. 验证用户签名，确保订单真实性
如果 not verifySignature(order.signature, order.content, order.userAddress):
记录日志("签名验证失败", order.id, order.userAddress)
返回 FAIL("签名无效")

// 3. 核心：保证金充足性检查
// 获取用户当前的保证金余额和所有仓位信息
用户账户状态 accountState = getUserAccountState(order.userAddress)
所需初始保证金 requiredMargin = calculateInitialMargin(order, accountState)

// 检查可用余额是否足够开新仓
如果 accountState.availableBalance < requiredMargin:
记录日志("保证金不足", order.id, order.userAddress, requiredMargin)
返回 FAIL("保证金不足，需要: " + requiredMargin)

// 4. 自成交保护检查
如果 checkSelfTradePrevention(order, getUserPendingOrders(order.userAddress)) 为真:
记录日志("自成交阻止", order.id)
返回 FAIL("订单可能与自己现有订单成交")

// 5. 根据订单类型进行路由
如果 order.orderType == MARKET:
// 市价单：直接尝试立即撮合
immediateMatchingResult = 撮合引擎.attemptImmediateMatch(order)
如果 immediateMatchingResult.status == SUCCESS:
记录日志("市价单立即成交", order.id)
返回 SUCCESS("市价单已立即成交")
否则:
// 市价单未能完全成交的部分会被取消 (FOK逻辑) 或转为限价单
返回 FAIL("市价单无法立即完全成交")

否则 如果 order.orderType == LIMIT:
// 限价单：按“价格-时间优先”规则加入订单簿队列
排序器. addToOrderBook(order)
记录日志("限价单已接受并进入订单簿", order.id)
返回 SUCCESS("限价单已接受")

// 所有检查通过，订单进入系统
记录日志("订单接受成功", order.id)
返回 SUCCESS("订单已接受")


⚙️ 撮合引擎模块伪代码

撮合引擎持续运行，检查订单簿中的买卖盘是否可能交叉，并严格按照价格优先、时间优先的原则执行成交。

算法: MatchingEngine.matchingLoop
输入: 无 (持续运行)
输出: 成交记录

循环执行:
// 1. 获取最优买卖盘
bestBid = 订单簿中最高买价订单
bestAsk = 订单簿中最低卖价订单

    // 2. 检查撮合条件：买一价 >= 卖一价
    如果 bestBid 存在 且 bestAsk 存在 且 bestBid.price >= bestAsk.price:
        
        // 确定成交价（通常为先进入订单簿的一方价格）
        tradePrice = determineTradePrice(bestBid, bestAsk)
        // 确定成交量为两者最小值
        tradeQuantity = min(bestBid.quantity, bestAsk.quantity)

        // 3. 执行成交
        // 记录成交
        tradeRecord = createTrade(bestBid, bestAsk, tradePrice, tradeQuantity)
        记录成交记录(tradeRecord)

        // 4. 更新订单状态
        // 减少订单剩余数量，完全成交的则从订单簿移除
        bestBid.quantity -= tradeQuantity
        bestAsk.quantity -= tradeQuantity

        如果 bestBid.quantity == 0:
            从订单簿买盘移除 bestBid
            通知买方用户("订单完全成交")
        否则:
            更新订单簿中 bestBid 的数量

        如果 bestAsk.quantity == 0:
            从订单簿卖盘移除 bestAsk
            通知卖方用户("订单完全成交")
        否则:
            更新订单簿中 bestAsk 的数量

        // 5. 更新用户仓位和保证金
        updateUserPosition(bestBid.userId, tradeRecord)
        updateUserPosition(bestAsk.userId, tradeRecord)
        // 特别重要：根据新的仓位价值，重新计算并冻结/释放相应保证金
        updateMarginForUsers([bestBid.userId, bestAsk.userId])

    否则:
        // 没有可撮合的订单，短暂休眠以避免空转
        休眠(极短时间，如1毫秒)


💎 核心机制总结

•   排序器：扮演风控网关的角色。它通过严格的保证金检查等手段，确保只有安全、有效的订单才能进入系统，从源头控制风险。它对市价单和限价单进行差异化处理，市价单追求即时性，限价单则进入订单簿维护市场深度。

•   撮合引擎：扮演市场核心的角色。它通过高效的订单匹配，确保交易的公平性（价格-时间优先）和流动性。所有关键操作（如成交、清算）都会生成零知识证明（ZK Proof）并提交至以太坊主网进行验证，实现了“中心化执行效率”与“去中心化信任”的结合。

希望这份详细的伪代码能帮助您深入理解 Lighter 排序器与撮合模块的协同工作机制。如果您对特定环节（如止损单的处理或 ZK 证明生成的细节）有进一步的兴趣，我们可以继续探讨。