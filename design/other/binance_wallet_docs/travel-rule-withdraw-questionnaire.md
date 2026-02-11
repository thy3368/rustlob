# 提币问卷内容

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/withdraw-questionnaire

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
旅行规则提币问卷内容(针对需要旅行规则的本地站)
提币问卷内容(针对需要旅行规则的本地站)
本地站列表​
日本
哈萨克斯坦
新西兰
巴林
阿联酋
印度
欧洲(波兰,法国)
南非

如果您不确定使用的问卷内容，请参阅检查问卷需求.

日本​
名称	类型	是否必需	描述
isAddressOwner	INTEGER	YES	1: 发给自己，2:发给其他收款人
bnfType	INTEGER	YES *1	0:个人账户，1:企业账户
kanjiName	STRING	YES *1	
kanaName	STRING	YES *1	
latinName	STRING	YES *1	姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES	收款人国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
city	STRING	YES	
sendTo	INTEGER	YES	1:虚拟货币服务商，2:私有钱包
vasp	STRING	YES *2	收款人的Vasp CODE
vaspCountry	STRING	YES *2	VASP国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
vaspRegion	STRING	YES *3	
txnPurpose	INTEGER	YES *4	1:在日本国内购物，2:遗产、赠予或生活费，3:跨境交易, 4:投资，5:支付第三方VASP的服务费用，6:偿还贷款，7:礼物或捐款
isAttested	BOOLEAN	YES	
当 isAddressOwner 是 2 时必填。 > 2当 sendTo 是 1 时必填。 > 3当 vaspCountry 是 cn(中国) 或 ua(乌克兰) 时。 > 1. 如果 vaspCountry 是 cn(中国)，vaspRegion 必须是 notNortheasternProvinces(东北三省) 或者 other， 即黑龙江，吉林和辽宁。
如果 vaspCountry 是 ua(乌克兰)，vaspRegion 不能为 crimea(克里米亚)，donetsk(顿涅茨克) 或 luhansk(卢甘斯克), 可以是 other。
当 txnPurpose 是 others 时必填。
如果 txnPurpose 为 3，提款将被拒绝，因为 Binance Japan 禁止用于支付进口和/或中间贸易的交易。
您可以从Vasp List API中获取VASP，如果找不到VASP，请在vasp list中输入others，并在vaspName字段中输入VASP的名称。
Binance entities的VASP code是BINANCE。
哈萨克斯坦​
名称	类型	是否必需	描述
isAddressOwner	BOOLEAN	YES	收款人是不是自己
bnfType	INTEGER	YES *1	0:个人账户, 1:企业账户
beneficiaryName	STRING	YES *1	姓名的相关信息，请参阅附录中的姓名限制部分。
beneficiaryCountry	STRING	YES	收款人国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
beneficiaryCity	STRING	YES	
txnPurpose	STRING	YES	合理值: service, goods, p2p, charity, others
txnPurposeOthers	STRING	YES *6	
sendTo	INTEGER	YES	2:交易所, 1:私有钱包
vasp	STRING	YES *2	收款人的VASP Code
vaspName	STRING	YES *3	VASP名
isAttested	BOOLEAN	YES	
当 isAddressOwner 是 false 时必填。
当 sendTo 是 2 时必填。
当 vasp 是 others 时必填。
您可以从Vasp List API中获取VASP，如果找不到VASP，请在vasp list中输入others，并在vaspName字段中输入VASP的名称。
Binance entities的VASP code是BINANCE。
当 txnPurpose 是 others 时必填.
新西兰​
名称	类型	是否必需	描述
isAddressOwner	INTEGER	YES	1:提现给自己, 2:提现给其他人
bnfType	INTEGER	YES *1	0:个人账户, 1:企业账户
bnfName	STRING	YES *2	姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *2	收款人国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
bnfCorpName	STRING	YES *3	收款人企业名称.
bnfCorpCountry	STRING	YES *3	收款人企业所在国家, 有关此信息，请参阅附录中的国家和地区部分。
sendTo	INTEGER	YES	1:私有钱包, 2:交易所
vasp	STRING	YES *4	收款人的VASP Code
vaspName	STRING	YES *5	交易所名称
declaration	BOOLEAN	YES	
当 isAddressOwner 是 2 时必填。
当 bnfType 是 0 时必填.
当 bnfType 是 1 时必填.
当 sendTo 是 2 时必填.
当 vasp 是 others 时必填.
您可以从Vasp List API中获取VASP，如果找不到VASP，请在vasp list中输入others，并在vaspName字段中输入VASP的名称。
Binance entities的 VASP code是BINANCE。
巴林​
名称	类型	是否必需	描述
isAddressOwner	INTEGER	YES	1:提现给自己, 2:提现给其他人
bnfType	INTEGER	YES *1	0:个人账户, 1:企业账户
bnfFirstName	STRING	YES *1	姓名的相关信息，请参阅附录中的姓名限制部分。
bnfLastName	STRING	YES *1	姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *1	收款人国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
city	STRING	YES *1	
sendTo	INTEGER	YES	1:私有钱包, 2:交易所
vasp	STRING	YES *2	收款人的VASP Code
vaspName	STRING	YES *3	VASP名
当 isAddressOwner 是 2 时必填。
当 sendTo 是 2 时必填。
当 vasp 是 others 时必填。
您可以从Vasp List API中获取VASP，如果找不到VASP，请在vasp list中输入others，并在vaspName字段中输入VASP的名称。
Binance entities的VASP code是BINANCE。
阿联酋​
名称	类型	是否必需	描述
isAddressOwner	INTEGER	YES	1:提现给自己, 2:提现给其他人
bnfType	INTEGER	YES *1	0:个人账户, 1:企业账户
bnfName	STRING	YES *1	姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *1	收款人国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
city	STRING	YES *1	
sendTo	INTEGER	YES	1:私有钱包, 2:交易所
vasp	STRING	YES *2	收款人的VASP Code
vaspName	STRING	YES *3	VASP名
当 isAddressOwner 是 2 时必填。
当 sendTo 是 2 时必填。
当 vasp 是 others 时必填。
您可以从Vasp List API中获取VASP，如果找不到VASP，请在vasp list中输入others，并在vaspName字段中输入VASP的名称。
Binance entities的VASP code是BINANCE。
印度​
名称	类型	是否必需	描述
isAddressOwner	INTEGER	YES	1:提现给自己, 2:提现给其他人
bnfType	INTEGER	YES *1	0:个人账户, 1:企业账户
bnfName	STRING	YES *1	姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *1	收款人国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
city	STRING	NO	
sendTo	INTEGER	YES	1:私有钱包, 2:交易所
vasp	STRING	YES *2	收款人的VASP Code
vaspName	STRING	YES *3	VASP名
当 isAddressOwner 是 2 时必填。
当 sendTo 是 2 时必填。
当 vasp 是 others 时必填。
您可以从Vasp List API中获取VASP，如果找不到VASP，请在vasp list中输入others，并在vaspName字段中输入VASP的名称。
Binance entities的VASP code是BINANCE。
欧洲(波兰,法国)​
名称	类型	是否必需	描述
isAddressOwner	INTEGER	YES	1:提现给自己, 2:提现给其他人
bnfType	INTEGER	YES *1	0:个人账户, 1:企业账户
bnfFirstName	STRING	YES *2	姓名的相关信息，请参阅附录中的姓名限制部分。
bnfLastName	STRING	YES *2	姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *2	收款人国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
bnfCorpName	STRING	YES *3	收款人企业名称.
bnfCorpCountry	STRING	YES *3	收款人企业所在国家, 有关此信息，请参阅附录中的国家和地区部分。
sendTo	INTEGER	YES	1:私有钱包, 2:交易所
vasp	STRING	YES *4	收款人的VASP Code
vaspName	STRING	YES *5	交易所名称
declaration	BOOLEAN	YES	
当 isAddressOwner 是 2 时必填。
当 bnfType 是 0 时必填.
当 bnfType 是 1 时必填.
当 sendTo 是 2 时必填.
当 vasp 是 others 时必填.
您可以从Vasp List API中获取VASP，如果找不到VASP，请在vasp list中输入others，并在vaspName字段中输入VASP的名称。
Binance entities的 VASP code是BINANCE。
南非​
名称	类型	是否必需	描述
isAddressOwner	INTEGER	YES	1:提现给自己, 2:提现给其他人
bnfType	INTEGER	YES *1	0:个人账户, 1:企业账户
bnfName	STRING	YES *2	姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *2	收款人国家二位字母代码(ISO-3166)，必须为小写, 有关此信息，请参阅附录中的国家和地区部分。
bnfCorpName	STRING	YES *3	收款人企业名称.
bnfCorpCountry	STRING	YES *3	收款人企业所在国家, 有关此信息，请参阅附录中的国家和地区部分。
sendTo	INTEGER	YES	1:私有钱包, 2:交易所
vasp	STRING	YES *4	收款人的VASP Code
vaspName	STRING	YES *5	交易所名称
declaration	BOOLEAN	YES	
当 isAddressOwner 是 2 时必填。
当 bnfType 是 0 时必填.
当 bnfType 是 1 时必填.
当 sendTo 是 2 时必填.
当 vasp 是 others 时必填.
您可以从Vasp List API中获取VASP，如果找不到VASP，请在vasp list中输入others，并在vaspName字段中输入VASP的名称。
Binance entities的 VASP code是BINANCE。
上一页
获取提币历史V2(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
下一页
提交充值问卷(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
本地站列表
日本
哈萨克斯坦
新西兰
巴林
阿联酋
印度
欧洲(波兰,法国)
南非
Copyright © 2026 Binance.
