# rustlog 部署架构

## 单体部署 开发/测试环境

### 部署拓扑图

主要组件

#### spot 应用服务

/Users/hongyaotang/src/rustlob/app/gw_axum/src/main.rs
spot::starter::start_spot_module()

订单内存库 以嵌入式方式集成

lob repo

### 数据库



PostgreSQL 或内存库，不分库分表

按uid分
user/account/balance

按交易对分
order

### 连路

client ---> spot 应用服务---->各种repo

### todo  拓pu图