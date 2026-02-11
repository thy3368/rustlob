# 提币

**来源**: https://developers.binance.com/docs/zh-CN/wallet/capital/withdraw

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
钱包提币(USER_DATA)
提币(USER_DATA)
接口描述​

提币

HTTP请求​

POST /sapi/v1/capital/withdraw/apply

请求权重(UID)​

900

请求参数​
名称	类型	是否必需	描述
coin	STRING	YES	
withdrawOrderId	STRING	NO	用户自定义提币ID, 后续可以在GET /sapi/v1/capital/withdraw/history 中使用这个字段进行查询
network	STRING	NO	提币网络
address	STRING	YES	提币地址
addressTag	STRING	NO	某些币种例如 XRP,XMR 允许填写次级地址标签
amount	DECIMAL	YES	数量
transactionFeeFlag	BOOLEAN	NO	当站内转账时免手续费, true: 手续费归资金转入方; false: 手续费归资金转出方; . 默认 false.
name	STRING	NO	地址的备注，填写该参数后会加入该币种的提现地址簿。地址簿上限为200，超出后会造成提现失败。地址中的空格需要encode成%20
walletType	INTEGER	NO	表示出金使用的钱包，0为现货钱包，1为资金钱包。默认walletType为"充币账户"是您设置在钱包->现货账户或资金账户->充值。
recvWindow	LONG	NO	
timestamp	LONG	YES	
如果network未发送，则返回该币种的默认网络。
您可以在Get /sapi/v1/capital/config/getall (HMAC SHA256)的响应中，获取某个币种的networkList中的network和isDefault。
要检查是否需要遵守旅行规则，可以使用接口 GET /sapi/v1/localentity/questionnaire-requirements，如果返回结果不是 NIL，则需要使用更新后的 SAPI 接口 POST /sapi/v1/localentity/withdraw/apply；否则，可以继续使用 POST /sapi/v1/capital/withdraw/apply。请注意，如果需要遵守旅行规则，请参考旅行规则相关的 SAPI 文档。
响应示例​
{
    "id":"7213fea8e94b4a5593d507237e5a555b"
}

上一页
获取所有币信息(USER_DATA)
下一页
获取提币历史(支持多网络)(USER_DATA)
接口描述
HTTP请求
请求权重(UID)
请求参数
响应示例
Copyright © 2026 Binance.
