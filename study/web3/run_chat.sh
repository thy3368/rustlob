#!/bin/bash

# libp2p Chat 运行脚本
# 用于快速启动多个节点进行测试

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 检查 Rust 是否安装
check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo 未安装，请先安装 Rust"
        echo "访问 https://rustup.rs/"
        exit 1
    fi
    print_success "Rust 已安装: $(rustc --version)"
}

# 编译项目
build_project() {
    print_info "正在编译项目..."
    if cargo build --release --bin chat; then
        print_success "编译完成"
    else
        print_error "编译失败"
        exit 1
    fi
}

# 运行单个节点
run_single() {
    print_info "启动单个节点..."
    RUST_LOG=info cargo run --release --bin chat
}

# 在多个终端窗口中运行节点（macOS）
run_multiple_macos() {
    local count=$1
    print_info "在 macOS 上启动 $count 个节点..."

    # 确保已编译
    if [ ! -f "target/release/chat" ]; then
        build_project
    fi

    for i in $(seq 1 $count); do
        osascript <<EOF
tell application "Terminal"
    do script "cd \"$(pwd)\" && RUST_LOG=info ./target/release/chat"
end tell
EOF
        print_success "节点 $i 已在新终端启动"
        sleep 1
    done

    print_success "所有节点已启动"
    print_info "等待节点相互发现..."
}

# 在多个终端窗口中运行节点（Linux with gnome-terminal）
run_multiple_linux() {
    local count=$1
    print_info "在 Linux 上启动 $count 个节点..."

    # 确保已编译
    if [ ! -f "target/release/chat" ]; then
        build_project
    fi

    for i in $(seq 1 $count); do
        gnome-terminal -- bash -c "cd $(pwd) && RUST_LOG=info ./target/release/chat; exec bash"
        print_success "节点 $i 已在新终端启动"
        sleep 1
    done

    print_success "所有节点已启动"
}

# 显示帮助信息
show_help() {
    cat <<EOF
libp2p P2P Chat 运行脚本

用法:
    $0 [选项]

选项:
    run              运行单个节点（默认）
    build            仅编译项目
    multi <n>        在多个终端窗口中启动 n 个节点
    debug            启用调试日志运行
    clean            清理编译产物
    test             运行测试
    help             显示此帮助信息

示例:
    $0 run           # 运行单个节点
    $0 build         # 编译项目
    $0 multi 3       # 启动3个节点
    $0 debug         # 调试模式运行

环境变量:
    RUST_LOG         设置日志级别（info, debug, trace）

EOF
}

# 清理编译产物
clean_project() {
    print_info "清理编译产物..."
    cargo clean
    print_success "清理完成"
}

# 运行测试
run_tests() {
    print_info "运行测试..."
    cargo test
}

# 调试模式运行
run_debug() {
    print_info "调试模式启动..."
    RUST_LOG=libp2p=debug,libp2p_gossipsub=trace cargo run --bin chat
}

# 主函数
main() {
    # 检查是否在项目目录
    if [ ! -f "Cargo.toml" ]; then
        print_error "请在项目根目录运行此脚本"
        exit 1
    fi

    # 解析命令行参数
    case "${1:-run}" in
        run)
            check_rust
            run_single
            ;;
        build)
            check_rust
            build_project
            ;;
        multi)
            check_rust
            local node_count=${2:-3}

            if [[ "$OSTYPE" == "darwin"* ]]; then
                run_multiple_macos $node_count
            elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
                run_multiple_linux $node_count
            else
                print_error "不支持的操作系统: $OSTYPE"
                print_info "请手动在多个终端运行: cargo run --release --bin chat"
                exit 1
            fi
            ;;
        debug)
            check_rust
            run_debug
            ;;
        clean)
            clean_project
            ;;
        test)
            check_rust
            run_tests
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "未知命令: $1"
            show_help
            exit 1
            ;;
    esac
}

# 运行主函数
main "$@"
