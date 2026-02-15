事件驱动的并发模型 actor 分配

收单_actor 【uid,交易对】

inbound_event:new_order
state:order
outbound_event:order_pending/order_cond_pending

条件单_actor【uid,交易对】

inbound_event:order_cond_pending,lob_price
state:order
outbound_event:order_pending

撮合_actor【uid,交易对】

inbound_event:order_pending
state:order,trade
outbound_event:trade_created

结算_actor【uid,交易对】

inbound_event:trade_created
state:balance
outbound_event:balance_changed

K线聚合_actor【uid,交易对】

inbound_event:trade_created
state:kline
outbound_event:kline_created

user_data/market_data推送_actor

inbound_event:kline_created,balance_changed
outbound_event:


