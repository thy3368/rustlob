//! 强平流程完整实现（参考 XPDL 行 1849-2156）
//!
//! 币安三级强平机制：
//! 1. 市场强平（Market Liquidation）
//! 2. 风险保障基金接管（Insurance Fund）
//! 3. 自动减仓（ADL）

use std::sync::Arc;
use std::time::Duration;

use base_types::PrepPosition;
use base_types::exchange::spot::spot_types::TimeInForce;

use crate::proc::liquidation_types::*;
use crate::proc::trading_prep_order_behavior::*;

/// 强平流程处理器
pub struct LiquidationProcessor {
    matching_service: Arc<dyn PerpOrderExchBehavior>,
    insurance_fund: Arc<dyn InsuranceFund>,
    adl_engine: Arc<dyn ADLEngine>,
}

impl LiquidationProcessor {
    /// 创建新的强平处理器
    pub fn new(
        matching_service: Arc<dyn PerpOrderExchBehavior>,
        insurance_fund: Arc<dyn InsuranceFund>,
        adl_engine: Arc<dyn ADLEngine>,
    ) -> Self {
        Self { matching_service, insurance_fund, adl_engine }
    }

    /// 执行三级强平流程
    ///
    /// # 参数
    /// - `position_id`: 触发强平的持仓ID
    /// - `trigger_price`: 触发价格（标记价格）
    ///
    /// # 返回
    /// 强平结果，包含强平类型和损失分配
    pub async fn execute_liquidation(
        &self,
        position_id: PositionId,
        trigger_price: Price,
    ) -> Result<LiquidationResult, PrepCmdError> {
        // 0. 冻结持仓
        self.freeze_position(&position_id).await?;

        // 获取持仓信息
        let position = self.get_position(&position_id).await?;

        self.execute_liquidation_with_position(position, trigger_price).await
    }

    /// 使用持仓信息执行强平（用于测试或已有持仓数据的场景）
    ///
    /// # 参数
    /// - `position`: 持仓信息
    /// - `trigger_price`: 触发价格（标记价格）
    ///
    /// # 返回
    /// 强平结果，包含强平类型和损失分配
    pub async fn execute_liquidation_with_position(
        &self,
        position: PrepPosition,
        trigger_price: Price,
    ) -> Result<LiquidationResult, PrepCmdError> {
        // 确定平仓方向（与持仓方向相反）
        let liquidation_side = match position.position_side {
            PositionSide::Long => OrderSide::Sell,
            PositionSide::Short => OrderSide::Buy,
            PositionSide::Both => {
                // 单向持仓模式，根据数量判断方向
                if position.quantity.raw() > 0 { OrderSide::Sell } else { OrderSide::Buy }
            }
        };

        // ========================================
        // 1️⃣ 第一级：市场强平（Market Liquidation）
        // ========================================
        log::info!(
            "🔥 Liquidation triggered for position {}: mark_price={}, liq_price={:?}",
            position.position_id,
            trigger_price.to_f64(),
            position.liquidation_price
        );

        let market_result = self.try_market_liquidation(&position, liquidation_side).await;

        if let Ok(result) = market_result {
            log::info!("✅ Market liquidation succeeded for position {}", position.position_id);
            return Ok(result);
        }

        log::warn!(
            "⚠️ Market liquidation failed for position {}: insufficient liquidity",
            position.position_id
        );

        // ========================================
        // 2️⃣ 第二级：风险保障基金接管（Insurance Fund）
        // ========================================
        let insurance_result = self.try_insurance_fund_takeover(&position).await;

        if let Ok(result) = insurance_result {
            log::info!(
                "✅ Insurance fund takeover succeeded for position {}",
                position.position_id
            );
            return Ok(result);
        }

        log::error!("⚠️ Insurance fund insufficient for position {}", position.position_id);

        // ========================================
        // 3️⃣ 第三级：自动减仓（Auto-Deleveraging, ADL）
        // ========================================
        let adl_result = self.trigger_auto_deleveraging(&position, liquidation_side).await?;

        log::info!("✅ ADL liquidation completed for position {}", position.position_id);

        Ok(adl_result)
    }

    /// 尝试市场强平
    async fn try_market_liquidation(
        &self,
        position: &PrepPosition,
        side: OrderSide,
    ) -> Result<LiquidationResult, PrepCmdError> {
        // 提交紧急市价单
        let order_cmd = OpenPositionCmd {
            trading_pair: position.trading_pair,
            side,
            order_type: OrderType::Market,
            quantity: position.quantity,
            price: None,
            position_side: match side {
                OrderSide::Buy => PositionSide::Long,
                OrderSide::Sell => PositionSide::Short,
            },
            time_in_force: TimeInForce::IOC, // 立即成交或取消
            leverage: position.leverage,
        };

        // 设置5秒超时等待成交
        let order_result = tokio::time::timeout(Duration::from_secs(5), async {
            self.matching_service.open_position(order_cmd)
        })
        .await;

        match order_result {
            Ok(Ok(result)) if result.status == FutureOrderStatus::Filled => {
                // 成交成功
                let avg_price = result.avg_price.unwrap_or(Price::from_raw(0));
                let loss = Self::calculate_liquidation_loss(position, avg_price);

                // 结算市场强平
                self.settle_market_liquidation(position, avg_price, loss).await
            }
            _ => Err(PrepCmdError::market_liquidity_insufficient()),
        }
    }

    /// 尝试保险基金接管
    async fn try_insurance_fund_takeover(
        &self,
        position: &PrepPosition,
    ) -> Result<LiquidationResult, PrepCmdError> {
        // 检查保险基金容量
        let capacity = self.insurance_fund.check_capacity().await?;

        if !capacity.can_takeover(position) {
            return Err(PrepCmdError::insurance_fund_insufficient());
        }

        // 执行接管
        let takeover = self.insurance_fund.takeover(position).await?;

        // 结算保险基金强平
        self.settle_insurance_fund_liquidation(position, takeover).await
    }

    /// 触发自动减仓
    async fn trigger_auto_deleveraging(
        &self,
        position: &PrepPosition,
        side: OrderSide,
    ) -> Result<LiquidationResult, PrepCmdError> {
        // 查找对手方盈利仓位（按ADL队列优先级）
        let counterparties =
            self.adl_engine.find_counterparties(position.trading_pair, side).await?;

        if counterparties.is_empty() {
            return Err(PrepCmdError::no_counterparties_for_adl());
        }

        // 执行ADL
        let adl_result = self.adl_engine.execute_adl(position, counterparties).await?;

        // 通知被ADL的对手方
        for counterparty_id in &adl_result.affected_positions {
            self.notify_adl_counterparty(counterparty_id).await?;
        }

        // 结算ADL强平
        self.settle_adl_liquidation(position, adl_result).await
    }

    /// 冻结持仓
    async fn freeze_position(&self, position_id: &PositionId) -> Result<(), PrepCmdError> {
        // 更新持仓状态为 LIQUIDATING
        // 防止用户继续操作该持仓
        log::info!("🔒 Freezing position {}", position_id);
        // TODO: 实现持仓状态更新逻辑
        Ok(())
    }

    /// 获取持仓信息
    async fn get_position(&self, position_id: &PositionId) -> Result<PrepPosition, PrepCmdError> {
        // 尝试从 matching_service 查询持仓
        // 但是 query_position 需要 Symbol，而我们只有 position_id
        // 这是一个设计问题：需要一个 position_id -> Symbol 的映射

        // 临时解决方案：从 position_id 中提取信息
        // 或者在实际系统中，应该有一个 PositionRepository 来管理这个映射
        log::warn!("TODO: Implement proper get_position by position_id: {}", position_id);

        // 返回错误，提示需要实现
        Err(PrepCmdError::Unknown(format!(
            "Position lookup by ID not implemented. Need PositionRepository. ID: {}",
            position_id
        )))
    }

    /// 计算强平损失
    pub fn calculate_liquidation_loss(position: &PrepPosition, close_price: Price) -> Price {
        let entry = position.entry_price.to_f64();
        let close = close_price.to_f64();
        let qty = position.quantity.to_f64();

        let loss = match position.position_side {
            PositionSide::Long => {
                // 多仓损失 = (开仓价 - 平仓价) × 数量
                (entry - close) * qty
            }
            PositionSide::Short => {
                // 空仓损失 = (平仓价 - 开仓价) × 数量
                (close - entry) * qty
            }
            PositionSide::Both => {
                // 单向模式，根据数量判断
                if qty > 0.0 { (entry - close) * qty } else { (close - entry) * qty.abs() }
            }
        };

        Price::from_f64(loss.max(0.0))
    }

    async fn settle_market_liquidation(
        &self,
        position: &PrepPosition,
        avg_price: Price,
        loss: Price,
    ) -> Result<LiquidationResult, PrepCmdError> {
        // 扣除保证金
        let margin_loss = position.margin;

        // 如有超出部分，从保险基金扣除
        let insurance_fund_loss = if loss > margin_loss {
            Price::from_f64(loss.to_f64() - margin_loss.to_f64())
        } else {
            Price::from_raw(0)
        };

        Ok(LiquidationResult {
            position_id: position.position_id.clone(),
            liquidation_type: LiquidationType::Market,
            liquidation_price: avg_price,
            liquidated_quantity: position.quantity,
            margin_loss,
            insurance_fund_loss,
            order_status: FutureOrderStatus::Filled,
        })
    }

    async fn settle_insurance_fund_liquidation(
        &self,
        position: &PrepPosition,
        takeover: InsuranceFundTakeover,
    ) -> Result<LiquidationResult, PrepCmdError> {
        Ok(LiquidationResult {
            position_id: position.position_id.clone(),
            liquidation_type: LiquidationType::InsuranceFund,
            liquidation_price: position.liquidation_price.unwrap_or(Price::from_raw(0)),
            liquidated_quantity: position.quantity,
            margin_loss: position.margin,
            insurance_fund_loss: takeover.total_loss,
            order_status: FutureOrderStatus::Filled, // TODO: 使用新状态 NewInsurance
        })
    }

    async fn settle_adl_liquidation(
        &self,
        position: &PrepPosition,
        _adl_result: ADLResult,
    ) -> Result<LiquidationResult, PrepCmdError> {
        Ok(LiquidationResult {
            position_id: position.position_id.clone(),
            liquidation_type: LiquidationType::ADL,
            liquidation_price: position.liquidation_price.unwrap_or(Price::from_raw(0)),
            liquidated_quantity: position.quantity,
            margin_loss: position.margin,
            insurance_fund_loss: Price::from_raw(0),
            order_status: FutureOrderStatus::Filled, // TODO: 使用新状态 NewADL
        })
    }

    async fn notify_adl_counterparty(&self, position_id: &PositionId) -> Result<(), PrepCmdError> {
        // 发送通知给被ADL的用户
        log::info!("📧 Sending ADL notification to position {}", position_id);
        // TODO: 实际实现：发送邮件/推送通知
        Ok(())
    }
}

/// 辅助函数：计算强平价格
///
/// # 参数
/// - `entry_price`: 开仓价格
/// - `leverage`: 杠杆倍数
/// - `side`: 持仓方向
///
/// # 返回
/// 强平价格
///
/// # 公式（参考XPDL 2329-2331）
/// - 维持保证金率: 0.5%
/// - 强平手续费率: 0.5%
///
/// 多仓强平价 = 开仓价 × (1 - 1/杠杆 + 0.005 + 0.005)
/// 空仓强平价 = 开仓价 × (1 + 1/杠杆 - 0.005 - 0.005)
pub fn calculate_liquidation_price(
    entry_price: Price,
    leverage: u8,
    position_side: PositionSide,
) -> Price {
    const MAINTENANCE_MARGIN_RATE: f64 = 0.005; // 0.5%
    const LIQUIDATION_FEE_RATE: f64 = 0.005; // 0.5%

    let entry = entry_price.to_f64();
    let leverage_factor = 1.0 / leverage as f64;

    let liq_price = match position_side {
        PositionSide::Long => {
            // 多仓强平价
            entry * (1.0 - leverage_factor + MAINTENANCE_MARGIN_RATE + LIQUIDATION_FEE_RATE)
        }
        PositionSide::Short => {
            // 空仓强平价
            entry * (1.0 + leverage_factor - MAINTENANCE_MARGIN_RATE - LIQUIDATION_FEE_RATE)
        }
        PositionSide::Both => {
            // 单向模式：假设是多仓
            entry * (1.0 - leverage_factor + MAINTENANCE_MARGIN_RATE + LIQUIDATION_FEE_RATE)
        }
    };

    Price::from_f64(liq_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_liquidation_price_long() {
        // 多仓开仓价 50000 USDT，10倍杠杆
        // 强平价 = 50000 × (1 - 1/10 + 0.005 + 0.005) = 50000 × 0.91 = 45500
        let entry_price = Price::from_f64(50000.0);
        let leverage = 10;
        let liq_price = calculate_liquidation_price(entry_price, leverage, PositionSide::Long);

        assert!((liq_price.to_f64() - 45500.0).abs() < 1.0);
    }

    #[test]
    fn test_calculate_liquidation_price_short() {
        // 空仓开仓价 50000 USDT，10倍杠杆
        // 强平价 = 50000 × (1 + 1/10 - 0.005 - 0.005) = 50000 × 1.09 = 54500
        let entry_price = Price::from_f64(50000.0);
        let leverage = 10;
        let liq_price = calculate_liquidation_price(entry_price, leverage, PositionSide::Short);

        assert!((liq_price.to_f64() - 54500.0).abs() < 1.0);
    }
}
