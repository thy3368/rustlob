#[cfg(target_os = "linux")]
use std::path::PathBuf;
#[cfg(target_os = "linux")]
use anyhow::{Context, Result};
#[cfg(target_os = "linux")]
use clap::Parser;
#[cfg(target_os = "linux")]
use nix::net::if_::if_nametoindex;
#[cfg(target_os = "linux")]
use tokio::sync::broadcast;
#[cfg(target_os = "linux")]
use websocket_axum::{start_server, WebSocketEvent};
#[cfg(target_os = "linux")]
use aya::maps::RingBuf;
#[cfg(target_os = "linux")]
use aya::programs::Xdp;
#[cfg(target_os = "linux")]
use aya::Bpf;
#[cfg(target_os = "linux")]
use aya_log::BpfLogger;

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

#[cfg(target_os = "linux")]
#[derive(Parser, Debug)]
struct Args {
    /// 网络接口名称
    #[arg(short, long)]
    iface: String,

    /// WebSocket 服务器端口
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = run_linux().await {
            eprintln!("Error: {}", e);
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        println!("Warning: XDP program attachment is only supported on Linux");
    }

    Ok(())
}

#[cfg(target_os = "linux")]
async fn run_linux() -> Result<()> {
    let args = Args::parse();

    // 创建事件广播通道
    let (tx, _) = broadcast::channel(1024);

    // 加载编译后的 eBPF 程序
    println!("Loading eBPF program...");
    let mut bpf = Bpf::load_file(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("xdp-aya-ebpf")
            .join("target")
            .join("bpfel-unknown-none")
            .join("release")
            .join("xdp-aya-ebpf.bpf.o")
    ).context("Failed to load eBPF file")?;

    // 初始化日志
    BpfLogger::init(&mut bpf).context("Failed to initialize logger")?;

    println!("eBPF program loaded successfully!");

    let if_index = if_nametoindex(args.iface.as_str())
        .context("Failed to get interface index")?;

    // 获取并附加 XDP 程序
    let program: &mut Xdp = bpf.program_mut("xdp_hello")
        .context("Failed to find xdp_hello program")?
        .try_into()?;
    program.load().context("Failed to load XDP program")?;
    program.attach(if_index as i32).context("Failed to attach XDP program to interface")?;

    println!("XDP program attached to interface: {}", args.iface);

    // 获取环形缓冲区
    let mut ringbuf = RingBuf::try_from(bpf.map_mut("xdp_events")?)?;

    // 启动环形缓冲区监听
    let tx_ebpf = tx.clone();
    tokio::spawn(async move {
        println!("eBPF ring buffer listener started");
        loop {
            match ringbuf.next() {
                Some(Ok(data)) => {
                    let event = unsafe { &*(data.as_ptr() as *const XdpEvent) };
                    let json = event.to_json();

                    // 发送 WebSocket 事件
                    let ws_event = WebSocketEvent {
                        r#type: "network_event".to_string(),
                        data: json,
                    };
                    let _ = tx_ebpf.send(ws_event);
                }
                Some(Err(e)) => {
                    eprintln!("Ring buffer read error: {}", e);
                }
                None => {
                    // 环形缓冲区为空，等待
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
            }
        }
    });

    // 启动 WebSocket 服务器
    let server_tx = tx.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(args.port, server_tx).await {
            eprintln!("WebSocket server error: {}", e);
        }
    });

    println!("WebSocket server starting on http://localhost:{}", args.port);
    println!("Press Ctrl+C to detach and exit");

    // 等待 Ctrl+C 信号
    tokio::signal::ctrl_c().await?;

    println!("\nDetaching XDP program...");
    program.detach(if_index as i32).context("Failed to detach XDP program")?;
    println!("XDP program detached from interface: {}", args.iface);

    Ok(())
}
