/*!
# libp2p P2P Chat 教学程序

这是一个用于学习 libp2p 的完整示例程序，实现了去中心化的聊天室功能。

## 学习目标

1. 理解 libp2p 的基本架构
2. 掌握 Transport、Network Behaviour 的使用
3. 学习节点发现（mDNS）
4. 实现消息广播（Gossipsub）
5. 了解节点身份识别（Identify）

## 架构概览

```
┌─────────────────────────────────────────┐
│           Swarm (事件循环)               │
│  ┌────────────┐  ┌────────────────────┐ │
│  │  mDNS      │  │   Gossipsub        │ │
│  │  (发现)    │  │   (消息传播)       │ │
│  └────────────┘  └────────────────────┘ │
│  ┌────────────┐                         │
│  │  Identify  │                         │
│  │  (身份)    │                         │
│  └────────────┘                         │
└─────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────┐
│       Transport Layer (传输层)           │
│   TCP + Noise加密 + Yamux多路复用       │
└─────────────────────────────────────────┘
```
*/

use futures::StreamExt;
use libp2p::{
    gossipsub, identify, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, SwarmBuilder,
};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tracing::{error, info, warn};

/// # 第一步：定义网络行为（Network Behaviour）
///
/// NetworkBehaviour 是 libp2p 的核心抽象，定义了节点的行为。
/// 我们组合多个协议来实现完整的聊天功能。
#[derive(NetworkBehaviour)]
struct ChatBehaviour {
    /// mDNS: 本地网络发现协议
    /// 作用：在局域网内自动发现其他节点，无需手动配置
    mdns: mdns::tokio::Behaviour,

    /// Gossipsub: 发布-订阅协议
    /// 作用：实现消息的广播传播，构建 mesh 网络
    gossipsub: gossipsub::Behaviour,

    /// Identify: 身份识别协议
    /// 作用：节点之间交换身份信息（PeerId、支持的协议、监听地址等）
    identify: identify::Behaviour,
}

/// # 第二步：配置和初始化
///
/// 创建 libp2p Swarm，配置传输层和网络行为
async fn create_swarm() -> Result<
    libp2p::Swarm<ChatBehaviour>,
    Box<dyn Error>,
> {
    // 2.1 生成密钥对和 PeerId
    // 每个节点都有唯一的身份标识
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    info!("🔑 本地节点 PeerID: {}", local_peer_id);

    // 2.2 使用 SwarmBuilder 创建 Swarm
    // libp2p 0.54 使用新的 builder API，自动配置传输层
    //
    // 这个 builder 会自动配置：
    // - TCP 传输协议
    // - Noise 加密
    // - Yamux 多路复用
    let swarm = SwarmBuilder::with_existing_identity(local_key.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            // 2.3 配置 Gossipsub（消息广播协议）
            //
            // Gossipsub 配置说明：
            // - heartbeat_interval: 心跳间隔，用于维护 mesh 网络
            // - validation_mode: 消息验证模式（Strict=严格验证签名）
            // - message_id_fn: 消息去重函数
            let message_id_fn = |message: &gossipsub::Message| {
                let mut hasher = DefaultHasher::new();
                message.data.hash(&mut hasher);
                gossipsub::MessageId::from(hasher.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(1))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            info!("✅ Gossipsub 协议配置完成");

            // 2.4 配置 mDNS（本地发现协议）
            //
            // mDNS 在局域网内广播节点存在，实现零配置发现
            // 注意：配置 query_interval 减少网络负载
            let mdns_config = mdns::Config {
                query_interval: Duration::from_secs(5),  // 查询间隔5秒
                ..Default::default()
            };

            let mdns = mdns::tokio::Behaviour::new(
                mdns_config,
                key.public().to_peer_id(),
            )?;

            info!("✅ mDNS 协议配置完成");

            // 2.5 配置 Identify（身份协议）
            //
            // Identify 协议用于节点间交换信息
            let identify = identify::Behaviour::new(identify::Config::new(
                "/chat/1.0.0".to_string(),
                key.public(),
            ));

            info!("✅ Identify 协议配置完成");

            // 2.6 组合所有行为
            Ok(ChatBehaviour {
                mdns,
                gossipsub,
                identify,
            })
        })?
        .build();

    info!("✅ Swarm 创建完成 (传输层: TCP + Noise + Yamux)");

    Ok(swarm)
}

/// # 第三步：主函数 - 启动聊天程序
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 3.1 初始化日志系统
    // 过滤掉 mDNS 的 ERROR 级别日志，减少虚拟接口的噪音
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
                .add_directive("libp2p_mdns=warn".parse()?),  // mDNS只显示warn及以上
        )
        .init();

    info!("🚀 启动 libp2p P2P Chat 程序");
    info!("📖 这是一个教学示例，展示 libp2p 的基本用法");

    // 3.2 创建 Swarm
    let mut swarm = create_swarm().await?;

    // 3.3 创建聊天室主题
    //
    // Gossipsub 使用主题（Topic）来组织消息
    // 只有订阅了相同主题的节点才能收到消息
    let topic = gossipsub::IdentTopic::new("chat-room");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    info!("📢 已订阅聊天室主题: {}", topic);

    // 3.4 监听网络地址
    //
    // 节点需要监听地址来接受入站连接
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    info!("🎧 开始监听网络连接...");

    // 3.5 启动命令行输入线程
    //
    // 使用独立线程处理用户输入，避免阻塞网络事件循环
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    tokio::spawn(async move {
        let stdin = std::io::stdin();
        loop {
            let mut line = String::new();
            if stdin.read_line(&mut line).is_ok() {
                let trimmed = line.trim().to_string();
                if !trimmed.is_empty() {
                    if tx.send(trimmed).is_err() {
                        break;
                    }
                }
            }
        }
    });

    info!("💬 聊天室已准备就绪！输入消息后按回车发送");
    info!("📝 提示：输入 /quit 退出程序");
    println!("\n════════════════════════════════════════");
    println!("   欢迎来到 libp2p P2P 聊天室！");
    println!("════════════════════════════════════════\n");

    // 3.6 主事件循环
    //
    // 这是程序的核心：处理网络事件和用户输入
    loop {
        tokio::select! {
            // 处理 Swarm 网络事件
            event = swarm.select_next_some() => {
                match event {
                    // 处理 Gossipsub 消息事件
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(
                        gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: _,
                            message,
                        },
                    )) => {
                        // 收到聊天消息
                        let msg_str = String::from_utf8_lossy(&message.data);
                        let sender = format!("{:.8}", peer_id.to_string());

                        println!("\n💬 [{}] {}", sender, msg_str);
                        println!("════════════════════════════════════════");
                    }

                    // 处理 mDNS 发现事件
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(
                        mdns::Event::Discovered(list),
                    )) => {
                        // 发现新节点
                        for (peer_id, multiaddr) in list {
                            info!("🔍 发现新节点: {} at {}", peer_id, multiaddr);

                            // 将发现的节点添加到 Gossipsub 的已知节点列表
                            swarm
                                .behaviour_mut()
                                .gossipsub
                                .add_explicit_peer(&peer_id);

                            println!("✅ 新成员加入聊天室: {:.8}", peer_id.to_string());
                        }
                    }

                    // 处理 mDNS 过期事件
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(
                        mdns::Event::Expired(list),
                    )) => {
                        // 节点离线
                        for (peer_id, _multiaddr) in list {
                            info!("👋 节点离线: {}", peer_id);

                            swarm
                                .behaviour_mut()
                                .gossipsub
                                .remove_explicit_peer(&peer_id);

                            println!("❌ 成员离开聊天室: {:.8}", peer_id.to_string());
                        }
                    }

                    // 处理 Identify 事件
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Identify(
                        identify::Event::Received { peer_id, info, .. },
                    )) => {
                        info!(
                            "📋 收到节点信息: {} (协议版本: {})",
                            peer_id, info.protocol_version
                        );
                    }

                    // 新的监听地址
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("🎧 监听地址: {}", address);
                        println!("🌐 节点地址: {}/p2p/{}", address, swarm.local_peer_id());
                    }

                    // 建立连接
                    SwarmEvent::ConnectionEstablished {
                        peer_id,
                        endpoint,
                        ..
                    } => {
                        info!(
                            "🔗 建立连接: {} ({})",
                            peer_id,
                            endpoint.get_remote_address()
                        );
                    }

                    // 连接关闭
                    SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                        warn!("🔌 连接关闭: {} (原因: {:?})", peer_id, cause);
                    }

                    // 其他事件
                    _ => {}
                }
            }

            // 处理用户输入
            Some(line) = rx.recv() => {
                // 检查退出命令
                if line.starts_with("/quit") {
                    info!("👋 退出程序");
                    break;
                }

                // 发布消息到聊天室
                if let Err(e) = swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(topic.clone(), line.as_bytes())
                {
                    error!("❌ 发送消息失败: {:?}", e);
                } else {
                    // 显示自己发送的消息
                    println!("📤 [我] {}", line);
                }
            }
        }
    }

    Ok(())
}

/* ============================================================================
   教学笔记：关键概念解释
   ============================================================================

   1. **PeerId (节点ID)**
      - 每个节点的唯一标识符
      - 由公钥生成，确保身份的唯一性和安全性
      - 格式：12D3KooW...（Base58编码）

   2. **Transport (传输层)**
      - 负责建立底层网络连接
      - 支持多种协议：TCP、UDP、QUIC、WebSocket等
      - 升级链：基础传输 -> 认证 -> 加密 -> 多路复用

   3. **NetworkBehaviour (网络行为)**
      - 定义节点的协议行为
      - 可组合：多个行为可以组合成复杂的网络功能
      - 事件驱动：通过事件处理网络交互

   4. **Swarm (群)**
      - 管理所有网络连接和协议
      - 事件循环：持续处理网络事件
      - 协调多个 NetworkBehaviour 的交互

   5. **Gossipsub (八卦协议)**
      - 构建 mesh 网络拓扑
      - 消息洪泛传播
      - 自动去重和路由优化
      - 适合中等规模的消息广播

   6. **mDNS (多播DNS)**
      - 零配置网络发现
      - 仅在局域网内有效
      - 广播节点存在性
      - 自动发现和连接

   7. **Identify (身份协议)**
      - 交换节点信息
      - 协议协商
      - 地址发现
      - 网络诊断

   ============================================================================
   性能优化提示（基于 CLAUDE.md）
   ============================================================================

   1. **消息批处理**
      - 收集多条消息后一次性发送
      - 减少网络往返次数

   2. **连接池管理**
      - 限制最大连接数
      - 复用现有连接
      - 配置连接超时

   3. **内存优化**
      - 使用 bytes::Bytes 避免拷贝
      - 限制消息缓冲区大小
      - 定期清理过期连接

   4. **并发处理**
      - Swarm 事件在单线程处理
      - CPU密集型任务使用 spawn_blocking
      - 避免阻塞事件循环

   ============================================================================
   练习建议
   ============================================================================

   1. **基础练习**
      - 添加用户昵称功能
      - 实现私聊功能（点对点消息）
      - 添加消息时间戳

   2. **中级练习**
      - 实现多个聊天室（多主题）
      - 添加消息持久化（保存到文件）
      - 实现在线用户列表

   3. **高级练习**
      - 添加消息加密（端到端加密）
      - 实现文件传输功能
      - 添加 NAT 穿透支持
      - 集成 Kademlia DHT 实现全网发现

   ============================================================================
   故障排查提示
   ============================================================================

   1. **节点无法发现**
      - 检查防火墙设置
      - 确认在同一局域网
      - 查看 mDNS 是否启用

   2. **消息收不到**
      - 确认订阅了正确的主题
      - 检查 Gossipsub 配置
      - 查看节点连接状态

   3. **连接频繁断开**
      - 检查网络稳定性
      - 调整心跳间隔
      - 查看错误日志

   ============================================================================
*/
