//! PREP - 永续合约交易引擎
//!
//! Perpetual Contract Trading Engine
//!
//! ## 命令优先级
//!
//! - **P0**: 核心交易 (OpenPosition, ClosePosition, LimitOrder, MarketOrder,
//!   CancelOrder)
//! - **P1**: 风险控制 (SetLeverage, AdjustMargin, Liquidate, SetStopLoss,
//!   SetTakeProfit)
//! - **P2**: 高级功能 (SwitchMarginMode, SettleFundingRate, ADL, TrailingStop)
//! - **P3**: 扩展功能 (FlashClose, ReversePosition, BatchCancelOrders)

pub mod adaptor;
pub mod domain;
pub fn add(left: u64, right: u64) -> u64 { left + right }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
