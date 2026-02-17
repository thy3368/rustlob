
# 委托订单端到端场景中的 支持 单机（单线程/多线程）/分布式版 seda架构

order的生命周期，有主要几个stage

验证支持多种布署架构，逻辑内聚
1.单机单线程版

3.kafka版

seda vs 传统架构的优缺点

去耦合
延时
省api调用。


亮点 用changlogentry统一事件。

参考 
- /Users/hongyaotang/src/rustlob/app/axum_server/src/interfaces/spot/http_server.rs
- /Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/actor
- /Users/hongyaotang/src/rustlob/proc/operating/exchange/spot/src/proc/v2/s_thread_pipeline

