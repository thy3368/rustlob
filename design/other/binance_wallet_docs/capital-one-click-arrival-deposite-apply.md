# 一键上账

**来源**: https://developers.binance.com/docs/zh-CN/wallet/capital/one-click-arrival-deposite-apply

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
钱包一键上账(充值到过期地址)(USER_DATA)
一键上账(充值到过期地址)(USER_DATA)
接口描述​

申请充值到过期地址的一键上账.

HTTP请求​

POST /sapi/v1/capital/deposit/credit-apply

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
depositId	LONG	NO	充值记录Id，优先使用
txId	STRING	NO	充值txId，当depositId没指定时使用
subAccountId	LONG	NO	Cloud的子账户ID
subUserId	LONG	NO	母账户的子账户userId
参数应在POST BODY
响应示例​
{
    "code": "000000",
    "message": "success",
    "data":true,
    "success": true
}

上一页
查询充值地址列表(USER_DATA)
下一页
上架资产详情(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
