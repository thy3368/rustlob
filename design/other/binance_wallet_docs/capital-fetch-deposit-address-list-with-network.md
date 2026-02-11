# 查询充值地址列表

**来源**: https://developers.binance.com/docs/zh-CN/wallet/capital/fetch-deposit-address-list-with-network

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
钱包查询充值地址列表(USER_DATA)
查询充值地址列表(USER_DATA)
接口描述​

根据网络币种或币种获取充值地址列表

HTTP请求​

GET /sapi/v1/capital/deposit/address/list

请求权重(IP)​

10

请求参数​
名称	类型	是否必需	描述
coin	STRING	YES	coin是网络的地址空间名称
network	STRING	NO	网络
timestamp	LONG	YES	时间戳
如果没传网络，会返回网络对应的默认网络。
可以通过后面的接口，来获取网络和 isDefault 字段，在返回的响应里Get /sapi/v1/capital/config/getall.
响应示例​
[
  {
    "coin": "ETH",  //这里 coin 实际上指 network 的地址空间, 类 ETH 网络都使用 ETH 的地址
    "address": "0xD316E95Fd9E8E237Cb11f8200Babbc5D8D177BA4",
    "tag":"",
    "isDefault": 0
  }, 
  {
    "coin": "ETH",
    "address": "0xD316E95Fd9E8E237Cb11f8200Babbc5D8D177BA4",
    "tag":"",
    "isDefault": 0
  }, 
  {
    "coin": "ETH",
    "address": "0x00003ada75e7da97ba0db2fcde72131f712455e2",
    "tag":"",
    "isDefault": 1  
  }
]

上一页
获取充值地址(支持多网络)(USER_DATA)
下一页
一键上账(充值到过期地址)(USER_DATA)
接口描述
HTTP请求
请求权重(IP)
请求参数
响应示例
Copyright © 2026 Binance.
