# 获取充值地址

**来源**: https://developers.binance.com/docs/zh-CN/wallet/capital/deposite-address

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
获取所有币信息(USER_DATA)
提币(USER_DATA)
获取提币历史(支持多网络)(USER_DATA)
查询提现地址簿
获取用户提现额度
获取充值历史(支持多网络)
获取充值地址(支持多网络)(USER_DATA)
查询充值地址列表(USER_DATA)
一键上账(充值到过期地址)(USER_DATA)
资产
账户
旅行规则
其他
错误代码
联系我们
钱包获取充值地址(支持多网络)(USER_DATA)
获取充值地址(支持多网络)(USER_DATA)
接口描述​

获取充值地址

HTTP请求​

GET /sapi/v1/capital/deposit/address

请求权重(IP)​

10

请求参数​
名称	类型	是否必需	描述
coin	STRING	YES	
network	STRING	NO	
amount	DECIMAL	NO	
recvWindow	LONG	NO	
timestamp	LONG	YES	
响应示例​
{
	"address": "1HPn8Rx2y6nNSfagQBKy27GB99Vbzg89wv",
 	"coin": "BTC",
 	"tag": "",
 	"url": "https://btc.com/1HPn8Rx2y6nNSfagQBKy27GB99Vbzg89wv"
}

上一页
获取充值历史(支持多网络)
下一页
查询充值地址列表(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
