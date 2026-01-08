# XDP Libbpf Hello World

这是一个使用 libbpf-rs 库编写的简单 XDP (eXpress Data Path) 程序，实现了一个"Hello World"示例。

## 项目结构

```
xdp_libbpf/
├── Cargo.toml         # 项目配置文件
├── build.rs           # 编译eBPF程序的构建脚本
├── Dockerfile         # Docker镜像配置
├── docker-compose.yml # Docker Compose配置
├── src/
│   ├── bpf/
│   │   └── xdp_hello.bpf.c  # eBPF程序源代码
│   └── main.rs        # Rust用户态程序
└── README.md          # 说明文档
```

## 功能说明

- `xdp_hello.bpf.c`: 简单的eBPF程序，使用 `bpf_printk()` 打印"Hello from XDP!"
- `main.rs`: Rust用户态程序，负责加载、附加和管理XDP程序的生命周期
- `build.rs`: 自动编译eBPF程序的构建脚本

## 运行方式

### 方法1：使用Docker（推荐）

```bash
# 构建并运行Docker容器
cd /Users/hongyaotang/src/rustlob/app/xdp_libbpf
docker-compose up --build
```

### 方法2：直接在Linux上运行

```bash
# 安装依赖（Ubuntu/Debian）
sudo apt-get install -y clang libclang-dev libelf-dev zlib1g-dev

# 编译程序
cd /Users/hongyaotang/src/rustlob
cargo build --package xdp_libbpf --release

# 运行程序（需要root权限）
sudo ./target/release/xdp_libbpf --iface lo
```

## 查看输出

程序运行后，可以使用以下命令查看eBPF程序的输出：

```bash
sudo cat /sys/kernel/debug/tracing/trace_pipe
```

## 停止程序

按 Ctrl+C 停止程序，它会自动分离XDP程序。

## 注意事项

- XDP程序需要root权限才能加载和附加到网络接口
- eBPF程序只能在Linux内核4.8及以上版本运行
- 使用 `--iface` 参数指定要附加XDP程序的网络接口
- 默认情况下，程序使用 `XDP_PASS` 动作，允许数据包继续传递

## 常用网络接口名称

- `lo`: 回环接口
- `eth0`, `eth1`: 以太网接口
- `wlan0`: 无线接口