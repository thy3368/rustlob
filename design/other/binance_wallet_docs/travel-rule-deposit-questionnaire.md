# 充值问卷内容

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/deposit-questionnaire

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
旅行规则充值问卷内容(针对需要旅行规则的本地站)
充值问卷内容(针对需要旅行规则的本地站)
本地站列表​
日本
哈萨克斯坦
巴林
阿联酋
印度
欧洲(波兰,法国)
南非
日本​
名称	类型	是否必须	描述
depositOriginator	INTEGER	YES	0:发款人自己, 1:发款人不是自己
bnfType	INTEGER	YES	0:个人账号, 1:企业账号
country	STRING	YES *1	发款人国家二位字母代码(ISO-3166)，必须为小写. 有关此信息，请参阅附录中的国家和地区部分。
region	STRING	YES *2	发款人所在地区.
city	STRING	YES *1	发款人所在城市
kanjiName	STRING	YES *1	
kanaName	STRING	YES *1	
latinName	STRING	YES *1	姓名的相关信息，请参阅附录中的姓名限制部分。
vaspName	STRING	YES	
isAttested	BOOLEAN	YES	
当 depositOriginator 是 1 时必填。
当 country 是 cn(中国) 或 ua(乌克兰) 时。
如果 country 是 cn(中国)，region 需要为 isNortheasternProvinces(东北三省)，即黑龙江，吉林和辽宁，或者 other。
如果 country 是 ua(乌克兰)，region 不能为 crimea(克里米亚)，donetsk(顿涅茨克) 或 luhansk(卢甘斯克), 可以为 other。
哈萨克斯坦​
名称	类型	是否必须	描述
originatorName	STRING	YES	发款人姓名
country	STRING	YES	发款人国家二位字母代码(ISO-3166)，必须为小写. 有关此信息，请参阅附录中的国家和地区部分。
city	STRING	YES	发款人所在城市
txnPurpose	STRING	YES	合理值: service, goods, p2p, charity, others
txnPurposeOthers	STRING	YES *1	
当 txnPurpose 是 others 时必填.
巴林​
名称	类型	是否必须	描述
depositOriginator	INTEGER	YES	1:发款人是自己, 2:发款人不是自己
orgType	INTEGER	YES *1	0:个人账号, 1:企业账号
orgFirstName	STRING	YES *1	发款人名, 姓名的相关信息，请参阅附录中的姓名限制部分。
orgLastName	STRING	YES *1	发款人姓, 姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *1	发款人居住国二位字母代码(ISO-3166)，必须为小写. 有关此信息，请参阅附录中的国家和地区部分。
city	STRING	YES *1	
receiveFrom	INTEGER	YES	1:私有钱包, 2:其他交易所
vasp	STRING	YES *2	发款人的VASP code
vaspName	STRING	YES *3	VASP名
当 depositOriginator 是 2 时必填。
当 receiveFrom 是 2 时必填。
当 vasp 是 others 时必填。
如果vasp不在预先加载VASP列表中, vasp字段请填others, vaspName字段请填交易所的名字。
Binance entities的VASP code是BINANCE。
阿联酋​
名称	类型	是否必须	描述
depositOriginator	INTEGER	YES	1:发款人是自己, 2:发款人不是自己
orgType	INTEGER	YES *1	0:个人账号, 1:企业账号
orgName	STRING	YES *1	姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *1	发款人国籍二位字母代码(ISO-3166)，必须为小写. 有关此信息，请参阅附录中的国家和地区部分。
city	STRING	YES *1	
receiveFrom	INTEGER	YES	1:私有钱包, 2:其他交易所
vasp	STRING	YES *2	收款人的VASP code
vaspName	STRING	YES *3	VASP名
当 depositOriginator 是 2 时必填。
当 receiveFrom 是 2 时必填。
当 vasp 是 others 时必填。
如果vasp不在预先加载VASP列表中, vasp字段请填others, vaspName字段请填交易所的名字。
Binance entities的VASP code是BINANCE。
印度​
名称	类型	是否必须	描述
depositOriginator	INTEGER	YES	1:发款人是自己, 2:发款人不是自己
orgType	INTEGER	YES *1	0:个人账号, 1:企业账号
orgName	STRING	YES *1	发款人姓名, 姓名的相关信息，请参阅附录中的姓名限制部分。
pan	STRING	YES *1	永久账号（PAN）或国民身份证号码
country	STRING	YES *1	发款人国籍二位字母代码(ISO-3166)，必须为小写. 有关此信息，请参阅附录中的国家和地区部分。
state	STRING	YES *1	发款人所在州
city	STRING	YES *1	发款人所在 城市/城镇/村庄
pinCode	STRING	YES *1	
address	STRING	YES *1	
receiveFrom	INTEGER	YES	1:私有钱包, 2:其他交易所
vasp	STRING	YES *2	收款人的VASP code
vaspName	STRING	YES *3	VASP名
当 depositOriginator 是 2时必填.
当 receiveFrom 是 2时必填.
当 vasp 是 others 时必填。
如果vasp不在预先加载VASP列表中, vasp字段请填others, vaspName字段请填交易所的名字。
Binance entities的VASP code是BINANCE。
欧洲(波兰,法国)​
名称	类型	是否必须	描述
depositOriginator	INTEGER	YES	1:发款人是自己, 2:发款人不是自己
orgType	INTEGER	YES *1	0:个人账号, 1:企业账号
orgFirstName	STRING	YES *2	发款人名, 姓名的相关信息，请参阅附录中的姓名限制部分。
orgLastName	STRING	YES *2	发款人姓, 姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *2	发款人国家, 有关此信息，请参阅附录中的国家和地区部分。
corpName	STRING	YES *3	发款企业姓名
corpCountry	STRING	YES *3	发款企业所在国家
receiveFrom	INTEGER	YES	1:私有钱包, 2:其他交易所
vasp	STRING	YES *4	交易所编码
vaspName	STRING	YES *5	交易所名称
declaration	BOOLEAN	YES	
当 depositOriginator 是 2 时必填.
当 receiveFrom is 2 时必填.
当 orgType is 0 时必填.
当 orgType is 1 时必填.
如果 vasp 不是 Binance, 请在 vasp 中填写 others 并在 vaspName 中填写交易所名称.
南非​
名称	类型	是否必须	描述
depositOriginator	INTEGER	YES	1:发款人是自己, 2:发款人不是自己
orgType	INTEGER	YES *1	0:个人账号, 1:企业账号
orgName	STRING	YES *2	发款人姓名, 姓名的相关信息，请参阅附录中的姓名限制部分。
country	STRING	YES *2	发款人国家, 有关此信息，请参阅附录中的国家和地区部分。
corpName	STRING	YES *3	发款企业姓名
corpCountry	STRING	YES *3	发款企业所在国家, 有关此信息，请参阅附录中的国家和地区部分。
receiveFrom	INTEGER	YES	1:私有钱包, 2:其他交易所
vasp	STRING	YES *4	交易所编码
vaspName	STRING	YES *5	交易所名称
declaration	BOOLEAN	YES	
当 depositOriginator 是 2 时必填.
当 receiveFrom is 2 时必填.
当 orgType is 0 时必填.
当 orgType is 1 时必填.
如果 vasp 不是 Binance, 请在 vasp 中填写 others 并在 vaspName 中填写交易所名称.
上一页
获取充值历史(针对需要旅行规则的本地站)(支持多网络)
下一页
VASP List
本地站列表
日本
哈萨克斯坦
巴林
阿联酋
印度
欧洲(波兰,法国)
南非
Copyright © 2026 Binance.
