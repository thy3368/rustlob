#!/bin/bash

# Reth Node Docker Runner Script
# This script helps manage Reth Ethereum execution client in Docker

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Configuration
JWT_FILE="jwt.hex"
ENV_FILE=".env"

# Functions
print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Reth Node Docker Manager${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Generate JWT secret if not exists
generate_jwt() {
    if [ ! -f "$JWT_FILE" ]; then
        print_info "Generating JWT secret..."
        openssl rand -hex 32 > "$JWT_FILE"
        chmod 600 "$JWT_FILE"
        print_success "JWT secret generated: $JWT_FILE"
    else
        print_info "JWT secret already exists: $JWT_FILE"
    fi
}

# Create .env file if not exists
create_env() {
    if [ ! -f "$ENV_FILE" ]; then
        print_info "Creating .env file..."
        cat > "$ENV_FILE" << 'EOF'
# Reth Node Configuration

# Network: mainnet, sepolia, holesky, goerli
CHAIN=sepolia

# Logging level: error, warn, info, debug, trace
RUST_LOG=info

# Optional: Enable full node with consensus client
# Uncomment the line below and use: docker-compose --profile full-node up -d
# COMPOSE_PROFILES=full-node
EOF
        print_success ".env file created"
    else
        print_info ".env file already exists"
    fi
}

# Check dependencies
check_dependencies() {
    print_info "Checking dependencies..."

    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        exit 1
    fi

    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi

    if ! command -v openssl &> /dev/null; then
        print_warning "OpenSSL is not installed. Cannot generate JWT secret automatically."
    fi

    print_success "Dependencies check passed"
}

# Initialize setup
init_setup() {
    print_header
    check_dependencies
    generate_jwt
    create_env

    # Create config directory
    mkdir -p config

    print_success "Setup completed!"
    echo ""
    print_info "Next steps:"
    echo "  1. Edit .env file to configure your network (mainnet/sepolia/etc)"
    echo "  2. Run: ./run-reth.sh start"
    echo ""
}

# Start Reth node
start_node() {
    print_header
    print_info "Starting Reth node..."

    if [ ! -f "$JWT_FILE" ]; then
        print_error "JWT file not found. Run: ./run-reth.sh init"
        exit 1
    fi

    docker-compose up -d
    print_success "Reth node started"
    echo ""
    print_info "View logs: ./run-reth.sh logs"
    print_info "Check status: ./run-reth.sh status"
}

# Stop Reth node
stop_node() {
    print_header
    print_info "Stopping Reth node..."
    docker-compose down
    print_success "Reth node stopped"
}

# Restart Reth node
restart_node() {
    print_header
    print_info "Restarting Reth node..."
    docker-compose restart
    print_success "Reth node restarted"
}

# Show logs
show_logs() {
    docker-compose logs -f reth
}

# Show status
show_status() {
    print_header
    docker-compose ps
    echo ""

    # Check if reth is responding
    if curl -s -X POST -H "Content-Type: application/json" \
        --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
        http://localhost:8545 > /dev/null 2>&1; then
        print_success "RPC endpoint is responding on http://localhost:8545"

        # Get current block number
        BLOCK=$(curl -s -X POST -H "Content-Type: application/json" \
            --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
            http://localhost:8545 | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

        if [ -n "$BLOCK" ]; then
            BLOCK_DEC=$((16#${BLOCK#0x}))
            print_info "Current block: $BLOCK_DEC ($BLOCK)"
        fi
    else
        print_warning "RPC endpoint is not responding yet"
    fi
}

# Clean up (remove containers and volumes)
cleanup() {
    print_header
    print_warning "This will remove all containers and data volumes!"
    read -p "Are you sure? (yes/no): " -r
    echo

    if [[ $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        print_info "Cleaning up..."
        docker-compose down -v
        print_success "Cleanup completed"
    else
        print_info "Cleanup cancelled"
    fi
}

# Execute RPC call
rpc_call() {
    local method=$1
    local params=${2:-[]}

    curl -s -X POST -H "Content-Type: application/json" \
        --data "{\"jsonrpc\":\"2.0\",\"method\":\"$method\",\"params\":$params,\"id\":1}" \
        http://localhost:8545 | jq .
}

# Show help
show_help() {
    print_header
    echo ""
    echo "Usage: ./run-reth.sh [command]"
    echo ""
    echo "Commands:"
    echo "  init      - Initialize setup (generate JWT, create .env)"
    echo "  start     - Start Reth node"
    echo "  stop      - Stop Reth node"
    echo "  restart   - Restart Reth node"
    echo "  status    - Show node status"
    echo "  logs      - Show logs (follow mode)"
    echo "  cleanup   - Remove containers and volumes"
    echo "  rpc       - Execute RPC call (requires method name)"
    echo "  help      - Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./run-reth.sh init"
    echo "  ./run-reth.sh start"
    echo "  ./run-reth.sh logs"
    echo "  ./run-reth.sh rpc eth_blockNumber"
    echo "  ./run-reth.sh rpc eth_getBlockByNumber '[\"latest\",false]'"
    echo ""
}

# Main script
case "${1:-}" in
    init)
        init_setup
        ;;
    start)
        start_node
        ;;
    stop)
        stop_node
        ;;
    restart)
        restart_node
        ;;
    logs)
        show_logs
        ;;
    status)
        show_status
        ;;
    cleanup)
        cleanup
        ;;
    rpc)
        if [ -z "${2:-}" ]; then
            print_error "RPC method required"
            echo "Usage: ./run-reth.sh rpc <method> [params]"
            exit 1
        fi
        rpc_call "$2" "${3:-[]}"
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        print_error "Unknown command: ${1:-}"
        echo ""
        show_help
        exit 1
        ;;
esac
