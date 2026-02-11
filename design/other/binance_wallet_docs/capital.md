# 获取所有币信息

**来源**: https://developers.binance.com/docs/zh-CN/wallet/capital

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
钱包获取所有币信息(USER_DATA)
获取所有币信息(USER_DATA)
接口描述​

获取针对用户的所有(Binance支持充提操作的)币种信息。

HTTP请求​

GET /sapi/v1/capital/config/getall

请求权重(IP)​

10

请求参数​
名称	类型	是否必需	描述
recvWindow	LONG	NO	
timestamp	LONG	YES	
响应示例​
[
    {
        "coin": "1MBABYDOGE",
        "depositAllEnable": true,
        "withdrawAllEnable": true,
        "name": "1M x BABYDOGE",
        "free": "34941.1",
        "locked": "0",
        "freeze": "0",
        "withdrawing": "0",
        "ipoing": "0",
        "ipoable": "0",
        "storage": "0",
        "isLegalMoney": false,
        "trading": true,
        "networkList": [
            {
                "network": "BSC",
                "coin": "1MBABYDOGE",
                "withdrawIntegerMultiple": "0.01",
                "isDefault": false,
                "depositEnable": true,
                "withdrawEnable": true,
                "depositDesc": "",   // 仅在充值关闭时返回
                "withdrawDesc": "",  // 仅在提现关闭时返回
                "specialTips": "",
                "specialWithdrawTips": "",
                "name": "BNB Smart Chain (BEP20)",
                "resetAddressStatus": false,
                "addressRegex": "^(0x)[0-9A-Fa-f]{40}$",
                "memoRegex": "",
                "withdrawFee": "10",
                "withdrawMin": "20",
                "withdrawMax": "9999999999",
                "withdrawInternalMin": "0.01",  // 内部转账最小提现数
                "depositDust": "0.01",
                "minConfirm": 5,  // 上账所需的最小确认数
                "unLockConfirm": 0,  // 解锁需要的确认数 
                "sameAddress": false, // 过时字段，建议使用withdrawTag
                "withdrawTag": false, // 提现时是否需要memo
                "estimatedArrivalTime": 1,
                "busy": false,
                "contractAddressUrl": "https://bscscan.com/token/",
                "contractAddress": "0xc748673057861a797275cd8a068abb95a902e8de",
                "denomination": 1000000  // 1 1MBABYDOGE = 1000000 BABYDOGE
            },
            {
                "network": "ETH",
                "coin": "1MBABYDOGE",
                "withdrawIntegerMultiple": "0.01",
                "isDefault": true,
                "depositEnable": true,
                "withdrawEnable": true,
                "depositDesc": "",
                "withdrawDesc": "",
                "specialTips": "",
                "specialWithdrawTips": "",
                "name": "Ethereum (ERC20)",
                "resetAddressStatus": false,
                "addressRegex": "^(0x)[0-9A-Fa-f]{40}$",
                "memoRegex": "",
                "withdrawFee": "1511",
                "withdrawMin": "3022",
                "withdrawMax": "9999999999",
                "withdrawInternalMin": "0.01",  
                "depositDust": "0.01",
                "minConfirm": 6,
                "unLockConfirm": 64,
                "sameAddress": false,
                "withdrawTag": false,
                "estimatedArrivalTime": 2,
                "busy": false,
                "contractAddressUrl": "https://etherscan.io/address/",
                "contractAddress": "0xac57de9c1a09fec648e93eb98875b212db0d460b",
                "denomination": 1000000 
            }
        ]
    }
]

上一页
基本信息
下一页
提币(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
