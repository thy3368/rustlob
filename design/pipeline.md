# exchange

## queue<new_order_cmd> mpsc

## queue<spot_order/change_log> spmc

## queue<trade/change_log> mpsc 按订单薄

## queue<balance/change_log>

## queue<kline/change_log>

# hft

## queue<market_data>

## queue<order>

## queue<exchange_order>

模型关系

变更记录

## order

- 变更原因trade

## balance

- balance_change_log
- 变更原因 trade

=========

actor 线程模型 per thread per core
无锁队列
seda 阶段事件驱动
soa simd
kernel by pass

==========

写个低时延cex编码规范文档 “soa/simd友好的POD类型 command/query/event/entity”





重新写文档 定义 低时延中心化交易所（CEX）系统的编码规范，重点关注SOA（Structure of Arrays）/SIMD友好的POD（Plain Old Data）类型设计，涵盖Command、Query、Event、Entity四种核心数据模型。

## 核心设计原则

//todo 补充 设计理念：cex 观注 高吞吐 低时延，本质上是微批向量计算流式数据系统 






