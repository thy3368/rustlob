# 查询每日资产快照

**来源**: https://developers.binance.com/docs/zh-CN/wallet/account/daily-account-snapshoot

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
帐户信息
查询每日资产快照(USER_DATA)
关闭站内划转(USER_DATA)
开启站内划转(USER_DATA)
账户状态(USER_DATA)
账户API交易状态(USER_DATA)
查询用户API Key权限(USER_DATA)
旅行规则
其他
错误代码
联系我们
账户查询每日资产快照(USER_DATA)
查询每日资产快照(USER_DATA)
接口描述​

查询每日资产快照

HTTP请求​

GET /sapi/v1/accountSnapshot

请求权重(IP)​

2400

请求参数​
名称	类型	是否必需	描述
type	STRING	YES	"SPOT", "MARGIN", "FUTURES"
startTime	LONG	NO	
endTime	LONG	NO	
limit	INT	NO	min 7, max 30, default 7
recvWindow	LONG	NO	
timestamp	LONG	YES	
响应示例​
{
   "code":200, // 200表示返回正确，否则即为错误码
   "msg":"", // 与错误码对应的报错信息
   "snapshotVos":[
      {
         "data":{
            "balances":[
               {
                  "asset":"BTC",
                  "free":"0.09905021",
                  "locked":"0.00000000"
               },
               {
                  "asset":"USDT",
                  "free":"1.89109409",
                  "locked":"0.00000000"
               }
            ],
            "totalAssetOfBtc":"0.09942700"
         },
         "type":"spot",
         "updateTime":1576281599000
      }
   ]
}



或

{
   "code":200, // 200表示返回正确，否则即为错误码
   "msg":"", // 与错误码对应的报错信息
   "snapshotVos":[
      {
         "data":{
            "marginLevel":"2748.02909813",
            "totalAssetOfBtc":"0.00274803",
            "totalLiabilityOfBtc":"0.00000100",
            "totalNetAssetOfBtc":"0.00274750",
            "userAssets":[
               {
                  "asset":"XRP",
                  "borrowed":"0.00000000",
                  "free":"1.00000000",
                  "interest":"0.00000000",
                  "locked":"0.00000000",
                  "netAsset":"1.00000000"
               }
            ]
         },
         "type":"margin",
         "updateTime":1576281599000
      }
   ]
}


或

{
   "code":200, // 200表示返回正确，否则即为错误码
   "msg":"", // 与错误码对应的报错信息
   "snapshotVos":[
      {
         "data":{
            "assets":[
               {
                  "asset":"USDT",
                  "marginBalance":"118.99782335", // 不会实时更新，可以忽略
                  "walletBalance":"120.23811389"
               }
            ],
            "position":[
               {
                  "entryPrice":"7130.41000000",
                  "markPrice":"7257.66239673",
                  "positionAmt":"0.01000000",
                  "symbol":"BTCUSDT",
                  "unRealizedProfit":"1.24029054" // 只显示开仓当时的未实现盈亏，不会实时更新，可以忽略
               }
            ]
         },
         "type":"futures",
         "updateTime":1576281599000
      }
   ]
}

上一页
帐户信息
下一页
关闭站内划转(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
