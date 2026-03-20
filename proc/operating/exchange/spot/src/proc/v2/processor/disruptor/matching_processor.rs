//! LMAX Disruptor 撮合处理器
//!
//! 基于 LMAX Disruptor 模式实现的极致性能撮合处理器
//!
//! # 设计原则
//! - 无锁环形缓冲区：使用 Disruptor 的 RingBuffer
//! - 单线程消费：每个 symbol 一个独立的 EventHandler
//! - 批量处理：利用 end_of_batch 标志批量提交
//! - 缓存行对齐：避免 false sharing
//!
//! # 性能优化
//! - CPU 亲和性绑定
//! - 预分配内存
//! - 零拷贝设计
//! - 批量发布事件

use std::sync::Arc;
use std::thread;

use base_types::exchange::spot::spot_types::SpotOrder;
use base_types::TradingPair;
use diff::ChangeLogEntry;
use disruptor::*;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;
use crate::proc::v2::trade_handlers::matching_engine::{MatchResult, MatchingEngine};

/// Disruptor 事件：订单变更日志
///
/// # 缓存行对齐
/// 使用 64 字节对齐避免 false sharing
#[repr(align(64))]
#[derive(Clone)]
pub struct OrderEvent {
    /// 订单变更日志
    pub order_log: Option<ChangeLogEntry>,
    /// 交易对（用于路由）
    pub symbol: TradingPair,
    /// 序列号（用于调试）
    pub sequence: i64,
}

impl OrderEvent {
    /// 创建空事件（用于初始化 RingBuffer）
    pub fn empty() -> Self {
        Self {
            order_log: None,
            symbol: TradingPair::default(),
            sequence: 0,
        }
    }

    /// 检查事件是否有效
    pub fn is_valid(&self) -> bool {
        self.order_log.is_some()
    }

    /// 重置事件（用于复用）
    pub fn reset(&mut self) {
        self.order_log = None;
        self.sequence = 0;
    }
}

/// Disruptor 事件处理器：撮合引擎
///
/// # 单线程设计
/// 每个 EventHandler 在独立线程中运行，处理特定 symbol 的订单
pub struct MatchingEventHandler {
    /// 撮合引擎
    matching_engine: Arc<MatchingEngine>,
    /// 事件发布器
    event_publisher: Arc<dyn EventPublisher>,
    /// 交易对（该 handler 只处理此 symbol）
    symbol: TradingPair,
    /// 批量缓冲区（用于批量发布）
    batch_buffer: Vec<MatchResult>,
    /// 统计信息
    processed_count: u64,
    error_count: u64,
}

impl MatchingEventHandler {
    /// 创建新的事件处理器
    pub fn new(
        matching_engine: Arc<MatchingEngine>,
        event_publisher: Arc<dyn EventPublisher>,
        symbol: TradingPair,
    ) -> Self {
        Self {
            matching_engine,
            event_publisher,
            symbol,
            batch_buffer: Vec::with_capacity(128), // 预分配批量缓冲区
            processed_count: 0,
            error_count: 0,
        }
    }

    /// 重构订单（从变更日志）
    fn reconstruct_order(&self, order_log: &ChangeLogEntry) -> Result<SpotOrder, SpotCmdErrorAny> {
        // TODO: 实现完整的订单重构逻辑
        // 当前占位实现
        tracing::warn!(
            entity_id = %order_log.entity_id(),
            "Order reconstruction not implemented yet"
        );

        Err(SpotCmdErrorAny::Common(
            crate::proc::behavior::spot_trade_behavior::CommonError::Internal {
                message: "Order reconstruction not implemented".to_string(),
            },
        ))
    }

    /// 批量发布撮合结果
    fn flush_batch(&mut self) {
        if self.batch_buffer.is_empty() {
            return;
        }

        // 收集所有订单日志
        let mut all_order_logs = Vec::new();
        let mut all_trade_logs = Vec::new();

        for result in self.batch_buffer.drain(..) {
            all_order_logs.extend(result.order_logs);
            all_trade_logs.extend(result.trade_logs);
        }

        // 批量发布订单日志
        if !all_order_logs.is_empty() {
            if let Err(e) = self.event_publisher.publish_order_logs(&all_order_logs) {
                tracing::error!(
                    symbol = ?self.symbol,
                    count = all_order_logs.len(),
                    error = ?e,
                    "Failed to publish order logs batch"
                );
            } else {
                tracing::debug!(
                    symbol = ?self.symbol,
                    count = all_order_logs.len(),
                    "Published order logs batch"
                );
            }
        }

        // 批量发布成交日志
        if !all_trade_logs.is_empty() {
            if let Err(e) = self.event_publisher.publish_trade_logs(&all_trade_logs) {
                tracing::error!(
                    symbol = ?self.symbol,
                    count = all_trade_logs.len(),
                    error = ?e,
                    "Failed to publish trade logs batch"
                );
            } else {
                tracing::debug!(
                    symbol = ?self.symbol,
                    count = all_trade_logs.len(),
                    "Published trade logs batch"
                );
            }
        }
    }
}

impl EventHandler<OrderEvent> for MatchingEventHandler {
    fn on_event(&mut self, event: &OrderEvent, sequence: i64, end_of_batch: bool) {
        // 跳过无效事件
        if !event.is_valid() {
            return;
        }

        // 只处理本 symbol 的订单
        if event.symbol != self.symbol {
            return;
        }

        let order_log = match &event.order_log {
            Some(log) => log,
            None => return,
        };

        tracing::trace!(
            symbol = ?self.symbol,
            sequence = sequence,
            entity_id = %order_log.entity_id(),
            end_of_batch = end_of_batch,
            "Processing order event"
        );

        // 重构订单
        let order = match self.reconstruct_order(order_log) {
            Ok(order) => order,
            Err(e) => {
                tracing::error!(
                    symbol = ?self.symbol,
                    entity_id = %order_log.entity_id(),
                    error = ?e,
                    "Failed to reconstruct order"
                );
                self.error_count += 1;
                return;
            }
        };

        // 执行撮合
        match self.matching_engine.match_order(order) {
            Ok(match_result) => {
                self.processed_count += 1;

                // 添加到批量缓冲区
                if match_result.has_trades() || !match_result.order_logs.is_empty() {
                    self.batch_buffer.push(match_result);
                }

                // 批次结束时批量发布
                if end_of_batch {
                    self.flush_batch();

                    // 定期输出统计信息
                    if self.processed_count % 10000 == 0 {
                        tracing::info!(
                            symbol = ?self.symbol,
                            processed = self.processed_count,
                            errors = self.error_count,
                            "Matching statistics"
                        );
                    }
                }
            }
            Err(e) => {
                tracing::error!(
                    symbol = ?self.symbol,
                    order_id = %order_log.entity_id(),
                    error = ?e,
                    "Failed to match order"
                );
                self.error_count += 1;
            }
        }
    }
}

/// Disruptor 撮合处理器配置
#[derive(Debug, Clone)]
pub struct DisruptorMatchingConfig {
    /// RingBuffer 大小（必须是 2 的幂）
    pub buffer_size: usize,
    /// 等待策略
    pub wait_strategy: WaitStrategyType,
    /// CPU 亲和性（可选）
    pub cpu_affinity: Option<usize>,
    /// 交易对
    pub symbol: TradingPair,
}

impl Default for DisruptorMatchingConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1024 * 64, // 64K 事件
            wait_strategy: WaitStrategyType::BusySpin,
            cpu_affinity: None,
            symbol: TradingPair::default(),
        }
    }
}

/// 等待策略类型
#[derive(Debug, Clone, Copy)]
pub enum WaitStrategyType {
    /// 忙等待（最低延迟，高 CPU 占用）
    BusySpin,
    /// 让出 CPU（平衡延迟和 CPU）
    Yielding,
    /// 阻塞等待（最低 CPU，高延迟）
    Blocking,
}

/// Disruptor 撮合处理器
///
/// # 架构
/// ```text
/// Producer(s) → RingBuffer → EventHandler → MatchingEngine
///                              ↓
///                         EventPublisher
/// ```
pub struct DisruptorMatchingProcessor {
    /// 生产者（用于发布订单事件）
    producer: Producer<OrderEvent>,
    /// 配置
    config: DisruptorMatchingConfig,
    /// 线程句柄
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl DisruptorMatchingProcessor {
    /// 创建新的 Disruptor 撮合处理器
    ///
    /// # 参数
    /// - `matching_engine`: 撮合引擎
    /// - `event_publisher`: 事件发布器
    /// - `config`: 配置
    ///
    /// # 返回
    /// 处理器实例
    pub fn new(
        matching_engine: Arc<MatchingEngine>,
        event_publisher: Arc<dyn EventPublisher>,
        config: DisruptorMatchingConfig,
    ) -> Result<Self, String> {
        // 验证 buffer_size 是 2 的幂
        if !config.buffer_size.is_power_of_two() {
            return Err(format!(
                "buffer_size must be power of 2, got {}",
                config.buffer_size
            ));
        }

        tracing::info!(
            symbol = ?config.symbol,
            buffer_size = config.buffer_size,
            wait_strategy = ?config.wait_strategy,
            cpu_affinity = ?config.cpu_affinity,
            "Creating Disruptor matching processor"
        );

        // 创建 RingBuffer
        let factory = || OrderEvent::empty();
        let mut builder = disruptor::build_single_producer(config.buffer_size, factory);

        // 设置等待策略
        builder = match config.wait_strategy {
            WaitStrategyType::BusySpin => builder.with_busy_spin_wait_strategy(),
            WaitStrategyType::Yielding => builder.with_yielding_wait_strategy(),
            WaitStrategyType::Blocking => builder.with_blocking_wait_strategy(),
        };

        // 创建事件处理器
        let handler = MatchingEventHandler::new(
            matching_engine.clone(),
            event_publisher.clone(),
            config.symbol,
        );

        // 构建 Disruptor
        let mut disruptor = builder.build();
        let producer = disruptor.get_producer();

        // 启动消费者线程
        let cpu_affinity = config.cpu_affinity;
        let symbol = config.symbol;
        let thread_handle = thread::Builder::new()
            .name(format!("disruptor-matching-{:?}", symbol))
            .spawn(move || {
                // 设置 CPU 亲和性
                if let Some(cpu_id) = cpu_affinity {
                    if let Err(e) = core_affinity::set_for_current(core_affinity::CoreId { id: cpu_id }) {
                        tracing::warn!(
                            symbol = ?symbol,
                            cpu_id = cpu_id,
                            error = ?e,
                            "Failed to set CPU affinity"
                        );
                    } else {
                        tracing::info!(
                            symbol = ?symbol,
                            cpu_id = cpu_id,
                            "Set CPU affinity"
                        );
                    }
                }

                // 启动事件处理
                tracing::info!(symbol = ?symbol, "Starting Disruptor event handler");
                disruptor.handle_events_with(handler);
                tracing::info!(symbol = ?symbol, "Disruptor event handler stopped");
            })
            .map_err(|e| format!("Failed to spawn thread: {}", e))?;

        Ok(Self {
            producer,
            config,
            thread_handle: Some(thread_handle),
        })
    }

    /// 发布订单事件
    ///
    /// # 参数
    /// - `order_log`: 订单变更日志
    ///
    /// # 返回
    /// - `Ok(())`: 发布成功
    /// - `Err(String)`: 发布失败
    ///
    /// # 性能
    /// 此方法是无锁的，使用 CAS 操作获取序列号
    pub fn publish_order(&mut self, order_log: ChangeLogEntry) -> Result<(), String> {
        let symbol = self.config.symbol;

        self.producer.publish(|event, sequence| {
            event.order_log = Some(order_log);
            event.symbol = symbol;
            event.sequence = sequence;
        });

        Ok(())
    }

    /// 批量发布订单事件
    ///
    /// # 参数
    /// - `order_logs`: 订单变更日志列表
    ///
    /// # 返回
    /// - `Ok(())`: 发布成功
    /// - `Err(String)`: 发布失败
    ///
    /// # 性能
    /// 批量发布比单个发布更高效，减少 CAS 操作次数
    pub fn publish_batch(&mut self, order_logs: Vec<ChangeLogEntry>) -> Result<(), String> {
        if order_logs.is_empty() {
            return Ok(());
        }

        let symbol = self.config.symbol;
        let count = order_logs.len();

        self.producer.batch_publish(count, |iter| {
            for (event, sequence, log) in iter.zip(order_logs.into_iter()) {
                event.order_log = Some(log);
                event.symbol = symbol;
                event.sequence = sequence;
            }
        });

        tracing::trace!(
            symbol = ?symbol,
            count = count,
            "Published order batch"
        );

        Ok(())
    }

    /// 获取配置
    pub fn config(&self) -> &DisruptorMatchingConfig {
        &self.config
    }

    /// 停止处理器
    ///
    /// # 说明
    /// 等待所有事件处理完成后停止
    pub fn shutdown(mut self) -> Result<(), String> {
        tracing::info!(symbol = ?self.config.symbol, "Shutting down Disruptor processor");

        // 等待线程结束
        if let Some(handle) = self.thread_handle.take() {
            handle
                .join()
                .map_err(|e| format!("Failed to join thread: {:?}", e))?;
        }

        tracing::info!(symbol = ?self.config.symbol, "Disruptor processor stopped");
        Ok(())
    }
}

/// Disruptor 撮合处理器工厂
pub struct DisruptorMatchingProcessorFactory;

impl DisruptorMatchingProcessorFactory {
    /// 创建并启动处理器
    ///
    /// # 参数
    /// - `matching_engine`: 撮合引擎
    /// - `event_publisher`: 事件发布器
    /// - `config`: 配置
    ///
    /// # 返回
    /// 处理器实例
    pub fn create(
        matching_engine: Arc<MatchingEngine>,
        event_publisher: Arc<dyn EventPublisher>,
        config: DisruptorMatchingConfig,
    ) -> Result<DisruptorMatchingProcessor, String> {
        DisruptorMatchingProcessor::new(matching_engine, event_publisher, config)
    }

    /// 创建多个处理器（每个 symbol 一个）
    ///
    /// # 参数
    /// - `matching_engine`: 撮合引擎
    /// - `event_publisher`: 事件发布器
    /// - `symbols`: 交易对列表
    /// - `base_config`: 基础配置
    ///
    /// # 返回
    /// 处理器列表
    pub fn create_multi(
        matching_engine: Arc<MatchingEngine>,
        event_publisher: Arc<dyn EventPublisher>,
        symbols: Vec<TradingPair>,
        base_config: DisruptorMatchingConfig,
    ) -> Result<Vec<DisruptorMatchingProcessor>, String> {
        let mut processors = Vec::with_capacity(symbols.len());

        for (idx, symbol) in symbols.into_iter().enumerate() {
            let mut config = base_config.clone();
            config.symbol = symbol;

            // 可选：为每个处理器分配不同的 CPU 核心
            if base_config.cpu_affinity.is_some() {
                config.cpu_affinity = Some(idx % num_cpus::get());
            }

            let processor = Self::create(
                matching_engine.clone(),
                event_publisher.clone(),
                config,
            )?;

            processors.push(processor);
        }

        tracing::info!(
            count = processors.len(),
            "Created multiple Disruptor processors"
        );

        Ok(processors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_event_empty() {
        let event = OrderEvent::empty();
        assert!(!event.is_valid());
        assert_eq!(event.sequence, 0);
    }

    #[test]
    fn test_config_default() {
        let config = DisruptorMatchingConfig::default();
        assert!(config.buffer_size.is_power_of_two());
        assert_eq!(config.buffer_size, 1024 * 64);
    }

    #[test]
    fn test_config_validation() {
        let mut config = DisruptorMatchingConfig::default();
        config.buffer_size = 1000; // 不是 2 的幂

        // 创建处理器应该失败
        // 注意：需要 mock matching_engine 和 event_publisher
    }
}
