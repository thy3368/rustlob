# 获取充值历史V2(旅行规则)

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/deposit-history-v2

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
旅行规则获取充值历史(针对需要旅行规则的本地站)(支持多网络)
获取充值历史V2(针对需要旅行规则的本地站)(支持多网络)
接口描述​

获取充值历史(针对需要旅行规则的本地站)(支持多网络)

HTTP请求​

GET /sapi/v2/localentity/deposit/history

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
depositId	STRING	NO	入金记录ID，支持多条查询，以半角逗号(,) 分隔
txId	STRING	NO	链上TxId，支持多条查询，以半角逗号(,) 分隔
network	STRING	NO	
coin	STRING	NO	
retrieveQuestionnaire	BOOLEAN	NO	true:返回的记录中带有问卷内容，false/缺省.
startTime	LONG	NO	默认当前时间90天前的时间戳
endTime	LONG	NO	默认当前时间戳
offset	INTEGER	NO	默认:0
limit	INTEGER	NO	默认:1000，最大1000
timestamp	LONG	YES	
请注意startTime 与 endTime 的默认时间戳，保证请求时间间隔不超过90天。
同时提交startTime 与 endTime间隔不得超过90天。
请注意，由于网络特定的特性，返回的源地址可能不准确。 如果找到多个源地址，则仅返回第一个地址。
响应示例​
[
    {
        "depositId": "4615328107052018945",
        "amount": "0.01",
        "network": "AVAXC",
        "coin": "AVAX",
        "depositStatus": 1,
        "travelRuleReqStatus": 0, // 0:PASS,2:REJECTED,3:PENDING,-1:FAILED 
        "address": "0x0010627ab66d69232f4080d54e0f838b4dc3894a",
        "addressTag": "",
        "txId": "0xdde578983015741eed764e7ca10defb5a2caafdca3db5f92872d24a96beb1879",
        "transferType": 0,
        "confirmTimes": "12/12",
        "requireQuestionnaire": false, // true: This deposit require user to answer questionnaire to get it credited
                                       // false: This deposit doesn't require user to answer questionnaire as it's already completed or information has been verified
        "questionnaire": {
            "vaspName": "BINANCE",
            "depositOriginator": 0
        },
        "insertTime": 1753053392000
    }
]

上一页
获取充值历史(针对需要旅行规则的本地站)(支持多网络)
下一页
充值问卷内容(针对需要旅行规则的本地站)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
