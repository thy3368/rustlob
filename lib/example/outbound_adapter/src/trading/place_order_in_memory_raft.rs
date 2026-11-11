use std::sync::{Arc, Mutex};

use base_types_v2::{Coin, Symbol};
use common_entity::{
    Entity, EntityReplayableEvent, StateSink, StateSource, stable_string_entity_id,
};
use example_core_use_case::{
    ActivateMatchSpotOrderV2Cmd, ActivateMatchSpotOrderV2State, ActivateMatchSpotOrderV2UseCase,
    BalanceV2, MarketRules, OrderGroupRelation, OrderSide, OrderTif, OrderType,
    PendingTriggerSpotOrderSource, PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase,
    PlaceOnlySpotOrderV2Cmd, SpotOrderStatus, SpotOrderStatusReason, SpotOrderV2, SpotTrade,
};
use lob_repo_v2::lob::AssetLobV2;
use rust_decimal::Decimal;

use crate::shared::{
    InMemorySpotPipelineBroker, InMemoryStore, PlaceOrderOutboundError, SpotOrderPlacedMessage,
    SpotPipelineBroker, SpotPipelineMessage, balance_key, event_identity_field, event_string_field,
    event_u64_field,
};

#[derive(Debug, Clone)]
pub struct InMemoryRaftCluster {
    stores: Arc<Vec<InMemoryStore>>,
    leader: Arc<Mutex<u64>>,
    committed: Arc<Mutex<u64>>,
}

impl InMemoryRaftCluster {
    pub fn new(nodes: usize, symbol: Symbol) -> Self {
        let count = nodes.max(1);
        Self {
            stores: Arc::new((0..count).map(|_| InMemoryStore::new(symbol)).collect()),
            leader: Arc::new(Mutex::new(0)),
            committed: Arc::new(Mutex::new(0)),
        }
    }

    pub fn three_node(symbol: Symbol) -> Self {
        Self::new(3, symbol)
    }
    pub fn node_count(&self) -> usize {
        self.stores.len()
    }
    pub fn leader(&self) -> u64 {
        self.leader.lock().map(|v| *v).unwrap_or(0)
    }
    pub fn set_leader(&self, node_id: u64) -> Result<(), PlaceOrderOutboundError> {
        if node_id as usize >= self.stores.len() {
            return Err(PlaceOrderOutboundError::Raft("unknown leader".into()));
        }
        *self
            .leader
            .lock()
            .map_err(|_| PlaceOrderOutboundError::Raft("leader lock poisoned".into()))? = node_id;
        Ok(())
    }
    pub fn committed_index(&self) -> u64 {
        self.committed.lock().map(|v| *v).unwrap_or(0)
    }
    pub fn snapshot(&self, node_id: u64) -> Result<crate::StoreSnapshot, PlaceOrderOutboundError> {
        self.stores
            .get(node_id as usize)
            .ok_or_else(|| PlaceOrderOutboundError::Raft("unknown node".into()))?
            .snapshot()
            .map_err(Into::into)
    }
    fn store(&self, node_id: u64) -> Result<&InMemoryStore, PlaceOrderOutboundError> {
        self.stores
            .get(node_id as usize)
            .ok_or_else(|| PlaceOrderOutboundError::Raft("unknown node".into()))
    }
    fn commit(
        &self,
        node_id: u64,
        events: &[EntityReplayableEvent],
    ) -> Result<u64, PlaceOrderOutboundError> {
        if self.leader() != node_id {
            return Err(PlaceOrderOutboundError::Raft("proposal sent to non-leader".into()));
        }
        let next = self
            .committed_index()
            .checked_add(1)
            .ok_or_else(|| PlaceOrderOutboundError::Raft("log index overflow".into()))?;
        for store in self.stores.iter() {
            let mut state = store.lock_state()?;
            state.persisted_events.extend(events.iter().cloned());
        }
        for (replica_id, store) in self.stores.iter().enumerate() {
            let replica = InMemoryPlaceOrderRaftOutbound {
                store: store.clone(),
                broker: None,
                cluster: self.clone(),
                node_id: replica_id as u64,
            };
            replica.replay_events(events)?;
        }
        *self
            .committed
            .lock()
            .map_err(|_| PlaceOrderOutboundError::Raft("commit lock poisoned".into()))? = next;
        Ok(next)
    }
}

impl Default for InMemoryRaftCluster {
    fn default() -> Self {
        Self::three_node(Symbol::new(1, Coin::BTC, Coin::USDT))
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryPlaceOrderRaftOutbound {
    store: InMemoryStore,
    broker: Option<InMemorySpotPipelineBroker>,
    cluster: InMemoryRaftCluster,
    node_id: u64,
}

impl InMemoryPlaceOrderRaftOutbound {
    pub fn new() -> Self {
        Self::from_cluster(InMemoryRaftCluster::default(), 0)
    }
    pub fn from_cluster(cluster: InMemoryRaftCluster, node_id: u64) -> Self {
        let store = cluster.store(node_id).cloned().unwrap_or_default();
        Self { store, broker: None, cluster, node_id }
    }

    pub fn from_store(store: InMemoryStore) -> Self {
        let cluster = InMemoryRaftCluster::new(1, Symbol::new(1, Coin::BTC, Coin::USDT));
        Self { store, broker: None, cluster, node_id: 0 }
    }

    pub fn from_store_with_broker(
        store: InMemoryStore,
        broker: InMemorySpotPipelineBroker,
    ) -> Self {
        let cluster = InMemoryRaftCluster::new(1, Symbol::new(1, Coin::BTC, Coin::USDT));
        Self { store, broker: Some(broker), cluster, node_id: 0 }
    }

    pub fn seeded(
        base_balance: BalanceV2,
        quote_balance: BalanceV2,
        market_rules: MarketRules,
    ) -> Result<Self, crate::StoreError> {
        Ok(Self::from_store(InMemoryStore::seed_balances(
            base_balance,
            quote_balance,
            market_rules,
        )?))
    }

    pub fn seed_balance(&self, balance: BalanceV2) -> Result<(), crate::StoreError> {
        self.store.seed_balance(balance)
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), crate::StoreError> {
        self.store.seed_market_rules(market_rules)
    }

    pub fn seed_order(&self, order: SpotOrderV2) -> Result<(), crate::StoreError> {
        self.store.seed_order(order)
    }

    pub fn snapshot(&self) -> Result<crate::StoreSnapshot, crate::StoreError> {
        self.store.snapshot()
    }

    fn replay_events(
        &self,
        events: &[EntityReplayableEvent],
    ) -> Result<(), PlaceOrderOutboundError> {
        let mut state = self.store.lock_state()?;

        for event in events {
            if event.is_created() && event_string_field(event, "order_id").is_some() {
                let order = decode_order_from_event(event)?;
                state.orders.insert(order.order_id, order.clone());
                let lob_event = event.clone();
                state
                    .lob
                    .replay_events(&[lob_event])
                    .map_err(|_| PlaceOrderOutboundError::EventDecodeFailed)?;
                state.next_order_sequence = state
                    .next_order_sequence
                    .checked_add(1)
                    .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
                continue;
            }

            if event.is_created() && event_string_field(event, "trade_id").is_some() {
                let trade = decode_created_trade(event)?;
                state.trades.insert(trade.trade_id.clone(), trade);
                continue;
            }

            if event.entity_type == SpotOrderV2::entity_type() && event.is_updated() {
                apply_order_update_event(&mut state.orders, event)?;
                let lob_event = event.clone();
                state
                    .lob
                    .replay_events(&[lob_event])
                    .map_err(|_| PlaceOrderOutboundError::EventDecodeFailed)?;
                state.next_order_sequence = state
                    .next_order_sequence
                    .checked_add(1)
                    .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
                continue;
            }

            if event.is_updated() && event_u64_field(event, "coin_idx").is_some() {
                let account_id = event_string_field(event, "account_id")
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
                let coin_idx = event_identity_field(event, "coin_idx")
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
                let account_numeric_id = stable_string_entity_id(account_id.as_str());
                let key = balance_key(account_numeric_id, coin_idx);
                let balance = state.balances.entry(key).or_insert_with(|| {
                    BalanceV2::new(account_numeric_id, account_numeric_id, coin_idx, 0, 0, 0, 0)
                });
                let available = event_u64_field(event, "available")
                    .map(rust_decimal::Decimal::from)
                    .unwrap_or_else(|| balance.available());
                let frozen = event_u64_field(event, "frozen")
                    .map(rust_decimal::Decimal::from)
                    .unwrap_or_else(|| balance.frozen());
                *balance = BalanceV2::new(
                    balance.party_id(),
                    balance.account_id(),
                    balance.coin_idx(),
                    available,
                    frozen,
                    event.lsn,
                    balance.created_at(),
                );
            }
        }

        Ok(())
    }
}

const DEFAULT_FEE_ACCOUNT_ID: &str = "fee";

impl PendingTriggerSpotOrderSource for InMemoryPlaceOrderRaftOutbound {
    type Error = PlaceOrderOutboundError;

    fn load_pending_trigger_orders(
        &self,
        asset: u32,
        market_price: u64,
    ) -> Result<Vec<SpotOrderV2>, Self::Error> {
        let state = self.store.lock_state()?;
        let mut orders = state
            .orders
            .values()
            .filter(|order| {
                order.trades_asset(asset) && order.is_triggered_by_market_price(market_price)
            })
            .cloned()
            .collect::<Vec<_>>();
        orders.sort_by_key(|order| (order.created_at, order.order_id));
        Ok(orders)
    }
}

impl StateSource<ActivateMatchSpotOrderV2UseCase> for InMemoryPlaceOrderRaftOutbound {
    type Error = PlaceOrderOutboundError;

    fn load_given_state(
        &self,
        cmd: &LogEntry<ActivateMatchSpotOrderV2Cmd>,
    ) -> Result<ActivateMatchSpotOrderV2State, Self::Error> {
        let state = self.store.lock_state()?;
        let pending_order = state
            .orders
            .get(&cmd.order_id)
            .cloned()
            .ok_or(PlaceOrderOutboundError::OrderNotFound)?;
        let market_rules = state
            .market_rules_by_symbol
            .get(pending_order.symbol())
            .cloned()
            .ok_or(PlaceOrderOutboundError::MarketRulesNotFound)?;
        let base_asset_id = base_asset_id_for(market_rules.symbol.as_str()).to_string();
        let quote_asset_id = quote_asset_id_for(market_rules.symbol.as_str()).to_string();
        let mut settlement_balances = state.balances.values().cloned().collect::<Vec<_>>();
        if !state
            .balances
            .contains_key(&balance_key(DEFAULT_FEE_ACCOUNT_ID, quote_asset_id.as_str()))
        {
            settlement_balances.push(BalanceV2::new(
                stable_string_entity_id(DEFAULT_FEE_ACCOUNT_ID),
                stable_string_entity_id(DEFAULT_FEE_ACCOUNT_ID),
                stable_string_entity_id(quote_asset_id.as_str()),
                0,
                0,
                1,
                0,
            ));
        }

        let maker_orders = state
            .lob
            .match_orders(Decimal::MAX, Decimal::MAX, pending_order.side())
            .0
            .unwrap_or_default()
            .into_iter()
            .cloned()
            .collect();

        Ok(ActivateMatchSpotOrderV2State {
            pending_order,
            maker_orders,
            settlement_balances,
            base_asset_id,
            quote_asset_id,
            fee_account_id: DEFAULT_FEE_ACCOUNT_ID.to_string(),
            maker_fee_bps: 1,
            taker_fee_bps: 1,
        })
    }
}

impl StateSink<ActivateMatchSpotOrderV2UseCase> for InMemoryPlaceOrderRaftOutbound {
    type Error = PlaceOrderOutboundError;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.cluster.commit(self.node_id, events)?;
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.replay_events(events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.published_events.extend(events.iter().cloned());
        Ok(())
    }
}

impl StateSource<PlaceMatchSpotOrderV2UseCase> for InMemoryPlaceOrderRaftOutbound {
    type Error = PlaceOrderOutboundError;

    fn load_given_state(
        &self,
        cmd: &LogEntry<PlaceOnlySpotOrderV2Cmd>,
    ) -> Result<PlaceMatchSpotOrderV2State, Self::Error> {
        let order_cmd = match cmd.payload() {
            PlaceOnlySpotOrderV2Cmd::Single(order)
            | PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent: order, .. } => order,
        };
        let state = self.store.lock_state()?;
        let symbol = symbol_for_asset(order_cmd.asset);
        let market_rules = state
            .market_rules_by_symbol
            .get(symbol)
            .cloned()
            .ok_or(PlaceOrderOutboundError::MarketRulesNotFound)?;
        let base_asset_id = base_asset_id_for(market_rules.symbol.as_str()).to_string();
        let quote_asset_id = quote_asset_id_for(market_rules.symbol.as_str()).to_string();
        let mut settlement_balances = state.balances.values().cloned().collect::<Vec<_>>();
        if !state
            .balances
            .contains_key(&balance_key(DEFAULT_FEE_ACCOUNT_ID, quote_asset_id.as_str()))
        {
            settlement_balances.push(BalanceV2::new(
                stable_string_entity_id(DEFAULT_FEE_ACCOUNT_ID),
                stable_string_entity_id(DEFAULT_FEE_ACCOUNT_ID),
                stable_string_entity_id(quote_asset_id.as_str()),
                0,
                0,
                1,
                0,
            ));
        }
        if !state
            .balances
            .contains_key(&balance_key(order_cmd.party_id.as_str(), base_asset_id.as_str()))
            || !state
                .balances
                .contains_key(&balance_key(order_cmd.party_id.as_str(), quote_asset_id.as_str()))
        {
            return Err(PlaceOrderOutboundError::BalanceNotFound);
        }

        let taker_side = if order_cmd.is_buy { OrderSide::Buy } else { OrderSide::Sell };
        let maker_orders = state
            .lob
            .match_orders(Decimal::MAX, Decimal::MAX, taker_side)
            .0
            .unwrap_or_default()
            .into_iter()
            .cloned()
            .collect();

        Ok(PlaceMatchSpotOrderV2State {
            maker_orders,
            settlement_balances,
            fee_account_id: DEFAULT_FEE_ACCOUNT_ID.to_string(),
        })
    }
}

impl StateSink<PlaceMatchSpotOrderV2UseCase> for InMemoryPlaceOrderRaftOutbound {
    type Error = PlaceOrderOutboundError;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.cluster.commit(self.node_id, events)?;
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        self.replay_events(events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.published_events.extend(events.iter().cloned());
        drop(state);

        let Some(broker) = &self.broker else {
            return Ok(());
        };

        for event in events {
            if !event.is_created() || event_string_field(event, "order_id").is_none() {
                continue;
            }

            let order_id = event_string_field(event, "order_id")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            let party_id = event_string_field(event, "account_id")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            let symbol = event_string_field(event, "symbol")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            let asset = event_u64_field(event, "asset")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)? as u32;
            broker
                .publish(SpotPipelineMessage::SpotOrderPlaced(SpotOrderPlacedMessage {
                    match_id: order_id.clone(),
                    order_id,
                    party_id,
                    symbol,
                    asset,
                    trace_id: None,
                    command_id: None,
                }))
                .map_err(|_| PlaceOrderOutboundError::BrokerPublishFailed)?;
        }
        Ok(())
    }
}

fn decode_side(event: &EntityReplayableEvent) -> Result<OrderSide, PlaceOrderOutboundError> {
    match event_string_field(event, "side").as_deref() {
        Some("buy") => Ok(OrderSide::Buy),
        Some("sell") => Ok(OrderSide::Sell),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_limit_price(event: &EntityReplayableEvent) -> Result<u64, PlaceOrderOutboundError> {
    event_u64_field(event, "limit_price")
        .or_else(|| event_u64_field(event, "price"))
        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)
}

fn decode_order_type(event: &EntityReplayableEvent) -> Result<OrderType, PlaceOrderOutboundError> {
    let tif = match event_string_field(event, "effective_tif")
        .or_else(|| event_string_field(event, "time_in_force"))
        .as_deref()
    {
        Some("gtc") => OrderTif::Gtc,
        Some("ioc") => OrderTif::Ioc,
        Some("alo") => OrderTif::Alo,
        _ => return Err(PlaceOrderOutboundError::EventDecodeFailed),
    };
    Ok(match event_string_field(event, "order_type").as_deref() {
        Some("trigger_market") => OrderType::Trigger {
            is_market: true,
            trigger_price: event_u64_field(event, "trigger_price")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
            tpsl: decode_trigger_role(event_string_field(event, "tpsl").as_deref())?,
        },
        Some("trigger_limit") => OrderType::Trigger {
            is_market: false,
            trigger_price: event_u64_field(event, "trigger_price")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
            tpsl: decode_trigger_role(event_string_field(event, "tpsl").as_deref())?,
        },
        Some("limit") | None => OrderType::Limit { tif },
        Some(_) => return Err(PlaceOrderOutboundError::EventDecodeFailed),
    })
}

fn decode_trigger_role(
    value: Option<&str>,
) -> Result<example_core_use_case::OrderTriggerRole, PlaceOrderOutboundError> {
    match value {
        Some("take_profit") => Ok(example_core_use_case::OrderTriggerRole::TakeProfit),
        Some("stop_loss") => Ok(example_core_use_case::OrderTriggerRole::StopLoss),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_status(
    event: &EntityReplayableEvent,
) -> Result<SpotOrderStatus, PlaceOrderOutboundError> {
    match event_string_field(event, "status").as_deref() {
        Some("pending") => Ok(SpotOrderStatus::Pending),
        Some("open") => Ok(SpotOrderStatus::Open),
        Some("partially_filled") => Ok(SpotOrderStatus::PartiallyFilled),
        Some("filled") => Ok(SpotOrderStatus::Filled),
        Some("canceled") => Ok(SpotOrderStatus::Canceled),
        Some("rejected") => Ok(SpotOrderStatus::Rejected),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_bool(value: &str) -> Result<bool, PlaceOrderOutboundError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_group_relation(
    value: Option<&str>,
) -> Result<OrderGroupRelation, PlaceOrderOutboundError> {
    match value {
        None | Some("") | Some("standalone") => Ok(OrderGroupRelation::Standalone),
        Some("normal_tpsl_parent") => Ok(OrderGroupRelation::NormalTpslParent),
        Some(value) => {
            let parent_order_id = value
                .strip_prefix("normal_tpsl_child:")
                .filter(|parent_order_id| !parent_order_id.is_empty())
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                .parse()
                .map_err(|_| PlaceOrderOutboundError::EventDecodeFailed)?;
            Ok(OrderGroupRelation::NormalTpslChild { parent_order_id })
        }
    }
}

fn decode_status_reason(
    value: Option<&str>,
) -> Result<Option<SpotOrderStatusReason>, PlaceOrderOutboundError> {
    let Some(value) = value.filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    let reason = match value {
        "filled" => SpotOrderStatusReason::Filled,
        "canceled" => SpotOrderStatusReason::CanceledByUser,
        "triggered" => SpotOrderStatusReason::Triggered,
        "rejected" => SpotOrderStatusReason::RejectedAtPlacement,
        "marginCanceled" => SpotOrderStatusReason::MarginCanceled,
        "vaultWithdrawalCanceled" => SpotOrderStatusReason::VaultWithdrawalCanceled,
        "openInterestCapCanceled" => SpotOrderStatusReason::OpenInterestCapCanceled,
        "selfTradeCanceled" => SpotOrderStatusReason::SelfTradeCanceled,
        "reduceOnlyCanceled" => SpotOrderStatusReason::ReduceOnlyCanceled,
        "siblingFilledCanceled" => SpotOrderStatusReason::SiblingFilledCanceled,
        "delistedCanceled" => SpotOrderStatusReason::DelistedCanceled,
        "liquidatedCanceled" => SpotOrderStatusReason::LiquidatedCanceled,
        "scheduledCancel" => SpotOrderStatusReason::ScheduledCancel,
        "tickRejected" => SpotOrderStatusReason::TickRejected,
        "minTradeNtlRejected" => SpotOrderStatusReason::MinTradeNtlRejected,
        "perpMarginRejected" => SpotOrderStatusReason::PerpMarginRejected,
        "reduceOnlyRejected" => SpotOrderStatusReason::ReduceOnlyRejected,
        "badAloPxRejected" => SpotOrderStatusReason::BadAloPxRejected,
        "iocCancelRejected" => SpotOrderStatusReason::IocCancelRejected,
        "badTriggerPxRejected" => SpotOrderStatusReason::BadTriggerPxRejected,
        "marketOrderNoLiquidityRejected" => SpotOrderStatusReason::MarketOrderNoLiquidityRejected,
        "positionIncreaseAtOpenInterestCapRejected" => {
            SpotOrderStatusReason::PositionIncreaseAtOpenInterestCapRejected
        }
        "positionFlipAtOpenInterestCapRejected" => {
            SpotOrderStatusReason::PositionFlipAtOpenInterestCapRejected
        }
        "tooAggressiveAtOpenInterestCapRejected" => {
            SpotOrderStatusReason::TooAggressiveAtOpenInterestCapRejected
        }
        "openInterestIncreaseRejected" => SpotOrderStatusReason::OpenInterestIncreaseRejected,
        "insufficientSpotBalanceRejected" => SpotOrderStatusReason::InsufficientSpotBalanceRejected,
        "oracleRejected" => SpotOrderStatusReason::OracleRejected,
        "perpMaxPositionRejected" => SpotOrderStatusReason::PerpMaxPositionRejected,
        _ => return Err(PlaceOrderOutboundError::EventDecodeFailed),
    };
    Ok(Some(reason))
}

pub fn base_asset_id_for(symbol: &str) -> &str {
    match symbol {
        "BTCUSDT" => "BTC",
        _ => "BTC",
    }
}

pub fn quote_asset_id_for(symbol: &str) -> &str {
    match symbol {
        "BTCUSDT" => "USDT",
        _ => "USDT",
    }
}

pub fn symbol_for_asset(asset: u32) -> &'static str {
    match asset {
        10_000 | 10_001 => "BTCUSDT",
        _ => "BTCUSDT",
    }
}

fn decode_created_trade(
    event: &EntityReplayableEvent,
) -> Result<SpotTrade, PlaceOrderOutboundError> {
    Ok(SpotTrade::new(
        event_string_field(event, "trade_id").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "match_id").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "asset").ok_or(PlaceOrderOutboundError::EventDecodeFailed)? as u32,
        event_string_field(event, "symbol").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "taker_order_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "maker_order_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "taker_account_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "maker_account_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        decode_trade_side(event)?,
        event_u64_field(event, "price").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "qty").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "taker_fee").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "maker_fee").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "executed_at_ms")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
    ))
}

fn decode_trade_side(event: &EntityReplayableEvent) -> Result<OrderSide, PlaceOrderOutboundError> {
    match event_string_field(event, "taker_side").as_deref() {
        Some("buy") => Ok(OrderSide::Buy),
        Some("sell") => Ok(OrderSide::Sell),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn apply_order_update_event(
    orders: &mut std::collections::HashMap<u64, SpotOrderV2>,
    event: &EntityReplayableEvent,
) -> Result<(), PlaceOrderOutboundError> {
    let order_id = order_id_from_event(event)?;
    if !orders.contains_key(&order_id) {
        return Err(PlaceOrderOutboundError::OrderNotFound);
    }
    let order = orders.get_mut(&order_id).ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    if let Some(status) = event_string_field(event, "status") {
        order.status = decode_status_value(status.as_str())?;
    }
    if let Some(filled_qty) = event_u64_field(event, "filled_qty") {
        order.filled_size = filled_qty;
    }
    if let Some(status_reason) = event_string_field(event, "status_reason") {
        order.status_reason = decode_status_reason(Some(status_reason.as_str()))?;
    }
    if let Some(reduce_only) = event_string_field(event, "reduce_only") {
        order.reduce_only = decode_bool(reduce_only.as_str())?;
    }
    if let Some(group_relation) = event_string_field(event, "group_relation") {
        order.group_relation = decode_group_relation(Some(group_relation.as_str()))?;
    }
    if let Some(updated_at) = event_u64_field(event, "updated_at") {
        order.updated_at = updated_at;
    }
    if let Some(qty) = event_u64_field(event, "qty") {
        order.size = qty;
    }
    if let Some(limit_price) = event_u64_field(event, "limit_price") {
        order.limit_price = limit_price;
    }
    if let Some(client_order_id) = event_string_field(event, "client_order_id") {
        order.cloid = (!client_order_id.is_empty()).then_some(client_order_id.clone());
    }
    order.version = event.lsn;
    Ok(())
}

fn order_id_from_event(event: &EntityReplayableEvent) -> Result<u64, PlaceOrderOutboundError> {
    if let Some(order_id) = event_u64_field(event, "order_id") {
        return Ok(order_id);
    }
    Ok(event.entity_id)
}

fn decode_order_from_event(
    event: &EntityReplayableEvent,
) -> Result<SpotOrderV2, PlaceOrderOutboundError> {
    let order_id =
        event_u64_field(event, "order_id").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    let asset =
        event_u64_field(event, "asset").ok_or(PlaceOrderOutboundError::EventDecodeFailed)? as u32;
    let account_id = event_string_field(event, "account_id")
        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    let symbol =
        event_string_field(event, "symbol").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    let side = decode_side(event)?;
    let limit_price = decode_limit_price(event)?;
    let order_type = decode_order_type(event)?;
    let qty = event_u64_field(event, "qty").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    let client_order_id =
        event_string_field(event, "client_order_id").filter(|value| !value.is_empty());
    let version = event_u64_field(event, "version").unwrap_or(1);
    let created_at =
        event_u64_field(event, "created_at").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    let updated_at =
        event_u64_field(event, "updated_at").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    let mut order = if order_type.is_trigger() {
        SpotOrderV2::new_pending_trigger(
            order_id,
            asset,
            account_id,
            symbol,
            side,
            qty,
            limit_price,
            order_type,
            client_order_id,
            version,
            created_at,
        )
    } else {
        SpotOrderV2::new_pending_limit(
            order_id,
            asset,
            account_id,
            symbol,
            side,
            qty,
            limit_price,
            order_type,
            client_order_id,
            version,
            created_at,
        )
    };
    order.filled_size =
        event_u64_field(event, "filled_qty").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    order.status = decode_status(event)?;
    order.status_reason =
        decode_status_reason(event_string_field(event, "status_reason").as_deref())?;
    order.reduce_only = event_string_field(event, "reduce_only")
        .map(|value| decode_bool(value.as_str()))
        .transpose()?
        .unwrap_or(false);
    order.group_relation =
        decode_group_relation(event_string_field(event, "group_relation").as_deref())?;
    order.updated_at = updated_at;
    Ok(order)
}

fn decode_status_value(value: &str) -> Result<SpotOrderStatus, PlaceOrderOutboundError> {
    match value {
        "pending" => Ok(SpotOrderStatus::Pending),
        "open" => Ok(SpotOrderStatus::Open),
        "partially_filled" => Ok(SpotOrderStatus::PartiallyFilled),
        "filled" => Ok(SpotOrderStatus::Filled),
        "canceled" => Ok(SpotOrderStatus::Canceled),
        "rejected" => Ok(SpotOrderStatus::Rejected),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}
use common_entity::LogEntry;
