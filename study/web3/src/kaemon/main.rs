use futures::StreamExt;
use libp2p::{
    kad::{self, Mode, Record, RecordKey},
    noise, tcp, yamux, Multiaddr, SwarmBuilder,
};
use std::error::Error;
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("kaemon=info".parse()?))
        .init();

    // 创建 libp2p Swarm
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            // 创建 Kademlia 行为
            let peer_id = key.public().to_peer_id();
            let store = kad::store::MemoryStore::new(peer_id);
            kad::Behaviour::new(peer_id, store)
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // 设置为服务器模式
    swarm.behaviour_mut().set_mode(Some(Mode::Server));

    // 监听地址
    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(listen_addr)?;

    println!("本地节点 PeerId: {}", swarm.local_peer_id());
    println!("\n命令:");
    println!("  PUT <key> <value> - 存储键值对");
    println!("  GET <key>         - 查询键值");
    println!("  CONNECT <addr>    - 连接到其他节点");
    println!("  QUIT              - 退出\n");

    // 处理用户输入
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                if let Some(line) = line? {
                    handle_command(&mut swarm, &line);
                }
            }
            event = swarm.select_next_some() => {
                handle_event(event);
            }
        }
    }
}

fn handle_command(swarm: &mut libp2p::Swarm<kad::Behaviour<kad::store::MemoryStore>>, line: &str) {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();

    match parts.first().map(|s| s.to_uppercase()).as_deref() {
        Some("PUT") if parts.len() == 3 => {
            let key = parts[1].as_bytes().to_vec();
            let value = parts[2].as_bytes().to_vec();

            let record = Record {
                key: RecordKey::new(&key),
                value,
                publisher: None,
                expires: None,
            };

            match swarm.behaviour_mut().put_record(record, kad::Quorum::One) {
                Ok(_) => println!("✓ 存储记录: {}", parts[1]),
                Err(e) => eprintln!("✗ 存储失败: {:?}", e),
            }
        }
        Some("GET") if parts.len() == 2 => {
            let key = RecordKey::new(&parts[1].as_bytes());
            swarm.behaviour_mut().get_record(key);
            println!("🔍 查询中: {}", parts[1]);
        }
        Some("CONNECT") if parts.len() == 2 => {
            match parts[1].parse::<Multiaddr>() {
                Ok(addr) => {
                    if let Err(e) = swarm.dial(addr.clone()) {
                        eprintln!("✗ 连接失败: {:?}", e);
                    } else {
                        println!("📡 正在连接: {}", addr);
                    }
                }
                Err(e) => eprintln!("✗ 地址格式错误: {:?}", e),
            }
        }
        Some("QUIT") => {
            println!("👋 退出...");
            std::process::exit(0);
        }
        _ => {
            println!("❌ 未知命令。使用: PUT <key> <value> | GET <key> | CONNECT <addr> | QUIT");
        }
    }
}

fn handle_event(event: libp2p::swarm::SwarmEvent<kad::Event>) {
    match event {
        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
            println!("🎧 监听地址: {}", address);
        }
        libp2p::swarm::SwarmEvent::Behaviour(kad::Event::OutboundQueryProgressed {
            result,
            ..
        }) => {
            match result {
                kad::QueryResult::GetRecord(Ok(kad::GetRecordOk::FoundRecord(record))) => {
                    let key = String::from_utf8_lossy(record.record.key.as_ref());
                    let value = String::from_utf8_lossy(&record.record.value);
                    println!("✓ 查询到记录: {} = {}", key, value);
                }
                kad::QueryResult::GetRecord(Err(e)) => {
                    eprintln!("✗ 查询失败: {:?}", e);
                }
                kad::QueryResult::PutRecord(Ok(kad::PutRecordOk { key })) => {
                    println!("✓ 记录已发布到网络: {}", String::from_utf8_lossy(key.as_ref()));
                }
                kad::QueryResult::PutRecord(Err(e)) => {
                    eprintln!("✗ 发布失败: {:?}", e);
                }
                _ => {}
            }
        }
        libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
            println!("🤝 已连接节点: {} ({})", peer_id, endpoint.get_remote_address());
            // 自动添加到路由表
        }
        libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
            println!("👋 断开连接: {} ({:?})", peer_id, cause);
        }
        _ => {}
    }
}
