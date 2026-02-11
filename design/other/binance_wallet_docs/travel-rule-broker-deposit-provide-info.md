# Submit Broker Deposit Questionnaire

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/broker-deposit-provide-info

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
旅行规则Submit Broker Deposit Questionnaire
Submit Deposit Questionnaire (For local entities that require travel rule) (supporting network) (USER_DATA)
API Description​

Submit questionnaire for brokers of local entities that require travel rule. The questionnaire is only applies to transactions from un-hosted wallets or VASPs that are not yet onboarded with GTR.

HTTP Request​

PUT /sapi/v1/localentity/broker/deposit/provide-info

Request Weight(UID)​

600

Request Parameters​
Name	Type	Mandatory	Description
subAccountId	STRING	YES	External user ID.
depositId	STRING	YES	Wallet deposit ID.
questionnaire	STRING	YES	JSON format questionnaire answers.
beneficiaryPii	STRING	YES	JSON format beneficiary Pii.
network	STRING	NO	
coin	STRING	NO	
amount	BigDecimal	NO	
address	STRING	NO	
addressTag	STRING	NO	
timestamp	LONG	YES	Epoch Sec
signature	STRING	YES	Must be the last parameter
Questionnaire is different for each local entity, please refer to Deposit Questionnaire Content page.
If getting error like Questionnaire format not valid. or Questionnaire must not be blank, please try to verify the format of the questionnaire and use URL-encoded format.
StandardPii​

For Natural Person

Name	Type	Mandatory	Description
piiType	INTEGER	YES	Fix to 0: Natural Person
latinNames	List	YES	In case a person have complicated names or multiple names, this parameter is a list
localNames	List	NO	In case a person have complicated names or multiple names, this parameter is a list
nationality	STRING	NO	
residenceCountry	STRING	YES	
nationalIdentifier	STRING	NO	
nationalIdentifierType	STRING	NO	
nationalIdentifierIssueCountry	STRING	NO	
dateOfBirth	STRING	NO	yyyy-mm-dd. Not required but strongly recommended. Providing DOB could greatly reduce false positive rate during risk checking process.
placeOfBirth	STRING	NO	
address	STRING	NO	

For Legal Person

Name	Type	Mandatory	Description
piiType	INTEGER	YES	Fix to 1: Legal Person
latinName	STRING	YES	It's company name for Legal Person
localName	STRING	NO	
registrationCountry	STRING	YES	
nationalIdentifier	STRING	NO	
nationalIdentifierType	STRING	NO	
nationalIdentifierIssueCountry	STRING	NO	
registrationDate	STRING	NO	yyyy-mm-dd. Not required but strongly recommended.
address	STRING	NO	
walletAddress	STRING	NO	
walletTag	STRING	NO	

PiiName

Name	Type	Mandatory	Description
firstName	STRING	YES	Mandatory for Natural person
middleName	STRING	NO	
lastName	STRING	NO	
Response Example​
{
	"trId": 765127651,
 	"accepted": true,
 	"info": "Deposit questionnaire accepted."
}

上一页
Broker Withdraw
下一页
Check Questionnaire Requirements
API Description
HTTP Request
Request Weight(UID)
Request Parameters
StandardPii
Response Example
Copyright © 2026 Binance.
