# libp2p Chat 学习路径

通过这个项目逐步掌握 libp2p 的核心概念。

## 学习路线图

```
第1天: 运行和理解
    ↓
第2天: 代码结构分析
    ↓
第3-4天: 核心概念深入
    ↓
第5-7天: 实现基础练习
    ↓
第8-10天: 实现中级功能
    ↓
第11-14天: 高级特性开发
```

---

## 第1天：运行和理解

### 目标
- 成功运行 chat 程序
- 理解 P2P 通信的基本原理
- 观察节点发现和消息传播过程

### 任务清单
- [ ] 阅读 [QUICKSTART.md](./QUICKSTART.md)
- [ ] 编译并运行第一个节点
- [ ] 启动第二个节点，观察自动发现过程
- [ ] 发送和接收消息
- [ ] 启动 3-5 个节点进行多节点测试

### 实验
```bash
# 实验1：观察节点发现时间
# 记录从启动第二个节点到建立连接的时间

# 实验2：测试消息传播
# 启动5个节点，在节点1发消息，观察其他节点收到的顺序

# 实验3：节点离线
# 关闭某个节点，观察其他节点的反应
```

### 思考题
1. 为什么节点能自动发现彼此？
2. 消息是如何从一个节点传播到所有节点的？
3. 关闭一个节点后，其他节点还能正常通信吗？

---

## 第2天：代码结构分析

### 目标
- 理解项目的整体架构
- 掌握 libp2p 的核心组件
- 了解异步编程模型

### 任务清单
- [ ] 阅读 [chat.rs](./chat.rs) 完整代码
- [ ] 理解每个函数的作用
- [ ] 绘制架构图
- [ ] 追踪一条消息的完整流程

### 核心组件理解

#### 1. Transport Layer（传输层）
```rust
let transport = tcp::tokio::Transport::default()
    .upgrade(upgrade::Version::V1Lazy)
    .authenticate(noise::Config::new(&local_key)?)
    .multiplex(yamux::Config::default())
    .boxed();
```

**问题**：
- 为什么需要三层包装（认证、加密、多路复用）？
- 如果去掉 Noise 会怎样？
- Yamux 的作用是什么？

#### 2. Network Behaviour（网络行为）
```rust
#[derive(NetworkBehaviour)]
struct ChatBehaviour {
    mdns: mdns::tokio::Behaviour,
    gossipsub: gossipsub::Behaviour,
    identify: identify::Behaviour,
}
```

**问题**：
- 为什么使用宏 `#[derive(NetworkBehaviour)]`？
- 可以只用 Gossipsub 而不用 mDNS 吗？
- Identify 协议的具体作用是什么？

#### 3. Event Loop（事件循环）
```rust
loop {
    tokio::select! {
        event = swarm.select_next_some() => { /* 网络事件 */ }
        Some(line) = rx.recv() => { /* 用户输入 */ }
    }
}
```

**问题**：
- `tokio::select!` 是如何工作的？
- 为什么需要两个事件源？
- 如果事件处理很慢会发生什么？

### 练习
```rust
// 练习1：添加日志
// 在每个事件处理中添加详细日志，观察执行顺序

// 练习2：绘制时序图
// 画出从启动到发送消息的完整时序图

// 练习3：修改配置
// 尝试修改 Gossipsub 的心跳间隔，观察影响
```

---

## 第3-4天：核心概念深入

### Day 3: PeerId 和 Multiaddr

#### PeerId 深入理解
```rust
// 实验：生成多个 PeerId
for i in 0..5 {
    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());
    println!("{}: {}", i, peer_id);
}
```

**练习**：
1. 比较不同密钥算法（Ed25519, RSA, Secp256k1）
2. 理解 PeerId 的结构和验证机制
3. 实现 PeerId 的序列化和反序列化

#### Multiaddr 理解
```rust
// 实验：解析 Multiaddr
let addr = "/ip4/192.168.1.100/tcp/4001/p2p/12D3KooW...";
let multiaddr: Multiaddr = addr.parse()?;

// 提取各个组件
for protocol in multiaddr.iter() {
    println!("{:?}", protocol);
}
```

**练习**：
1. 构建不同类型的 Multiaddr
2. 实现地址的合并和拆分
3. 处理无效地址

### Day 4: Gossipsub 深入

#### Mesh 网络理解
```
节点 A ─────┐
            │
节点 B ─── 节点 D ─── 节点 F
            │
节点 C ─────┘
```

**核心参数**：
```rust
let config = gossipsub::ConfigBuilder::default()
    .mesh_n_low(4)       // mesh网络最小节点数
    .mesh_n(6)           // mesh网络目标节点数
    .mesh_n_high(12)     // mesh网络最大节点数
    .heartbeat_interval(Duration::from_secs(1))
    .build()?;
```

**实验**：
1. 修改 `mesh_n` 参数，观察消息传播路径
2. 增加心跳间隔，测试网络稳定性
3. 实现消息追踪功能

#### 消息去重机制
```rust
// 自定义消息ID
let message_id_fn = |message: &gossipsub::Message| {
    // 实验：不同的ID生成策略
    // 1. 基于内容哈希
    // 2. 基于时间戳
    // 3. 基于序列号
};
```

---

## 第5-7天：基础练习实现

### Day 5: 用户昵称功能

**需求**：
- 启动时输入昵称
- 消息显示昵称而非 PeerId
- 昵称唯一性验证

**实现步骤**：

#### 1. 定义消息结构
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    nickname: String,
    content: String,
    timestamp: u64,
}
```

#### 2. 修改消息发送
```rust
// 序列化消息
let msg = ChatMessage {
    nickname: nickname.clone(),
    content: line,
    timestamp: SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs(),
};

let json = serde_json::to_string(&msg)?;
swarm.behaviour_mut()
    .gossipsub
    .publish(topic.clone(), json.as_bytes())?;
```

#### 3. 修改消息接收
```rust
// 反序列化消息
let msg: ChatMessage = serde_json::from_slice(&message.data)?;
println!("[{}] {}: {}", msg.timestamp, msg.nickname, msg.content);
```

**测试清单**：
- [ ] 多个节点可以设置不同昵称
- [ ] 昵称正确显示在消息中
- [ ] 时间戳正确显示

### Day 6: 消息历史功能

**需求**：
- 保存最近 100 条消息
- 支持查看历史
- 持久化到文件（可选）

**实现提示**：
```rust
use std::collections::VecDeque;

struct MessageHistory {
    messages: VecDeque<ChatMessage>,
    max_size: usize,
}

impl MessageHistory {
    fn new(max_size: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    fn add(&mut self, msg: ChatMessage) {
        if self.messages.len() >= self.max_size {
            self.messages.pop_front();
        }
        self.messages.push_back(msg);
    }

    fn get_last(&self, n: usize) -> Vec<&ChatMessage> {
        self.messages.iter().rev().take(n).collect()
    }
}
```

**命令实现**：
```rust
match line.as_str() {
    "/history" => {
        let recent = history.get_last(10);
        for msg in recent {
            println!("[{}] {}: {}", msg.timestamp, msg.nickname, msg.content);
        }
    }
    // ...
}
```

### Day 7: 多聊天室支持

**需求**：
- 创建多个主题（聊天室）
- 切换聊天室
- 列出所有聊天室

**命令设计**：
```bash
/join <room>    # 加入聊天室
/leave <room>   # 离开聊天室
/rooms          # 列出聊天室
/current        # 显示当前聊天室
```

**实现提示**：
```rust
struct ChatRooms {
    current: Option<gossipsub::IdentTopic>,
    rooms: HashMap<String, gossipsub::IdentTopic>,
}

impl ChatRooms {
    fn join(&mut self, swarm: &mut Swarm<ChatBehaviour>, room: &str) -> Result<()> {
        let topic = gossipsub::IdentTopic::new(room);
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        self.rooms.insert(room.to_string(), topic.clone());
        self.current = Some(topic);
        Ok(())
    }

    fn leave(&mut self, swarm: &mut Swarm<ChatBehaviour>, room: &str) -> Result<()> {
        if let Some(topic) = self.rooms.remove(room) {
            swarm.behaviour_mut().gossipsub.unsubscribe(&topic)?;
        }
        Ok(())
    }
}
```

---

## 第8-10天：中级功能实现

### Day 8: 在线用户列表

**需求**：
- 显示当前在线的所有用户
- 实时更新用户状态
- 显示用户的连接信息

**实现架构**：
```rust
struct OnlineUsers {
    peers: HashMap<PeerId, UserInfo>,
}

struct UserInfo {
    peer_id: PeerId,
    nickname: String,
    connected_at: SystemTime,
    last_seen: SystemTime,
    addresses: Vec<Multiaddr>,
}

impl OnlineUsers {
    fn add_peer(&mut self, peer_id: PeerId, info: UserInfo) {
        self.peers.insert(peer_id, info);
    }

    fn remove_peer(&mut self, peer_id: &PeerId) {
        self.peers.remove(peer_id);
    }

    fn update_last_seen(&mut self, peer_id: &PeerId) {
        if let Some(info) = self.peers.get_mut(peer_id) {
            info.last_seen = SystemTime::now();
        }
    }

    fn list_online(&self) -> Vec<&UserInfo> {
        self.peers.values().collect()
    }
}
```

**命令实现**：
```rust
"/peers" => {
    let online = online_users.list_online();
    println!("在线用户 ({}):", online.len());
    for user in online {
        println!("  - {} ({})", user.nickname, user.peer_id);
    }
}
```

### Day 9: 私聊功能

**需求**：
- 点对点私密消息
- 消息加密
- 发送状态反馈

**实现提示**：

#### 1. 添加 Request-Response 协议
```rust
use libp2p::request_response::{
    ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseEvent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivateMessage {
    from: String,
    to: String,
    content: String,
}

// 实现编解码器
struct PrivateMessageCodec;

#[async_trait]
impl RequestResponseCodec for PrivateMessageCodec {
    type Protocol = StreamProtocol;
    type Request = PrivateMessage;
    type Response = PrivateMessageResponse;

    async fn read_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        // 实现请求读取
    }

    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        // 实现响应读取
    }

    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        // 实现请求写入
    }

    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        // 实现响应写入
    }
}
```

#### 2. 命令实现
```bash
/msg <nickname> <content>   # 发送私聊消息
```

### Day 10: 文件传输

**需求**：
- 发送小文件（< 10MB）
- 进度显示
- 完整性校验

**实现步骤**：

1. 文件分块
2. 使用 Request-Response 传输
3. 重组文件
4. 校验哈希

```rust
const CHUNK_SIZE: usize = 64 * 1024; // 64KB

struct FileTransfer {
    file_id: String,
    file_name: String,
    total_size: usize,
    chunks: Vec<Vec<u8>>,
    hash: [u8; 32],
}

impl FileTransfer {
    fn from_file(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        let hash = sha256(&data);

        let chunks: Vec<Vec<u8>> = data
            .chunks(CHUNK_SIZE)
            .map(|chunk| chunk.to_vec())
            .collect();

        Ok(Self {
            file_id: Uuid::new_v4().to_string(),
            file_name: path.file_name().unwrap().to_string_lossy().to_string(),
            total_size: data.len(),
            chunks,
            hash,
        })
    }

    fn verify(&self, data: &[u8]) -> bool {
        sha256(data) == self.hash
    }
}
```

---

## 第11-14天：高级特性

### Day 11-12: 端到端加密

使用 `age` 或 `x25519` 实现加密通信。

### Day 13: 性能优化

根据 CLAUDE.md 标准进行优化：
- 消息批处理
- 连接池管理
- 内存优化

### Day 14: 完整测试

编写完整的测试套件和文档。

---

## 学习检查点

### 第1周结束
- [ ] 理解 libp2p 基本架构
- [ ] 能够运行和修改代码
- [ ] 完成基础练习

### 第2周结束
- [ ] 实现中级功能
- [ ] 理解 P2P 协议细节
- [ ] 能够独立开发新特性

### 完成标志
- [ ] 所有练习完成
- [ ] 代码有详细注释
- [ ] 编写完整文档
- [ ] 通过性能测试

---

## 下一步

完成这个项目后：
1. 阅读 [libp2p 学习计划](../../../doc/libp2p_learning_plan.md)
2. 学习 Substrate 框架
3. 参与开源项目
4. 构建自己的 DApp

**加油！🚀**
