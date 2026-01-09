use std::os::fd::AsFd;
use anyhow::{Context, Result};
use clap::Parser;
use nix::libc;
use nix::net::if_::if_nametoindex;
use tokio::sync::broadcast;
use websocket_axum::{start_server, WebSocketEvent};
use std::mem::MaybeUninit;
use libbpf_rs::skel::SkelBuilder;
use libbpf_rs::skel::OpenSkel;

// 包含生成的 eBPF 骨架代码
mod xdp_hello {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bpf/xdp_hello.skel.rs"
    ));
}
use xdp_hello::*;


#[derive(Parser, Debug)]
struct Args {
    /// 网络接口名称
    #[arg(short, long)]
    iface: String,

    /// WebSocket 服务器端口
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

// 网络事件数据结构（与 eBPF 程序中的结构匹配）
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct XdpEvent {
    timestamp: u64,
    ifindex: u32,
    protocol: u32,
    src_ip: u32,
    dst_ip: u32,
    src_port: u16,
    dst_port: u16,
    pkt_len: u32,
    eth_proto: [u8; 2],
}

impl XdpEvent {
    // 转换为可序列化的 JSON 格式
    fn to_json(&self) -> simd_json::owned::Value {
        use simd_json::json;
        json!({
            "timestamp": self.timestamp,
            "ifindex": self.ifindex,
            "protocol": self.protocol,
            "src_ip": format!("{}.{}.{}.{}",
                (self.src_ip >> 24) & 0xFF,
                (self.src_ip >> 16) & 0xFF,
                (self.src_ip >> 8) & 0xFF,
                self.src_ip & 0xFF),
            "dst_ip": format!("{}.{}.{}.{}",
                (self.dst_ip >> 24) & 0xFF,
                (self.dst_ip >> 16) & 0xFF,
                (self.dst_ip >> 8) & 0xFF,
                self.dst_ip & 0xFF),
            "src_port": self.src_port,
            "dst_port": self.dst_port,
            "pkt_len": self.pkt_len,
            "eth_proto": format!("0x{:04x}", u16::from_be_bytes(self.eth_proto))
        })
    }
}

fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        anyhow::bail!("Failed to increase rlimit");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 增加 memlock 限制
    bump_memlock_rlimit()?;

    // 创建事件广播通道
    let (tx, _) = broadcast::channel(1024);

    println!("Loading eBPF skeleton...");

    // 使用骨架代码加载 eBPF 程序
    let mut builder = XdpHelloSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skel = builder.open(&mut open_object)?;
    let mut skel = open_skel.load()?;

    println!("XDP program loaded successfully!");

    // 尝试附加到网络接口（仅在Linux上支持）
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = run_xdp_program(args.iface, args.port, tx, skel).await {
            eprintln!("Error: {}", e);
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        println!("Warning: XDP program attachment is only supported on Linux");
    }

    Ok(())
}

async fn run_xdp_program(
    iface: String,
    port: u16,
    tx: broadcast::Sender<WebSocketEvent>,
    mut skel: XdpHelloSkel<'_>,
) -> Result<()> {
    use libbpf_rs::{Xdp, XdpFlags};

    let if_index = if_nametoindex(iface.as_str())
        .context("Failed to get interface index")?;

    // 尝试使用默认模式附加 XDP 程序，失败时使用通用模式
    let xdp_prog = Xdp::new(skel.progs.xdp_hello.as_fd());
    match xdp_prog.attach(if_index as i32, XdpFlags::empty()) {
        Ok(_) => println!("XDP program attached to interface: {} (driver mode)", iface),
        Err(e) => {
            println!("Failed to attach in driver mode: {}, trying generic mode", e);
            xdp_prog.attach(if_index as i32, XdpFlags::SKB_MODE)
                .context("Failed to attach XDP program to interface (generic mode)")?;
            println!("XDP program attached to interface: {} (generic mode)", iface);
        }
    }

    // 设置环形缓冲区回调
    let tx_ebpf = tx.clone();
    let mut builder = libbpf_rs::RingBufferBuilder::new();
    builder
        .add(&skel.maps.xdp_events as &dyn libbpf_rs::MapCore, move |data| {
            let event = unsafe { &*(data.as_ptr() as *const XdpEvent) };
            let json = event.to_json();

            // 发送 WebSocket 事件
            let ws_event = WebSocketEvent {
                r#type: "network_event".to_string(),
                data: json,
            };
            let _ = tx_ebpf.send(ws_event);

            0
        })
        .context("Failed to add ringbuf callback")?;
    let ringbuf = builder.build().context("Failed to build ring buffer")?;

    // 启动环形缓冲区监听
    tokio::spawn(async move {
        println!("eBPF ring buffer listener started");
        loop {
            match ringbuf.poll(std::time::Duration::from_millis(100)) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Ring buffer poll error: {}", e);
                    break;
                }
            }
        }
        println!("eBPF ring buffer listener stopped");
    });

    // 启动 WebSocket 服务器
    let server_tx = tx.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(port, server_tx).await {
            eprintln!("WebSocket server error: {}", e);
        }
    });

    println!("WebSocket server starting on http://localhost:{}", port);
    println!("Press Ctrl+C to detach and exit");

    // 等待 Ctrl+C 信号
    tokio::signal::ctrl_c().await?;

    println!("\nDetaching XDP program...");
    xdp_prog.detach(if_index as i32, XdpFlags::empty())
        .context("Failed to detach XDP program")?;
    println!("XDP program detached from interface: {}", iface);

    Ok(())
}