场景一：用户如何下单？
如上图序号所示，交易处理流程依次如下：

用户通过网页或API下单，该订单被发送至API Server；

API Server将该请求转发至某一Validator节点；

Validator节点向网络中其他节点（含Proposer节点）广播该请求；

Proposer节点将请求存入本地mempool（交易池）；

Proposer节点按序从mempool中提取一定数量请求，打包生成区块Block101（假设当前区块高度为101），此步骤中节点会从Oracle获取最新链上价格；

Proposer节点对请求执行处理流程：

下单（Order Checking）模块根据用户资金状况校验下单可行性；

撮合（Order Matching）模块将通过校验的订单纳入订单簿进行撮合，生成成交回报；

清算（Clearing）模块依据成交回报，更新用户资金、仓位及订单等信息。

节点将最终系统状态生成state root，存入区块Block101；

Proposer节点本地共识（Consensus）模块将区块Block101同步至其他Validator节点；

其他Validator节点独立处理区块Block101，计算本地state root并与区块中存储的state root比对，完全一致则认可并接收该区块，否则予以拒绝（注：此步骤未在图中标识）；

当网络中超过2/3的Validator节点接收该区块时，区块Block101即完成commit（确认）（注：此步骤亦未在图中标识）。

上述整体流程与我此前文章[2]中介绍的逻辑一致。
