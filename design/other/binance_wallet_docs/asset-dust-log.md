# 小额资产转换BNB历史

**来源**: https://developers.binance.com/docs/zh-CN/wallet/asset/dust-log

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
资产小额资产转换BNB历史(USER_DATA)
小额资产转换BNB历史(USER_DATA)
接口描述​

小额资产转换BNB历史(USER_DATA)

HTTP请求​

GET /sapi/v1/asset/dribblet

请求权重(IP)​

1

请求参数​
名称	类型	是否必需	描述
accountType	STRING	NO	SPOT或MARGIN,默认SPOT
startTime	LONG	NO	
endTime	LONG	NO	
recvWindow	LONG	NO	
timestamp	LONG	YES	
只返回最近100条记录
只返回 2020/12/01 之后记录
响应示例​
{
        "total": 8,   //共计发生过的转换笔数
        "userAssetDribblets": [
            {
                "operateTime": 1615985535000,
                "totalTransferedAmount": "0.00132256",   //本次转换所得BNB
                "totalServiceChargeAmount": "0.00002699",   //本次转换手续费(BNB)
                "transId": 45178372831,
                "userAssetDribbletDetails": [           //本次转换的细节
                    {
                        "transId": 4359321,
                        "serviceChargeAmount": "0.000009",
                        "amount": "0.0009",
                        "operateTime": 1615985535000,
                        "transferedAmount": "0.000441",
                        "fromAsset": "USDT"
                    },
                    {
                        "transId": 4359321,
                        "serviceChargeAmount": "0.00001799",
                        "amount": "0.0009",
                        "operateTime": 1615985535000,
                        "transferedAmount": "0.00088156",
                        "fromAsset": "ETH"
                    }
                ]
            },
            {
                "operateTime":1616203180000,
                "totalTransferedAmount": "0.00058795",
                "totalServiceChargeAmount": "0.000012",
                "transId": 4357015,
                "userAssetDribbletDetails": [       
                    {
                        "transId": 4357015,
                        "serviceChargeAmount": "0.00001"
                        "amount": "0.001",
                        "operateTime": 1616203180000,
                        "transferedAmount": "0.00049",
                        "fromAsset": "USDT"
                    },
                    {
                        "transId": 4357015,
                        "serviceChargeAmount": "0.000002"         
                        "amount": "0.0001",
                        "operateTime": 1616203180000,
                        "transferedAmount": "0.00009795",
                        "fromAsset": "ETH"
                    }
                ]
            }
        ]
}

上一页
小额资产转换(USER_DATA)
下一页
资产利息记录(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
