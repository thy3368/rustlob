# Address Verification list

**来源**: https://developers.binance.com/docs/zh-CN/wallet/travel-rule/address-verification-list

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
旅行规则Address Verification list
Fetch address verification list (USER_DATA)
接口描述​

获取地址验证列表，以便用户检查地址簿中存储的地址的状态和其他详细信息。

HTTP 请求​

GET /sapi/v1/addressVerify/list

请求权重(IP)​

1

请求参数​
名称	类型	是否必须	描述
recvWindow	LONG	NO	
timestamp	LONG	YES	
响应示例​
[
  {
    "status": "PENDING",
    "token": "AVAX",
    "network": "AVAXC",
    "walletAddress": "0xc03a6aa728a8dde7464c33828424ede7553a0021",
    "addressQuestionnaire": { 
      "sendTo": 1,
      "satoshiToken": "AVAX",
      "isAddressOwner": 1,
      "verifyMethod": 1
    }
  }
]

status：指地址验证的状态。响应将返回以下状态之一 - 已验证、未验证、待验证。
token 和 network：已验证此特定代币/网络提现的地址。
walletAddress：已添加到地址簿的钱包地址。
addressQuestionaire：您在验证问卷中回答的详细信息。
上一页
VASP List
下一页
Broker Withdraw
接口描述
HTTP 请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
