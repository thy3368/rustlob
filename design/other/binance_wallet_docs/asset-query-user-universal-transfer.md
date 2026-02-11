# 查询用户万向划转历史

**来源**: https://developers.binance.com/docs/zh-CN/wallet/asset/query-user-universal-transfer

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
资产查询用户万向划转历史(USER_DATA)
查询用户万向划转历史(USER_DATA)
接口描述​

查询用户万向划转历史

HTTP请求​

GET /sapi/v1/asset/transfer

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
type	ENUM	YES	
startTime	LONG	NO	
endTime	LONG	NO	
current	INT	NO	默认 1
size	INT	NO	默认 10, 最大 100
fromSymbol	STRING	NO	
toSymbol	STRING	NO	
recvWindow	LONG	NO	
timestamp	LONG	YES	
fromSymbol 必须要发送，当类型为 ISOLATEDMARGIN_MARGIN 和 ISOLATEDMARGIN_ISOLATEDMARGIN
toSymbol 必须要发送，当类型为 MARGIN_ISOLATEDMARGIN 和 ISOLATEDMARGIN_ISOLATEDMARGIN
仅支持查询最近半年（6个月）数据
若startTime和endTime没传，则默认返回最近7天数据
响应示例​
{
    "total":2,
    "rows":[
        {
            "asset":"USDT",
            "amount":"1",
            "type":"MAIN_UMFUTURE"
            "status": "CONFIRMED", // status: CONFIRMED / FAILED / PENDING
            "tranId": 11415955596,
            "timestamp":1544433328000
        },
        {
            "asset":"USDT",
            "amount":"2",
            "type":"MAIN_UMFUTURE",
            "status": "CONFIRMED",
            "tranId": 11366865406,
            "timestamp":1544433328000
        }
    ]
}

上一页
用户万向划转(USER_DATA)
下一页
现货交易和杠杆利息BNB抵扣开关(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
