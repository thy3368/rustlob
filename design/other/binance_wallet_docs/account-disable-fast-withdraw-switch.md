# 关闭站内划转

**来源**: https://developers.binance.com/docs/zh-CN/wallet/account/disable-fast-withdraw-switch

---

跳到主要内容
产品
▼
搜索
⌘
K
当前
全站
简体中文
钱包
更新日志
快速开始
概述
基本信息
钱包
资产
账户
帐户信息
查询每日资产快照(USER_DATA)
关闭站内划转(USER_DATA)
开启站内划转(USER_DATA)
账户状态(USER_DATA)
账户API交易状态(USER_DATA)
查询用户API Key权限(USER_DATA)
旅行规则
其他
错误代码
联系我们
账户关闭站内划转(USER_DATA)
关闭站内划转(USER_DATA)

关闭站内划转

HTTP请求​

POST /sapi/v1/account/disableFastWithdrawSwitch

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
recvWindow	LONG	NO	
timestamp	LONG	YES	

注意:

  此请求会关闭您账户的站内快速划转。您需要为api-key开通"trade"权限才能发送此请求。

响应示例​
{}

上一页
查询每日资产快照(USER_DATA)
下一页
开启站内划转(USER_DATA)
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
