#!/bin/bash

# Reth RPC Test Script
# Tests various RPC endpoints to verify node functionality

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

RPC_URL="${RPC_URL:-http://localhost:8545}"
VERBOSE="${VERBOSE:-false}"

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Reth RPC Test Suite${NC}"
    echo -e "${BLUE}================================${NC}"
    echo ""
}

print_test() {
    echo -e "${YELLOW}Testing:${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓ PASS:${NC} $1"
}

print_fail() {
    echo -e "${RED}✗ FAIL:${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# RPC call helper
rpc_call() {
    local method=$1
    local params=${2:-[]}

    local response=$(curl -s -X POST -H "Content-Type: application/json" \
        --data "{\"jsonrpc\":\"2.0\",\"method\":\"$method\",\"params\":$params,\"id\":1}" \
        "$RPC_URL")

    if [ "$VERBOSE" = "true" ]; then
        echo "$response" | jq .
    fi

    echo "$response"
}

# Check if endpoint is reachable
test_connectivity() {
    print_test "Endpoint connectivity"

    if curl -s -f "$RPC_URL" > /dev/null 2>&1; then
        print_success "Endpoint is reachable at $RPC_URL"
        return 0
    else
        print_fail "Cannot reach endpoint at $RPC_URL"
        return 1
    fi
}

# Test web3_clientVersion
test_client_version() {
    print_test "web3_clientVersion"

    local response=$(rpc_call "web3_clientVersion")
    local result=$(echo "$response" | jq -r '.result')

    if [[ "$result" == *"reth"* ]]; then
        print_success "Client: $result"
        return 0
    else
        print_fail "Expected Reth client, got: $result"
        return 1
    fi
}

# Test net_version
test_net_version() {
    print_test "net_version"

    local response=$(rpc_call "net_version")
    local result=$(echo "$response" | jq -r '.result')

    if [ -n "$result" ] && [ "$result" != "null" ]; then
        case "$result" in
            "1") print_success "Network: Mainnet (ID: $result)" ;;
            "11155111") print_success "Network: Sepolia (ID: $result)" ;;
            "17000") print_success "Network: Holesky (ID: $result)" ;;
            "5") print_success "Network: Goerli (ID: $result)" ;;
            *) print_success "Network ID: $result" ;;
        esac
        return 0
    else
        print_fail "Invalid network version"
        return 1
    fi
}

# Test net_listening
test_net_listening() {
    print_test "net_listening"

    local response=$(rpc_call "net_listening")
    local result=$(echo "$response" | jq -r '.result')

    if [ "$result" = "true" ]; then
        print_success "Node is listening for connections"
        return 0
    else
        print_fail "Node is not listening"
        return 1
    fi
}

# Test net_peerCount
test_peer_count() {
    print_test "net_peerCount"

    local response=$(rpc_call "net_peerCount")
    local result=$(echo "$response" | jq -r '.result')

    if [ -n "$result" ] && [ "$result" != "null" ]; then
        local peer_count=$((16#${result#0x}))
        print_success "Peer count: $peer_count"

        if [ "$peer_count" -eq 0 ]; then
            print_info "Warning: No peers connected. Node may still be starting."
        fi
        return 0
    else
        print_fail "Cannot get peer count"
        return 1
    fi
}

# Test eth_blockNumber
test_block_number() {
    print_test "eth_blockNumber"

    local response=$(rpc_call "eth_blockNumber")
    local result=$(echo "$response" | jq -r '.result')

    if [ -n "$result" ] && [ "$result" != "null" ]; then
        local block_number=$((16#${result#0x}))
        print_success "Current block: $block_number ($result)"
        return 0
    else
        print_fail "Cannot get block number"
        return 1
    fi
}

# Test eth_syncing
test_syncing() {
    print_test "eth_syncing"

    local response=$(rpc_call "eth_syncing")
    local result=$(echo "$response" | jq -r '.result')

    if [ "$result" = "false" ]; then
        print_success "Node is fully synced"
        return 0
    elif [ "$result" != "null" ]; then
        local current=$(echo "$result" | jq -r '.currentBlock')
        local highest=$(echo "$result" | jq -r '.highestBlock')

        if [ -n "$current" ] && [ -n "$highest" ]; then
            local current_dec=$((16#${current#0x}))
            local highest_dec=$((16#${highest#0x}))
            local progress=$(awk "BEGIN {printf \"%.2f\", ($current_dec/$highest_dec)*100}")

            print_info "Syncing: $current_dec / $highest_dec ($progress%)"
        else
            print_info "Node is syncing..."
        fi
        return 0
    else
        print_fail "Cannot get sync status"
        return 1
    fi
}

# Test eth_gasPrice
test_gas_price() {
    print_test "eth_gasPrice"

    local response=$(rpc_call "eth_gasPrice")
    local result=$(echo "$response" | jq -r '.result')

    if [ -n "$result" ] && [ "$result" != "null" ]; then
        local gas_price=$((16#${result#0x}))
        local gas_gwei=$(awk "BEGIN {printf \"%.2f\", $gas_price/1000000000}")
        print_success "Gas price: $gas_gwei Gwei"
        return 0
    else
        print_fail "Cannot get gas price"
        return 1
    fi
}

# Test eth_getBlockByNumber
test_get_block() {
    print_test "eth_getBlockByNumber (latest)"

    local response=$(rpc_call "eth_getBlockByNumber" '["latest",false]')
    local result=$(echo "$response" | jq -r '.result')

    if [ "$result" != "null" ]; then
        local block_number=$(echo "$result" | jq -r '.number')
        local timestamp=$(echo "$result" | jq -r '.timestamp')
        local tx_count=$(echo "$result" | jq -r '.transactions | length')

        if [ -n "$block_number" ]; then
            local block_num_dec=$((16#${block_number#0x}))
            local timestamp_dec=$((16#${timestamp#0x}))
            local block_date=$(date -r $timestamp_dec "+%Y-%m-%d %H:%M:%S" 2>/dev/null || echo "N/A")

            print_success "Latest block: #$block_num_dec with $tx_count transactions"
            print_info "Block timestamp: $block_date"
            return 0
        fi
    fi

    print_fail "Cannot get latest block"
    return 1
}

# Test eth_getBalance
test_get_balance() {
    print_test "eth_getBalance (sample address)"

    # Vitalik's address
    local address="0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
    local response=$(rpc_call "eth_getBalance" "[\"$address\",\"latest\"]")
    local result=$(echo "$response" | jq -r '.result')

    if [ -n "$result" ] && [ "$result" != "null" ]; then
        local balance=$((16#${result#0x}))
        local balance_eth=$(awk "BEGIN {printf \"%.4f\", $balance/1000000000000000000}")
        print_success "Balance of $address: $balance_eth ETH"
        return 0
    else
        print_fail "Cannot get balance"
        return 1
    fi
}

# Test eth_chainId
test_chain_id() {
    print_test "eth_chainId"

    local response=$(rpc_call "eth_chainId")
    local result=$(echo "$response" | jq -r '.result')

    if [ -n "$result" ] && [ "$result" != "null" ]; then
        local chain_id=$((16#${result#0x}))
        case "$chain_id" in
            1) print_success "Chain ID: $chain_id (Mainnet)" ;;
            11155111) print_success "Chain ID: $chain_id (Sepolia)" ;;
            17000) print_success "Chain ID: $chain_id (Holesky)" ;;
            5) print_success "Chain ID: $chain_id (Goerli)" ;;
            *) print_success "Chain ID: $chain_id" ;;
        esac
        return 0
    else
        print_fail "Cannot get chain ID"
        return 1
    fi
}

# Run all tests
run_all_tests() {
    print_header

    local total=0
    local passed=0
    local failed=0

    tests=(
        "test_connectivity"
        "test_client_version"
        "test_chain_id"
        "test_net_version"
        "test_net_listening"
        "test_peer_count"
        "test_syncing"
        "test_block_number"
        "test_gas_price"
        "test_get_block"
        "test_get_balance"
    )

    for test in "${tests[@]}"; do
        total=$((total + 1))
        if $test; then
            passed=$((passed + 1))
        else
            failed=$((failed + 1))
        fi
        echo ""
    done

    echo -e "${BLUE}================================${NC}"
    echo -e "Total tests: $total"
    echo -e "${GREEN}Passed: $passed${NC}"
    if [ $failed -gt 0 ]; then
        echo -e "${RED}Failed: $failed${NC}"
    else
        echo -e "Failed: 0"
    fi
    echo -e "${BLUE}================================${NC}"

    if [ $failed -eq 0 ]; then
        echo -e "${GREEN}All tests passed!${NC}"
        return 0
    else
        echo -e "${RED}Some tests failed.${NC}"
        return 1
    fi
}

# Show usage
show_usage() {
    echo "Usage: $0 [options] [test_name]"
    echo ""
    echo "Options:"
    echo "  -v, --verbose    Show detailed output"
    echo "  -u, --url URL    Set RPC URL (default: http://localhost:8545)"
    echo "  -h, --help       Show this help"
    echo ""
    echo "Available tests:"
    echo "  connectivity     Test endpoint connectivity"
    echo "  version          Test client version"
    echo "  network          Test network info"
    echo "  peers            Test peer count"
    echo "  sync             Test sync status"
    echo "  block            Test block info"
    echo "  balance          Test balance query"
    echo "  all              Run all tests (default)"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run all tests"
    echo "  $0 -v all             # Run all tests with verbose output"
    echo "  $0 block              # Run only block test"
    echo "  $0 -u http://example.com:8545 all"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -u|--url)
            RPC_URL="$2"
            shift 2
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        connectivity)
            print_header
            test_connectivity
            exit $?
            ;;
        version)
            print_header
            test_client_version
            exit $?
            ;;
        network)
            print_header
            test_net_version
            test_chain_id
            exit $?
            ;;
        peers)
            print_header
            test_peer_count
            exit $?
            ;;
        sync)
            print_header
            test_syncing
            exit $?
            ;;
        block)
            print_header
            test_block_number
            test_get_block
            exit $?
            ;;
        balance)
            print_header
            test_get_balance
            exit $?
            ;;
        all|"")
            run_all_tests
            exit $?
            ;;
        *)
            echo "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

run_all_tests
