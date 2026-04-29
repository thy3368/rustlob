// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum OrderSide {
//     Buy,
//     Sell,
// }

use crate::core::entity::ProductType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerpSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionKind {
    Call,
    Put,
}


/// 现货下单：用户在 spot 市场提交一个新的限价单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotPlaceOrderCmd {
    pub trader_id: u64,
    pub market: String,
    pub side: SpotSide,
    pub price: u64,
    pub quantity: u64,
}

/// 现货撤单：用户撤销一个尚未完全成交的 spot 订单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotCancelOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
}

/// 现货改单：用户修改已有 spot 订单的价格或数量。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotAmendOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
    pub new_price: Option<u64>,
    pub new_quantity: Option<u64>,
}

/// 永续下单：用户在 perp 市场提交带杠杆/只减仓语义的新订单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpPlaceOrderCmd {
    pub trader_id: u64,
    pub market: String,
    pub side: PerpSide,
    pub price: u64,
    pub quantity: u64,
    pub leverage: u32,
    pub reduce_only: bool,
}

/// 永续撤单：用户撤销一个尚未完全成交的 perp 订单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpCancelOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
}

/// 永续改单：用户修改已有 perp 订单的价格或数量。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpAmendOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
    pub new_price: Option<u64>,
    pub new_quantity: Option<u64>,
}

/// 期权下单：用户提交一个带到期日、行权价和权利方向的 option 订单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionPlaceOrderCmd {
    pub trader_id: u64,
    pub underlying: String,
    pub expiry_ts: u64,
    pub strike_price: u64,
    pub kind: OptionKind,
    pub side: OptionSide,
    pub premium: u64,
    pub quantity: u64,
}

/// 期权撤单：用户撤销一个尚未完全成交的 option 订单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionCancelOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
}

/// 期权改单：用户修改已有 option 订单的价格或数量。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionAmendOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
    pub new_price: Option<u64>,
    pub new_quantity: Option<u64>,
}

/// 资金费结算：按 funding index 对某个 perp 市场头寸做资金费增减。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleFundingCmd {
    pub trader_id: u64,
    pub market: String,
    pub funding_index: u64,
    pub amount_delta: i64,
}

/// 入金：把外部资产充入交易账户。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositCmd {
    pub trader_id: u64,
    pub asset: String,
    pub amount: u64,
}

/// 出金：把账户内资产提走到外部系统。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawCmd {
    pub trader_id: u64,
    pub asset: String,
    pub amount: u64,
}

/// 账户划转：在同一 trader 的不同账户分区之间转移资产，如 spot 到 perp。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferCmd {
    pub trader_id: u64,
    pub asset: String,
    pub amount: u64,
    pub from_account: String,
    pub to_account: String,
}

/// 强平：对风险不足的仓位执行强制减仓或平仓。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiquidatePositionCmd {
    pub liquidator_trader_id: u64,
    pub liquidated_trader_id: u64,
    pub market: String,
    pub quantity: u64,
}

/// 成交落账：批处理执行阶段把一笔 maker/taker 撮合结果写成显式成交命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteTradeCmd {
    pub market: String,
    pub maker_order_id: u64,
    pub taker_order_id: u64,
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
    pub new_price: Option<u64>,
    pub new_quantity: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotCommand {
    PlaceOrder(SpotPlaceOrderCmd),
    CancelOrder(SpotCancelOrderCmd),
    AmendOrder(SpotAmendOrderCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerpCommand {
    PlaceOrder(PerpPlaceOrderCmd),
    CancelOrder(PerpCancelOrderCmd),
    AmendOrder(PerpAmendOrderCmd),
    SettleFunding(SettleFundingCmd),
    LiquidatePosition(LiquidatePositionCmd),
    ExecuteTrade(ExecuteTradeCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionCommand {
    PlaceOrder(OptionPlaceOrderCmd),
    CancelOrder(OptionCancelOrderCmd),
    AmendOrder(OptionAmendOrderCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreasuryCommand {
    Deposit(DepositCmd),
    Withdraw(WithdrawCmd),
    Transfer(TransferCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeCommand {
    TradingCommand(TradingCommand),
    TreasuryCommand(TreasuryCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradingCommand {
    Spot(SpotCommand),
    Perp(PerpCommand),
    Option(OptionCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeCommandEnvelope {
    pub command_id: u64,
    pub trader_id: u64,
    pub nonce: u64,
    pub timestamp_ns: u64,
    pub product_type: ProductType,
    pub command: ExchangeCommand,
}
