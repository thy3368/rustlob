# Spot Batch Block Design

## Goal

Make `ExecuteTradingBatchHandler` evolve from a stateless command counter into a spot-focused batch executor that produces a block-like execution result containing order outcomes, trade executions, and balance settlements.

## Scope

This spec only covers the first execution-meaningful slice:

- spot `PlaceOrder` real execution
- matching against the existing spot order book
- trade generation
- spot balance deltas
- batch output as a block-style aggregate

Out of scope for this phase:

- real `CancelOrder` / `AmendOrder` state transitions
- perp / option execution state
- fees, funding, liquidation, risk engine
- persistent block hash / state root
- durable storage / replay integration

## Execution Semantics

A batch is a sequential execution container, not an isolated internal matching pool.

For each spot place-order command:

1. create a new order intent
2. match it against the current resting opposite-side orders already in the handler's spot order book
3. emit zero or more trade records when crossing liquidity exists
4. emit balance deltas for each executed trade
5. leave any unfilled remainder as a resting order in the order book

This means:

- every `PlaceOrder` has order-creation semantics
- trades are conditional, not guaranteed
- a batch result is the aggregation of per-command execution outcomes
- later commands in the same batch observe the book state updated by earlier commands

## Recommended Shape

Use a dedicated block-style output structure instead of overloading the existing `BatchExecutionResult` counter object.

```rust
pub struct ExecutedBatchBlock {
    pub summary: BatchExecutionSummary,
    pub orders: Vec<ExecutedSpotOrder>,
    pub trades: Vec<TradeExecutionResult>,
    pub balance_deltas: Vec<BalanceDelta>,
}
```

### Summary

```rust
pub struct BatchExecutionSummary {
    pub total_commands: usize,
    pub accepted_commands: usize,
    pub rejected_commands: usize,
    pub orders_created: usize,
    pub trades_executed: usize,
    pub balance_updates: usize,
}
```

### Order Outcome

```rust
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
}

pub struct ExecutedSpotOrder {
    pub order_id: u64,
    pub trader_id: u64,
    pub market: String,
    pub side: SpotSide,
    pub price: u64,
    pub original_quantity: u64,
    pub remaining_quantity: u64,
    pub status: OrderStatus,
}
```

### Trade Output

Keep the existing `TradeExecutionResult` shape, since it already models the minimal trade payload needed for this phase.

### Balance Settlement Output

```rust
pub struct BalanceDelta {
    pub trader_id: u64,
    pub asset: String,
    pub delta: i64,
}
```

For the first phase, `asset` can be derived from the spot market string, e.g. `BTC-USDC` → base `BTC`, quote `USDC`.

## Handler Responsibilities

`ExecuteTradingBatchHandler` should stop being stateless.

Instead of:

```rust
pub struct ExecuteTradingBatchHandler;
```

it should hold at least:

```rust
pub struct ExecuteTradingBatchHandler {
    spot_order_book: Mutex<SpotOrderBook>,
    next_order_id: AtomicU64,
}
```

### Why

To match a new command against existing resting orders, the handler must preserve book state across command execution.

Without internal book state, the handler cannot implement the required semantics:

- command creates order
- command matches against existing book
- unfilled quantity rests in book for future commands and future batches

## Minimal Spot Order Book Model

For this phase, the order book only needs enough structure to support deterministic matching for tests.

Minimum requirements:

- separate bid and ask collections
- market-aware storage
- ability to locate best opposite-side resting order
- quantity reduction after fills
- removal of fully filled resting orders

No advanced requirements yet:

- no time-priority optimization work
- no persistent indexing layer
- no performance tuning beyond simple correctness
- no cross-market batching tricks

A simple in-memory book representation is acceptable for this phase.

## Matching Rules

Only support spot limit-order crossing semantics in phase one.

### Crossing rules

- buy order matches when `buy.price >= best_ask.price`
- sell order matches when `sell.price <= best_bid.price`

### Execution price

Use maker price for the first phase.

### Fill quantity

Use `min(maker.remaining_quantity, taker.remaining_quantity)`.

### Post-trade status

- remaining quantity `== original quantity` and no trade → `Open`
- remaining quantity `> 0` after one or more trades → `PartiallyFilled`
- remaining quantity `== 0` after trades → `Filled`

## Spot Balance Settlement

The first phase should produce balance deltas for each executed trade.

For a trade of `quantity` at `price` on market `BASE-QUOTE`:

### Buyer

- `BASE +quantity`
- `QUOTE -(price * quantity)`

### Seller

- `BASE -quantity`
- `QUOTE +(price * quantity)`

Therefore a single trade produces four balance delta records.

Fees are intentionally excluded in this phase.

## Testing Strategy

Replace the current spot batch count-only expectation with behavior-focused tests.

### Test 1: no counterparty

Name suggestion:

`spot_place_order_without_counterparty_creates_open_order`

Setup:

- empty spot book
- one spot place-order command

Expect:

- one order outcome
- zero trades
- zero balance deltas
- order status is `Open`
- remaining quantity equals original quantity

### Test 2: full match against resting order

Name suggestion:

`spot_place_order_crossing_existing_resting_order_executes_trade`

Setup:

1. first batch places a resting maker order
2. second batch places a taker order that crosses the resting price

Expect:

- second execution emits one trade
- second execution emits four balance deltas
- resting maker is fully consumed
- taker is fully filled
- no remaining open quantity for either side

### Test 3: partial fill

Name suggestion:

`spot_place_order_partial_fill_leaves_remaining_open_quantity`

Setup:

1. first batch places a smaller resting maker order
2. second batch places a larger crossing taker order

Expect:

- one trade emitted
- four balance deltas emitted
- maker becomes `Filled`
- taker becomes `PartiallyFilled`
- taker remainder is left resting in the book with correct remaining quantity

## External Test Style

To verify real handler behavior, tests should avoid mutating internal order-book state directly.

Preferred pattern:

- call `cmd_handle` once to place a resting order
- call `cmd_handle` again with the next batch
- assert on the returned block result from the second call

This keeps tests aligned with observable public behavior rather than hidden implementation details.

## File Plan

### Modify

- `proc/operating/dex/dex/src/cmd_handler/execute_trading_batch_handler.rs`
- `proc/operating/dex/dex/src/cmd_handler/mod.rs`
- `proc/operating/dex/dex/tests/trading/spot/execute_batch.rs`

### Keep unchanged for now

- `proc/operating/dex/dex/src/cmd_handler/trading_command.rs` as command-input vocabulary
- `proc/operating/dex/dex/src/cmd_handler/submit_trading_command_handler.rs` as queue-ingestion handler

## Design Choice Summary

Chosen direction:

- block-style batch result
- spot-only real execution in phase one
- stateful batch executor with internal spot order book
- per-trade balance delta generation
- behavior-first tests using sequential public handler calls

Rejected alternatives:

- keeping batch logic as pure command counting only, because it cannot express the required business semantics
- matching only within the incoming batch, because it ignores the real exchange requirement of matching against resting book state
- implementing full multi-product exchange execution now, because it is much broader than the confirmed scope
