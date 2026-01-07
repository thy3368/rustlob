# Substrate 学习计划

## 学习目标

掌握 Substrate 区块链开发框架，能够设计和构建高性能、可定制的区块链网络，深入理解 Polkadot 生态系统的技术架构。

## 前置知识要求

- ✅ Rust 编程语言（所有权、生命周期、异步编程）
- ✅ 区块链基础概念（共识、交易、区块）
- ✅ 密码学基础（哈希、签名、Merkle树）
- 🔄 libp2p 基础（建议先完成 libp2p 学习计划）

---

## 学习路线图

### 第一阶段：Substrate 基础 (Week 1-2)

#### 1.1 区块链基础回顾
- [ ] **区块链核心概念**
  - 区块结构与链式存储
  - 交易模型（UTXO vs Account）
  - 状态机与状态转换
  - 共识算法分类

- [ ] **智能合约平台对比**
  - Ethereum: EVM + Solidity
  - Substrate: Wasm + Rust
  - 性能与灵活性对比

#### 1.2 Substrate 架构概览

```
┌─────────────────────────────────────────────┐
│           Client (Outer Node)               │
│  ┌────────────┐  ┌──────────┐  ┌─────────┐ │
│  │  Network   │  │   RPC    │  │ Storage │ │
│  │  (libp2p)  │  │  Server  │  │   DB    │ │
│  └────────────┘  └──────────┘  └─────────┘ │
└──────────────────┬──────────────────────────┘
                   │ Host Functions
┌──────────────────▼──────────────────────────┐
│          Runtime (Wasm)                     │
│  ┌────────────────────────────────────────┐ │
│  │  FRAME: Modular Runtime Framework      │ │
│  │  ┌──────┐  ┌──────┐  ┌──────┐         │ │
│  │  │Pallet│  │Pallet│  │Pallet│  ...    │ │
│  │  │System│  │Balances│ │Staking│        │ │
│  │  └──────┘  └──────┘  └──────┘         │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
```

- [ ] **核心组件理解**
  - **Client (外部节点)**
    - 网络层：基于 libp2p 的 P2P 通信
    - RPC 服务器：JSON-RPC API
    - 存储引擎：RocksDB/ParityDB
    - 交易池：内存池管理

  - **Runtime (运行时)**
    - WebAssembly 执行环境
    - 状态转换逻辑（STF）
    - 可升级性（Forkless Upgrade）
    - 确定性执行

  - **FRAME (Framework for Runtime Aggregation of Modularized Entities)**
    - Pallet：可组合的功能模块
    - Executive：交易执行引擎
    - System：系统级功能

#### 1.3 开发环境搭建

```bash
# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# 安装 Substrate 依赖（macOS）
brew install cmake pkg-config openssl git llvm

# 克隆 Substrate 节点模板
git clone https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-node-template

# 编译节点
cargo build --release

# 运行开发链
./target/release/node-template --dev
```

#### 1.4 第一个 Substrate 链

```rust
// 项目1：运行和交互 Substrate 节点模板

// 1. 启动开发节点
./target/release/node-template --dev --tmp

// 2. 使用 Polkadot.js Apps 连接
// https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944

// 3. 基本操作
// - 查看区块生产
// - 创建账户
// - 转账操作
// - 查看事件和存储
```

**学习目标检查点**：
- [ ] 理解 Substrate 的分层架构
- [ ] 成功编译和运行节点
- [ ] 使用 Polkadot.js Apps 进行交互
- [ ] 理解 Runtime 和 Client 的关系

---

### 第二阶段：Runtime 开发基础 (Week 3-4)

#### 2.1 FRAME 框架深入

**Runtime 宏系统**：
```rust
// runtime/src/lib.rs
use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU32, ConstU64},
};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature,
};

// Runtime 版本定义
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("node-template"),
    impl_name: create_runtime_str!("node-template"),
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

// 参数配置
parameter_types! {
    pub const BlockHashCount: u64 = 2400;
    pub const Version: RuntimeVersion = VERSION;
}

// Runtime 构建
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,

        // 自定义 Pallet
        TemplateModule: pallet_template,
    }
);
```

#### 2.2 核心 Pallets 详解

**System Pallet（系统基础）**：
```rust
// 提供基础功能
// - 区块号管理
// - 账户nonce跟踪
// - 事件系统
// - 外部调用（Extrinsic）处理

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // 存储项
    #[pallet::storage]
    pub type Something<T> = StorageValue<_, u32>;

    // 事件定义
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SomethingStored { something: u32, who: T::AccountId },
    }

    // 错误类型
    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    // 可调用函数（外部调用）
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Something::<T>::put(something);
            Self::deposit_event(Event::SomethingStored { something, who });

            Ok(())
        }
    }
}
```

**Balances Pallet（账户余额）**：
```rust
// 配置示例
impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<500>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type MaxHolds = ();
}

// 常用操作
use pallet_balances::Pallet as Balances;

// 查询余额
let balance = Balances::<T>::free_balance(&account);

// 转账
Balances::<T>::transfer(
    origin,
    dest,
    value,
)?;

// 预留余额
Balances::<T>::reserve(&account, amount)?;
```

#### 2.3 存储系统详解

**存储类型**：
```rust
use frame_support::pallet_prelude::*;

#[pallet::storage]
// 1. StorageValue: 单一值存储
pub type SingleValue<T> = StorageValue<_, u32, ValueQuery>;

#[pallet::storage]
// 2. StorageMap: 键值对存储
pub type UserData<T: Config> = StorageMap<
    _,
    Blake2_128Concat,  // 哈希算法
    T::AccountId,      // 键类型
    UserInfo,          // 值类型
    OptionQuery,       // 查询类型
>;

#[pallet::storage]
// 3. StorageDoubleMap: 双键映射
pub type Approvals<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat, T::AccountId,  // 第一个键
    Blake2_128Concat, T::AccountId,  // 第二个键
    bool,                            // 值
    ValueQuery,
>;

#[pallet::storage]
// 4. StorageNMap: N键映射
pub type MultiKeyStorage<T: Config> = StorageNMap<
    _,
    (
        NMapKey<Blake2_128Concat, T::AccountId>,
        NMapKey<Blake2_128Concat, u32>,
        NMapKey<Blake2_128Concat, [u8; 32]>,
    ),
    Balance,
    ValueQuery,
>;

// 存储操作
impl<T: Config> Pallet<T> {
    pub fn store_data(account: &T::AccountId, data: UserInfo) {
        // 插入/更新
        UserData::<T>::insert(account, data);

        // 读取
        if let Some(info) = UserData::<T>::get(account) {
            // 处理数据
        }

        // 删除
        UserData::<T>::remove(account);

        // 修改
        UserData::<T>::mutate(account, |data| {
            if let Some(info) = data {
                info.value += 1;
            }
        });

        // 尝试修改（可能失败）
        UserData::<T>::try_mutate(account, |data| -> Result<(), Error<T>> {
            let info = data.as_mut().ok_or(Error::<T>::NoData)?;
            info.value = info.value.checked_add(1).ok_or(Error::<T>::Overflow)?;
            Ok(())
        });
    }
}
```

**存储最佳实践**：
```rust
// ❌ 错误：无界存储
#[pallet::storage]
pub type UnboundedVec<T> = StorageValue<_, Vec<T::AccountId>>;

// ✅ 正确：有界存储
use frame_support::BoundedVec;

#[pallet::storage]
pub type BoundedAccounts<T: Config> = StorageValue<
    _,
    BoundedVec<T::AccountId, ConstU32<1000>>,
    ValueQuery,
>;

// 性能优化：选择合适的哈希算法
// - Blake2_128Concat: 默认选择，平衡性能和安全
// - Twox64Concat: 高性能，不抗碰撞（仅用于受信任的键）
// - Identity: 无哈希，键本身已是哈希值

#[pallet::storage]
pub type FastLookup<T> = StorageMap<
    _,
    Twox64Concat,  // 快速哈希
    u64,           // 数字键
    Data,
>;
```

#### 2.4 权重系统与费用

**权重计算**：
```rust
use frame_support::weights::Weight;

#[pallet::call]
impl<T: Config> Pallet<T> {
    // 固定权重
    #[pallet::weight(10_000)]
    pub fn simple_call(origin: OriginFor<T>) -> DispatchResult {
        let _ = ensure_signed(origin)?;
        Ok(())
    }

    // 动态权重
    #[pallet::weight(T::DbWeight::get().reads_writes(1, 1) + 50_000)]
    pub fn complex_call(
        origin: OriginFor<T>,
        data: Vec<u8>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 1次读取
        let stored = Something::<T>::get();

        // 1次写入
        Something::<T>::put(data.len() as u32);

        Ok(())
    }

    // 基于输入的权重
    #[pallet::weight(data.len() as u64 * 1_000)]
    pub fn variable_weight_call(
        origin: OriginFor<T>,
        data: Vec<u8>,
    ) -> DispatchResult {
        let _ = ensure_signed(origin)?;
        // 处理数据
        Ok(())
    }
}

// 自定义权重计算
pub trait WeightInfo {
    fn do_something() -> Weight;
    fn batch_operations(n: u32) -> Weight;
}

impl WeightInfo for () {
    fn do_something() -> Weight {
        Weight::from_parts(10_000, 0)
    }

    fn batch_operations(n: u32) -> Weight {
        Weight::from_parts((50_000 as u64).saturating_mul(n as u64), 0)
    }
}
```

**Benchmarking（基准测试）**：
```rust
// pallets/template/src/benchmarking.rs
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    do_something {
        let caller: T::AccountId = whitelisted_caller();
        let value: u32 = 100;
    }: _(RawOrigin::Signed(caller), value)
    verify {
        assert_eq!(Something::<T>::get(), Some(value));
    }

    complex_operation {
        let n in 1 .. 1000;  // 参数化基准
        let caller: T::AccountId = whitelisted_caller();
        let data = vec![0u8; n as usize];
    }: _(RawOrigin::Signed(caller), data)
}

// 运行基准测试
// cargo build --release --features runtime-benchmarks
// ./target/release/node-template benchmark pallet \
//   --pallet pallet_template \
//   --extrinsic '*' \
//   --output pallets/template/src/weights.rs
```

#### 实践项目
```rust
// 项目2：简单的 Token Pallet
// 目标：实现ERC20风格的代币系统

#[pallet::pallet]
pub struct Pallet<T>(_);

#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;
}

#[pallet::storage]
pub type TotalSupply<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

#[pallet::storage]
pub type Balances<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    T::Balance,
    ValueQuery,
>;

#[pallet::storage]
pub type Allowances<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat, T::AccountId,  // owner
    Blake2_128Concat, T::AccountId,  // spender
    T::Balance,
    ValueQuery,
>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    // mint, transfer, approve, transferFrom
    // 实现完整的代币功能
}
```

---

### 第三阶段：高级 Pallet 开发 (Week 5-6)

#### 3.1 Hooks 与生命周期

```rust
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    // 区块初始化时调用
    fn on_initialize(n: T::BlockNumber) -> Weight {
        log::info!("Block {:?} initializing", n);

        // 清理过期数据
        Self::cleanup_expired_items();

        Weight::from_parts(10_000, 0)
    }

    // 区块结束时调用
    fn on_finalize(n: T::BlockNumber) {
        log::info!("Block {:?} finalizing", n);

        // 计算奖励分配
        Self::distribute_rewards();
    }

    // 空闲时间处理（低优先级任务）
    fn on_idle(n: T::BlockNumber, remaining_weight: Weight) -> Weight {
        // 可选的后台任务
        Self::background_cleanup(remaining_weight)
    }

    // Runtime升级后调用
    fn on_runtime_upgrade() -> Weight {
        log::info!("Runtime upgrading");

        // 数据迁移逻辑
        migrations::migrate_v1_to_v2::<T>()
    }

    // 完整性检查
    fn integrity_test() {
        assert!(T::MaxMembers::get() > 0, "MaxMembers must be positive");
    }
}

// 离线工作者（Off-chain Worker）
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) {
        log::info!("Offchain worker at block {:?}", block_number);

        // 执行链外任务
        // - HTTP请求
        // - 数据聚合
        // - 签名交易提交

        if let Err(e) = Self::fetch_external_data() {
            log::error!("Offchain worker error: {:?}", e);
        }
    }
}
```

#### 3.2 Off-chain Workers (OCW)

```rust
use sp_runtime::offchain::{http, Duration};

impl<T: Config> Pallet<T> {
    fn fetch_external_data() -> Result<(), &'static str> {
        // 1. 发起HTTP请求
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(5000));

        let request = http::Request::get("https://api.example.com/data")
            .deadline(deadline)
            .send()
            .map_err(|_| "HTTP request failed")?;

        let response = request
            .wait()
            .map_err(|_| "Response timeout")?;

        if response.code != 200 {
            return Err("Non-200 status code");
        }

        let body = response.body().collect::<Vec<u8>>();
        let data: PriceData = serde_json::from_slice(&body)
            .map_err(|_| "JSON parse error")?;

        // 2. 签名并提交链上交易
        Self::submit_signed_transaction(data)?;

        Ok(())
    }

    fn submit_signed_transaction(data: PriceData) -> Result<(), &'static str> {
        use frame_system::offchain::CreateSignedTransaction;

        let signer = Signer::<T, T::AuthorityId>::any_account();

        let result = signer.send_signed_transaction(|_account| {
            Call::submit_price_data { data: data.clone() }
        });

        if let Some((_, res)) = result {
            res.map_err(|_| "Submit transaction failed")?;
        }

        Ok(())
    }
}

// 无签名交易（Unsigned Transaction）
#[pallet::validate_unsigned]
impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
        match call {
            Call::submit_price_data { data } => {
                // 验证数据有效性
                if !Self::is_valid_price_data(data) {
                    return InvalidTransaction::BadProof.into();
                }

                ValidTransaction::with_tag_prefix("PriceOracle")
                    .priority(100)
                    .and_provides(vec![data.encoded_signature()])
                    .longevity(5)
                    .propagate(true)
                    .build()
            }
            _ => InvalidTransaction::Call.into(),
        }
    }
}
```

#### 3.3 Pallet 耦合与 Tight Coupling

```rust
// Loose Coupling（松耦合）- 推荐
#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    // 使用 trait 定义依赖
    type Currency: Currency<Self::AccountId>;
    type RandomSource: Randomness<Self::Hash, Self::BlockNumber>;
}

// 使用依赖
impl<T: Config> Pallet<T> {
    pub fn transfer_funds(from: &T::AccountId, to: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
        T::Currency::transfer(from, to, amount, ExistenceRequirement::KeepAlive)?;
        Ok(())
    }
}

// Tight Coupling（紧耦合）- 直接依赖其他 Pallet
#[pallet::config]
pub trait Config: frame_system::Config + pallet_balances::Config {
    // 直接依赖 pallet_balances
}

impl<T: Config> Pallet<T> {
    pub fn get_balance(account: &T::AccountId) -> BalanceOf<T> {
        // 直接调用 Balances pallet
        pallet_balances::Pallet::<T>::free_balance(account)
    }
}
```

#### 3.4 Genesis 配置

```rust
// Pallet 的创世配置
#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
    pub initial_members: Vec<T::AccountId>,
    pub max_members: u32,
}

#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
        Self {
            initial_members: Default::default(),
            max_members: 100,
        }
    }
}

#[pallet::genesis_build]
impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
        // 设置初始状态
        MaxMembers::<T>::put(self.max_members);

        for member in &self.initial_members {
            Members::<T>::insert(member, ());
        }
    }
}

// 在 chain_spec.rs 中使用
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                // ...其他配置
                pallet_template::GenesisConfig {
                    initial_members: vec![
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                    ],
                    max_members: 1000,
                },
            )
        },
        // ...
    )
}
```

#### 实践项目
```rust
// 项目3：去中心化拍卖系统
// 目标：实现英式拍卖和荷兰式拍卖

pub struct Auction<T: Config> {
    creator: T::AccountId,
    item_id: ItemId,
    auction_type: AuctionType,
    start_price: BalanceOf<T>,
    current_price: BalanceOf<T>,
    highest_bidder: Option<T::AccountId>,
    start_block: T::BlockNumber,
    end_block: T::BlockNumber,
}

pub enum AuctionType {
    English,   // 价格递增
    Dutch,     // 价格递减
}

// 功能要求：
// 1. 创建拍卖
// 2. 出价系统
// 3. 自动结算（使用 on_finalize）
// 4. 退款机制
// 5. 拍卖历史记录（Off-chain Indexing）
```

---

### 第四阶段：共识与网络 (Week 7-8)

#### 4.1 共识机制深入

**Substrate 支持的共识算法**：

1. **Aura (Authority Round)**：
```rust
// 轮流出块的PoA共识
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type MaxAuthorities = ConstU32<32>;
    type DisabledValidators = ();
}

// 区块时间配置
impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<3000>;  // 6秒出块
    type WeightInfo = ();
}
```

2. **GRANDPA (GHOST-based Recursive Ancestor Deriving Prefix Agreement)**：
```rust
// 最终性算法
use sp_consensus_grandpa::AuthorityId as GrandpaId;

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}
```

3. **BABE (Blind Assignment for Blockchain Extension)**：
```rust
// Polkadot 使用的 VRF 随机选举
use sp_consensus_babe::AuthorityId as BabeId;

impl pallet_babe::Config for Runtime {
    type EpochDuration = ConstU64<2400>;  // Epoch 长度
    type ExpectedBlockTime = ConstU64<6000>;  // 6秒
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    // ...
}
```

4. **自定义共识引擎**：
```rust
// 实现 ConsensusEngine trait
use sp_consensus::{
    BlockImport, Environment, Proposer, SelectChain,
};

pub struct CustomConsensus<B: BlockT, C, E> {
    client: Arc<C>,
    env: E,
    _phantom: PhantomData<B>,
}

impl<B, C, E> CustomConsensus<B, C, E>
where
    B: BlockT,
    C: BlockchainEvents<B> + HeaderBackend<B>,
    E: Environment<B>,
{
    pub async fn run(&mut self) -> Result<(), ConsensusError> {
        // 共识逻辑实现
        loop {
            // 1. 选择父区块
            // 2. 创建区块提案
            // 3. 达成共识
            // 4. 导入区块
        }
    }
}
```

#### 4.2 网络层配置（libp2p 集成）

```rust
// node/src/service.rs
use sc_network::{NetworkService, config::{NetworkConfiguration, TransportConfig}};

pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    // 网络配置
    let mut network_config = NetworkConfiguration::new(
        config.network.node_name.clone(),
        "substrate-node",
        Default::default(),
        None,
    );

    // 设置 Bootstrap 节点
    network_config.boot_nodes = vec![
        "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooW...".parse().unwrap(),
    ];

    // 传输配置
    network_config.transport = TransportConfig::Normal {
        enable_mdns: true,
        allow_private_ipv4: true,
    };

    // 创建网络服务
    let (network, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync: None,
        })?;

    // 启动网络
    network_starter.start_network();

    Ok(task_manager)
}
```

**自定义网络协议**：
```rust
use sc_network::{
    config::ProtocolId,
    NetworkService,
};
use sc_network_gossip::{GossipEngine, MessageIntent, ValidationResult, ValidatorContext};

// 定义自定义协议
const PROTOCOL_NAME: &str = "/custom-protocol/1";

pub struct CustomProtocolValidator;

impl<B: BlockT> Validator<B> for CustomProtocolValidator {
    fn validate(
        &self,
        _context: &mut dyn ValidatorContext<B>,
        _sender: &PeerId,
        data: &[u8],
    ) -> ValidationResult<B::Hash> {
        // 验证消息
        if data.len() > 1024 {
            return ValidationResult::Discard;
        }

        ValidationResult::ProcessAndKeep(H256::random())
    }
}

// 使用 Gossip 引擎
pub struct CustomGossip {
    gossip_engine: Arc<Mutex<GossipEngine<Block>>>,
}

impl CustomGossip {
    pub fn new(network: Arc<NetworkService<Block, H256>>) -> Self {
        let gossip_engine = GossipEngine::new(
            network,
            PROTOCOL_NAME,
            Arc::new(CustomProtocolValidator),
            None,
        );

        Self {
            gossip_engine: Arc::new(Mutex::new(gossip_engine)),
        }
    }

    pub fn gossip_message(&self, topic: H256, data: Vec<u8>) {
        let mut engine = self.gossip_engine.lock();
        engine.gossip_message(topic, data, false);
    }
}
```

#### 4.3 交易池管理

```rust
use sc_transaction_pool::{BasicPool, FullChainApi};
use sp_runtime::transaction_validity::{
    TransactionPriority, TransactionValidity, ValidTransaction,
};

// 自定义交易优先级
impl<T: Config> Pallet<T> {
    fn prioritize_transaction(who: &T::AccountId) -> TransactionPriority {
        // VIP 用户高优先级
        if Self::is_vip_user(who) {
            return 100;
        }

        // 根据质押金额计算优先级
        let staked = Self::get_staked_balance(who);
        (staked / 1000) as TransactionPriority
    }
}

// 交易验证
#[pallet::validate_unsigned]
impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    fn validate_unsigned(
        _source: TransactionSource,
        call: &Self::Call,
    ) -> TransactionValidity {
        match call {
            Call::submit_data { data, signature } => {
                // 验证签名
                if !Self::verify_signature(data, signature) {
                    return InvalidTransaction::BadProof.into();
                }

                ValidTransaction::with_tag_prefix("CustomProtocol")
                    .priority(Self::calculate_priority(data))
                    .and_provides(vec![data.hash()])
                    .longevity(64)
                    .propagate(true)
                    .build()
            }
            _ => InvalidTransaction::Call.into(),
        }
    }
}
```

#### 实践项目
```rust
// 项目4：自定义共识 PoA 链
// 目标：实现权威证明共识机制

// 功能要求：
// 1. 验证者集合管理（添加/移除）
// 2. 轮流出块机制
// 3. 最终性确认（简化版 GRANDPA）
// 4. 网络层配置（bootstrap nodes）
// 5. 监控指标（区块生产、网络连接）

// 性能目标：
// - 出块时间：3秒
// - 最终性：2个区块后
// - 支持 100+ 验证者
```

---

### 第五阶段：性能优化与架构 (Week 9-10)

#### 5.1 低延迟优化（基于 CLAUDE.md 标准）

**Runtime 性能优化**：

```rust
// 1. 使用 WeightToFee 优化权重计算
use frame_support::weights::{
    WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
};

pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
    type Balance = Balance;

    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        // 线性映射：1 weight = 1 / 10_000 UNIT
        let p = UNIT / 10_000;
        let q = Balance::from(ExtrinsicBaseWeight::get().ref_time());
        smallvec![WeightToFeeCoefficient {
            degree: 1,
            negative: false,
            coeff_frac: Perbill::from_rational(p % q, q),
            coeff_integer: p / q,
        }]
    }
}

// 2. 缓存优化
use frame_support::storage::with_transaction;

impl<T: Config> Pallet<T> {
    // 使用事务避免重复读取
    pub fn batch_update(updates: Vec<(T::AccountId, Balance)>) -> DispatchResult {
        with_transaction(|| {
            for (account, balance) in updates {
                Self::update_balance(&account, balance)?;
            }
            Ok(())
        })
    }
}

// 3. 避免不必要的编解码
use frame_support::storage::StorageValue;

// ❌ 低效：多次编解码
let value = StorageItem::<T>::get().unwrap_or_default();
let new_value = value + 1;
StorageItem::<T>::put(new_value);

// ✅ 高效：使用 mutate
StorageItem::<T>::mutate(|value| {
    *value += 1;
});

// 4. 使用 Blake2_128Concat 替代 Blake2_256
#[pallet::storage]
pub type FastMap<T: Config> = StorageMap<
    _,
    Blake2_128Concat,  // 更快的哈希
    T::AccountId,
    Data,
>;

// 5. 批量操作优化
impl<T: Config> Pallet<T> {
    pub fn batch_insert(items: Vec<(Key, Value)>) -> DispatchResult {
        // 预先计算总权重
        let total_weight = items.len() as u64 * T::DbWeight::get().writes(1);

        ensure!(
            total_weight < T::BlockWeights::get().max_block,
            Error::<T>::TooManyItems
        );

        // 批量插入
        for (key, value) in items {
            Storage::<T>::insert(key, value);
        }

        Ok(())
    }
}
```

**数据库性能调优**：

```rust
// node/src/service.rs
use sc_service::config::DatabaseSource;

pub fn database_config() -> DatabaseSource {
    DatabaseSource::ParityDb {
        path: db_path,
    }
}

// ParityDB 配置（相比 RocksDB 更快）
pub fn parity_db_config() -> parity_db::Options {
    parity_db::Options {
        columns: vec![
            parity_db::ColumnOptions {
                preimage: true,
                compression: parity_db::CompressionType::Lz4,
                ..Default::default()
            };
            12  // 12 个列族
        ],
        sync_wal: true,
        sync_data: false,  // 异步刷盘提升性能
        stats: true,
        ..Default::default()
    }
}
```

**并发优化**：

```rust
// 使用并行迭代器
use rayon::prelude::*;

impl<T: Config> Pallet<T> {
    pub fn parallel_validation(transactions: Vec<Transaction>) -> Vec<bool> {
        transactions
            .par_iter()  // 并行迭代
            .map(|tx| Self::validate_transaction(tx))
            .collect()
    }
}

// Off-chain Worker 并发
use sp_runtime::offchain::Duration;

impl<T: Config> Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) {
        // 使用线程池处理多个任务
        let handles: Vec<_> = (0..10)
            .map(|i| {
                sp_io::offchain::spawn(move || {
                    Self::fetch_data_source(i);
                })
            })
            .collect();

        // 等待所有任务完成
        for handle in handles {
            handle.join();
        }
    }
}
```

#### 5.2 Clean Architecture 实践

**遵循 CLAUDE.md 的架构要求**：

```
substrate-project/
├── runtime/                         # Runtime 层
│   ├── src/
│   │   ├── lib.rs                  # Runtime 构建
│   │   └── configs.rs              # 配置聚合
│   └── Cargo.toml
│
├── pallets/                         # Pallets（领域层）
│   ├── trading/                    # 交易 Pallet
│   │   ├── src/
│   │   │   ├── lib.rs             # Pallet 定义
│   │   │   ├── types.rs           # 领域类型
│   │   │   ├── weights.rs         # 权重计算
│   │   │   └── benchmarking.rs   # 基准测试
│   │   └── Cargo.toml
│   │
│   └── order-book/                 # 订单簿 Pallet
│       └── src/
│           ├── lib.rs
│           ├── entities/          # 领域实体
│           │   ├── order.rs
│           │   └── trade.rs
│           └── usecases/          # 业务用例
│               ├── place_order.rs
│               └── match_orders.rs
│
├── node/                            # 节点（基础设施层）
│   ├── src/
│   │   ├── main.rs
│   │   ├── service.rs             # 服务配置
│   │   ├── chain_spec.rs          # 链规范
│   │   ├── rpc.rs                 # RPC 接口
│   │   └── cli.rs                 # CLI 接口
│   └── Cargo.toml
│
└── primitives/                      # 共享原语
    └── src/
        ├── types.rs                # 基础类型
        └── traits.rs               # 共享 Traits
```

**领域驱动设计示例**：

```rust
// pallets/order-book/src/entities/order.rs
/// 订单实体 - 纯业务逻辑
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Order<AccountId, Balance, BlockNumber> {
    pub id: OrderId,
    pub trader: AccountId,
    pub side: OrderSide,
    pub price: Balance,
    pub quantity: Balance,
    pub filled: Balance,
    pub status: OrderStatus,
    pub created_at: BlockNumber,
}

impl<AccountId, Balance: AtLeast32BitUnsigned, BlockNumber> Order<AccountId, Balance, BlockNumber> {
    /// 领域方法：验证订单
    pub fn validate(&self) -> Result<(), OrderError> {
        if self.quantity == Zero::zero() {
            return Err(OrderError::InvalidQuantity);
        }
        if self.price == Zero::zero() {
            return Err(OrderError::InvalidPrice);
        }
        Ok(())
    }

    /// 领域方法：填充订单
    pub fn fill(&mut self, amount: Balance) -> Result<(), OrderError> {
        let new_filled = self.filled
            .checked_add(&amount)
            .ok_or(OrderError::Overflow)?;

        if new_filled > self.quantity {
            return Err(OrderError::OverFill);
        }

        self.filled = new_filled;

        if self.filled == self.quantity {
            self.status = OrderStatus::Filled;
        } else {
            self.status = OrderStatus::PartiallyFilled;
        }

        Ok(())
    }

    /// 领域方法：取消订单
    pub fn cancel(mut self) -> Result<Self, OrderError> {
        match self.status {
            OrderStatus::Pending | OrderStatus::PartiallyFilled => {
                self.status = OrderStatus::Cancelled;
                Ok(self)
            }
            _ => Err(OrderError::CannotCancel),
        }
    }

    /// 计算剩余数量
    pub fn remaining(&self) -> Balance {
        self.quantity.saturating_sub(self.filled)
    }
}

// pallets/order-book/src/usecases/match_orders.rs
/// 订单匹配用例
pub struct OrderMatchingEngine<T: Config> {
    _phantom: PhantomData<T>,
}

impl<T: Config> OrderMatchingEngine<T> {
    /// 匹配买卖订单
    pub fn match_orders(
        buy_order: &mut Order<T::AccountId, BalanceOf<T>, T::BlockNumber>,
        sell_order: &mut Order<T::AccountId, BalanceOf<T>, T::BlockNumber>,
    ) -> Result<Trade<T>, MatchError> {
        // 1. 验证价格匹配
        ensure!(
            buy_order.price >= sell_order.price,
            MatchError::PriceMismatch
        );

        // 2. 计算成交量
        let trade_quantity = buy_order.remaining().min(sell_order.remaining());
        let trade_price = sell_order.price;  // Taker价格

        // 3. 更新订单状态
        buy_order.fill(trade_quantity)?;
        sell_order.fill(trade_quantity)?;

        // 4. 创建交易记录
        Ok(Trade {
            buy_order_id: buy_order.id,
            sell_order_id: sell_order.id,
            buyer: buy_order.trader.clone(),
            seller: sell_order.trader.clone(),
            price: trade_price,
            quantity: trade_quantity,
            timestamp: <frame_system::Pallet<T>>::block_number(),
        })
    }

    /// 撮合订单簿
    pub fn match_order_book() -> DispatchResult {
        // 获取最优买卖盘
        let mut buy_orders = Pallet::<T>::get_best_buy_orders(10);
        let mut sell_orders = Pallet::<T>::get_best_sell_orders(10);

        let mut trades = Vec::new();

        // 持续匹配直到无法匹配
        loop {
            let buy = buy_orders.first_mut();
            let sell = sell_orders.first_mut();

            match (buy, sell) {
                (Some(buy_order), Some(sell_order)) => {
                    if buy_order.price >= sell_order.price {
                        // 执行匹配
                        let trade = Self::match_orders(buy_order, sell_order)?;
                        trades.push(trade);

                        // 移除已完全成交的订单
                        if buy_order.status == OrderStatus::Filled {
                            buy_orders.remove(0);
                        }
                        if sell_order.status == OrderStatus::Filled {
                            sell_orders.remove(0);
                        }
                    } else {
                        break;  // 价格不匹配，停止撮合
                    }
                }
                _ => break,  // 没有订单了
            }
        }

        // 发布交易事件
        for trade in trades {
            Pallet::<T>::deposit_event(Event::TradeExecuted(trade));
        }

        Ok(())
    }
}

// Pallet 实现（应用层）
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(T::WeightInfo::place_order())]
    pub fn place_order(
        origin: OriginFor<T>,
        side: OrderSide,
        price: BalanceOf<T>,
        quantity: BalanceOf<T>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 创建订单实体
        let mut order = Order {
            id: Self::next_order_id(),
            trader: who.clone(),
            side,
            price,
            quantity,
            filled: Zero::zero(),
            status: OrderStatus::Pending,
            created_at: <frame_system::Pallet<T>>::block_number(),
        };

        // 领域验证
        order.validate()?;

        // 锁定资金
        Self::lock_funds(&who, side, price, quantity)?;

        // 存储订单
        Orders::<T>::insert(order.id, order.clone());

        // 尝试立即撮合
        OrderMatchingEngine::<T>::match_order_book()?;

        // 发布事件
        Self::deposit_event(Event::OrderPlaced {
            order_id: order.id,
            trader: who,
            side,
            price,
            quantity,
        });

        Ok(())
    }
}
```

**测试分层**：

```rust
// 单元测试：领域实体
#[cfg(test)]
mod entity_tests {
    use super::*;

    #[test]
    fn test_order_fill() {
        let mut order = Order {
            id: 1,
            trader: 1,
            side: OrderSide::Buy,
            price: 100,
            quantity: 10,
            filled: 0,
            status: OrderStatus::Pending,
            created_at: 0,
        };

        assert!(order.fill(5).is_ok());
        assert_eq!(order.filled, 5);
        assert_eq!(order.status, OrderStatus::PartiallyFilled);

        assert!(order.fill(5).is_ok());
        assert_eq!(order.filled, 10);
        assert_eq!(order.status, OrderStatus::Filled);

        assert!(order.fill(1).is_err());  // 超量填充
    }
}

// 集成测试：用例层
#[cfg(test)]
mod usecase_tests {
    use super::*;
    use crate::mock::*;

    #[test]
    fn test_order_matching() {
        new_test_ext().execute_with(|| {
            let mut buy_order = create_buy_order(100, 10);
            let mut sell_order = create_sell_order(100, 10);

            let result = OrderMatchingEngine::<Test>::match_orders(
                &mut buy_order,
                &mut sell_order,
            );

            assert!(result.is_ok());
            let trade = result.unwrap();
            assert_eq!(trade.quantity, 10);
            assert_eq!(buy_order.status, OrderStatus::Filled);
            assert_eq!(sell_order.status, OrderStatus::Filled);
        });
    }
}

// E2E 测试：完整流程
#[cfg(test)]
mod e2e_tests {
    use super::*;
    use crate::mock::*;

    #[test]
    fn test_full_trading_flow() {
        new_test_ext().execute_with(|| {
            // 1. Alice 下买单
            assert_ok!(OrderBook::place_order(
                RuntimeOrigin::signed(ALICE),
                OrderSide::Buy,
                100,
                10,
            ));

            // 2. Bob 下卖单
            assert_ok!(OrderBook::place_order(
                RuntimeOrigin::signed(BOB),
                OrderSide::Sell,
                100,
                10,
            ));

            // 3. 验证成交
            assert_eq!(OrderBook::orders(1).unwrap().status, OrderStatus::Filled);
            assert_eq!(OrderBook::orders(2).unwrap().status, OrderStatus::Filled);

            // 4. 验证余额变化
            assert_eq!(Balances::free_balance(ALICE), 900);
            assert_eq!(Balances::free_balance(BOB), 1100);
        });
    }
}
```

#### 实践项目
```rust
// 项目5：高性能 DEX（去中心化交易所）
// 目标：构建生产级链上交易系统

// 核心功能：
// 1. 订单簿管理（限价单、市价单）
// 2. 自动做市商（AMM）池
// 3. 流动性挖矿
// 4. 交易手续费分配
// 5. K线数据聚合（Off-chain Indexing）

// 架构要求：
// - Clean Architecture 分层
// - 领域驱动设计（DDD）
// - 单元测试覆盖率 > 90%
// - 基准测试和性能报告

// 性能目标（基于 CLAUDE.md）：
// - 订单匹配延迟 < 1ms
// - 吞吐量 > 1000 TPS
// - 区块时间 3秒
// - 支持 100+ 交易对
```

---

### 第六阶段：生产部署与生态集成 (Week 11-12)

#### 6.1 XCM（跨链消息传递）

```rust
// XCM 配置
use xcm::latest::prelude::*;
use xcm_builder::*;
use xcm_executor::XcmExecutor;

// 定义资产转换
pub type LocationToAccountId = (
    ParentIsPreset<AccountId>,
    SiblingParachainConvertsVia<Sibling, AccountId>,
);

// XCM 执行器配置
pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type IsReserve = NativeAsset;
    type IsTeleporter = ();
    type LocationInverter = LocationInverter<Ancestry>;
    type Barrier = Barrier;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type Trader = UsingComponents<WeightToFee, RelayLocation, AccountId, Balances, ()>;
    type ResponseHandler = PolkadotXcm;
    type AssetTrap = PolkadotXcm;
    type AssetClaims = PolkadotXcm;
    type SubscriptionService = PolkadotXcm;
}

// 跨链转账示例
impl<T: Config> Pallet<T> {
    pub fn transfer_to_parachain(
        origin: OriginFor<T>,
        para_id: ParaId,
        beneficiary: AccountId,
        amount: Balance,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 构建 XCM 消息
        let message = Xcm(vec![
            WithdrawAsset((Here, amount).into()),
            BuyExecution {
                fees: (Here, amount).into(),
                weight_limit: Unlimited,
            },
            DepositAsset {
                assets: All.into(),
                max_assets: 1,
                beneficiary: Junction::AccountId32 {
                    network: NetworkId::Any,
                    id: beneficiary.into(),
                }
                .into(),
            },
        ]);

        // 发送消息
        let dest = (Parent, Parachain(para_id.into())).into();
        <pallet_xcm::Pallet<T>>::send_xcm(Here, dest, message)
            .map_err(|_| Error::<T>::XcmSendFailed)?;

        Ok(())
    }
}
```

#### 6.2 平行链接入 Polkadot

```rust
// 配置 Cumulus（平行链框架）
use cumulus_pallet_parachain_system;
use cumulus_primitives_core::ParaId;

// Runtime 配置
impl cumulus_pallet_parachain_system::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnSystemEvent = ();
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type DmpMessageHandler = DmpQueue;
    type ReservedDmpWeight = ReservedDmpWeight;
    type OutboundXcmpMessageSource = XcmpQueue;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

// 收集人（Collator）配置
pub fn start_collator(
    parachain_id: ParaId,
    collator_key: CollatorPair,
) -> Result<(), sc_service::Error> {
    let collator_service = CumulusCollatorService::new(
        parachain_id,
        collator_key,
        // ...
    );

    collator_service.start();
    Ok(())
}
```

#### 6.3 监控与可观测性

```rust
// Prometheus 指标导出
use substrate_prometheus_endpoint as prometheus;

#[derive(Clone)]
pub struct Metrics {
    pub block_height: prometheus::Gauge<prometheus::U64>,
    pub transaction_count: prometheus::Counter<prometheus::U64>,
    pub block_production_time: prometheus::Histogram,
}

impl Metrics {
    pub fn register(registry: &prometheus::Registry) -> Result<Self, prometheus::Error> {
        Ok(Self {
            block_height: prometheus::register(
                prometheus::Gauge::new("substrate_block_height", "Current block height")?,
                registry,
            )?,
            transaction_count: prometheus::register(
                prometheus::Counter::new("substrate_transaction_total", "Total transactions")?,
                registry,
            )?,
            block_production_time: prometheus::register(
                prometheus::Histogram::with_opts(
                    prometheus::HistogramOpts::new(
                        "substrate_block_production_time",
                        "Block production time in seconds",
                    )
                    .buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0]),
                )?,
                registry,
            )?,
        })
    }
}

// 在 Runtime 中使用
impl<T: Config> Pallet<T> {
    fn on_finalize(n: T::BlockNumber) {
        if let Some(metrics) = T::Metrics::get() {
            metrics.block_height.set(n.saturated_into());
        }
    }
}
```

**Grafana Dashboard 配置**：
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'substrate'
    static_configs:
      - targets: ['localhost:9615']  # Substrate metrics endpoint

# 常用指标查询
# - substrate_block_height: 当前区块高度
# - substrate_finalized_height: 最终确定高度
# - substrate_transaction_pool_validations_scheduled: 交易池大小
# - substrate_network_peers_count: 网络节点数
```

#### 6.4 日志与追踪

```rust
// 结构化日志
use tracing::{info, warn, error, debug, trace};

impl<T: Config> Pallet<T> {
    pub fn complex_operation(param: u32) -> DispatchResult {
        // 使用结构化日志
        info!(
            target: "runtime::my_pallet",
            param = %param,
            "Starting complex operation"
        );

        // Span 追踪
        let _span = tracing::info_span!(
            "complex_operation",
            param = %param
        ).entered();

        // 子操作
        Self::sub_operation()?;

        info!("Complex operation completed");
        Ok(())
    }
}

// 节点启动时配置日志
// RUST_LOG=runtime=debug,pallet_my_pallet=trace ./node-template --dev
```

#### 6.5 安全审计清单

**Runtime 安全**：
```rust
// 1. 算术溢出检查
use sp_arithmetic::traits::{CheckedAdd, CheckedSub, CheckedMul};

let result = value1
    .checked_add(&value2)
    .ok_or(Error::<T>::Overflow)?;

// 2. 权限验证
let who = ensure_signed(origin)?;
ensure!(Self::is_admin(&who), Error::<T>::NotAuthorized);

// 3. 重入保护
#[pallet::storage]
pub type ReentrancyGuard<T> = StorageValue<_, bool, ValueQuery>;

impl<T: Config> Pallet<T> {
    pub fn guarded_function(origin: OriginFor<T>) -> DispatchResult {
        ensure!(!ReentrancyGuard::<T>::get(), Error::<T>::Reentrant);
        ReentrancyGuard::<T>::put(true);

        // 执行操作
        Self::do_work()?;

        ReentrancyGuard::<T>::put(false);
        Ok(())
    }
}

// 4. 输入验证
pub fn validate_input(data: &[u8]) -> Result<(), Error<T>> {
    ensure!(data.len() <= 1024, Error::<T>::DataTooLarge);
    ensure!(!data.is_empty(), Error::<T>::EmptyData);
    // 更多验证...
    Ok(())
}

// 5. DOS 防护
#[pallet::storage]
#[pallet::getter(fn operation_count)]
pub type OperationCount<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    u32,
    ValueQuery,
>;

const MAX_OPS_PER_BLOCK: u32 = 10;

impl<T: Config> Pallet<T> {
    pub fn rate_limited_operation(origin: OriginFor<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;
        let count = OperationCount::<T>::get(&who);

        ensure!(count < MAX_OPS_PER_BLOCK, Error::<T>::RateLimitExceeded);

        OperationCount::<T>::insert(&who, count + 1);

        // 执行操作...

        Ok(())
    }
}
```

#### 6.6 部署脚本

```bash
#!/bin/bash
# deploy.sh - 生产环境部署脚本

set -e

# 配置
CHAIN_SPEC="production"
NODE_NAME="my-validator-01"
BASE_PATH="/data/substrate"
VALIDATOR_KEY_SEED="your-secure-seed"

# 编译 Release 版本
echo "Building release binary..."
cargo build --release --features runtime-benchmarks

# 生成链规范
echo "Generating chain spec..."
./target/release/node-template build-spec \
    --chain $CHAIN_SPEC \
    --raw > chain-spec-raw.json

# 生成验证者密钥
echo "Generating validator keys..."
./target/release/node-template key insert \
    --base-path $BASE_PATH \
    --chain chain-spec-raw.json \
    --scheme Sr25519 \
    --suri "$VALIDATOR_KEY_SEED" \
    --key-type aura

./target/release/node-template key insert \
    --base-path $BASE_PATH \
    --chain chain-spec-raw.json \
    --scheme Ed25519 \
    --suri "$VALIDATOR_KEY_SEED" \
    --key-type gran

# 启动节点（systemd 服务）
echo "Starting node service..."
sudo systemctl start substrate-node
sudo systemctl enable substrate-node

echo "Deployment complete!"
```

**Systemd 服务配置**：
```ini
# /etc/systemd/system/substrate-node.service
[Unit]
Description=Substrate Node
After=network.target

[Service]
Type=simple
User=substrate
WorkingDirectory=/opt/substrate
ExecStart=/opt/substrate/node-template \
    --base-path /data/substrate \
    --chain /opt/substrate/chain-spec-raw.json \
    --port 30333 \
    --ws-port 9944 \
    --rpc-port 9933 \
    --validator \
    --name "MyValidator01" \
    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
Restart=always
RestartSec=10
LimitNOFILE=10000

[Install]
WantedBy=multi-user.target
```

#### 实践项目
```rust
// 项目6：生产级 Substrate 链
// 目标：部署可运行的公链

// 技术栈：
// - 自定义 Runtime（5+ 自定义 Pallets）
// - Aura + GRANDPA 共识
// - 平行链接入 Polkadot Rococo 测试网
// - XCM 跨链通信
// - 监控与告警系统

// 功能模块：
// 1. 身份认证系统
// 2. 治理模块（提案、投票）
// 3. 质押与奖励
// 4. 链上治理
// 5. 跨链资产转移

// 部署要求：
// - 3+ 验证者节点
// - 完整监控（Prometheus + Grafana）
// - 自动化部署脚本
// - 安全审计报告
// - 用户文档和 API 文档
```

---

## 学习资源

### 官方文档
- [Substrate 官方文档](https://docs.substrate.io/)
- [Substrate Tutorials](https://docs.substrate.io/tutorials/)
- [Polkadot Wiki](https://wiki.polkadot.network/)
- [Rust Docs](https://paritytech.github.io/substrate/master/)

### 在线课程
- [Substrate Kitties Tutorial](https://docs.substrate.io/tutorials/v3/kitties/pt1/)
- [Polkadot Blockchain Academy](https://polkadot.network/development/academy/)
- [Web3 Foundation MOOC](https://www.youtube.com/@Web3Foundation)

### 书籍与论文
- "Mastering Substrate" (社区书籍)
- Polkadot Whitepaper
- GRANDPA 共识论文

### 开源项目参考
- [Polkadot](https://github.com/paritytech/polkadot) - 中继链实现
- [Moonbeam](https://github.com/PureStake/moonbeam) - EVM 兼容平行链
- [Acala](https://github.com/AcalaNetwork/Acala) - DeFi 平行链
- [Subscan](https://github.com/itering/subscan-essentials) - 区块浏览器

### 社区资源
- [Substrate Stack Exchange](https://substrate.stackexchange.com/)
- [Substrate Technical](https://matrix.to/#/#substrate-technical:matrix.org)
- [Polkadot Discord](https://discord.gg/polkadot)

---

## 评估标准

### 知识掌握度
- [ ] 理解 Substrate 架构和设计理念
- [ ] 掌握 FRAME 框架和 Pallet 开发
- [ ] 理解共识机制和网络层
- [ ] 熟悉 XCM 和跨链通信
- [ ] 掌握性能优化技巧

### 实践能力
- [ ] 独立开发自定义 Pallet
- [ ] 配置和优化 Runtime
- [ ] 部署和运维节点
- [ ] 调试和性能分析
- [ ] 编写测试和基准

### 综合项目
完成至少3个项目：
1. 基础 Pallet（Token、NFT等）
2. 中级应用（DEX、拍卖系统）
3. 生产级链（完整公链部署）

---

## 学习笔记模板

```markdown
## Week N 学习总结

### 学习内容
- Substrate 核心概念
- Pallet 开发实践

### 代码实现
```rust
// 本周重要代码
```

### 问题与解决
1. 问题描述
   - 解决方案
   - 参考链接

### 性能基准
- 权重计算结果
- 基准测试数据

### 下周计划
- [ ] 任务列表
```

---

## 项目检查清单

### 代码质量
- [ ] Clean Architecture 分层
- [ ] 单元测试 > 80%
- [ ] 基准测试完整
- [ ] 文档齐全

### 性能指标
- [ ] 权重计算准确
- [ ] 存储优化
- [ ] 无算术溢出
- [ ] 基准报告

### 安全性
- [ ] 权限验证
- [ ] 输入校验
- [ ] 重入防护
- [ ] 审计通过

### 部署就绪
- [ ] 编译成功
- [ ] 测试网验证
- [ ] 监控配置
- [ ] 文档完整

---

## 时间线

| Week | 阶段 | 里程碑 |
|------|------|--------|
| 1-2  | 基础概念 | 理解架构，运行节点 |
| 3-4  | Runtime开发 | 开发 Token Pallet |
| 5-6  | 高级特性 | OCW、拍卖系统 |
| 7-8  | 共识网络 | 自定义共识链 |
| 9-10 | 性能架构 | DEX 交易系统 |
| 11-12| 生产部署 | 完整公链上线 |

---

## 持续学习

Substrate 生态快速发展，建议：
- 关注 Polkadot Decoded 大会
- 参与 Substrate Builders Program
- 贡献开源代码
- 加入 Hackathon 活动

**构建下一代区块链，从 Substrate 开始！**
