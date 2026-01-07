.PHONY: help init start stop restart logs status cleanup rpc-test pull update

# Default target
.DEFAULT_GOAL := help

# Colors
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m

help: ## Show this help message
	@echo "$(BLUE)Reth Node Docker Manager$(NC)"
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""

init: ## Initialize setup (generate JWT, create .env)
	@./run-reth.sh init

start: ## Start Reth node
	@./run-reth.sh start

stop: ## Stop Reth node
	@./run-reth.sh stop

restart: ## Restart Reth node
	@./run-reth.sh restart

logs: ## Show logs (follow mode)
	@./run-reth.sh logs

status: ## Show node status
	@./run-reth.sh status

cleanup: ## Remove containers and volumes
	@./run-reth.sh cleanup

pull: ## Pull latest Reth Docker image
	@echo "$(BLUE)Pulling latest Reth image...$(NC)"
	@docker-compose pull

update: pull restart ## Update Reth to latest version and restart
	@echo "$(GREEN)✓ Reth updated and restarted$(NC)"

rpc-test: ## Test RPC endpoint with sample calls
	@echo "$(BLUE)Testing RPC endpoint...$(NC)"
	@echo ""
	@echo "$(YELLOW)1. Getting block number:$(NC)"
	@./run-reth.sh rpc eth_blockNumber
	@echo ""
	@echo "$(YELLOW)2. Getting client version:$(NC)"
	@./run-reth.sh rpc web3_clientVersion
	@echo ""
	@echo "$(YELLOW)3. Getting network ID:$(NC)"
	@./run-reth.sh rpc net_version
	@echo ""

full-node: ## Start full node (execution + consensus)
	@echo "$(BLUE)Starting full node (Reth + Lighthouse)...$(NC)"
	@docker-compose --profile full-node up -d
	@echo "$(GREEN)✓ Full node started$(NC)"

backup: ## Backup blockchain data
	@echo "$(BLUE)Backing up Reth data...$(NC)"
	@./run-reth.sh stop
	@docker run --rm -v reth-data:/data -v $(PWD):/backup \
		alpine tar czf /backup/reth-backup-$(shell date +%Y%m%d-%H%M%S).tar.gz /data
	@./run-reth.sh start
	@echo "$(GREEN)✓ Backup completed$(NC)"

stats: ## Show resource usage statistics
	@echo "$(BLUE)Resource usage:$(NC)"
	@docker stats --no-stream reth-node

shell: ## Open shell in Reth container
	@docker exec -it reth-node sh

metrics: ## Show Prometheus metrics
	@echo "$(BLUE)Fetching metrics from http://localhost:9001/metrics$(NC)"
	@curl -s http://localhost:9001/metrics | grep -E "^(reth_|process_)" | head -20

clean-logs: ## Clean up old log files
	@echo "$(YELLOW)Cleaning up old logs...$(NC)"
	@docker exec reth-node sh -c "find /root/.local/share/reth/logs -name '*.log' -mtime +7 -delete" || true
	@echo "$(GREEN)✓ Old logs cleaned$(NC)"

inspect: ## Inspect Reth container
	@docker inspect reth-node | jq .

volumes: ## Show volume information
	@echo "$(BLUE)Reth volumes:$(NC)"
	@docker volume ls | grep reth
	@echo ""
	@docker volume inspect reth-data | jq .

network-info: ## Show network and peer information
	@echo "$(BLUE)Network Information:$(NC)"
	@./run-reth.sh rpc net_peerCount
	@echo ""
	@./run-reth.sh rpc net_listening
	@echo ""
	@./run-reth.sh rpc net_version
