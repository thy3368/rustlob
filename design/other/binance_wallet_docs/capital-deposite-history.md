# 获取充值历史

**来源**: https://developers.binance.com/docs/zh-CN/wallet/capital/deposite-history

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
钱包获取充值历史(支持多网络)
获取充值历史(支持多网络)
接口描述​

获取充值历史(支持多网络)

HTTP请求​

GET /sapi/v1/capital/deposit/hisrec

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
includeSource	Boolean	NO	默认 false，如果为true时会返回sourceAddress字段
coin	STRING	NO	
status	INT	NO	0(0:待确认,6:已上账待解锁,7:错误充值,8:待用户申请确认,1:成功,2:已拒绝)
startTime	LONG	NO	默认当前时间90天前的时间戳
endTime	LONG	NO	默认当前时间戳
offset	INT	NO	默认:0
limit	INT	NO	默认：1000，最大1000
recvWindow	LONG	NO	
timestamp	LONG	YES	
txId	STRING	NO	
请注意startTime 与 endTime 的默认时间戳，保证请求时间间隔不超过90天.
同时提交startTime 与 endTime间隔不得超过90天.
请注意，由于网络特定的特性，返回的源地址可能不准确。 如果找到多个源地址，则仅返回第一个地址
响应示例​
[
    {
        "id": "769800519366885376",
        "amount": "0.001",
        "coin": "BNB",
        "network": "BNB",
        "status": 1,
        "address": "bnb136ns6lfw4zs5hg4n85vdthaad7hq5m4gtkgf23",
        "addressTag": "101764890",
        "txId": "98A3EA560C6B3336D348B6C83F0F95ECE4F1F5919E94BD006E5BF3BF264FACFC",
        "insertTime": 1661493146000,
        "completeTime":1661493146000,
        "transferType": 0,
        "confirmTimes": "1/1",
        "unlockConfirm": 0,
        "walletType": 0,
        "travelRuleStatus": 0 //0: travel rule not required OR info already provided and funds ready to use, 1: travel rule required to provide deposit info
    },
    {
        "id": "769754833590042625",
        "amount":"0.50000000",
        "coin":"IOTA",
        "network":"IOTA",
        "status":1,
        "address":"SIZ9VLMHWATXKV99LH99CIGFJFUMLEHGWVZVNNZXRJJVWBPHYWPPBOSDORZ9EQSHCZAMPVAPGFYQAUUV9DROOXJLNW",
        "addressTag":"",
        "txId":"ESBFVQUTPIWQNJSPXFNHNYHSQNTGKRVKPRABQWTAXCDWOAKDKYWPTVG9BGXNVNKTLEJGESAVXIKIZ9999",
        "insertTime":1599620082000,
        "completeTime":1599620082000,// 代表充值完成时间，仅适用于2025年03月06日之后的充值。
        "transferType":0,
        "confirmTimes": "1/1",
        "unlockConfirm": 0,
        "walletType": 0,
        "travelRuleStatus": 1 //0: travel rule not required OR info already provided and funds ready to use, 1: travel rule required to provide deposit info
    }
]

上一页
获取用户提现额度
下一页
获取充值地址(支持多网络)(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
