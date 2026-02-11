# 提交充值问卷

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/deposit-provide-info

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
旅行规则提交充值问卷(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
提交充值问卷(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
接口描述​

提交充值问卷(针对需要旅行规则的本地站)。 只有来自私有钱包或尚未接入GTR的交易所的充值交易才需要提交充值问卷。

HTTP请求​

PUT /sapi/v1/localentity/deposit/provide-info

请求权重(UID)​

600

请求参数​
名称	类型	是否必需	描述
tranId	LONG	YES	充值记录ID
questionnaire	STRING	YES	JSON 格式问卷内容
timestamp	LONG	YES	
每个本地站点的问卷内容都不一样，请参考充值问卷内容页。
如果API返回 Questionnaire format not valid. 或 Questionnaire must not be blank 错误，请尝检查Questionnaire格式并使用 URL-encoded format。
响应示例​
{
	"trId": 765127651,
 	"accepted": true,
 	"info": "Deposit questionnaire accepted."
}

上一页
提币问卷内容(针对需要旅行规则的本地站)
下一页
提交充值问卷(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
接口描述
HTTP请求
请求权重(UID)
请求参数
响应示例
Copyright © 2026 Binance.
