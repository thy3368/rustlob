# Check Questionnaire Requirements

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/questionnaire-requirements

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
旅行规则
提币(针对需要旅行规则的本地站)(USER_DATA)
获取提币历史(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
获取提币历史V2(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
提币问卷内容(针对需要旅行规则的本地站)
提交充值问卷(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
提交充值问卷(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
获取充值历史(针对需要旅行规则的本地站)(支持多网络)
获取充值历史(针对需要旅行规则的本地站)(支持多网络)
充值问卷内容(针对需要旅行规则的本地站)
VASP List
Address Verification list
Broker Withdraw
Submit Broker Deposit Questionnaire
Check Questionnaire Requirements
Appendix
其他
错误代码
联系我们
旅行规则Check Questionnaire Requirements
检查问卷需求(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
接口描述​

基于当前的用户的API key，接口讲返回针对改用户提交问卷所需的信息。

HTTP 请求​

GET /sapi/v1/localentity/questionnaire-requirements

请求权重(IP)​

1

请求参数​
名称	类型	是否必须	描述
recvWindow	LONG	NO	
timestamp	LONG	YES	
响应示例​

{
    "questionnaireCountryCode":"AE"
}


上一页
Submit Broker Deposit Questionnaire
下一页
Appendix
接口描述
HTTP 请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
