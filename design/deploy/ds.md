# rustlog 部署架构

## 分布式部署 生产环境

### 设计目标：

水平扩展

容灾

指标： rto=3分钟；rpo=0

### 部署拓扑图

主要组件

#### 网关

- pingora/nginx


#### spot 应用服务

/Users/hongyaotang/src/rustlob/app/gw_axum/src/main.rs
spot::starter::start_spot_module()

订单内存库 以嵌入式方式集成

lob repo

### 数据库

PostgreSQL

按uid分仓储
user/account/balance/order/trade

按交易对分
lob
market data

### 连路

client -(http/websocket)--> 网关--(grpc)-->spot 应用服务---->各种repo

### todo  拓pu图