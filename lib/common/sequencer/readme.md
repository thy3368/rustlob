Lighter 作为一个去中心化永续合约交易平台，其核心优势在于通过订单簿模式和链下匹配-链上结算的混合架构，实现了高性能的交易体验。下面这份伪代码将详细展示其排序器与撮合引擎的协同工作流程。

⚙️ Lighter 排序器模块伪代码

排序器是交易进入系统后的第一站，负责接收、验证并管理所有订单，并遵循价格-时间优先原则进行排序，为撮合引擎做好准备。
算法: LighterSequencer
输入: 新订单流
输出: 有序的买卖订单队列

常量:
MAX_QUEUE_DEPTH = 10000  // 单个订单队列最大深度

数据结构 Order:
orderId: string
type: enum {BUY, SELL}
orderType: enum {LIMIT, MARKET, STOP_MARKET, STOP_LIMIT, FOK, IOC, POST_ONLY}
price: float       // 市价单此字段为NaN
quantity: float
timestamp: int64
userId: string
stopPrice: float   // 仅止损单有效

初始化:
buyOrders = new PriorityQueue(comparator=descending(price), then(ascending(timestamp)))
sellOrders = new PriorityQueue(comparator=ascending(price), then(ascending(timestamp)))

函数 main():
循环:
newOrder ← 从网络接收新订单()

        // 1. 订单验证
        if not validateOrder(newOrder):
            向用户发送拒绝消息("订单格式或签名无效")
            continue
        
        // 2. 检查自成交（STP）
        if checkSelfTradePrevention(newOrder, getUserOrders(newOrder.userId)):
            if newOrder.orderType == IOC or newOrder.orderType == FOK:
                向用户发送拒绝消息("自成交阻止")
            else:
                // 修改订单ID或取消旧订单以避免自成交
                handleSTP(newOrder, getUserOrders(newOrder.userId))
            continue
        
        // 3. 根据订单类型路由
        switch newOrder.orderType:
            case LIMIT, POST_ONLY:
                addToOrderBook(newOrder)
            case MARKET:
                immediatelyAttemptMatch(newOrder)  // 立即尝试撮合
            case STOP_MARKET, STOP_LIMIT:
                monitorForTrigger(newOrder)  // 监控触发条件
            case FOK, IOC:
                attemptImmediateExecution(newOrder)  // 尝试立即执行

函数 addToOrderBook(order):
if order.type == BUY:
buyOrders.push(order)
else:
sellOrders.push(order)

    // 通知撮合引擎有新订单可用
    notifyMatchingEngine()

函数 getBestBidAndAsk():
bestBid = buyOrders.peek()  // 最高买价
bestAsk = sellOrders.peek()  // 最低卖价
return (bestBid, bestAsk)


🔄 Lighter 撮合引擎伪代码

撮合引擎是交易系统的核心，它持续运行，检查买卖盘是否可能交叉，并按照价格优先、时间优先的原则执行成交。
算法: LighterMatchingEngine
输入: 来自排序器的有序买卖队列
输出: 成交记录、订单状态更新

数据结构 Trade:
tradeId: string
buyerOrderId: string
sellerOrderId: string
price: float
quantity: float
timestamp: int64

初始化:
matchedTrades = []  // 本次循环的匹配结果

函数 matchingLoop():  // 主循环，持续运行
循环:
(bestBid, bestAsk) ← getBestBidAndAskFromSequencer()

        // 1. 检查是否可匹配（买一价 >= 卖一价）
        if bestBid != null and bestAsk != null and bestBid.price >= bestAsk.price:
            matchOrders(bestBid, bestAsk)
        
        // 2. 批量处理，减少链上交互
        if not matchedTrades.isEmpty() and (matchedTrades.size() >= BATCH_SIZE or timeoutReached()):
            settlementBatch ← prepareSettlementBatch(matchedTrades)
            sendToBlockchain(settlementBatch)  // 提交到链上结算
            matchedTrades.clear()  // 清空已处理交易
        
        休眠(1ms)  // 短暂让出CPU

函数 matchOrders(bid, ask):
// 确定成交价：价格优先方优先
if bid.timestamp < ask.timestamp:  // 买方先到，用买方价格
tradePrice = bid.price
else:  // 卖方先到，用卖方价格
tradePrice = ask.price

    // 取最小数量为成交量
    tradeQuantity = min(bid.quantity, ask.quantity)
    
    // 创建成交记录
    trade = new Trade(
        buyerOrderId = bid.orderId,
        sellerOrderId = ask.orderId,
        price = tradePrice,
        quantity = tradeQuantity,
        timestamp = currentTime()
    )
    matchedTrades.add(trade)
    
    // 更新订单剩余数量
    bid.quantity -= tradeQuantity
    ask.quantity -= tradeQuantity
    
    // 处理完全成交的订单
    if bid.quantity == 0:
        buyOrders.remove(bid)
        notifyUser(bid.userId, "订单完全成交")
    
    if ask.quantity == 0:
        sellOrders.remove(ask)
        notifyUser(ask.userId, "订单完全成交")
    
    // 部分成交处理
    if bid.quantity > 0:
        // 买方订单部分成交，保留在队列顶端
        updateOrderQuantity(bid.orderId, bid.quantity)
    
    if ask.quantity > 0:
        // 卖方订单部分成交，保留在队列顶端
        updateOrderQuantity(ask.orderId, ask.quantity)
    
    return trade

函数 handleMarketOrder(marketOrder):
// 市价单与最佳对手方依次成交
if marketOrder.type == BUY:
// 市价买单：与最低卖价依次成交
while marketOrder.quantity > 0 and not sellOrders.isEmpty():
bestAsk = sellOrders.peek()
matchOrders(marketOrder, bestAsk)
else:
// 市价卖单：与最高买价依次成交
while marketOrder.quantity > 0 and not buyOrders.isEmpty():
bestBid = buyOrders.peek()
matchOrders(bestBid, marketOrder)

函数 handleFOKOrder(fokOrder):
// FOK订单：要么全部立即成交，要么完全取消
availableQuantity = calculateAvailableQuantity(fokOrder)

    if availableQuantity >= fokOrder.quantity:
        // 足够流动性，全部成交
        executeFullOrKill(fokOrder)
        return true
    else:
        // 流动性不足，立即取消
        cancelOrder(fokOrder)
        notifyUser(fokOrder.userId, "FOK订单无法全部成交，已取消")
        return false


💡 关键机制详解

1. 价格-时间优先原则

这是订单簿模式的核心规则。价格优先体现在：买单中价格越高优先级越高，卖单中价格越低优先级越高。当价格相同时，时间优先原则生效：先提交的订单优先成交。这确保了交易的公平性。

2. 订单类型处理

• 限价单：直接进入订单簿排队等待匹配。

• 市价单：不指定价格，立即与订单簿中最佳对手方价格成交，可能产生滑点。

• FOK/IOC订单：要求立即完全成交或立即取消，不进入订单簿。

• 止损单：当标记价格达到触发价时，转变为市价单或限价单。

3. 自成交防护

Lighter 包含 STP 机制，防止同一用户自己与自己成交。系统会检查新订单是否会与用户自己的现有订单匹配，如果是，则根据规则取消新订单或旧订单。

💎 核心优势总结

通过上述伪代码可以看出，Lighter 的排序器与撮合引擎设计具有以下优势：

1.  高性能：链下匹配极大提升了吞吐量，降低了延迟。
2.  公平性：严格的价格-时间优先原则保障了所有交易者的公平性。
3.  丰富功能：支持多种专业订单类型，满足不同交易策略。
4.  风险控制：自成交防护、保证金检查等机制保障系统稳定。

希望这份伪代码能帮助你深入理解 Lighter 排序器与撮合模块的工作原理。如果你对特定环节有更深入的兴趣，我们可以继续探讨其中的技术实现细节。