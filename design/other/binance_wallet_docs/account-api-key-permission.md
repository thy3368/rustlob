# 查询用户API Key权限

**来源**: https://developers.binance.com/docs/zh-CN/wallet/account/api-key-permission

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
账户查询用户API Key权限(USER_DATA)
查询用户API Key权限(USER_DATA)
接口描述​

查询用户API Key权

HTTP请求​

GET /sapi/v1/account/apiRestrictions

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
recvWindow	LONG	NO	
timestamp	LONG	YES	
响应示例​
{
   "ipRestrict": false,  // 是否限制ip访问
   "createTime": 1623840271000,   // 创建时间
   "enableReading": true,
   "enableWithdrawals": false,   // 此选项允许通过此api提现。开启提现选项必须添加IP访问限制过滤器
   "enableInternalTransfer": true,  // 此选项授权此密钥在您的母账户和子账户之间划转资金
   "enableMargin": false,   // 此选项在全仓账户完成划转后可编辑
   "enableFutures": false,  // 合约交易权限，需注意开通合约账户之前创建的API Key不支持合约API功能
   "permitsUniversalTransfer": true,  // 授权该密钥可用于专用的万向划转接口，用以操作其支持的多种类型资金划转。各业务自身的划转接口使用权限，不受本授权影响
   "enableVanillaOptions": false,  // 欧式期权交易权限
   "enableFixApiTrade": false,   // FIX API交易权限
   "enableFixReadOnly": true,   // FIX API读取权限
   "enableSpotAndMarginTrading": false, // 现货和杠杆交易权限
   "enablePortfolioMarginTrading":true  //  统一账户交易权限
}

上一页
账户API交易状态(USER_DATA)
下一页
提币(针对需要旅行规则的本地站)(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
