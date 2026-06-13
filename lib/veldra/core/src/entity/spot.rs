use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{ProductPluginError, stable_hash_hex};

/// 订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpotOrderSide {
    Buy,
    Sell,
}

impl SpotOrderSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// 账户某个资产的业务余额快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBalanceSnapshot {
    /// 资产代码，例如 `USDT`。
    pub asset: String,
    /// 可用余额。
    pub available: u64,
    /// 已冻结余额。
    pub reserved: u64,
    /// 乐观锁版本。
    pub version: u64,
}

impl SpotBalanceSnapshot {
    /// 返回冻结指定数量后的新快照；余额不足时返回 `None`。
    pub fn reserve(&self, amount: u64) -> Option<Self> {
        if self.available < amount {
            return None;
        }
        let available = self.available.checked_sub(amount)?;
        let reserved = self.reserved.checked_add(amount)?;
        let version = self.version.checked_add(1)?;
        Some(Self { asset: self.asset.clone(), available, reserved, version })
    }

    /// 余额快照的稳定业务承诺。
    pub fn commitment(&self) -> String {
        [
            self.asset.as_str(),
            &self.available.to_string(),
            &self.reserved.to_string(),
            &self.version.to_string(),
        ]
        .join(":")
    }
}

/// 市场规则快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotMarketRules {
    /// 交易对标识。
    pub symbol: String,
    /// base 资产代码。
    pub base_asset: String,
    /// quote 资产代码。
    pub quote_asset: String,
    /// 最小价格增量的简化约束。
    pub min_price: u64,
    /// 最小数量约束。
    pub min_qty: u64,
}

impl SpotMarketRules {
    /// 返回该委托需要冻结的资产代码。
    pub fn reserve_asset(&self, side: SpotOrderSide) -> &str {
        match side {
            SpotOrderSide::Buy => self.quote_asset.as_str(),
            SpotOrderSide::Sell => self.base_asset.as_str(),
        }
    }

    /// 返回该委托要冻结的数量；溢出时返回 `None`。
    pub fn required_reserve(&self, side: SpotOrderSide, price: u64, qty: u64) -> Option<u64> {
        match side {
            SpotOrderSide::Buy => price.checked_mul(qty),
            SpotOrderSide::Sell => Some(qty),
        }
    }

    /// 市场规则快照的稳定业务承诺。
    pub fn commitment(&self) -> String {
        [
            self.symbol.as_str(),
            self.base_asset.as_str(),
            self.quote_asset.as_str(),
            &self.min_price.to_string(),
            &self.min_qty.to_string(),
        ]
        .join(":")
    }
}

/// 简化限价单快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrder {
    /// 订单 ID。
    pub order_id: String,
    /// 账户 ID。
    pub account_id: String,
    /// 交易对。
    pub symbol: String,
    /// 买卖方向。
    pub side: SpotOrderSide,
    /// 委托价格。
    pub price: u64,
    /// 委托数量。
    pub qty: u64,
    /// 客户端订单号。
    pub client_order_id: Option<String>,
}

impl SpotOrder {
    /// 订单名义价值；仅买单冻结 quote 时常用。
    pub fn notional_quote(&self) -> Option<u64> {
        self.price.checked_mul(self.qty)
    }

    /// 订单快照的稳定业务承诺。
    pub fn commitment(&self) -> String {
        [
            self.order_id.as_str(),
            self.account_id.as_str(),
            self.symbol.as_str(),
            self.side.as_str(),
            &self.price.to_string(),
            &self.qty.to_string(),
            self.client_order_id.as_deref().unwrap_or(""),
        ]
        .join(":")
    }
}

/// `spot.place_order` 的请求载荷。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotPlaceOrderPayload {
    pub account_id: String,
    pub symbol: String,
    pub side: SpotOrderSide,
    pub price: u64,
    pub qty: u64,
    pub client_order_id: Option<String>,
}

/// `spot` 产品上下文。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotProductContext {
    /// 当前请求所属账户。
    pub account_id: String,
    /// 账户余额快照，按资产代码索引。
    pub balances: BTreeMap<String, SpotBalanceSnapshot>,
    /// 目标市场规则。
    pub market_rules: SpotMarketRules,
    /// 生成稳定订单 ID 的下一个序号。
    pub next_order_sequence: u64,
    /// 市场是否允许下单。
    pub trading_enabled: bool,
}

impl SpotProductContext {
    /// 返回指定资产余额快照。
    pub fn balance(&self, asset: &str) -> Option<&SpotBalanceSnapshot> {
        self.balances.get(asset)
    }

    /// 应用一次 `spot.place_order` 结果后的新上下文。
    pub fn apply_place_order(
        &mut self,
        result: &SpotPlaceOrderResult,
    ) -> Result<(), ProductPluginError> {
        self.next_order_sequence = self
            .next_order_sequence
            .checked_add(1)
            .ok_or(ProductPluginError::ArithmeticOverflow)?;
        self.balances.insert(
            result.affected_balance_after.asset.clone(),
            result.affected_balance_after.clone(),
        );
        Ok(())
    }

    /// 上下文的稳定业务承诺。
    pub fn commitment(&self) -> String {
        let balances = self
            .balances
            .values()
            .map(SpotBalanceSnapshot::commitment)
            .collect::<Vec<_>>()
            .join("|");
        let parts = vec![
            self.account_id.clone(),
            self.market_rules.commitment(),
            self.next_order_sequence.to_string(),
            self.trading_enabled.to_string(),
            balances,
        ];
        stable_hash_hex(&parts)
    }
}

/// `spot.place_order` 的业务结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotPlaceOrderResult {
    /// 原始 request id。
    pub request_id: String,
    /// 新创建的订单。
    pub order: SpotOrder,
    /// 受影响资产的下单前余额。
    pub affected_balance_before: SpotBalanceSnapshot,
    /// 受影响资产的下单后余额。
    pub affected_balance_after: SpotBalanceSnapshot,
}

impl SpotPlaceOrderResult {
    /// 结果的稳定业务承诺。
    pub fn commitment(&self) -> String {
        let parts = vec![
            self.request_id.clone(),
            self.order.commitment(),
            self.affected_balance_after.commitment(),
        ];
        stable_hash_hex(&parts)
    }
}

/// `spot` 产品插件。
#[derive(Debug, Clone, Copy, Default)]
pub struct SpotProductPlugin;

impl SpotProductPlugin {
    /// 执行 `spot.place_order`。
    pub fn place_order(
        &self,
        request_id: &str,
        payload: SpotPlaceOrderPayload,
        context: &SpotProductContext,
    ) -> Result<SpotPlaceOrderResult, ProductPluginError> {
        if !context.trading_enabled {
            return Err(ProductPluginError::MarketClosed);
        }
        if payload.account_id != context.account_id {
            return Err(ProductPluginError::AccountMismatch {
                expected: context.account_id.clone(),
                actual: payload.account_id,
            });
        }
        if payload.symbol != context.market_rules.symbol {
            return Err(ProductPluginError::SymbolMismatch {
                expected: context.market_rules.symbol.clone(),
                actual: payload.symbol,
            });
        }
        if payload.price < context.market_rules.min_price {
            return Err(ProductPluginError::InvalidPrice { price: payload.price });
        }
        if payload.qty < context.market_rules.min_qty {
            return Err(ProductPluginError::InvalidQty { qty: payload.qty });
        }

        let reserve_asset = context.market_rules.reserve_asset(payload.side).to_string();
        let reserve_amount = context
            .market_rules
            .required_reserve(payload.side, payload.price, payload.qty)
            .ok_or(ProductPluginError::ArithmeticOverflow)?;
        let affected_balance_before = context
            .balance(&reserve_asset)
            .cloned()
            .ok_or_else(|| ProductPluginError::MissingBalance { asset: reserve_asset.clone() })?;
        let affected_balance_after = affected_balance_before.reserve(reserve_amount).ok_or(
            ProductPluginError::InsufficientBalance {
                asset: reserve_asset.clone(),
                required: reserve_amount,
                available: affected_balance_before.available,
            },
        )?;

        let order_id =
            format!("{}-{}-{}", payload.account_id, payload.symbol, context.next_order_sequence);
        let order = SpotOrder {
            order_id,
            account_id: payload.account_id,
            symbol: payload.symbol,
            side: payload.side,
            price: payload.price,
            qty: payload.qty,
            client_order_id: payload.client_order_id,
        };
        Ok(SpotPlaceOrderResult {
            request_id: request_id.to_string(),
            order,
            affected_balance_before,
            affected_balance_after,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_context() -> SpotProductContext {
        let mut balances = BTreeMap::new();
        balances.insert(
            "USDT".to_string(),
            SpotBalanceSnapshot {
                asset: "USDT".to_string(),
                available: 10_000,
                reserved: 0,
                version: 7,
            },
        );
        balances.insert(
            "BTC".to_string(),
            SpotBalanceSnapshot { asset: "BTC".to_string(), available: 5, reserved: 0, version: 3 },
        );
        SpotProductContext {
            account_id: "acct-1".to_string(),
            balances,
            market_rules: SpotMarketRules {
                symbol: "BTCUSDT".to_string(),
                base_asset: "BTC".to_string(),
                quote_asset: "USDT".to_string(),
                min_price: 1,
                min_qty: 1,
            },
            next_order_sequence: 9,
            trading_enabled: true,
        }
    }

    fn buy_payload() -> SpotPlaceOrderPayload {
        SpotPlaceOrderPayload {
            account_id: "acct-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: SpotOrderSide::Buy,
            price: 100,
            qty: 3,
            client_order_id: Some("cl-1".to_string()),
        }
    }

    #[test]
    fn place_order_happy_path_creates_order_and_reserves_quote_balance() {
        let plugin = SpotProductPlugin;
        let result = plugin.place_order("req-1", buy_payload(), &sample_context()).unwrap();

        assert_eq!(result.order.order_id, "acct-1-BTCUSDT-9");
        assert_eq!(result.order.side, SpotOrderSide::Buy);
        assert_eq!(result.affected_balance_before.available, 10_000);
        assert_eq!(result.affected_balance_after.available, 9_700);
        assert_eq!(result.affected_balance_after.reserved, 300);
        assert_eq!(result.affected_balance_after.version, 8);
    }

    #[test]
    fn place_order_rejects_when_market_closed() {
        let plugin = SpotProductPlugin;
        let mut context = sample_context();
        context.trading_enabled = false;

        let result = plugin.place_order("req-1", buy_payload(), &context);

        assert!(matches!(result, Err(ProductPluginError::MarketClosed)));
    }

    #[test]
    fn place_order_rejects_invalid_price_or_qty() {
        let plugin = SpotProductPlugin;
        let invalid_price = SpotPlaceOrderPayload { price: 0, ..buy_payload() };
        let invalid_qty = SpotPlaceOrderPayload { qty: 0, ..buy_payload() };

        assert!(matches!(
            plugin.place_order("req-1", invalid_price, &sample_context()),
            Err(ProductPluginError::InvalidPrice { price: 0 })
        ));
        assert!(matches!(
            plugin.place_order("req-1", invalid_qty, &sample_context()),
            Err(ProductPluginError::InvalidQty { qty: 0 })
        ));
    }

    #[test]
    fn place_order_rejects_when_balance_is_insufficient() {
        let plugin = SpotProductPlugin;
        let payload = SpotPlaceOrderPayload { price: 10_000, qty: 2, ..buy_payload() };

        let result = plugin.place_order("req-1", payload, &sample_context());

        assert!(matches!(
            result,
            Err(ProductPluginError::InsufficientBalance { asset, required: 20_000, available: 10_000 })
            if asset == "USDT"
        ));
    }
}
