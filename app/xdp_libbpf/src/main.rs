use std::os::fd::AsFd;
use anyhow::{Context, Result};
use clap::Parser;
use libbpf_rs::{MapCore, ObjectBuilder};
use nix::net::if_::if_nametoindex;
use std::path::PathBuf;
use tokio::sync::broadcast;
use websocket_axum::{start_server, WebSocketEvent};
use glob;

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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 创建事件广播通道
    let (tx, _) = broadcast::channel(1024);

    // 获取编译后的eBPF目标文件路径
    let obj_path = match std::env::var("OUT_DIR") {
        Ok(out_dir) => PathBuf::from(out_dir).join("xdp_hello.bpf.o"),
        Err(_) => {
            let possible_paths = [
                "target/debug/build/xdp_libbpf-*/out/xdp_hello.bpf.o",
                "target/release/build/xdp_libbpf-*/out/xdp_hello.bpf.o",
                "target/debug/out/xdp_hello.bpf.o",
                "target/release/out/xdp_hello.bpf.o",
            ];

            println!("Looking for eBPF object file in possible locations...");
            for pattern in possible_paths.iter() {
                println!("Checking: {}", pattern);
                if let Ok(entries) = glob::glob(pattern) {
                    if entries.filter_map(Result::ok).next().is_some() {
                        return Err(anyhow::anyhow!(
                            "eBPF object file not found. Please rebuild the project."
                        ));
                    }
                }
            }

            return Err(anyhow::anyhow!(
                "eBPF object file not found. Please run 'cargo build' first."
            ));
        }
    };

    println!("Loading eBPF object from: {:?}", obj_path);

    // 使用 ObjectBuilder 打开 eBPF 对象
    let mut builder = ObjectBuilder::default();
    let obj = builder
        .open_file(obj_path)
        .context("Failed to open eBPF object file")?;

    // 加载 eBPF 程序
    let mut loaded_obj = obj.load().context("Failed to load eBPF object")?;

    println!("XDP program loaded successfully!");

    // 尝试附加到网络接口（仅在Linux上支持）
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = run_xdp_program(args.iface, args.port, tx, loaded_obj).await {
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
    mut loaded_obj: libbpf_rs::Object,
) -> Result<()> {
    use libbpf_rs::{Xdp, XdpFlags};

    let if_index = if_nametoindex(iface.as_str())
        .context("Failed to get interface index")?;

    // 在一个单独的作用域中完成所有对 loaded_obj 的操作，以避免借用检查冲突
    let (xdp_prog, ringbuf) = {
        // 查找并获取 XDP 程序
        let prog = loaded_obj
            .progs_mut()
            .find(|p| p.name() == "xdp_hello")
            .context("Failed to find xdp_hello program")?;

        let xdp_prog = Xdp::new(prog.as_fd());
        xdp_prog.attach(if_index as i32, XdpFlags::empty())
            .context("Failed to attach XDP program to interface")?;

        println!("XDP program attached to interface: {}", iface);

        // 获取环形缓冲区
        let mut builder = libbpf_rs::RingBufferBuilder::new();
        let xdp_events_map = loaded_obj.maps_mut()
            .find(|m| m.name() == "xdp_events")
            .context("Failed to find xdp_events map")?;
        let tx_ebpf = tx.clone();
        builder
            .add(&xdp_events_map as &dyn MapCore, move |data| {
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

        (xdp_prog, ringbuf)
    };

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