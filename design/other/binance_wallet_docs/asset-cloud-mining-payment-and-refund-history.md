# 云算力历史记录

**来源**: https://developers.binance.com/docs/zh-CN/wallet/asset/cloud-mining-payment-and-refund-history

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
资产云算力历史记录分页查询(USER_DATA)
云算力历史记录分页查询(USER_DATA)
接口描述​

云算力支付和退款历史分页查询

HTTP请求​

GET /sapi/v1/asset/ledger-transfer/cloud-mining/queryByPage

请求权重(UID)​

600

请求参数​
名称	类型	是否必需	描述
tranId	LONG	NO	流水号
clientTranId	STRING	NO	外部唯一流水号
asset	STRING	NO	不传或者空字符串查全部
startTime	LONG	YES	开始时间（包含），单位：毫秒
endTime	LONG	YES	结束时间（不包含），单位：毫秒
current	INTEGER	NO	当前页面，默认1，最小值为1
size	INTEGER	NO	页面大小，默认10，最大值为100
响应示例​
{
  "total":5,
  "rows":[
    {"createTime":1667880112000,"tranId":121230610120,"type":248,"asset":"USDT","amount":"25.0068","status":"S"},
    {"createTime":1666776366000,"tranId":119991507468,"type":249,"asset":"USDT","amount":"0.027","status":"S"},
    {"createTime":1666764505000,"tranId":119977966327,"type":248,"asset":"USDT","amount":"0.027","status":"S"},
    {"createTime":1666758189000,"tranId":119973601721,"type":248,"asset":"USDT","amount":"0.018","status":"S"},
    {"createTime":1666757278000,"tranId":119973028551,"type":248,"asset":"USDT","amount":"0.018","status":"S"}
  ]
}

上一页
查询资金账户(USER_DATA)
下一页
查询用户委托资金历史(适用主账户)(USER_DATA)
接口描述
HTTP请求
请求权重(UID)
请求参数
响应示例
Copyright © 2026 Binance.
