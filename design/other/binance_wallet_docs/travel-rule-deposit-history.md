# 获取充值历史(旅行规则)

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/deposit-history

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
获取充值历史(针对需要旅行规则的本地站)(支持多网络)
接口描述​

获取充值历史(针对需要旅行规则的本地站)(支持多网络)

HTTP请求​

GET /sapi/v1/localentity/deposit/history

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
trId	STRING	NO	旅行规则记录ID，支持多条查询，以半角逗号(,) 分隔
txId	STRING	NO	链上TxId，支持多条查询，以半角逗号(,) 分隔
tranId	STRING	NO	充值记录ID，支持多条查询，以半角逗号(,) 分隔
network	STRING	NO	
coin	STRING	NO	
travelRuleStatus	INTEGER	NO	0:处理完成，1:等待处理，2:请求被拒绝
pendingQuestionnaire	BOOLEAN	NO	true: 只返回需要回答充值问卷的记录，false/缺省:返回所有记录
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
        "trId": 123451123,
        "tranId": 17644346245865,
        "amount": "0.001",
        "coin": "BNB",
        "network": "BNB",
        "depositStatus": 0,
        "travelRuleStatus": 1,
        "address": "bnb136ns6lfw4zs5hg4n85vdthaad7hq5m4gtkgf23",
        "addressTag": "101764890",
        "txId": "98A3EA560C6B3336D348B6C83F0F95ECE4F1F5919E94BD006E5BF3BF264FACFC",
        "insertTime": 1661493146000,
        "transferType": 0,
        "confirmTimes": "1/1",
        "unlockConfirm": 0,
        "walletType": 0,
        "requireQuestionnaire": false, // true: This deposit require user to answer questionnaire to get it credited
                                       // false: This deposit doesn't require user to answer questionnaire as it's already completed or information has been verified
        "questionnaire": null
    },
    {
        "trId": 2451123,
        "tranId": 4544346245865,
        "amount":"0.50000000",
        "coin":"IOTA",
        "network":"IOTA",
        "depositStatus": 0,
        "travelRuleStatus": 0,
        "address":"SIZ9VLMHWATXKV99LH99CIGFJFUMLEHGWVZVNNZXRJJVWBPHYWPPBOSDORZ9EQSHCZAMPVAPGFYQAUUV9DROOXJLNW",
        "addressTag":"",
        "txId":"ESBFVQUTPIWQNJSPXFNHNYHSQNTGKRVKPRABQWTAXCDWOAKDKYWPTVG9BGXNVNKTLEJGESAVXIKIZ9999",
        "insertTime":1599620082000,
        "transferType":0,
        "confirmTimes": "1/1",
        "unlockConfirm": 0,
        "walletType": 0,
        "requireQuestionnaire": false,
        "questionnaire": "{\'question1\':\'answer1\',\'question2\':\'answer2\'}"
    }
]

上一页
提交充值问卷(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
下一页
获取充值历史(针对需要旅行规则的本地站)(支持多网络)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
