# CEX Option 模型说明

本文描述当前 `example_core` 中 CEX option 的核心特性、主要单据和实体边界。内容以
`entity/option/cex` 下的现有代码为准。

## 核心特性

CEX option 当前模型聚焦“普通下单”阶段：

- 合约规格与订单分离：行权价、到期时间、Call/Put 类型属于合约规格；订单只保存
  `instrument_id`。
- 订单价格表达权利金价格，不是行权价。
- 下单按仓位意图区分买方多头、卖方空头和平仓意图。
- 买方开多 / 加多会派生权利金占用要求。
- 卖方开空 / 加空会派生卖方保证金占用要求。
- 平仓意图会把订单标记为 `reduce_only`，但不在订单实体内直接修改仓位。
- 订单实体只表达订单自身生命周期与资金占用需求，不执行余额冻结、撮合、仓位变更或到期结算。

## 主要实体

### `CexOptionInstrument`

`CexOptionInstrument` 是 CEX option 合约规格快照，四颜色分类为 `Description`。

它保存挂牌规则中的稳定事实：

- `instrument_id`：交易所风格合约 ID，例如 `BTC-20260828-100000-PUT`
- `underlying_asset`：标的资产，例如 `BTC`
- `quote_asset`：权利金报价资产，例如 `USDT`
- `settle_asset`：交割或结算资产，例如 `USDT`
- `expiry_time`：到期时间，Unix 毫秒
- `strike_price`：行权价
- `option_type`：`Call` 或 `Put`
- `status`：挂牌状态

它提供的关键业务查询是 `is_tradable()`：只有 `Trading` 状态允许普通下单。

### `CexOptionOrder`

`CexOptionOrder` 是已接受并可由撮合层读取的 option 订单快照。

实体分类：

- 四颜色分类：`MomentInterval`
- 聚合角色：`AggregateRoot`
- 金融分类：`BusinessVoucher`

它保存订单自身执行事实：

- `order_id`：本系统生成的稳定订单 ID
- `account_id`：订单所属账户
- `instrument_id`：订单交易的 option 合约 ID
- `order_side`：买入或卖出
- `execution`：市价意图或限价意图，价格字段表达权利金报价
- `time_in_force`：`Gtc`、`Ioc`、`Alo`
- `quantity`：合约数量
- `filled_quantity`：已成交数量
- `reduce_only`：是否只减仓
- `status`：订单生命周期状态
- `client_order_id`：客户端自定义订单 ID

`CexOptionOrder::place(...)` 是当前主要聚合根行为。它负责：

- 校验输入合约 ID 与合约规格快照一致
- 校验合约当前可交易
- 校验行权价、订单数量、权利金报价为正数
- 按仓位意图创建订单
- 按需要派生资金占用要求

订单还提供少量业务查询和不变量判断：

- `remaining_quantity()`：剩余可成交数量
- `is_open()`：订单是否仍在开放生命周期中
- `is_terminal()`：订单是否处于终态
- `is_cancelable()`：订单当前是否允许撤销
- `is_matchable()`：订单当前是否可进入撮合
- `has_consistent_execution_state()`：生命周期状态与成交数量是否自洽
- `belongs_to_account(...)`：订单是否属于指定账户
- `trades_instrument(...)`：订单是否交易指定合约
- `order_price()` / `limit_price()`：权利金价格查询

## 主要单据 / 派生要求

### `PlaceCexOptionOrderInput`

`PlaceCexOptionOrderInput` 表达创建 CEX option 订单所需的已校验业务输入。

它不是持久化实体，而是下单行为的输入事实包，包含：

- 订单、账户、合约 ID
- 下单时加载的合约规格快照
- 买卖方向、执行方式、订单有效方式
- 合约数量
- 仓位业务意图
- 权利金、卖方保证金、手续费预占用金额
- 客户端自定义订单 ID

### `PlaceCexOptionOrderOutcome`

`PlaceCexOptionOrderOutcome` 表达一次下单行为的结果：

- 已创建的 `CexOptionOrder`
- 可选的权利金占用要求
- 可选的手续费占用要求
- 可选的卖方保证金占用要求

它是下单行为产生的业务结果包，供后续 use case 或 adapter 继续编排余额冻结、撮合或持久化。

### `CexOptionPremiumHoldRequirement`

买方开多 / 加多时派生的权利金占用要求。

核心字段：

- 被占用账户
- 来源订单 ID
- 合约 ID
- 权利金资产
- 权利金价格
- 合约数量
- 应占用权利金金额

### `CexOptionShortMarginHoldRequirement`

卖方开空 / 加空时派生的卖方保证金占用要求。

核心字段：

- 被占用账户
- 来源订单 ID
- 合约 ID
- 保证金资产
- 应占用保证金金额

### `CexOptionFeeHoldRequirement`

可选手续费预占用要求。

当前它只表达手续费占用事实，不承担手续费费率计算。

## 仓位意图与资金占用

`PlaceCexOptionOrderIntent` 当前覆盖六类普通下单意图：

| 意图 | 业务含义 | `reduce_only` | 派生资金占用 |
| --- | --- | --- | --- |
| `OpenLong` | 建立买方多头 | 否 | 权利金占用 |
| `IncreaseLong` | 增加买方多头 | 否 | 权利金占用 |
| `CloseShort` | 买回平空 | 是 | 无 |
| `OpenShort` | 建立卖方空头 | 否 | 卖方保证金占用 |
| `IncreaseShort` | 增加卖方空头 | 否 | 卖方保证金占用 |
| `CloseLong` | 卖出平多 | 是 | 无 |

## 生命周期状态

`CexOptionOrderStatus` 当前包含：

- `Open`：订单已进入执行流程，尚未成交
- `PartiallyFilled`：订单已部分成交
- `Filled`：订单已完全成交
- `Canceled`：订单已取消
- `Rejected`：订单提交时被拒绝

状态与成交数量的不变量由 `has_consistent_execution_state()` 表达：

- `Open` 要求 `filled_quantity == 0`
- `PartiallyFilled` 要求 `0 < filled_quantity < quantity`
- `Filled` 要求 `filled_quantity == quantity`
- `Canceled` 允许 `filled_quantity <= quantity`
- `Rejected` 要求 `filled_quantity == 0`

## 当前边界

当前 CEX option 模型不负责以下业务：

- 行权方式建模，例如欧式 / 美式
- 到期行权、交割、现金结算
- 隐含波动率、希腊值、保证金公式、风险限额
- 余额冻结的实际落账
- 仓位增减与持仓簿更新
- 订单簿撮合与成交单生成
- 市场数据、标记价格、指数价格

这些能力应由后续 use case 或相邻 bounded context 编排，不能下沉到 inbound adapter。
