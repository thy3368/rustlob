# 用户万向划转

**来源**: https://developers.binance.com/docs/zh-CN/wallet/asset/user-universal-transfer

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
资产用户万向划转(USER_DATA)
用户万向划转(USER_DATA)
接口描述​

用户万向划转

HTTP请求​

POST /sapi/v1/asset/transfer

您需要开通api key 允许万向划转权限来调用此接口。

请求权重(UID)​

900

请求参数​
名称	类型	是否必需	描述
type	ENUM	YES	
asset	STRING	YES	
amount	DECIMAL	YES	
fromSymbol	STRING	NO	
toSymbol	STRING	NO	
recvWindow	LONG	NO	
timestamp	LONG	YES	

fromSymbol 必须要发送，当类型为 ISOLATEDMARGIN_MARGIN 和 ISOLATEDMARGIN_ISOLATEDMARGIN

toSymbol 必须要发送，当类型为 MARGIN_ISOLATEDMARGIN 和 ISOLATEDMARGIN_ISOLATEDMARGIN

目前支持的type划转类型:

MAIN_UMFUTURE 现货钱包转向U本位合约钱包
MAIN_CMFUTURE 现货钱包转向币本位合约钱包
MAIN_MARGIN 现货钱包转向杠杆全仓钱包
UMFUTURE_MAIN U本位合约钱包转向现货钱包
UMFUTURE_MARGIN U本位合约钱包转向杠杆全仓钱包
CMFUTURE_MAIN 币本位合约钱包转向现货钱包
MARGIN_MAIN 杠杆全仓钱包转向现货钱包
MARGIN_UMFUTURE 杠杆全仓钱包转向U本位合约钱包
MARGIN_CMFUTURE 杠杆全仓钱包转向币本位合约钱包
CMFUTURE_MARGIN 币本位合约钱包转向杠杆全仓钱包
ISOLATEDMARGIN_MARGIN 杠杆逐仓钱包转向杠杆全仓钱包
MARGIN_ISOLATEDMARGIN 杠杆全仓钱包转向杠杆逐仓钱包
ISOLATEDMARGIN_ISOLATEDMARGIN 杠杆逐仓钱包转向杠杆逐仓钱包
MAIN_FUNDING 现货钱包转向资金钱包
FUNDING_MAIN 资金钱包转向现货钱包
FUNDING_UMFUTURE 资金钱包转向U本位合约钱包
UMFUTURE_FUNDING U本位合约钱包转向资金钱包
MARGIN_FUNDING 杠杆全仓钱包转向资金钱包
FUNDING_MARGIN 资金钱包转向杠杆全仓钱包
FUNDING_CMFUTURE 资金钱包转向币本位合约钱包
CMFUTURE_FUNDING 币本位合约钱包转向资金钱包
MAIN_OPTION 现货钱包转向期权钱包
OPTION_MAIN 期权钱包转向现货钱包
UMFUTURE_OPTION U本位合约钱包转向期权钱包
OPTION_UMFUTURE 期权钱包转向U本位合约钱包
MARGIN_OPTION 杠杆全仓钱包转向期权钱包
OPTION_MARGIN 期权全仓钱包转向杠杆钱包
FUNDING_OPTION 资金钱包转向期权钱包
OPTION_FUNDING 期权钱包转向资金钱包
MAIN_PORTFOLIO_MARGIN 现货钱包转向统一账户钱包
PORTFOLIO_MARGIN_MAIN 统一账户钱包转向现货钱包
MAIN_ISOLATED_MARGIN 现货钱包转向逐仓账户钱包
ISOLATED_MARGIN_MAIN 逐仓钱包转向现货账户钱包
响应示例​
{
    "tranId":13526853623
}

上一页
用户持仓(USER_DATA)
下一页
查询用户万向划转历史(USER_DATA)
接口描述
HTTP请求
请求权重(UID)
请求参数
响应示例
Copyright © 2026 Binance.
