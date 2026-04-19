#!/usr/bin/env python3
"""
期货交易流程可视化工具

展示三个核心业务流程:
1. 正常交易: set_leverage → open_position → close_position
2. 强平流程: open_position → liquidation (三级机制)
3. 杠杆影响: 不同杠杆倍数的风险对比
"""

import matplotlib.pyplot as plt
import matplotlib.patches as patches
import numpy as np

# 设置中文字体
plt.rcParams['font.sans-serif'] = ['Arial Unicode MS', 'SimHei', 'DejaVu Sans']
plt.rcParams['axes.unicode_minus'] = False

def plot_normal_trading_flow():
    """绘制正常交易流程"""
    fig, ax = plt.subplots(figsize=(14, 10))
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 10)
    ax.axis('off')

    # 标题
    ax.text(5, 9.5, '正常交易流程', ha='center', fontsize=20, weight='bold')
    ax.text(5, 9, 'set_leverage → open_position → close_position',
            ha='center', fontsize=12, style='italic', color='gray')

    # Step 1: 初始化
    rect1 = patches.FancyBboxPatch((0.5, 7), 2, 1,
                                   boxstyle="round,pad=0.1",
                                   edgecolor='blue', facecolor='lightblue')
    ax.add_patch(rect1)
    ax.text(1.5, 7.5, 'Step 1\n初始化账户\n余额: 10,000 USDT',
            ha='center', va='center', fontsize=10)

    # Arrow 1→2
    ax.annotate('', xy=(3, 7.5), xytext=(2.5, 7.5),
                arrowprops=dict(arrowstyle='->', lw=2, color='green'))

    # Step 2: 设置杠杆
    rect2 = patches.FancyBboxPatch((3, 7), 2, 1,
                                   boxstyle="round,pad=0.1",
                                   edgecolor='green', facecolor='lightgreen')
    ax.add_patch(rect2)
    ax.text(4, 7.5, 'Step 2\n设置杠杆\n10x',
            ha='center', va='center', fontsize=10, weight='bold')

    # Arrow 2→3
    ax.annotate('', xy=(5.5, 7.5), xytext=(5, 7.5),
                arrowprops=dict(arrowstyle='->', lw=2, color='orange'))

    # Step 3: 开仓
    rect3 = patches.FancyBboxPatch((5.5, 7), 2, 1,
                                   boxstyle="round,pad=0.1",
                                   edgecolor='orange', facecolor='lightyellow')
    ax.add_patch(rect3)
    ax.text(6.5, 7.5, 'Step 3\n开多仓\n1 BTC @ 50,000',
            ha='center', va='center', fontsize=10)

    # Arrow 3→4 (向下)
    ax.annotate('', xy=(6.5, 6.5), xytext=(6.5, 7),
                arrowprops=dict(arrowstyle='->', lw=2, color='purple'))

    # Step 4: 持仓信息
    rect4 = patches.FancyBboxPatch((5, 5), 3, 1.2,
                                   boxstyle="round,pad=0.1",
                                   edgecolor='purple', facecolor='lavender')
    ax.add_patch(rect4)
    ax.text(6.5, 5.9, 'Step 4: 持仓创建', ha='center', fontsize=10, weight='bold')
    ax.text(6.5, 5.6, '保证金: 5,000 USDT', ha='center', fontsize=9)
    ax.text(6.5, 5.3, '强平价: 45,500 USDT', ha='center', fontsize=9, color='red')

    # Arrow 4→5
    ax.annotate('', xy=(6.5, 4.5), xytext=(6.5, 5),
                arrowprops=dict(arrowstyle='->', lw=2, color='blue'))

    # Step 5: 价格变化
    rect5 = patches.FancyBboxPatch((4.5, 3.5), 4, 0.8,
                                   boxstyle="round,pad=0.1",
                                   edgecolor='blue', facecolor='lightcyan')
    ax.add_patch(rect5)
    ax.text(6.5, 4.1, 'Step 5: 价格上涨至 55,000 USDT',
            ha='center', fontsize=10)
    ax.text(6.5, 3.7, '未实现盈亏: +5,000 USDT (+10%)',
            ha='center', fontsize=9, color='green', weight='bold')

    # Arrow 5→6
    ax.annotate('', xy=(6.5, 2.8), xytext=(6.5, 3.5),
                arrowprops=dict(arrowstyle='->', lw=2, color='green'))

    # Step 6: 平仓
    rect6 = patches.FancyBboxPatch((5, 1.5), 3, 1.2,
                                   boxstyle="round,pad=0.1",
                                   edgecolor='green', facecolor='lightgreen')
    ax.add_patch(rect6)
    ax.text(6.5, 2.4, 'Step 6: 主动平仓', ha='center', fontsize=10, weight='bold')
    ax.text(6.5, 2.1, '平仓价: 55,000 USDT', ha='center', fontsize=9)
    ax.text(6.5, 1.8, '实现盈亏: +5,000 USDT', ha='center', fontsize=9, color='green')
    ax.text(6.5, 1.5, '收益率: 100%', ha='center', fontsize=10,
            weight='bold', color='darkgreen')

    # 结果总结框
    summary = patches.FancyBboxPatch((0.5, 0.2), 9, 0.8,
                                     boxstyle="round,pad=0.1",
                                     edgecolor='gold', facecolor='lightyellow', lw=2)
    ax.add_patch(summary)
    ax.text(5, 0.75, '✅ 交易完成', ha='center', fontsize=12, weight='bold')
    ax.text(5, 0.45, '投入: 5,000 USDT → 收益: 5,000 USDT → 总资产: 15,000 USDT',
            ha='center', fontsize=10)

    plt.tight_layout()
    return fig

def plot_liquidation_flow():
    """绘制强平流程"""
    fig, ax = plt.subplots(figsize=(14, 12))
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 12)
    ax.axis('off')

    # 标题
    ax.text(5, 11.5, '三级强平机制流程', ha='center', fontsize=20, weight='bold')
    ax.text(5, 11, 'Market Liquidation → Insurance Fund → ADL',
            ha='center', fontsize=12, style='italic', color='gray')

    # 触发条件
    trigger = patches.FancyBboxPatch((3.5, 9.5), 3, 1,
                                     boxstyle="round,pad=0.1",
                                     edgecolor='red', facecolor='mistyrose', lw=2)
    ax.add_patch(trigger)
    ax.text(5, 10.3, '⚠️ 触发条件', ha='center', fontsize=11, weight='bold')
    ax.text(5, 9.9, '标记价格 ≤ 强平价格', ha='center', fontsize=10, color='red')

    # Arrow 触发→冻结
    ax.annotate('', xy=(5, 9), xytext=(5, 9.5),
                arrowprops=dict(arrowstyle='->', lw=2, color='red'))

    # 冻结持仓
    freeze = patches.FancyBboxPatch((3.5, 8), 3, 0.8,
                                    boxstyle="round,pad=0.1",
                                    edgecolor='darkred', facecolor='lightcoral')
    ax.add_patch(freeze)
    ax.text(5, 8.4, '🔒 冻结持仓', ha='center', fontsize=10, weight='bold')

    # Arrow 冻结→Level1
    ax.annotate('', xy=(5, 7.5), xytext=(5, 8),
                arrowprops=dict(arrowstyle='->', lw=3, color='darkred'))

    # Level 1: 市场强平
    level1 = patches.FancyBboxPatch((0.5, 6), 4, 1.3,
                                    boxstyle="round,pad=0.1",
                                    edgecolor='orange', facecolor='lightyellow', lw=2)
    ax.add_patch(level1)
    ax.text(2.5, 7, '1️⃣ 市场强平', ha='center', fontsize=11, weight='bold')
    ax.text(2.5, 6.6, '提交紧急市价单', ha='center', fontsize=9)
    ax.text(2.5, 6.3, '5秒超时', ha='center', fontsize=9, style='italic')

    # Arrow Level1→判断
    ax.annotate('成交?', xy=(2.5, 5.5), xytext=(2.5, 6),
                arrowprops=dict(arrowstyle='->', lw=2, color='orange'),
                fontsize=9, ha='center')

    # 成功分支
    ax.annotate('', xy=(1, 4.5), xytext=(2, 5.3),
                arrowprops=dict(arrowstyle='->', lw=2, color='green'))
    ax.text(1.5, 5, 'YES', fontsize=9, color='green', weight='bold')

    success1 = patches.FancyBboxPatch((0.2, 3.5), 1.6, 0.8,
                                      boxstyle="round,pad=0.1",
                                      edgecolor='green', facecolor='lightgreen')
    ax.add_patch(success1)
    ax.text(1, 4, '✅ 结算', ha='center', fontsize=10, weight='bold')
    ax.text(1, 3.7, '损失<保证金', ha='center', fontsize=8)

    # 失败分支→Level2
    ax.annotate('', xy=(5, 5.5), xytext=(3.5, 5.5),
                arrowprops=dict(arrowstyle='->', lw=2, color='red'))
    ax.text(4, 5.7, 'NO', fontsize=9, color='red', weight='bold')

    # Level 2: 保险基金
    level2 = patches.FancyBboxPatch((5, 6), 4, 1.3,
                                    boxstyle="round,pad=0.1",
                                    edgecolor='blue', facecolor='lightblue', lw=2)
    ax.add_patch(level2)
    ax.text(7, 7, '2️⃣ 保险基金接管', ha='center', fontsize=11, weight='bold')
    ax.text(7, 6.6, '检查基金容量', ha='center', fontsize=9)
    ax.text(7, 6.3, '执行接管', ha='center', fontsize=9, style='italic')

    # Arrow Level2→判断
    ax.annotate('足够?', xy=(7, 5.5), xytext=(7, 6),
                arrowprops=dict(arrowstyle='->', lw=2, color='blue'),
                fontsize=9, ha='center')

    # 成功分支
    ax.annotate('', xy=(8.5, 4.5), xytext=(7.5, 5.3),
                arrowprops=dict(arrowstyle='->', lw=2, color='green'))
    ax.text(8, 5, 'YES', fontsize=9, color='green', weight='bold')

    success2 = patches.FancyBboxPatch((8.2, 3.5), 1.6, 0.8,
                                      boxstyle="round,pad=0.1",
                                      edgecolor='green', facecolor='lightgreen')
    ax.add_patch(success2)
    ax.text(9, 4, '✅ 结算', ha='center', fontsize=10, weight='bold')
    ax.text(9, 3.7, '基金承担', ha='center', fontsize=8)

    # 失败分支→Level3
    ax.annotate('', xy=(5, 3), xytext=(6.5, 5.3),
                arrowprops=dict(arrowstyle='->', lw=2, color='darkred'))
    ax.text(5.5, 4, 'NO', fontsize=9, color='red', weight='bold')

    # Level 3: ADL
    level3 = patches.FancyBboxPatch((3, 1.5), 4, 1.3,
                                    boxstyle="round,pad=0.1",
                                    edgecolor='purple', facecolor='lavender', lw=2)
    ax.add_patch(level3)
    ax.text(5, 2.5, '3️⃣ 自动减仓 (ADL)', ha='center', fontsize=11, weight='bold')
    ax.text(5, 2.1, '查找对手方盈利仓位', ha='center', fontsize=9)
    ax.text(5, 1.8, '强制平仓', ha='center', fontsize=9, style='italic', color='red')

    # Arrow Level3→强制结算
    ax.annotate('', xy=(5, 1), xytext=(5, 1.5),
                arrowprops=dict(arrowstyle='->', lw=3, color='purple'))

    # 强制结算
    final = patches.FancyBboxPatch((3.5, 0.2), 3, 0.7,
                                   boxstyle="round,pad=0.1",
                                   edgecolor='darkred', facecolor='mistyrose', lw=2)
    ax.add_patch(final)
    ax.text(5, 0.65, '⚠️ 强制结算', ha='center', fontsize=11, weight='bold')
    ax.text(5, 0.35, '持仓已关闭 | 损失已分配', ha='center', fontsize=9)

    plt.tight_layout()
    return fig

def plot_leverage_comparison():
    """绘制杠杆倍数对比图"""
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 6))

    # 数据准备
    leverages = [5, 10, 20, 50]
    entry_price = 50000

    # 计算不同杠杆的强平价和安全距离
    liq_prices_long = []
    distances = []
    margins = []

    for lev in leverages:
        # 多仓强平价
        liq_price = entry_price * (1 - 1/lev + 0.01)
        liq_prices_long.append(liq_price)

        # 安全距离 (百分比)
        distance = (entry_price - liq_price) / entry_price * 100
        distances.append(distance)

        # 保证金占用
        margin = 100 / lev  # 百分比
        margins.append(margin)

    # 图1: 强平价格对比
    ax1.set_title('不同杠杆倍数的强平价格对比', fontsize=14, weight='bold', pad=20)
    ax1.set_xlabel('杠杆倍数', fontsize=12)
    ax1.set_ylabel('价格 (USDT)', fontsize=12)

    # 开仓价水平线
    ax1.axhline(y=entry_price, color='green', linestyle='--', linewidth=2,
                label=f'开仓价: {entry_price:,} USDT')

    # 强平价柱状图
    colors = ['lightgreen', 'yellow', 'orange', 'red']
    bars = ax1.bar([f'{l}x' for l in leverages], liq_prices_long,
                    color=colors, alpha=0.7, edgecolor='black', linewidth=1.5)

    # 添加数值标签
    for i, (bar, price, dist) in enumerate(zip(bars, liq_prices_long, distances)):
        height = bar.get_height()
        ax1.text(bar.get_x() + bar.get_width()/2., height + 500,
                f'{price:,.0f}\n({dist:.1f}%)',
                ha='center', va='bottom', fontsize=10, weight='bold')

    ax1.set_ylim(43000, 52000)
    ax1.legend(loc='upper right', fontsize=11)
    ax1.grid(axis='y', alpha=0.3)

    # 图2: 安全距离对比
    ax2.set_title('安全距离 vs 保证金占用', fontsize=14, weight='bold', pad=20)
    ax2.set_xlabel('杠杆倍数', fontsize=12)
    ax2.set_ylabel('百分比 (%)', fontsize=12)

    x = np.arange(len(leverages))
    width = 0.35

    bars1 = ax2.bar(x - width/2, distances, width, label='安全距离 (%)',
                     color='lightblue', edgecolor='blue', linewidth=1.5)
    bars2 = ax2.bar(x + width/2, margins, width, label='保证金占用 (%)',
                     color='lightcoral', edgecolor='red', linewidth=1.5)

    # 添加数值标签
    for bars in [bars1, bars2]:
        for bar in bars:
            height = bar.get_height()
            ax2.text(bar.get_x() + bar.get_width()/2., height + 0.5,
                    f'{height:.1f}%',
                    ha='center', va='bottom', fontsize=10, weight='bold')

    ax2.set_xticks(x)
    ax2.set_xticklabels([f'{l}x' for l in leverages])
    ax2.legend(loc='upper right', fontsize=11)
    ax2.grid(axis='y', alpha=0.3)
    ax2.set_ylim(0, 25)

    # 添加风险警告区域
    ax2.axhspan(0, 5, alpha=0.2, color='red', label='高风险区')
    ax2.axhspan(5, 10, alpha=0.1, color='yellow')
    ax2.axhspan(10, 25, alpha=0.1, color='green')

    plt.tight_layout()
    return fig

def plot_pnl_analysis():
    """绘制盈亏分析图"""
    fig, ax = plt.subplots(figsize=(14, 8))

    # 数据准备
    entry_price = 50000
    leverage = 10
    margin = 5000

    # 价格范围: 40000 - 60000
    prices = np.linspace(40000, 60000, 200)

    # 计算不同价格下的盈亏
    pnl = (prices - entry_price) * 1  # 1 BTC
    pnl_pct = pnl / margin * 100  # 基于保证金的收益率

    # 强平价
    liq_price = entry_price * 0.91

    # 绘制盈亏曲线
    ax.plot(prices, pnl_pct, linewidth=3, color='blue', label='盈亏曲线')

    # 强平价垂直线
    ax.axvline(x=liq_price, color='red', linestyle='--', linewidth=2,
               label=f'强平价: {liq_price:,.0f} USDT')

    # 开仓价垂直线
    ax.axvline(x=entry_price, color='green', linestyle='--', linewidth=2,
               label=f'开仓价: {entry_price:,} USDT')

    # 盈亏分界线
    ax.axhline(y=0, color='black', linestyle='-', linewidth=1)

    # 盈利区域
    ax.fill_between(prices, 0, pnl_pct, where=(prices > entry_price),
                     alpha=0.3, color='green', label='盈利区')

    # 亏损区域
    ax.fill_between(prices, 0, pnl_pct, where=(prices < entry_price),
                     alpha=0.3, color='red', label='亏损区')

    # 强平区域
    ax.fill_between(prices, -150, pnl_pct, where=(prices <= liq_price),
                     alpha=0.5, color='darkred', label='强平区')

    # 标注关键点
    # 强平点
    ax.plot(liq_price, -100, 'ro', markersize=12)
    ax.text(liq_price, -110, f'强平\n-100%\n({liq_price:,.0f})',
            ha='center', fontsize=10, weight='bold', color='red')

    # 开仓点
    ax.plot(entry_price, 0, 'go', markersize=12)
    ax.text(entry_price, -10, f'开仓\n0%\n({entry_price:,})',
            ha='center', fontsize=10, weight='bold', color='green')

    # 盈利示例点
    profit_price = 55000
    profit_pnl = (profit_price - entry_price) / margin * 100
    ax.plot(profit_price, profit_pnl, 'bo', markersize=12)
    ax.text(profit_price, profit_pnl + 10, f'+{profit_pnl:.0f}%\n({profit_price:,})',
            ha='center', fontsize=10, weight='bold', color='blue')

    ax.set_title('多仓盈亏分析 (10倍杠杆, 1 BTC @ 50,000 USDT)',
                 fontsize=16, weight='bold', pad=20)
    ax.set_xlabel('BTC价格 (USDT)', fontsize=12)
    ax.set_ylabel('收益率 (%)', fontsize=12)
    ax.set_xlim(40000, 60000)
    ax.set_ylim(-120, 120)
    ax.legend(loc='upper left', fontsize=11)
    ax.grid(True, alpha=0.3)

    # 添加注释
    ax.text(57000, -100,
            '注意:\n· 10倍杠杆放大收益和风险\n· 价格跌9%即触发强平\n· 最大损失 = 保证金',
            fontsize=10, bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.5))

    plt.tight_layout()
    return fig

if __name__ == '__main__':
    # 生成所有图表
    print("生成流程图...")

    # 1. 正常交易流程
    print("1. 正常交易流程...")
    fig1 = plot_normal_trading_flow()
    fig1.savefig('normal_trading_flow.png', dpi=300, bbox_inches='tight')
    print("   ✅ 保存为: normal_trading_flow.png")

    # 2. 强平流程
    print("2. 三级强平机制...")
    fig2 = plot_liquidation_flow()
    fig2.savefig('liquidation_flow.png', dpi=300, bbox_inches='tight')
    print("   ✅ 保存为: liquidation_flow.png")

    # 3. 杠杆对比
    print("3. 杠杆倍数对比...")
    fig3 = plot_leverage_comparison()
    fig3.savefig('leverage_comparison.png', dpi=300, bbox_inches='tight')
    print("   ✅ 保存为: leverage_comparison.png")

    # 4. 盈亏分析
    print("4. 盈亏分析...")
    fig4 = plot_pnl_analysis()
    fig4.savefig('pnl_analysis.png', dpi=300, bbox_inches='tight')
    print("   ✅ 保存为: pnl_analysis.png")

    print("\n✅ 所有图表生成完成!")
    print("\n使用方法:")
    print("  python3 visualize_trading_flow.py")
    print("\n或在Python中使用:")
    print("  import matplotlib.pyplot as plt")
    print("  from visualize_trading_flow import *")
    print("  plot_normal_trading_flow()")
    print("  plt.show()")
