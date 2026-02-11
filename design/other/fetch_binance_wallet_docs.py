#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import subprocess
import os
import json
import time
from pathlib import Path

OUTPUT_DIR = Path.home() / "binance_wallet_docs"
BASE_URL = "https://developers.binance.com"

# 所有文档链接
DOCS = [
    # 基础文档
    ("change-log", "更新日志"),
    ("quick-start", "快速开始"),
    ("Introduction", "概述"),
    ("general-info", "基本信息"),

    # 钱包部分
    ("capital", "获取所有币信息"),
    ("capital/withdraw", "提币"),
    ("capital/withdraw-history", "获取提币历史"),
    ("capital/fetch-withdraw-address", "查询提现地址簿"),
    ("capital/fetch-withdraw-quota", "获取用户提现额度"),
    ("capital/deposite-history", "获取充值历史"),
    ("capital/deposite-address", "获取充值地址"),
    ("capital/fetch-deposit-address-list-with-network", "查询充值地址列表"),
    ("capital/one-click-arrival-deposite-apply", "一键上账"),

    # 资产部分
    ("asset", "上架资产详情"),
    ("asset/query-user-wallet-balance", "查询用户钱包余额"),
    ("asset/user-assets", "用户持仓"),
    ("asset/user-universal-transfer", "用户万向划转"),
    ("asset/query-user-universal-transfer", "查询用户万向划转历史"),
    ("asset/Toggle-BNB-Burn-On-Spot-Trade-And-Margin-Interest", "BNB抵扣开关"),
    ("asset/assets-can-convert-bnb", "获取可以转换成BNB的小额资产"),
    ("asset/dust-transfer", "小额资产转换"),
    ("asset/dust-log", "小额资产转换BNB历史"),
    ("asset/assets-divided-record", "资产利息记录"),
    ("asset/trade-fee", "交易手续费率查询"),
    ("asset/funding-wallet", "查询资金账户"),
    ("asset/cloud-mining-payment-and-refund-history", "云算力历史记录"),
    ("asset/query-user-delegation", "查询用户委托资金历史"),
    ("asset/spot-delist-schedule", "查询现货下架计划"),
    ("asset/open-symbol-list", "查询即将开放币对列表"),
    ("asset/dust-convert", "小额资产兑换"),
    ("asset/dust-convertible-assets", "小额可以兑换的资产"),

    # 账户部分
    ("account", "帐户信息"),
    ("account/daily-account-snapshoot", "查询每日资产快照"),
    ("account/disable-fast-withdraw-switch", "关闭站内划转"),
    ("account/enable-fast-withdraw-switch", "开启站内划转"),
    ("account/account-status", "账户状态"),
    ("account/account-api-trading-status", "账户API交易状态"),
    ("account/api-key-permission", "查询用户API Key权限"),

    # 旅行规则部分
    ("travel-rule/withdraw", "提币(旅行规则)"),
    ("travel-rule/withdraw-history", "获取提币历史(旅行规则)"),
    ("travel-rule/withdraw-history-v2", "获取提币历史V2(旅行规则)"),
    ("travel-rule/withdraw-questionnaire", "提币问卷内容"),
    ("travel-rule/deposit-provide-info", "提交充值问卷"),
    ("travel-rule/deposit-provide-info-v2", "提交充值问卷V2"),
    ("travel-rule/deposit-history", "获取充值历史(旅行规则)"),
    ("travel-rule/deposit-history-v2", "获取充值历史V2(旅行规则)"),
    ("travel-rule/deposit-questionnaire", "充值问卷内容"),
    ("travel-rule/onboarded-vasp-list", "VASP List"),
    ("travel-rule/address-verification-list", "Address Verification list"),
    ("travel-rule/broker-withdraw", "Broker Withdraw"),
    ("travel-rule/broker-deposit-provide-info", "Submit Broker Deposit Questionnaire"),
    ("travel-rule/questionnaire-requirements", "Check Questionnaire Requirements"),
    ("travel-rule/appendix", "Appendix"),

    # 其他
    ("error-code", "错误代码"),
    ("contact-us", "联系我们"),
]

def sanitize_filename(path):
    """将路径转换为安全的文件名"""
    return path.replace("/", "-")

def fetch_document(doc_path, doc_title):
    """获取单个文档的内容"""
    url = f"{BASE_URL}/docs/zh-CN/wallet/{doc_path}"
    filename = sanitize_filename(doc_path) + ".md"
    filepath = OUTPUT_DIR / filename

    print(f"正在获取: {doc_title} ({doc_path})...")

    try:
        # 打开浏览器
        subprocess.run(["actionbook", "browser", "open", url], check=True, capture_output=True)
        time.sleep(2)  # 等待页面加载

        # 获取页面文本
        result = subprocess.run(["actionbook", "browser", "text"], capture_output=True, text=True, check=True)
        content = result.stdout

        # 关闭浏览器
        subprocess.run(["actionbook", "browser", "close"], capture_output=True)

        # 保存为 markdown 文件
        with open(filepath, "w", encoding="utf-8") as f:
            f.write(f"# {doc_title}\n\n")
            f.write(f"**来源**: {url}\n\n")
            f.write("---\n\n")
            f.write(content)

        print(f"✓ 已保存: {filename}")
        return True

    except Exception as e:
        print(f"✗ 获取失败: {doc_title} - {str(e)}")
        return False

def main():
    # 创建输出目录
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    print(f"币安钱包文档爬取工具")
    print(f"输出目录: {OUTPUT_DIR}")
    print(f"总共需要获取 {len(DOCS)} 个文档\n")

    success_count = 0
    failed_count = 0

    for doc_path, doc_title in DOCS:
        if fetch_document(doc_path, doc_title):
            success_count += 1
        else:
            failed_count += 1
        time.sleep(1)  # 避免请求过快

    print(f"\n完成!")
    print(f"成功: {success_count}")
    print(f"失败: {failed_count}")
    print(f"总计: {len(DOCS)}")

if __name__ == "__main__":
    main()
