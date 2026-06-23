下面按时间线把「强平触发 → 接管执行 → 算账发现穿仓 → IF兜底 →（IF不够）ADL」整个链路拆开：每一步只要产生了"必须留底、不可Rewrite"的事实，就会产生至少一个 MI（时标凭证）。

0）先给全局视图：链路长这样（MI 就是那些方框）


[仓位状态机: 正常 → 强平触发]
│
▼
🔴 LiquidationEvent              ← MI-1  "强平接管事件"
│
▼
🔴 LiquidationExecution/Fill(s) ← MI-2  "强平执行/成交记录"
│
▼ 算账：fill_price vs bankruptcy_price
│
├─ 成交价 优于 破产价 ──→ 🔵 InsuranceFundTx(SURPLUS)   ← MI-3a
│
└─ 成交价 劣于 破产价 ──→ 🔴 BankruptcyShortfallEvent ← MI-3b（穿仓缺口）
│
▼
🔴 InsuranceFundDrawdownEvent ← MI-4（IF尝试兜底）
│
IF够? ──YES──→ DONE（缺口closed）
│NO
▼
🔴 ADLRound                 ← MI-5（ADL批次锚点）
├─ 🔴 ADLDeleveragingRecord ← MI-6（减了谁/减多少/啥价）
├─ 🔴 ADLDeleveragingRecord
└─ 🔵 InsuranceFundTx(REPLENISH) ← MI-7（溢出回充IF）


一句话定性：LiquidationEvent / BankruptcyShortfallEvent / ADLRound 这类是"业务事实MI"；InsuranceFundTx 是"资金侧MI"。两者用ID链条绑死，才是可审计的。

MI-1：LiquidationEvent（强平接管事件）

回答：“什么时候、谁的仓位、为什么进了强平引擎？”

要点 说明

为什么必须是MI 它是所有后续清算/穿仓/ADL的因果起点。你需要证明：不是系统乱关人，而是 mark/price 触达了 liquidation_price，且保证金不足。

关键字段 liq_event_id、account_id、position_id、market、side（LONG/SHORT）、trigger_reason（MaintenanceMargin / Manual / etc.）、triggered_at、mark_price_at_trigger、liquidation_price、bankruptcy_price、pre_liquidation_equity

引用链 后续 Fill / Shortfall / ADL 都回指 liq_event_id

这一条在很多团队会被误写成"只改 status=Liquidating"。但一旦出事（极端行情仲裁/审计），你需要的是事件凭证，而不是一个会翻覆的状态值。

MI-2：LiquidationExecution（强平执行/成交明细）

回答：“系统到底怎么把仓位平掉的？成交在多少？剩多少没平完？”

强平引擎通常走的是：订单簿 IOC / 逐步吃单 / 或“接管后按破产价结算”。

要点 说明

形态 一般是 1:N（一次 LiquidationEvent 可能产生多条 Fill/执行记录）

关键字段 exec_id、liq_event_id、filled_qty、fill_price、timestamp、fee、remaining_qty（若吃单不完全）

为什么是MI 成交价分布决定：surplus 还是 shortfall；哪怕接管后直接按破产价结算，也要有一行“结算记录”证明用了哪个价、什么时候结的。

MI-3b（若存在）：BankruptcyShortfallEvent（穿仓缺口事件）

回答：“亏超保证金多少？缺口怎么算出来的？”

计算核心就是：
• 破产价（bankruptcy price）：margin balance 恰好 = 0 的价位

• 实际处理价：市场成交均价（或更常见表述：最终执行价/接管结算价）

• 当最终执行价 劣于破产价 → 出现负余额（穿仓）→ 缺口必须被记录 & 被覆盖

要点 说明

关键字段 shortfall_id、liq_event_id、(optional) exec_id、gap_amount（负数绝对值）、bankruptcy_price、actual_settlement_price、detected_at

为什么必须是独立MI 缺口不是每个强平均有（大多数反而是 surplus），所以它不该污染 LiquidationEvent；它又是 IF 拨付/ADL 的分叉条件，需要独立身份证。

如果成交价优于破产价，你得到的就是 MI-3a：InsuranceFundTx(SURPLUS)（见下），不会产生 shortfall。

MI-4：InsuranceFundDrawdownEvent（IF 拨付/扣减凭证）

回答：“保险基金什么时候、因为哪个缺口、掏了多少、余额剩多少？”

保险基金的作用正是：当强平相关损失 > 保证金时，用基金覆盖缺口，从而降低 ADL 概率。

要点 说明

关键字段 if_tx_id、shortfall_id、fund_currency、delta=-gap、if_balance_before/after、status=COVERED｜INSUFFICIENT、settled_at

为什么是MI 这是财务级别的“付款凭证”。没有它，所谓“IF cover”只是内存里改个数。

MI-5：ADLRound（ADL 批次锚点）

回答：“因为哪个缺口、在什么时刻启动了ADL、最终填平没？”

当 IF 不够，ADL 作为强平流程的最后一步被激活：系统从对侧盈利/高杠杆队列开始减仓，并按破产价结算。

要点 说明

关键字段 adl_round_id、shortfall_id、if_drawdown_id、direction_to_reduce（减哪侧）、bankruptcy_price_used、total_gap、status=RUNNING→COVERED｜EXHAUSTED、started_at、resolved_at

为什么是MI ADL 不是“偷偷改别人仓”，而是一段强制性、可申诉的外部干预。你需要一个批次级身份证把一批减仓捆在一起。

MI-6：ADLDeleveragingRecord（每条被减仓的明细凭证）

回答：“第N个被ADL选中是谁？减了多少？以什么价结算？贡献了多少去填坑？”

要点 说明

关键字段 adl_record_id、adl_round_id、victim_account_id、position_id、side、reduced_qty、settlement_price (= bankruptcy_price_of_liquidated)、realized_pnl_for_victim、covered_amount、executed_at

为什么必须是MI（而且Append-only） 被减的用户来投诉/申请仲裁时，你要能证明：系统依据的是那条穿仓记录的破产价，不是暗箱市价；且只减了填补缺口所需量。

关键点：搜索结果明确写到——ADL 减仓以被强平头寸的破产价（Bankruptcy Price）结算，而不是当前 mark price。这个“结算价来源”必须写进每条 Record 里，否则审计链断裂。

MI-7（常见但可选）：InsuranceFundTx(REPLENISH_FROM_ADL)

回答：“ADL 截回来的钱有没有溢出？溢出多少回充IF？”

如果 ADL 填平缺口后还有余量（或按你们规则把“破产价与更优价之间的差额”归入基金池），就产生一条 REPLENISH 类 IF-Tx 收尾。

一张“你必须落库的最小 MI 集”（工程口径）

# MI 名称 产生条件 它解决的可审计问题

1 LiquidationEvent 触发强平（接管开始） 谁、何时、为什么被接管

2 LiquidationExecution/Fill(s) 接管后执行/结算 实际怎么平的、成交价分布

3a InsuranceFundTx(SURPLUS) fill 价优于破产价 盈余怎么进 IF（可查历史）

3b BankruptcyShortfallEvent fill 价劣于破产价 穿仓缺口的计算凭证

4 InsuranceFundDrawdownEvent IF 尝试兜底 IF 出账凭证

5 ADLRound IF 不够 → ADL 启动 “一次ADL任务”的批次锚点

6 ADLDeleveragingRecord×N ADL 逐条减仓 每一条“强制减仓”的不可篡改明细

7 InsuranceFundTx(REPLENISH…) ADL 回充（可选） 资金侧闭环

你可以用一句话自检：这条链齐不齐

任意一个外部争议（“我为什么被强平 / 为什么被ADL / IF去哪了”）能不能只用 liq_event_id → shortfall_id → if_tx_id / adl_round_id → adl_record_id 这几张表就复现整件事？

能复现＝MI边界是对的；复现不了＝一定有人把“事件”偷懒写成“状态字段”了。

如果你愿意，我可以把这套 MI 直接写成一份表结构草案（含幂等键/唯一约束/索引建议），并帮你把 LiquidationEvent 与 Position 主MI状态机（OPEN→LIQUIDATING→LIQUIDATED）的边界划清楚：哪些写在状态机里、哪些必须写事件表、哪一步必须事务、哪一步可以异步（极端行情下这决定你会不会写出脏账）。