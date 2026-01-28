// 手续费计算模式枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeeCalculationMode {
    Fixed,          // 固定费率
    Tiered,         // 分层费率
    MarketMaker,    // 做市商优惠
    VIP,            // VIP等级费率
}

// 手续费类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeeType {
    Taker,          // 吃单手续费
    Maker,          // 挂单手续费
    Withdrawal,     // 提现手续费
    Deposit,        // 充值手续费
    Transfer,       // 转账手续费
}

// 资产类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    Crypto(String), // 加密货币
    Fiat(String),   // 法币
    Stablecoin,     // 稳定币
}




use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::InstrumentType;

/// 产品特定的分层费率配置
#[derive(Debug, Clone)]
pub struct ProductTierConfig {
    /// 分层ID
    pub tier_id: u32,
    /// 分层名称
    pub tier_name: String,
    /// 30天最小交易量（用于判断用户属于哪个分层）
    pub min_volume_30d: f64,
    /// 该分层的做市商费率
    pub maker_fee: f64,
    /// 该分层的吃单费率
    pub taker_fee: f64,
    /// 是否激活
    pub is_active: bool,
}

/// 产品特定的VIP等级配置
#[derive(Debug, Clone)]
pub struct ProductVIPLevel {
    /// VIP等级ID
    pub level_id: u32,
    /// 等级名称
    pub level_name: String,
    /// 费率折扣（0.0 - 1.0）
    pub fee_discount: f64,
    /// 特殊权益
    pub special_benefits: Vec<String>,
}

/// 产品特定的促销活动配置
#[derive(Debug, Clone)]
pub struct ProductPromotion {
    /// 活动ID
    pub promotion_id: String,
    /// 活动描述
    pub description: String,
    /// 折扣率（0.0 - 1.0）
    pub discount_rate: f64,
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
    /// 目标用户
    pub target_users: Vec<String>,
    /// 参与条件
    pub conditions: Vec<String>,
}

/// 产品特定费率配置
///
/// 为不同的金融产品（现货、永续、期货、期权等）配置差异化的费率
#[derive(Debug, Clone)]
pub struct ProductFeeConfig {
    /// 产品类型
    pub instrument_type: InstrumentType,

    /// 默认做市商费率（Maker）
    pub default_maker_fee: f64,

    /// 默认吃单费率（Taker）
    pub default_taker_fee: f64,

    /// 结算费率（仅合约类产品，交割时收取）
    pub settlement_fee: Option<f64>,

    /// 资金费率上限（仅永续合约）
    pub funding_rate_cap: Option<f64>,

    /// 最小交易手续费（基础资产计价）
    pub min_trading_fee: f64,

    /// 产品级分层费率配置
    pub product_tiers: Vec<ProductTierConfig>,

    /// 产品级VIP等级配置
    pub product_vip_levels: Vec<ProductVIPLevel>,

    /// 产品级促销活动配置
    pub product_promotions: Vec<ProductPromotion>,
}

impl ProductFeeConfig {
    /// 创建现货产品费率配置
    ///
    /// 特点：无到期日，无杠杆，即时交割
    pub fn spot(maker_fee: f64, taker_fee: f64) -> Self {
        Self {
            instrument_type: InstrumentType::Spot,
            default_maker_fee: maker_fee,
            default_taker_fee: taker_fee,
            settlement_fee: None,
            funding_rate_cap: None,
            min_trading_fee: 0.0001,
            product_tiers: Vec::new(),
            product_vip_levels: Vec::new(),
            product_promotions: Vec::new(),
        }
    }

    /// 创建永续合约费率配置
    ///
    /// 特点：无到期日，支持杠杆，有资金费率
    pub fn perpetual(maker_fee: f64, taker_fee: f64, funding_cap: f64) -> Self {
        Self {
            instrument_type: InstrumentType::Perpetual,
            default_maker_fee: maker_fee,
            default_taker_fee: taker_fee,
            settlement_fee: None,
            funding_rate_cap: Some(funding_cap),
            min_trading_fee: 0.00005,
            product_tiers: Vec::new(),
            product_vip_levels: Vec::new(),
            product_promotions: Vec::new(),
        }
    }

    /// 创建交割合约费率配置
    ///
    /// 特点：有固定到期日，支持杠杆，到期交割
    pub fn futures(maker_fee: f64, taker_fee: f64, settlement_fee: f64) -> Self {
        Self {
            instrument_type: InstrumentType::Futures,
            default_maker_fee: maker_fee,
            default_taker_fee: taker_fee,
            settlement_fee: Some(settlement_fee),
            funding_rate_cap: None,
            min_trading_fee: 0.00005,
            product_tiers: Vec::new(),
            product_vip_levels: Vec::new(),
            product_promotions: Vec::new(),
        }
    }

    /// 创建期权费率配置
    ///
    /// 特点：有到期日，支付权利金，行权交割
    pub fn options(maker_fee: f64, taker_fee: f64) -> Self {
        Self {
            instrument_type: InstrumentType::Options,
            default_maker_fee: maker_fee,
            default_taker_fee: taker_fee,
            settlement_fee: None,
            funding_rate_cap: None,
            min_trading_fee: 0.0001,
            product_tiers: Vec::new(),
            product_vip_levels: Vec::new(),
            product_promotions: Vec::new(),
        }
    }

    /// 添加分层费率配置
    pub fn add_tier(&mut self, tier: ProductTierConfig) {
        self.product_tiers.push(tier);
    }

    /// 添加VIP等级配置
    pub fn add_vip_level(&mut self, vip_level: ProductVIPLevel) {
        self.product_vip_levels.push(vip_level);
    }

    /// 添加促销活动配置
    pub fn add_promotion(&mut self, promotion: ProductPromotion) {
        self.product_promotions.push(promotion);
    }

    /// 计算产品交易手续费
    pub fn calculate_trading_fee(
        &self,
        fee_type: FeeType,
        quote_asset: &str,
        quantity: f64,
        price: f64,
        user_volume_30d: Option<f64>,
        user_vip_level: Option<u32>,
        is_market_maker: bool,
    ) -> Result<FeeCalculationResult, FeeError> {
        let trade_value = quantity * price;

        // 1. 确定基础费率
        let base_rate = match fee_type {
            FeeType::Maker => {
                if is_market_maker {
                    self.default_maker_fee * 0.5
                } else {
                    self.get_tiered_fee(user_volume_30d, user_vip_level, FeeType::Maker)
                }
            },
            FeeType::Taker => {
                self.get_tiered_fee(user_volume_30d, user_vip_level, FeeType::Taker)
            },
            _ => return Err(FeeError::InvalidFeeTypeForTrade),
        };

        // 2. 应用促销折扣
        let discount_rate = self.get_promotion_discount();
        let final_rate = base_rate * (1.0 - discount_rate);

        // 3. 计算手续费金额
        let fee_amount = trade_value * final_rate;

        // 4. 应用最小值
        let final_fee = fee_amount.max(self.min_trading_fee);

        Ok(FeeCalculationResult {
            base_rate,
            discount_rate,
            final_rate,
            fee_amount: final_fee,
            fee_asset: quote_asset.to_string(),
            calculation_details: format!(
                "[{}] Base: {:.6}%, Discount: {:.2}%, Final: {:.6}%",
                self.instrument_type,
                base_rate * 100.0,
                discount_rate * 100.0,
                final_rate * 100.0
            ),
        })
    }

    /// 获取分层费率（私有方法）
    fn get_tiered_fee(
        &self,
        user_volume_30d: Option<f64>,
        user_vip_level: Option<u32>,
        fee_type: FeeType,
    ) -> f64 {
        // 1. 根据交易量找到对应的分层
        let mut base_fee = match fee_type {
            FeeType::Maker => self.default_maker_fee,
            FeeType::Taker => self.default_taker_fee,
            _ => self.default_taker_fee,
        };

        if let Some(volume) = user_volume_30d {
            // 查找最高的满足条件的分层（倒序搜索，从高到低）
            for tier in self.product_tiers.iter().rev() {
                if tier.is_active && volume >= tier.min_volume_30d {
                    base_fee = match fee_type {
                        FeeType::Maker => tier.maker_fee,
                        FeeType::Taker => tier.taker_fee,
                        _ => tier.taker_fee,
                    };
                    break;
                }
            }
        }

        // 2. 应用VIP折扣
        if let Some(vip_level) = user_vip_level {
            if let Some(vip_config) = self.product_vip_levels.iter()
                .find(|vip| vip.level_id == vip_level) {
                base_fee *= 1.0 - vip_config.fee_discount;
            }
        }

        base_fee
    }

    /// 获取促销折扣（私有方法）
    fn get_promotion_discount(&self) -> f64 {
        let now = Utc::now();
        let mut max_discount: f64 = 0.0;

        for promotion in &self.product_promotions {
            if now >= promotion.start_time && now <= promotion.end_time {
                max_discount = max_discount.max(promotion.discount_rate);
            }
        }

        max_discount
    }
}


// 核心手续费实体
#[derive(Debug, Clone)]
pub struct CexFeeEntity {
    // 产品特定费率配置
    pub product_fee_configs: HashMap<InstrumentType, ProductFeeConfig>, // 产品特定费率
}

impl CexFeeEntity {
    // 创建新的手续费实体
    pub fn new() -> Self {
        Self {
            product_fee_configs: HashMap::new(),
        }
    }

    /// 添加产品费率配置
    pub fn add_product_config(&mut self, config: ProductFeeConfig) {
        self.product_fee_configs.insert(config.instrument_type, config);
    }

    /// 获取产品的基础费率配置
    pub fn get_product_config(&self, instrument_type: InstrumentType) -> Option<&ProductFeeConfig> {
        self.product_fee_configs.get(&instrument_type)
    }

    /// 计算特定产品的交易手续费
    ///
    /// 委托给产品配置进行计算
    pub fn calculate_product_trading_fee(
        &self,
        instrument_type: InstrumentType,
        fee_type: FeeType,
        _base_asset: &str,
        quote_asset: &str,
        quantity: f64,
        price: f64,
        user_volume_30d: Option<f64>,
        user_vip_level: Option<u32>,
        is_market_maker: bool,
    ) -> Result<FeeCalculationResult, FeeError> {
        let product_config = self.get_product_config(instrument_type)
            .ok_or(FeeError::CalculationError(
                format!("Product type {:?} not configured", instrument_type)
            ))?;

        product_config.calculate_trading_fee(
            fee_type,
            quote_asset,
            quantity,
            price,
            user_volume_30d,
            user_vip_level,
            is_market_maker,
        )
    }
}

// 手续费计算结果
#[derive(Debug, Clone)]
pub struct FeeCalculationResult {
    pub base_rate: f64,           // 基础费率
    pub discount_rate: f64,      // 折扣率
    pub final_rate: f64,         // 最终费率
    pub fee_amount: f64,         // 手续费金额
    pub fee_asset: String,       // 手续费资产
    pub calculation_details: String, // 计算详情
}

// 手续费错误类型
#[derive(Debug, PartialEq)]
pub enum FeeError {
    InvalidFeeTypeForTrade,
    InsufficientBalance,
    AmountBelowMinimum,
    AmountAboveMaximum,
    AssetNotSupported,
    CalculationError(String),
}

impl std::fmt::Display for FeeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeError::InvalidFeeTypeForTrade => write!(f, "Invalid fee type for v1 operation"),
            FeeError::InsufficientBalance => write!(f, "Insufficient balance for fee calculation"),
            FeeError::AmountBelowMinimum => write!(f, "Amount below minimum fee threshold"),
            FeeError::AmountAboveMaximum => write!(f, "Amount above maximum fee threshold"),
            FeeError::AssetNotSupported => write!(f, "Asset not supported for fee calculation"),
            FeeError::CalculationError(msg) => write!(f, "Fee calculation error: {}", msg),
        }
    }
}

impl std::error::Error for FeeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_fee_config_creation() {
        let spot_config = ProductFeeConfig::spot(0.001, 0.001);
        assert_eq!(spot_config.instrument_type, InstrumentType::Spot);
        assert_eq!(spot_config.default_maker_fee, 0.001);
        assert_eq!(spot_config.default_taker_fee, 0.001);
        assert_eq!(spot_config.settlement_fee, None);
        assert_eq!(spot_config.funding_rate_cap, None);

        let perp_config = ProductFeeConfig::perpetual(0.0002, 0.0005, 0.0075);
        assert_eq!(perp_config.instrument_type, InstrumentType::Perpetual);
        assert_eq!(perp_config.default_maker_fee, 0.0002);
        assert_eq!(perp_config.default_taker_fee, 0.0005);
        assert_eq!(perp_config.funding_rate_cap, Some(0.0075));

        let futures_config = ProductFeeConfig::futures(0.0002, 0.0005, 0.0001);
        assert_eq!(futures_config.instrument_type, InstrumentType::Futures);
        assert_eq!(futures_config.settlement_fee, Some(0.0001));

        let options_config = ProductFeeConfig::options(0.0005, 0.001);
        assert_eq!(options_config.instrument_type, InstrumentType::Options);
    }

    #[test]
    fn test_multi_product_fee_calculation() {
        let mut fee_entity = CexFeeEntity::new();

        // 添加现货配置
        fee_entity.add_product_config(
            ProductFeeConfig::spot(0.001, 0.001)
        );

        // 添加永续合约配置
        fee_entity.add_product_config(
            ProductFeeConfig::perpetual(0.0002, 0.0005, 0.0075)
        );

        // 添加交割合约配置
        fee_entity.add_product_config(
            ProductFeeConfig::futures(0.0002, 0.0005, 0.0001)
        );

        // 添加期权配置
        fee_entity.add_product_config(
            ProductFeeConfig::options(0.0005, 0.001)
        );

        // 测试现货费用计算
        let spot_result = fee_entity.calculate_product_trading_fee(
            InstrumentType::Spot,
            FeeType::Taker,
            "BTC",
            "USDT",
            1.0,
            50000.0,
            None,
            None,
            false,
        ).unwrap();

        assert_eq!(spot_result.base_rate, 0.001);
        assert_eq!(spot_result.fee_amount, 50.0); // 50000 * 0.001

        // 测试永续合约费用计算
        let perp_result = fee_entity.calculate_product_trading_fee(
            InstrumentType::Perpetual,
            FeeType::Taker,
            "BTC",
            "USDT",
            1.0,
            50000.0,
            None,
            None,
            false,
        ).unwrap();

        assert_eq!(perp_result.base_rate, 0.0005);
        assert_eq!(perp_result.fee_amount, 25.0); // 50000 * 0.0005

        // 测试交割合约费用计算
        let futures_result = fee_entity.calculate_product_trading_fee(
            InstrumentType::Futures,
            FeeType::Maker,
            "BTC",
            "USDT",
            1.0,
            50000.0,
            None,
            None,
            false,
        ).unwrap();

        assert_eq!(futures_result.base_rate, 0.0002);
        assert_eq!(futures_result.fee_amount, 10.0); // 50000 * 0.0002

        // 测试期权费用计算（作为做市商）
        let options_result = fee_entity.calculate_product_trading_fee(
            InstrumentType::Options,
            FeeType::Maker,
            "BTC",
            "USDT",
            1.0,
            50000.0,
            None,
            None,
            true,  // is_market_maker = true
        ).unwrap();

        assert_eq!(options_result.base_rate, 0.00025); // 0.0005 * 0.5 (做市商折扣)
        assert_eq!(options_result.fee_amount, 12.5); // 50000 * 0.00025
    }

    #[test]
    fn test_product_fee_config_min_trading_fee() {
        let mut fee_entity = CexFeeEntity::new();

        // 添加永续合约配置（最小费用较低）
        fee_entity.add_product_config(
            ProductFeeConfig::perpetual(0.0002, 0.0005, 0.0075)
        );

        // 测试小额交易的最小费用限制
        let result = fee_entity.calculate_product_trading_fee(
            InstrumentType::Perpetual,
            FeeType::Taker,
            "BTC",
            "USDT",
            0.001,
            10.0, // 极小金额：0.001 * 10 = 0.01
            None,
            None,
            false,
        ).unwrap();

        // 应用最小费用 0.00005
        assert_eq!(result.fee_amount, 0.00005);
    }

    #[test]
    fn test_product_config_not_found() {
        let fee_entity = CexFeeEntity::new();

        // 未配置的产品应该返回错误
        let result = fee_entity.calculate_product_trading_fee(
            InstrumentType::Spot,
            FeeType::Taker,
            "BTC",
            "USDT",
            1.0,
            50000.0,
            None,
            None,
            false,
        );

        assert!(result.is_err());
        match result {
            Err(FeeError::CalculationError(msg)) => {
                assert!(msg.contains("not configured"));
            },
            _ => panic!("Expected CalculationError"),
        }
    }
}
