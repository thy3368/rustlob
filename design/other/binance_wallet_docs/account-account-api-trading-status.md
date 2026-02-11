# 账户API交易状态

**来源**: https://developers.binance.com/docs/zh-CN/wallet/account/account-api-trading-status

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
账户账户API交易状态(USER_DATA)
账户API交易状态(USER_DATA)
接口描述​

获取 api 账户交易状态详情。

HTTP请求​

GET /sapi/v1/account/apiTradingStatus

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
recvWindow	LONG	NO	
timestamp	LONG	YES	
响应示例​
{
	"data": {          // 账户API交易状态详情
			"isLocked": false,   // API交易功能是否被锁
			"plannedRecoverTime": 0,  // API交易功能被锁情况下的预计恢复时间
			"triggerCondition": { 
					"GCR": 150,  // Number of GTC orders
					"IFER": 150, // Number of FOK/IOC orders
					"UFR": 300   // Number of orders
			},
			"updateTime": 1547630471725   
	}
}

上一页
账户状态(USER_DATA)
下一页
查询用户API Key权限(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
