# 用户持仓

**来源**: https://developers.binance.com/docs/zh-CN/wallet/asset/user-assets

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
上架资产详情(USER_DATA)
查询用户钱包余额(USER_DATA)
用户持仓(USER_DATA)
用户万向划转(USER_DATA)
查询用户万向划转历史(USER_DATA)
现货交易和杠杆利息BNB抵扣开关(USER_DATA)
获取可以转换成BNB的小额资产(USER_DATA)
小额资产转换(USER_DATA)
小额资产转换BNB历史(USER_DATA)
资产利息记录(USER_DATA)
交易手续费率查询(USER_DATA)
查询资金账户(USER_DATA)
云算力历史记录分页查询(USER_DATA)
查询用户委托资金历史(适用主账户)(USER_DATA)
查询现货下架计划 (MARKET_DATA)
查询即将开放币对列表 (MARKET_DATA)
小额资产兑换(USER_DATA)
小额可以兑换的资产(USER_DATA)
账户
旅行规则
其他
错误代码
联系我们
资产用户持仓(USER_DATA)
用户持仓(USER_DATA)
接口描述​

获取用户持仓，仅返回>0的数据。

HTTP请求​

POST /sapi/v3/asset/getUserAsset

请求权重(IP)​

5

请求参数​
名称	类型	是否必需	描述
asset	STRING	NO	如果资产为空，则查询用户所有的正资产。
needBtcValuation	BOOLEAN	NO	是否需要返回兑换成BTC的估值
recvWindow	LONG	NO	
timestamp	LONG	YES	
响应示例​
[
  {
    "asset": "AVAX",
    "free": "1",
    "locked": "0",
    "freeze": "0",
    "withdrawing": "0",
    "ipoable": "0",
    "btcValuation": "0"
  },
  {
    "asset": "BCH",
    "free": "0.9",
    "locked": "0",
    "freeze": "0",
    "withdrawing": "0",
    "ipoable": "0",
    "btcValuation": "0"
  },
  {
    "asset": "BNB",
    "free": "887.47061626",
    "locked": "0",
    "freeze": "10.52",
    "withdrawing": "0.1",
    "ipoable": "0",
    "btcValuation": "0"
  },
  {
    "asset": "BUSD",
    "free": "9999.7",
    "locked": "0",
    "freeze": "0",
    "withdrawing": "0",
    "ipoable": "0",
    "btcValuation": "0"
  },
  {
    "asset": "SHIB",
    "free": "532.32",
    "locked": "0",
    "freeze": "0",
    "withdrawing": "0",
    "ipoable": "0",
    "btcValuation": "0"
  },
  {
    "asset": "USDT",
    "free": "50300000001.44911105",
    "locked": "0",
    "freeze": "0",
    "withdrawing": "0",
    "ipoable": "0",
    "btcValuation": "0"
  },
  {
    "asset": "WRZ",
    "free": "1",
    "locked": "0",
    "freeze": "0",
    "withdrawing": "0",
    "ipoable": "0",
    "btcValuation": "0"
  }
]

上一页
查询用户钱包余额(USER_DATA)
下一页
用户万向划转(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
