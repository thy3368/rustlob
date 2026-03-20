减少数据竞争的线程模型，根据业务特点

以单市场（user_id/符号）为布署单元，各流程 单线程

一个线程 支持 1-N个market

order 无锁 （new[单用户多条/多用多条],cancel,update )
trade 无锁
lob 无锁
balance 有锁
kline 无锁



