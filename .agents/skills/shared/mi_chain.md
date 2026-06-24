一、先搞清楚：什么叫"MI 的因果链"？

1. MI 为什么天生带因果？

四色里四类原型：

颜色 原型 存在方式

🟢 PPT Party/Place/Thing 空间存在——"它在那"，不依赖某次交易

🔵 DESC Description 规格/模板——"长成什么样/按什么规则"

🔴 MI Moment-Interval 时间存在——"它在某个时刻发生了"

🟡 Role Role PPT 在某段上下文里的面具

只有 MI 携带 when（时间戳/时间区间）。所以时间箭头只活在 MI 世界里。

因果链 = 把 MI 按"哪个事实生出了哪个事实"串起来的一根绳。

不是"步骤1→步骤2"的流程，而是"铁证A出现了 → 读A的数据 → 按规则判定 → 铁证B必须（或允许）出现"。

2. 因果链的三元组结构

每一个链节都是同一件事的重复：

MIₙ（已发生的事实，带 IDₙ 和 数据载荷）
│
│  读 MIₙ 的载荷数据
│  应用判定规则 R(MIₙ.data)
│
├─ R 为真 ──→ 必须创造 MIₙ₊₁
│                 MIₙ₊₁.caused_by = IDₙ   ← 因果指针，不是外键那么浅
│
└─ R 为假 ─→ 链在此等（不消灭，等下一个外部触发或条件成熟）


关键区别：

流程图 / 状态机 MI 因果链

箭头含义 "下一步走哪" "下一块铁证为什么必须存在"

回滚怎么处理 UPDATE status 就行 原 MI 不动，追加新 MI（红冲/补偿）

证据 状态字段是覆写型 每个环节独立可指认、可追责

状态机回答"它现在是什么"，因果链回答"它怎么变成这样的，每一步的证据在哪"。

3. 因果链的三要素（你建模时死死盯住这三个）

① 因果指针（caused_by / due_to）

每个 MI 必须能指回"我因谁而生"。没有这根线，你放的是散落的票据，不是链。

② 判定规则（Predicate）

链不靠"感觉下一步该干嘛"，靠写在纸上的布尔条件：
• 例：filledQty == orderQty → 必须产生 OrderFilled

• 例：actual_fill_price 劣于 bankruptcy_price → 必须产生 BankruptcyShortfallEvent

规则读的是前驱 MI 的数据，不是全局变量。

③ 守恒不变量（Invariant）

尤其金融：因果链展开时，某些东西必须守恒（钱/币总量、借贷平衡）。不变量是链的"物理定律"，破了就是 bug，不是需求差异。

二、用现货交易（Spot）举例：从业务公理推演出完整因果链

不碰表结构，不碰代码，只从"两个人要交换 BTC 和 USDT，且事后必须能追责"推出需要哪些 MI、为什么它们必须存在、谁生谁。

公理（最少前提）

1. 有两人：甲方（想卖 BTC 换 USDT）、乙方（想买 BTC 花 USDT）
2. 交换必须成对：BTC 往一方走，USDT 往另一方走
3. 任何一方不能花不存在的钱 → 交换前必须锁定对应资产
4. 事后争议必须可查 → 每一步是独立可指认的事实

从这四条，MI 因果链被逼出来，没有一个是为了好看而加的装饰。

链节 ①：表达意图 —— OrderSubmitted

某人（PPT）在某个时刻说："我愿以 P 价买/卖 Q 量 BTC/USDT。"

必须的铁证字段：
• submissionNo（这事自己的身份证）

• submitter（PPT 引用）

• side / price / qty / pair

• submittedAt（时间戳——MI 的灵魂）

此刻什么都没锁定、没生效。它只是"嘴上说的"。

链节 ②：受理判定 → 两条互斥支路

系统（规则/风控）必须回答两个问题：
• Q-A：请求本身合法吗？（精度、交易对开放、参数合理）

• Q-B：submitter 有能力履约吗？（BUY 需要至少 P×Q 的 USDT 可见；SELL 需要至少 Q 的 BTC 可见）

支路 ②a：任一不满足 → SubmissionRejected


① OrderSubmitted
└─→ ②a SubmissionRejected
caused_by = submissionNo
reason = INVALID_PARAM | PAIR_SUSPENDED | INSUFFICIENT_BALANCE


⚫ 链收敛——没产生委托，没锁资产。

支路 ②b：都满足 → 两个 MI 同时诞生（这是因果链第一个重要跃迁）


① OrderSubmitted
└─→ ②b-1 OrderEstablished          ← ★ 委托契约本体（主轴 MI）
│             · orderNo（新身份！≠submissionNo）
│             · caused_by = submissionNo
│             · establishedAt
│             · side / price / qty / remainingQty
│
└─→ ②b-2 FundsLocked
· lockNo
· caused_by = orderNo
· owner = submitter
· asset（BUY→USDT；SELL→BTC）
· lockedQty = 所需量
· lockedAt


为什么 Locked 必须是独立 MI，不能只是 OrderEstablished 里一个字段？

因为"锁没锁、什么时候锁的、锁了多少"本身就是独立可争议的事实。把它压进 status 字段，审计就只能靠猜。

至此，委托进入"可撮合"状态（业务语义上的等候簿）。

链节 ③：撮合点火 —— TradeExecuted（链的心脏）

当簿上存在一个方向相反、价格可交叉的 OrderEstablished，业务必须承认一次交换发生了。

这个事实不能归属为"甲的状态变了"或"乙的状态变了"——它是两人之间的中立事实：

③ TradeExecuted
· tradeNo（全局交换身份证）
· caused_by = 驱动方orderNo（通常取taker）
· executedAt
· price / qty
· buyerId / sellerId
· buyerOrderNo / sellerOrderNo


为什么它不可替代？  
如果把它降级为 orderA.status='PARTIAL'，你就丢失了：
• 成交的全局唯一身份证（tradeNo）

• 成交的双边关系（buyer/seller 各是谁）

• 成交的价格（可能≠buyer.price，撮合按maker价）

• 成交的时间戳精度（争议往往卡在毫秒级）

这些都是追责刚需 → 所以 TradeExecuted 必须是独立 MI。

链节 ④：交割裂变 —— 从一次交换推出成对转移

TradeExecuted 出现后，业务守恒律强制推出四笔转移（同一 tradeNo 罩住）：

③ TradeExecuted
├─→ ④a TransferApplied  (buyer 付 USDT)       delta = –price×qty
├─→ ④b TransferApplied  (seller 收 USDT)       delta = +price×qty
├─→ ④c TransferApplied  (seller 付 BTC)        delta = –qty
└─→ ④d TransferApplied  (buyer 收 BTC)         delta = +qty


每张 TransferApplied 必有：
• transferNo（自身ID）

• caused_by = tradeNo

• owner / asset / delta

• appliedAt

守恒不变量（写进业务章程，不是代码注释）：

对同一 tradeNo，按 asset 分组：Σδ(USDT) = 0，Σδ(BTC) = 0（不计手续费时）

然后手续费：

③ TradeExecuted
└─→ ⑤a FeeCollected (from buyer's payment leg,  or from received BTC)
└─→ ⑤b FeeCollected (from seller's receipt leg, or from delivered BTC)
· caused_by = tradeNo


手续费的处理有两种诚实写法：

① Fee 作为独立 MI（更可审计）

② Fee 折进 Transfer 的净值里（更紧凑，但 Fee 本身仍须能指回 tradeNo）

两种都行，只要 fee 的出处链不断。

链节 ⑥：委托推进 → 终态判定

回到驱动方 OrderEstablished，用 TradeExecuted 的 qty 推进 remainingQty：

stillRemaining = previousRemaining – qty


判定 R₁：stillRemaining > 0

→ 委托仍是 PARTIAL（业务语义），回簿等下一个撮合。  
链不收敛，下一轮撮合可能再产一个 TradeExecuted。

判定 R₂：stillRemaining = 0

→ 必须承认委托履约完毕：

③ TradeExecuted（最后一笔）
└─→ ⑥a OrderFilled
· fillNo
· caused_by = tradeNo（最后一笔的）
· orderNo
· filledAt


平行支路：撤单（在 remainingQty > 0 时合法触发）


OrderEstablished（remaining > 0）
│  [人撤 / 超时 / 风控强撤]
▼
⑥b-1 OrderCancelled
· cancelNo
· caused_by = orderNo
· cancelledAt
· reason
│
└─→ ⑥b-2 FundsUnlocked
· unlockNo
· caused_by = cancelNo
· owner
· asset
· unlockedQty = remaining
· unlockedAt


已成交部分（TradeExecuted + Transfers）不动、不撤回、不产生反事实。这是金融底线。

三、整条链铺开（纯业务，一张图看完）


① OrderSubmitted
├─ ②a SubmissionRejected  ⚫收敛
│
└─ ②b-1 OrderEstablished ─ ②b-2 FundsLocked
│
┌───────┤  （撮合条件成熟）
▼
③ TradeExecuted(tradeNo)
├─ ④a-d TransferApplied ×4   ← 成对守恒
├─ ⑤a-b FeeCollected
│
├─ stillRemaining>0 → 回簿（等下一轮③）
│
└─ stillRemaining=0
└─→ ⑥a OrderFilled  ⚫收敛
└─ (若有微量dust) FundsUnlocked

——或撤单打断——
⑥b-1 OrderCancelled ─→ ⑥b-2 FundsUnlocked  ⚫收敛


四、一句话把"MI因果链"钉死

MI 因果链不是流程图的漂亮说法——它是"一个不可篡改的事实（MIₙ）出现后，按业务规则读它的数据，逼出下一个不可篡改事实（MIₙ₊₁），并用 caused_by 把两者钉死"的链条。

> 现货交易完美演示了它：没有 TradeExecuted，交割无据可依；没有 TransferApplied 的四连星，守恒就崩；没有 caused_by 一路回链，审计就只剩玄学。

如果你下一步想把它落到团队沟通层面，我可以帮你把这条链改写成一份一页纸的《现货交易业务证据章程》：每行只写"MI名称 / 它证明什么 / 它因谁而生 / 缺失会导致什么审计事故"——拿这个跟产品/风控/财务对齐，比任何PRD都管用。