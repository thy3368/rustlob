# 查询提现地址簿

**来源**: https://developers.binance.com/docs/zh-CN/wallet/capital/fetch-withdraw-address

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
钱包查询提现地址簿
查询提现地址簿
接口描述​

查询提现地址簿

HTTP请求​

GET /sapi/v1/capital/withdraw/address/list

请求权重(IP)​

10

请求参数​

NONE

响应示例​
[
    {
        "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
        "addressTag": "",
        "coin": "BTC",
        "name": "Satoshi",        //用户自定义
        "network": "BTC",
        "origin": "bla",      //如果 originType != 'others'，则该值为空，否则 origin 由用户手动填写
        "originType": "others",  //地址来源类型，包括但不限于: 交易所地址类型: Binance, CoinBase, HTX, Bitfinex, OKX, Bithumb, Kraken, Kucoin, Gemini, Bitget, Bybit, Upbit, Gate.io; 个人钱包类型: Binance Web3 Wallet, Trust Wallet, MetaMask, Rabby Wallet, Phantom, OKX Web 3 Wallet, Coinbase Wallet, Bitget Wallet; 其他 Others 类型: others(支持多语言)
        "whiteStatus": true      //是否是白名单
    }
]

上一页
获取提币历史(支持多网络)(USER_DATA)
下一页
获取用户提现额度
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
