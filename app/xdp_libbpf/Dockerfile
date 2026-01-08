# 使用Ubuntu作为基础镜像，因为它有较好的eBPF支持
FROM ubuntu:22.04

# 设置非交互式安装
ARG DEBIAN_FRONTEND=noninteractive

# 安装必要的依赖
RUN apt-get update && apt-get install -y \
    build-essential \
    clang \
    libclang-dev \
    libelf-dev \
    zlib1g-dev \
    pkg-config \
    curl \
    git \
    linux-tools-common \
    linux-tools-generic \
    linux-headers-generic

# 安装Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 创建工作目录
WORKDIR /app

# 复制整个项目文件
COPY .. /app

# 构建项目
RUN cd /app && cargo build --package xdp_libbpf --release

# 运行时命令（需要指定网络接口）
CMD ["/app/target/release/xdp_libbpf", "--iface", "lo"]