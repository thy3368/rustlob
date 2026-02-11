# 获取提币历史V2(旅行规则)

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/withdraw-history-v2

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
旅行规则获取提币历史V2(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
获取提币历史V2(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
接口描述​

获取需要旅行规则的本地站的提币历史记录

HTTP请求​

GET /sapi/v2/localentity/withdraw/history

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
trId	STRING	NO	旅行规则记录ID, 支持多条查询，以半角逗号(,) 分隔.
txId	STRING	NO	链上TxId，支持多条查询，以半角逗号(,) 分隔.
withdrawOrderId	STRING	NO	客户端内部定义的提币ID.
network	STRING	NO	
coin	STRING	NO	
travelRuleStatus	INTEGER	NO	0:处理完成,1:处理中,2:请求被拒绝
offset	INT	NO	
limit	INT	NO	默认：1000， 最大：1000
startTime	LONG	NO	默认当前时间90天前的时间戳
endTime	LONG	NO	默认当前时间戳
timestamp	LONG	YES	
支持多网络提币前的历史记录可能不会返回network字段。
请注意startTime 与 endTime 的默认时间戳，保证请求时间间隔不得超过90天。
同时提交startTime 与 endTime间隔不得超过90天。
通过API：/sapi/v1/capital/withdraw/apply 提交的提币记录，可能通过该API无法获取内容。
如果通过withdrawOrderId查询startTime和endTime的默认时间间隔为7天.
如果通过withdrawOrderId查询startTime和endTime间隔不得超过7天.
如果通过trId,txId查询，最大支持的ID数量是45.
如果通过WithdrawOrderId 查询，ID最多只支持一个.
如果返回结果中不包含withdrawalStatus等信息, 请传入trId或者txId来查询数据.
响应示例​
[
  {
    "id": "b6ae22b3aa844210a7041aee7589627c",  // 该笔提现在币安的id, 只在sAPI提现的记录中返回
    "trId": 1234456,  // 旅行规则记录Id
    "amount": "8.91000000",   // 提现转出金额
    "transactionFee": "0.004", // 手续费, 只在sAPI提现的记录中返回
    "coin": "USDT",
    "withdrawalStatus": 6, // 提币状态, 只在sAPI提现的记录中返回
    "travelRuleStatus": 0, // 旅行规则处理状态，处理完成(0)之后才会继续提币流程
    "address": "0x94df8b352de7f46f64b01d3666bf6e936e44ce60",
    "txId": "0xb5ef8c13b968a406cc62a93a8bd80f9e9a906ef1b3fcf20a2e48573c17659268",   // 提现交易id
    "applyTime": "2019-10-12 11:12:02",  // UTC 时间
    "network": "ETH",
    "transferType": 0, // 1: 站内转账, 0: 站外转账, 只在sAPI提现的记录中返回
    "withdrawOrderId": "WITHDRAWtest123", // 自定义ID, 如果没有则不返回该字段, 只在sAPI提现的记录中返回
    "info": "The address is not valid. Please confirm with the recipient",  // 提币失败原因
    "confirmNo":3,  // 提现确认数, 只在sAPI提现的记录中返回
    "walletType": 1,  //1: 资金钱包 0:现货钱包, 只在sAPI提现的记录中返回
    "txKey": "", //只在sAPI提现的记录中返回
    "questionnaire": "{\'question1\':\'answer1\',\'question2\':\'answer2\'}", // 问卷回答
    "completeTime": "2023-03-23 16:52:41"  // 提现完成，成功下账时间(UTC)
  },
  {
    "id": "156ec387f49b41df8724fa744fa82719",
    "trId": 22334411,
    "amount": "0.00150000",
    "transactionFee": "0.004",
    "coin": "BTC",
    "withdrawalStatus": 6,
    "travelRuleStatus": 0,
    "address": "1FZdVHtiBqMrWdjPyRPULCUceZPJ2WLCsB",
    "txId": "60fd9007ebfddc753455f95fafa808c4302c836e4d1eebc5a132c36c1d8ac354",
    "applyTime": "2019-09-24 12:43:45",
    "network": "BTC",
    "transferType": 0, 
    "info": "",
    "confirmNo": 2,
    "walletType": 1,
    "txKey": "",
    "questionnaire": "{\'question1\':\'answer1\',\'question2\':\'answer2\'}",
    "completeTime": "2023-03-23 16:52:41" 
  }
]

上一页
获取提币历史(针对需要旅行规则的本地站)(支持多网络)(USER_DATA)
下一页
提币问卷内容(针对需要旅行规则的本地站)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
