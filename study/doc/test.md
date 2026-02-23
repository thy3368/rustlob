## 目标

- HyperCore：完全链上的中央限价订单簿（CLOB）
  架构设计：专为订单簿交易构建的高性能执行环境，与受限于EVM串行处理的通用区块链不同
  核心功能：
  链上订单簿、匹配引擎和清算系统一体化
  所有订单公开透明，防止抢跑和操纵
  支持限价单、止盈止损等专业订单类型
  技术实现：Rust语言开发，零链下组件，所有操作在链上完成


## todo 怎么实现线下撮合，线上结算 给出一个完整方案？

### 1. 架构概述

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Hybrid CLOB Architecture                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌──────────────┐     ┌──────────────┐     ┌──────────────┐             │
│   │   Maker      │     │  Off-chain   │     │   Taker      │             │
│   │   Signs      │────▶│   Matching   │◀────│   Signs      │             │
│   │   Order      │     │   Engine     │     │   Order      │             │
│   └──────────────┘     └──────┬───────┘     └──────────────┘             │
│                                │                                            │
│                                ▼                                            │
│                     ┌─────────────────────┐                               │
│                     │   Order Matched      │                               │
│                     │   (Signed by both)   │                               │
│                     └──────────┬──────────┘                               │
│                                │                                            │
│                                ▼                                            │
│   ┌──────────────────────────────────────────────────────────────────┐     │
│   │                      On-chain Settlement                         │     │
│   │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────┐  │     │
│   │  │  Validate  │─▶│  Transfer  │─▶│  Update    │─▶│  Emit    │  │     │
│   │  │  Signature │  │   Assets   │  │  Positions │  │  Event   │  │     │
│   │  └────────────┘  └────────────┘  └────────────┘  └──────────┘  │     │
│   └──────────────────────────────────────────────────────────────────┘     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2. 核心流程

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Trading Flow                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. Maker signs order (off-chain)                                          │
│     ┌──────────────────────────────────────────┐                           │
│     │ Order: {                                                         │
│       tokenA: "0.5",     // 卖 0.5 ETH                               │
│       tokenB: "1000",    // 换 1000 USDC                              │
│       price: 2000,                                                      │
│       expiry: 1700000000,                                              │
│       nonce: 123                                                        │
│     }                                                                   │
│     └──────────────────────────────────────────┘                           │
│                              │                                              │
│                              ▼                                              │
│  2. Taker finds maker's order (off-chain matching)                        │
│                              │                                              │
│                              ▼                                              │
│  3. Both parties sign the trade (atomic)                                   │
│     ┌──────────────────────────────────────────┐                           │
│     │ Trade: {                                                          │
│       maker_order_hash: "0x...",                                       │
│       taker_order_hash: "0x...",                                        │
│       filled_amount: "0.5",                                             │
│       profit: "0"                                                        │
│     }                                                                   │   
│     │ + EIP-712 signatures from both parties                          │
│     └──────────────────────────────────────────┘                           │
│                              │                                              │
│                              ▼                                              │
│  4. Submit to blockchain (anyone can trigger)                              │
│                              │                                              │
│                              ▼                                              │
│  5. Smart contract validates & settles                                    │
│     - Verify both signatures                                              │
│     - Check order validity (not expired, not filled)                     │
│     - Transfer tokens atomically                                          │
│     - Update positions                                                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3. 数据结构设计

#### 3.1 订单签名 (EIP-712)

```rust
// 链下订单结构
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LimitOrder {
    salt: u256,                    // 防止重放攻击
    maker: Address,                // 挂单用户
    signer: Address,               // 签名者（可以是maker或代理）
    token_sell: Address,           // 卖出的代币
    token_buy: Address,            // 买入的代币
    amounts: OrderAmounts {
        making_amount: u256,       // 卖出的数量
        taking_amount: u256,       // 买入的数量
    },
    // 订单属性
    attributes: OrderAttributes {
        predicate: Bytes,          // 前置条件（时间锁等）
        permit: Bytes,             // 代币授权
        pre_interaction: Bytes,    // 前置交互
        post_interaction: Bytes,   // 后置交互
    },
    // 有效期
    valid_from: u32,               // 生效时间
    valid_until: u32,              // 过期时间
    fee_asset: Address,             // 手续费代币
    max_bps: u16,                  // 最大手续费比例 (基点)
    // 状态
    fully_filled: bool,            // 是否已完全成交
}

// EIP-712 Domain Separator
struct EIP712Domain {
    name: "HyperCore",
    version: "1",
    chain_id: u256,
    verifying_contract: Address,    // 合约地址
    salt: u256,                     // 防止跨链重放
}

// 订单类型哈希
fn order_type_hash() -> bytes32 {
    keccak256(
        "LimitOrder("
        "uint256 salt,"
        "address maker,"
        "address signer,"
        "address tokenS,"
        "address tokenB,"
        "uint256 makingAmount,"
        "uint256 takingAmount,"
        "bytes predicate,"
        "bytes permit,"
        "bytes preInteraction,"
        "bytes postInteraction,"
        "uint256 validFrom,"
        "uint256 validUntil,"
        "address feeAsset,"
        "uint256 maxBps,"
        "bool fullyFilled)"
    )
}
```

#### 3.2 交易结构

```rust
// 链上结算的交易结构
struct Trade {
    // 订单信息
    maker: Address,
    taker: Address,
    token_sell: Address,
    token_buy: Address,
    making_amount: u256,
    taking_amount: u256,
    // 成交信息
    making_amount_filled: u256,
    taking_amount_filled: u256,
    // 手续费
    maker_fee: u256,
    taker_fee: u256,
    // 元数据
    exchange: Address,              // 交易所合约
    salt: u256,                     // 防止重放
}

// 用户签名的交易请求（由taker提交）
struct TradeCommand {
    data: TradeData,
    signature_maker: Bytes,
    signature_taker: Bytes,
}

// 批量交易请求
struct TradeCommandMany {
    data: Vec<TradeData>,
    maker_signature: Bytes,         // maker对所有交易签名
    taker_signatures: Vec<Bytes>,   // taker对每个交易签名
}
```

### 4. 智能合约实现

#### 4.1 核心合约接口

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IHyperCore {
    /// @notice 挂单（链下签名，链上取消）
    /// @param order 订单数据
    /// @param signature maker的EIP-712签名
    function placeOrder(LimitOrder calldata order, bytes calldata signature) 
        external returns (bytes32 orderHash);

    /// @notice 挂单并立即成交
    /// @param order maker的订单
    /// @param makerSignature maker签名
    /// @param takerOrderId taker订单ID（链下订单）
    /// @param takingAmount taker想要成交的数量
    /// @param takerSignature taker签名
    function fillOrder(
        LimitOrder calldata order,
        bytes calldata makerSignature,
        bytes32 takerOrderId,
        uint256 takingAmount,
        bytes calldata takerSignature
    ) external returns (uint256 filledAmount);

    /// @notice 批量成交
    /// @param orders maker订单数组
    /// @param makerSigs maker签名数组
    /// @param takingAmounts 各自成交数量
    /// @param takerSigs taker签名数组
    function fillOrders(
        LimitOrder[] calldata orders,
        bytes[] calldata makerSigs,
        uint256[] calldata takingAmounts,
        bytes[] calldata takerSigs
    ) external returns (uint256[] memory filledAmounts);

    /// @notice 取消订单
    /// @param orderKey 订单唯一标识（通常是orderHash）
    function cancelOrder(bytes32 orderKey) external;

    /// @notice 批量取消订单
    function cancelOrders(bytes32[] calldata orderKeys) external;

    /// @notice 撤销签名授权
    /// @param salt 授权的salt值
    function invalidateOrder(uint256 salt) external;
}
```

#### 4.2 订单验证逻辑

```solidity
contract HyperCore is IHyperCore {
    using ECDSA for bytes32;
    using SafeERC20 for IERC20;

    // 订单生命周期状态
    mapping(bytes32 => OrderStatus) public orderStatus;
    // 已使用的nonce（防止重放）
    mapping(address => mapping(uint256 => bool)) public nonceUsed;

    // 事件
    event OrderPlaced(bytes32 indexed orderHash, address indexed maker);
    event OrderFilled(
        bytes32 indexed orderHash, 
        address indexed taker, 
        uint256 filledAmount
    );
    event OrderCancelled(bytes32 indexed orderHash);

    /// @notice 验证订单签名
    function _validateOrderSignature(
        LimitOrder memory order, 
        bytes memory signature
    ) internal view returns (address signer) {
        // 构建EIP-712域
        bytes32 domainSeparator = _domainSeparator();

        // 计算订单哈希
        bytes32 orderHash = _hashOrder(order, domainSeparator);

        // 验证签名
        signer = ECDSA.recover(orderHash, signature);

        // 验证签名者权限
        require(
            signer == order.maker || signer == order.signer,
            "Invalid signer"
        );
    }

    /// @notice 验证订单有效性
    function _validateOrder(
        LimitOrder memory order, 
        bytes32 orderHash
    ) internal view {
        // 检查时间有效性
        require(
            block.timestamp >= order.validFrom,
            "Order not yet valid"
        );
        require(
            order.validUntil == 0 || block.timestamp <= order.validUntil,
            "Order expired"
        );

        // 检查是否已完全成交
        require(
            !order.fullyFilled,
            "Order fully filled"
        );

        // 检查nonce未使用
        require(
            !nonceUsed[order.maker][order.salt],
            "Nonce already used"
        );
    }

    /// @notice 执行成交
    function fillOrder(
        LimitOrder calldata order,
        bytes calldata makerSignature,
        bytes32 takerOrderId,
        uint256 takingAmount,
        bytes calldata takerSignature
    ) external override returns (uint256 filledAmount) {
        // 1. 验证maker签名
        address maker = _validateOrderSignature(order, makerSignature);

        // 2. 验证taker签名（可选，taker可以是合约）
        if (takerSignature.length == 65) {
            // 如果taker是EOA，验证签名
            require(
                msg.sender == takerOrderId.toAddress(),
                "Invalid taker"
            );
        }

        // 3. 计算成交数量
        filledAmount = _calculateFillAmount(order, takingAmount);
        require(filledAmount > 0, "Invalid fill amount");

        // 4. 检查订单状态
        bytes32 orderHash = _hashOrder(order, _domainSeparator());
        _validateOrder(order, orderHash);

        // 5. 执行代币转账
        _settleTrade(order, maker, msg.sender, filledAmount);

        // 6. 更新订单状态
        _updateOrderState(order, orderHash, filledAmount);

        // 7. 发放手续费
        _chargeFees(order, maker, msg.sender, filledAmount);

        // 8. 记录nonce（防止重放）
        nonceUsed[order.maker][order.salt] = true;

        emit OrderFilled(orderHash, msg.sender, filledAmount);
    }

    /// @notice 代币结算（原子转账）
    function _settleTrade(
        LimitOrder order,
        address maker,
        address taker,
        uint256 filledAmount
    ) internal {
        // 计算交换数量
        uint256 makerGets = (order.makingAmount * filledAmount) / order.takingAmount;
        uint256 takerGets = filledAmount;

        // taker -> maker: token_sell
        IERC20(order.tokenSell).safeTransferFrom(
            taker, 
            maker, 
            takerGets
        );

        // maker -> taker: token_buy
        IERC20(order.tokenBuy).safeTransferFrom(
            maker, 
            taker, 
            makerGets
        );
    }
}
```

### 5. 链下匹配引擎设计

```rust
// 链下匹配引擎
struct MatchingEngine {
    // 订单簿
    order_book: OrderBook,
    // 订单缓存
    pending_orders: HashMap<OrderId, LimitOrder>,
    // 成交历史
    trades: Vec<Trade>,
}

impl MatchingEngine {
    /// 链下撮合
    fn match_orders(&mut self, taker: &LimitOrder) -> Result<Vec<Match>, Error> {
        let side = taker.side();
        let opposite_orders = match side {
            Buy => self.order_book.get_asks(),
            Sell => self.order_book.get_bids(),
        };

        let mut matches = Vec::new();
        let mut remaining = taker.taking_amount;

        for order in opposite_orders.iter() {
            if !taker.can_match(order.price) { break; }
            if remaining.is_zero() { break; }

            let fill = min(remaining, order.remaining);
            
            matches.push(Match {
                maker_order: order.clone(),
                taker_order: taker.clone(),
                fill_amount: fill,
                price: order.price,
            });

            remaining -= fill;
        }

        Ok(matches)
    }

    /// 生成交易数据供签名
    fn build_trade(
        &self, 
        matches: Vec<Match>
    ) -> Vec<TradeData> {
        matches.iter().map(|m| TradeData {
            maker: m.maker_order.maker,
            taker: m.taker_order.maker,
            token_sell: m.maker_order.token_sell,
            token_buy: m.maker_order.token_buy,
            making_amount: m.fill_amount,
            taking_amount: m.calculate_taking(),
            salt: self.current_block,
            exchange: self.contract_address,
        }).collect()
    }
}
```

### 6. 完整交易流程示例

```typescript
// ========== Step 1: Maker creates order (off-chain) ==========
const makerOrder: LimitOrder = {
  salt: BigInt(Date.now()),
  maker: makerAddress,
  signer: makerAddress,
  tokenSell: ETH_ADDRESS,           // 卖 ETH
  tokenBuy: USDC_ADDRESS,           // 换 USDC
  makingAmount: parseEther("1"),    // 卖 1 ETH
  takingAmount: parseUnits("2000", 6), // 要 2000 USDC
  validFrom: Math.floor(Date.now() / 1000),
  validUntil: Math.floor(Date.now() / 1000) + 3600, // 1小时后过期
  feeAsset: USDC_ADDRESS,
  maxBps: 30,  // 0.3% 手续费
  fullyFilled: false,
  predicate: "0x",
  permit: "0x",
  preInteraction: "0x",
  postInteraction: "0x",
};

// Maker signs order
const makerSignature = await signOrder(makerOrder, makerPrivateKey);

// Submit to off-chain order book
await orderBookClient.placeOrder(makerOrder, makerSignature);

// ========== Step 2: Taker finds and fills order (off-chain) ==========
// Taker queries order book, finds maker's order
const order = await orderBookClient.getOrder(orderHash);

// Taker creates taker order (or uses existing)
const takerOrder: LimitOrder = {
  salt: BigInt(Date.now()),
  maker: takerAddress,
  signer: takerAddress,
  tokenSell: USDC_ADDRESS,
  tokenBuy: ETH_ADDRESS,
  makingAmount: parseUnits("2000", 6),
  takingAmount: parseEther("1"),
  // ...
};

// ========== Step 3: Both sign the trade (atomic) ==========
// Off-chain matching engine builds trade data
const tradeData: TradeData = {
  makerOrderHash: orderHash,
  takerOrderHash: takerOrderHash,
  filledAmount: parseEther("1"),
  // ...
};

// Both parties sign the trade
const makerTradeSig = await signTrade(tradeData, makerPrivateKey);
const takerTradeSig = await signTrade(tradeData, takerPrivateKey);

// ========== Step 4: Submit to blockchain (anyone can trigger) ==========
const tx = await hyperCoreContract.fillOrder(
  makerOrder,
  makerSignature,
  takerOrderHash,
  filledAmount,
  takerTradeSig
);

await tx.wait();

// ========== Step 5: On-chain settlement ==========
// Smart contract:
// 1. Validates both signatures
// 2. Checks order not expired / filled
// 3. Transfers ETH: maker -> taker
// 4. Transfers USDC: taker -> maker
// 5. Charges fees
// 6. Updates order status
// 7. Emits events
```

### 7. 安全性设计

| 风险 | 防护措施 |
|------|----------|
| **重放攻击** | nonce + salt + chainId + 合约地址 |
| **签名伪造** | EIP-712严格验证签名者权限 |
| **订单篡改** | 链上重新计算hash校验 |
| **抢先交易** | 链下撮合，链上原子结算 |
| **假订单** | 需提供资金证明或保证金 |
| **结算失败** | SafeMath防溢出，失败回滚 |

### 8. Gas优化策略

| 优化点 | 方案 |
|--------|------|
| 批量成交 | `fillOrders()` 批量处理多个taker |
| 手续费补贴 | maker/taker手续费分离 |
| 签名验证 | EIP-712预计算domain |
| 状态更新 | 增量更新，避免全量读取 |

### 9. 实现参考

- **Polymarket**: 链下匹配 + Polygon链上结算
- **0x Protocol**: 链下RFQ + 链上清算
- **dYdX**: 链下订单簿 + 链上结算（已迁移至Cosmos）

### 10. 部署配置

```
┌─────────────────────────────────────────────────────────────┐
│                    Deployment Configuration                 │
├─────────────────────────────────────────────────────────────┤
│  Layer 1:     Ethereum Mainnet / Arbitrum / Optimism       │
│  Layer 2:     Polygon / Base / zkSync                      │
│  Matching:    Off-chain (centralized or decentralized)      │
│  Settlement:  Smart Contract (atomic, atomicity ensured)   │
│  Gas Cost:    ~150k-300k gas per trade                     │
│  Latency:     < 2 seconds (block confirmation)              │
└─────────────────────────────────────────────────────────────┘
```

