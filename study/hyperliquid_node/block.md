Rust 里取 `orders[]` 本质就三步：**拿原始 JSON bytes → serde_json 反序列化 → 下钻到 `action.orders`**。Hyperliquid 没有把 `orders` 塞进强类型 protobuf 里给你——`StreamBlocks` 的 `Record.data` 就是 **replica_cmds 的 JSON 字节**，`hl_getBlock` 返回也是同一套 JSON。

下面给两套：一套「够用的最小 serde 结构」，一套「直接用 hypersdk 类型」。

---

## 一、hl_getBlock（JSON-RPC）取 orders[]

### 1. Cargo.toml
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rust_decimal = "1"
```

### 2. 最小结构（只留订单相关字段）
```rust
use serde::Deserialize;

type Hex = String;

#[derive(Debug, Deserialize)]
struct RpcResp {
    result: Option<serde_json::Value>, // 不同网关可能包一层，先透传
}

#[derive(Debug, Deserialize)]
struct ReplicaBlock {
    #[serde(default)]
    height: Option<u64>,
    #[serde(default)]
    abci_block: Option<AbciBlock>,
    #[serde(default)]
    signed_action_bundles: Vec<(Hex, Bundle)>,
    #[serde(default)]
    resps: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AbciBlock {
    #[serde(default)]
    round: Option<i64>,
    #[serde(default)]
    time: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Bundle {
    #[serde(default)]
    signed_actions: Vec<SignedAction>,
    #[serde(default)]
    broadcaster: Option<Hex>,
}

#[derive(Debug, Deserialize)]
struct SignedAction {
    #[serde(default)]
    signature: Option<serde_json::Value>,
    action: Action,
    #[serde(default)]
    nonce: Option<u64>,
    #[serde(default)]
    vault_address: Option<Hex>,
}

#[derive(Debug, Deserialize)]
struct Action {
    #[serde(rename = "type")]
    ty: String,
    // order action 用
    #[serde(default)]
    orders: Vec<OrderWire>,
    #[serde(default)]
    grouping: Option<String>,
    // 老块/某些网关会把单笔订单平铺到 action 上，兜底兼容
    #[serde(default)]
    a: Option<u32>,
    #[serde(default)]
    b: Option<bool>,
    #[serde(default)]
    p: Option<String>,
    #[serde(default)]
    s: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OrderWire {
    a: u32,            // asset index
    b: bool,           // is_buy
    p: String,         // limit price (string)
    s: String,         // size (string)
    #[serde(default)]
    r: bool,           // reduce_only
    t: OrderType,
    #[serde(default)]
    c: Option<Hex>,    // cloid
}

#[derive(Debug, Deserialize)]
struct OrderType {
    #[serde(default)]
    limit: Option<LimitType>,
    #[serde(default)]
    trigger: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct LimitType {
    tif: String, // Gtc / Ioc / Alo
}
```

> 注意：新块 `orders` 在 `action.orders: [...]`；老样本里单笔订单可能平铺到 `action` 上（`a/b/p/s/t/c`）。上面 `Action` 同时留了两种，解析时优先用 `orders`，为空再退化为平铺字段。

### 3. 发请求 + 下钻
```rust
async fn get_block(rpc: &str, n: u64) -> anyhow::Result<()> {
    let body = serde_json::json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "hl_getBlock",
        "params": [ { "stream": "replica_cmds", "block_number": n } ]
    });

    let v: serde_json::Value = reqwest::Client::new()
        .post(rpc)
        .json(&body)
        .send().await?
        .json().await?;

    // 不同网关可能包成 { result: <block> } 或 { result: { ... } }
    let block_val = v.get("result").and_then(|r| r.as_object())
        .and_then(|_| Some(v["result"].clone()))
        .unwrap_or(v.clone());

    let block: ReplicaBlock = serde_json::from_value(block_val)?;

    for (bh, bundle) in &block.signed_action_bundles {
        for sa in &bundle.signed_actions {
            if sa.action.ty == "order" {
                let orders = if !sa.action.orders.is_empty() {
                    sa.action.orders.clone()
                } else {
                    // 兼容平铺单笔
                    if let (Some(a), Some(b), Some(p), Some(sz)) =
                        (sa.action.a, sa.action.b, &sa.action.p, &sa.action.s)
                    {
                        vec![OrderWire {
                            a, b, p: p.clone(), s: sz.clone(),
                            r: false,
                            t: OrderType { limit: Some(LimitType { tif: "Gtc".into() }), trigger: None },
                            c: None,
                        }]
                    } else { continue }
                };

                for o in orders {
                    println!(
                        "block={:?} bundle={} asset={} side={} px={} sz={} tif={:?} cloid={:?}",
                        block.height, bh, o.a, if o.b { "BUY" } else { "SELL" },
                        o.p, o.s, o.t.limit.as_ref().map(|l| l.tif.as_str()), o.c
                    );
                }
            }
        }
    }
    Ok(())
}
```

---

## 二、StreamBlocks（gRPC）取 orders[]

`Record` 里 `data: bytes` = replica_cmds JSON 字节。拿到后跟上面完全一样：`serde_json::from_slice::<ReplicaBlock>(&rec.data)`。

伪代码（tonic + 官方/网关 v3 proto）：
```rust
let mut stream = client.stream_blocks(StreamRequest {
    position: Some(Position { block_number: n.to_string() }),
    ..Default::default()
}).await?.into_inner();

while let Some(rec) = stream.message().await? {
    if rec.data.is_empty() { continue; }
    let block: ReplicaBlock = serde_json::from_slice(&rec.data)?;
    // 同上文下钻 signed_action_bundles -> action.orders
}
```

> 如果走 Dwellir/Quicknode gRPC，把 `v3.proto` 里 `Record.data` 当 `bytes` 处理即可，不要试图在 protobuf 里找 `orders` 字段——它在 bytes 内的 JSON 里。

---

## 三、不想手搓 serde：用 hypersdk / hl-types

- `hypersdk::hypercore::types` 有 `BatchOrder` / `OrderRequest` / `OrderTypePlacement` / `TimeInForce`，但那是**发单/WS 用**的域模型，不是 replica_cmds 原始块模型。
- `motosan-hyperliquid` / `hl-types` 有 `OrderWire` / `Tif` 等。

所以正确姿势：
1. 先用上面的 `ReplicaBlock` 把原始块吃进来
2. 再把 `OrderWire`（字符串价格/尺寸）map 到 `rust_decimal::Decimal` 或 SDK 的 `OrderRequest`

```rust
use rust_decimal::Decimal;
let px = Decimal::from_str(&o.p)?;
let sz = Decimal::from_str(&o.s)?;
```

---

## 四、几个坑（做交易系统要记住）

1. **`height` 可能没有**：`< 1085381001` 的块无顶层 `height`，靠你自己用请求时的 `block_number` 补。
2. **`round != block_number`**：`abci_block.round` 是 HyperBFT round，会跳。
3. **`orders` 是「下单指令数组」不是「挂单簿行」**：没有 `oid`、没有剩余 size。要 oid/剩余量去 `node_order_statuses_by_block` / `node_raw_book_diffs_by_block`。
4. **价格尺寸都是 string**：别 `deserialize_with` 成 f64，清算/精度会炸。
5. **bundle 是 `(hash, bundle)` 二元组**：serde 里用 `Vec<(String, Bundle)>` 接。
6. **`resps.Full` 对齐 bundle**：想判断「这条 order action 进没进共识」要对 `resps.Full` 里同 bundle hash 的 `status: ok/err`。

---

最小可执行链路一句话：
**`hl_getBlock(stream=replica_cmds)` → `serde_json::Value` → `ReplicaBlock` → `signed_action_bundles[].1.signed_actions[].action` → `ty=="order"` 取 `orders[]` → 转 `Decimal` 进你自己的订单表。**

如果你要接进撮合/回放，建议落盘时存原始 `data` 字节 + 解析后的 `OrderWire` 两行，方便重放时不用再请求节点。

**如果要做完整的订单生命周期追踪，接下来需要哪部分的解析代码：**