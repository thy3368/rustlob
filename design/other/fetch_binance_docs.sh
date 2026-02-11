#!/bin/bash

# 币安钱包文档爬取脚本
OUTPUT_DIR="$HOME/binance_wallet_docs"
BASE_URL="https://developers.binance.com"

# 所有文档链接
declare -a DOCS=(
    # 基础文档
    "change-log:更新日志"
    "quick-start:快速开始"
    "Introduction:概述"
    "general-info:基本信息"

    # 钱包部分
    "capital:获取所有币信息"
    "capital/withdraw:提币"
    "capital/withdraw-history:获取提币历史"
    "capital/fetch-withdraw-address:查询提现地址簿"
    "capital/fetch-withdraw-quota:获取用户提现额度"
    "capital/deposite-history:获取充值历史"
    "capital/deposite-address:获取充值地址"
    "capital/fetch-deposit-address-list-with-network:查询充值地址列表"
    "capital/one-click-arrival-deposite-apply:一键上账"

    # 资产部分
    "asset:上架资产详情"
    "asset/query-user-wallet-balance:查询用户钱包余额"
    "asset/user-assets:用户持仓"
    "asset/user-universal-transfer:用户万向划转"
    "asset/query-user-universal-transfer:查询用户万向划转历史"
    "asset/Toggle-BNB-Burn-On-Spot-Trade-And-Margin-Interest:BNB抵扣开关"
    "asset/assets-can-convert-bnb:获取可以转换成BNB的小额资产"
    "asset/dust-transfer:小额资产转换"
    "asset/dust-log:小额资产转换BNB历史"
    "asset/assets-divided-record:资产利息记录"
    "asset/trade-fee:交易手续费率查询"
    "asset/funding-wallet:查询资金账户"
    "asset/cloud-mining-payment-and-refund-history:云算力历史记录"
    "asset/query-user-delegation:查询用户委托资金历史"
    "asset/spot-delist-schedule:查询现货下架计划"
    "asset/open-symbol-list:查询即将开放币对列表"
    "asset/dust-convert:小额资产兑换"
    "asset/dust-convertible-assets:小额可以兑换的资产"

    # 账户部分
    "account:帐户信息"
    "account/daily-account-snapshoot:查询每日资产快照"
    "account/disable-fast-withdraw-switch:关闭站内划转"
    "account/enable-fast-withdraw-switch:开启站内划转"
    "account/account-status:账户状态"
    "account/account-api-trading-status:账户API交易状态"
    "account/api-key-permission:查询用户API Key权限"

    # 旅行规则部分
    "travel-rule/withdraw:提币(旅行规则)"
    "travel-rule/withdraw-history:获取提币历史(旅行规则)"
    "travel-rule/withdraw-history-v2:获取提币历史V2(旅行规则)"
    "travel-rule/withdraw-questionnaire:提币问卷内容"
    "travel-rule/deposit-provide-info:提交充值问卷"
    "travel-rule/deposit-provide-info-v2:提交充值问卷V2"
    "travel-rule/deposit-history:获取充值历史(旅行规则)"
    "travel-rule/deposit-history-v2:获取充值历史V2(旅行规则)"
    "travel-rule/deposit-questionnaire:充值问卷内容"
    "travel-rule/onboarded-vasp-list:VASP List"
    "travel-rule/address-verification-list:Address Verification list"
    "travel-rule/broker-withdraw:Broker Withdraw"
    "travel-rule/broker-deposit-provide-info:Submit Broker Deposit Questionnaire"
    "travel-rule/questionnaire-requirements:Check Questionnaire Requirements"
    "travel-rule/appendix:Appendix"

    # 其他
    "error-code:错误代码"
    "contact-us:联系我们"
)

echo "开始获取币安钱包文档..."
echo "输出目录: $OUTPUT_DIR"

# 创建索引文件
INDEX_FILE="$OUTPUT_DIR/README.md"
cat > "$INDEX_FILE" << 'EOF'
# 币安钱包 API 文档

本文档是从 https://developers.binance.com/docs/zh-CN/wallet/Introduction 获取的完整中文文档。

## 目录

### 基础文档
- [更新日志](./change-log.md)
- [快速开始](./quick-start.md)
- [概述](./Introduction.md)
- [基本信息](./general-info.md)

### 钱包 API
- [获取所有币信息](./capital.md)
- [提币](./capital-withdraw.md)
- [获取提币历史](./capital-withdraw-history.md)
- [查询提现地址簿](./capital-fetch-withdraw-address.md)
- [获取用户提现额度](./capital-fetch-withdraw-quota.md)
- [获取充值历史](./capital-deposite-history.md)
- [获取充值地址](./capital-deposite-address.md)
- [查询充值地址列表](./capital-fetch-deposit-address-list-with-network.md)
- [一键上账](./capital-one-click-arrival-deposite-apply.md)

### 资产 API
- [上架资产详情](./asset.md)
- [查询用户钱包余额](./asset-query-user-wallet-balance.md)
- [用户持仓](./asset-user-assets.md)
- [用户万向划转](./asset-user-universal-transfer.md)
- [查询用户万向划转历史](./asset-query-user-universal-transfer.md)
- [BNB抵扣开关](./asset-Toggle-BNB-Burn-On-Spot-Trade-And-Margin-Interest.md)
- [获取可以转换成BNB的小额资产](./asset-assets-can-convert-bnb.md)
- [小额资产转换](./asset-dust-transfer.md)
- [小额资产转换BNB历史](./asset-dust-log.md)
- [资产利息记录](./asset-assets-divided-record.md)
- [交易手续费率查询](./asset-trade-fee.md)
- [查询资金账户](./asset-funding-wallet.md)
- [云算力历史记录](./asset-cloud-mining-payment-and-refund-history.md)
- [查询用户委托资金历史](./asset-query-user-delegation.md)
- [查询现货下架计划](./asset-spot-delist-schedule.md)
- [查询即将开放币对列表](./asset-open-symbol-list.md)
- [小额资产兑换](./asset-dust-convert.md)
- [小额可以兑换的资产](./asset-dust-convertible-assets.md)

### 账户 API
- [帐户信息](./account.md)
- [查询每日资产快照](./account-daily-account-snapshoot.md)
- [关闭站内划转](./account-disable-fast-withdraw-switch.md)
- [开启站内划转](./account-enable-fast-withdraw-switch.md)
- [账户状态](./account-account-status.md)
- [账户API交易状态](./account-account-api-trading-status.md)
- [查询用户API Key权限](./account-api-key-permission.md)

### 旅行规则 API
- [提币(旅行规则)](./travel-rule-withdraw.md)
- [获取提币历史(旅行规则)](./travel-rule-withdraw-history.md)
- [获取提币历史V2(旅行规则)](./travel-rule-withdraw-history-v2.md)
- [提币问卷内容](./travel-rule-withdraw-questionnaire.md)
- [提交充值问卷](./travel-rule-deposit-provide-info.md)
- [提交充值问卷V2](./travel-rule-deposit-provide-info-v2.md)
- [获取充值历史(旅行规则)](./travel-rule-deposit-history.md)
- [获取充值历史V2(旅行规则)](./travel-rule-deposit-history-v2.md)
- [充值问卷内容](./travel-rule-deposit-questionnaire.md)
- [VASP List](./travel-rule-onboarded-vasp-list.md)
- [Address Verification list](./travel-rule-address-verification-list.md)
- [Broker Withdraw](./travel-rule-broker-withdraw.md)
- [Submit Broker Deposit Questionnaire](./travel-rule-broker-deposit-provide-info.md)
- [Check Questionnaire Requirements](./travel-rule-questionnaire-requirements.md)
- [Appendix](./travel-rule-appendix.md)

### 其他
- [错误代码](./error-code.md)
- [联系我们](./contact-us.md)

---

生成时间: $(date)
来源: https://developers.binance.com/docs/zh-CN/wallet/Introduction
EOF

echo "索引文件已创建: $INDEX_FILE"
echo ""
echo "现在开始获取各个文档页面..."
