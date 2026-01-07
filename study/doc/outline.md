## 公链 rusteth

- 共识算法 pow工作量证明,pos,抗拜占庭算法，hot stuff
- merkel tree
- transaction，account，reception，block header
- 数据库
- inbound json rpc/restfull/websocket/libp2p
- outbound 网络libp2p
- 零知识证明

## cex交易所 rustlob

- 现货
- 永续合约 资金费率
- 领域模型 order,trade,account,user
- lob限价订单薄
- 订单类型：市价，限价，条件订单（市价，匹配）路透有100种
- rpo 几分钟恢复； rto, 通过raft实现多副本
- 仓储层算法，数组方式，前匹配，后插入特点最快，比红黑树什么的强
- 网络 组播
- 统一账户

## dex

- aster
- hyper liquid
- lighter

## 钱包技术 rust wallet

- 同链转
- 跨链转
- 闪兑
- amm调用
- 兑换路径

## 公链应用 dapp

- 借贷
- amm

## 编程技术

- clean架构，cqrs，event sourcing，actor线程模型

## rust

- clean架构 控制领域复杂度和扩展性
- ， 先写core[domain,service],[outbound,inbound]; 先单线程，再多线程，再分布式； 先故服务无状态，只有repo有状态，多用消息channel
  象erlang.