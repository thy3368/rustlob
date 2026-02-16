//! 多 Disruptor 交易流水线（Consumer-as-Producer 模式）
//!
//! # 架构设计
//!
//! ```text
//!  External Producer          Stage 1              Stage 2              Stage 3              Stage 4
//!       │                       │                     │                     │                     │
//!       │  publish()            │                     │                     │                     │
//!       ▼                       ▼                     ▼                     ▼                     ▼
//! ┌─────────────┐        ┌─────────────┐      ┌─────────────┐      ┌─────────────┐      ┌─────────────┐
//! │   Input     │        │  Disruptor  │      │  Disruptor  │      │  Disruptor  │      │  Disruptor  │
//! │   Event     │───────▶│    A        │─────▶│    B        │─────▶│    C        │─────▶│    D        │
//! │             │        │  (收单)      │      │  (撮合)      │      │  (结算)      │      │  (推送)      │
//! └─────────────┘        └─────────────┘      └─────────────┘      └─────────────┘      └─────────────┘
//!                               │                     │                     │                     │
//!                               │ Consumer A          │ Consumer B          │ Consumer C          │ Consumer D
//!                               │                     │                     │                     │
//!                               ▼                     ▼                     ▼                     ▼
//!                        ┌─────────────┐      ┌─────────────┐      ┌─────────────┐      ┌─────────────┐
//!                        │  handle_    │      │  handle_    │      │  handle_    │      │  handle_    │
//!                        │  acquiring  │      │  matching   │      │ settlement  │      │    push     │
//!                        │             │      │             │      │             │      │             │
//!                        │ clone()     │      │ clone()     │      │ clone()     │      │             │
//!                        │ producer_b  │      │ producer_c  │      │ producer_d  │      │             │
//!                        │ .publish()  │      │ .publish()  │      │ .publish()  │      │             │
//!                        └─────────────┘      └─────────────┘      └─────────────┘      └─────────────┘
//! ```
//!
//! # 核心设计模式
//!
//! **Consumer-as-Producer Pattern**：
//! - 每个 Disruptor 独立运行
//! - 当前阶段的 Consumer 处理完事件后，通过 clone 的 Producer 发布到下一阶段
//! - 实现完整的流水线解耦

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use disruptor::{BusySpin, Producer, build_multi_producer};

use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

// =============================================================================
// 各阶段事件定义
// =============================================================================

/// Stage 1: 收单事件
#[derive(Debug, Clone, Default)]
pub struct AcquiringEvent {
    pub cmd: Option<NewOrderCmd>,
    pub sequence: i64,
}

/// Stage 1 输出 → Stage 2 输入
#[derive(Debug, Clone, Default)]
pub struct MatchingEvent {
    pub order_cmd: Option<NewOrderCmd>,
    pub balance_change_log: Option<diff::ChangeLogEntry>,
    pub order_change_log: Option<diff::ChangeLogEntry>,
    pub sequence: i64,
}

/// Stage 2 输出 → Stage 3 输入
#[derive(Debug, Clone, Default)]
pub struct SettlementEvent {
    pub order_cmd: Option<NewOrderCmd>,
    pub acquiring_balance_log: Option<diff::ChangeLogEntry>,
    pub acquiring_order_log: Option<diff::ChangeLogEntry>,
    pub order_change_logs: Option<Vec<diff::ChangeLogEntry>>,
    pub trade_change_logs: Option<Vec<diff::ChangeLogEntry>>,
    pub trade_count: u32,
    pub sequence: i64,
}

/// Stage 3 输出 → Stage 4 输入
#[derive(Debug, Clone, Default)]
pub struct PushEvent {
    pub order_cmd: Option<NewOrderCmd>,
    pub acquiring_balance_log: Option<diff::ChangeLogEntry>,
    pub acquiring_order_log: Option<diff::ChangeLogEntry>,
    pub matching_order_logs: Option<Vec<diff::ChangeLogEntry>>,
    pub matching_trade_logs: Option<Vec<diff::ChangeLogEntry>>,
    pub matching_trade_count: u32,
    pub settlement_balance_logs: Vec<diff::ChangeLogEntry>,
    pub settlement_count: u32,
    pub sequence: i64,
}

// =============================================================================
// 多 Disruptor 流水线
// =============================================================================

/// 多 Disruptor 交易流水线
///
/// 每个阶段一个独立的 Disruptor，消费者同时也是下一阶段的生产者
pub struct MultiDisruptorPipeline;

impl MultiDisruptorPipeline {
    /// 创建并启动流水线
    ///
    /// 返回 Stage 1 的生产者，用于提交订单
    pub fn start(
        trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    ) -> impl Producer<AcquiringEvent> + Clone {
        // Stage 4: 推送阶段（最后构建，因为需要被 Stage 3 引用）
        let push_producer = build_multi_producer(1024, PushEvent::default, BusySpin)
            .handle_events_with(|event, seq, _| {
                tracing::info!(
                    "[Stage 4: 推送] 完成 seq={} trades={} settled={}",
                    seq,
                    event.matching_trade_count,
                    event.settlement_count
                );
            })
            .build();

        // Stage 3: 结算阶段
        let settlement_behavior = trade_behavior.clone();
        let push_prod_for_settlement = push_producer.clone();

        let settlement_producer = build_multi_producer(1024, SettlementEvent::default, BusySpin)
            .handle_events_with(move |event, seq, _| {
                tracing::debug!("[Stage 3: 结算] 开始 seq={}", seq);

                let mut settlement_logs = vec![];
                let mut settlement_count = 0u32;

                if let Some(ref trade_logs) = event.trade_change_logs {
                    for trade_log in trade_logs {
                        if let Ok(trade_id) = trade_log.entity_id().parse::<u64>() {
                            match settlement_behavior.handle_settlement2(trade_id) {
                                Ok(balance_logs) => {
                                    settlement_logs.extend(balance_logs);
                                    settlement_count += 1;
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "[Stage 3] trade_id={} 结算失败: {:?}",
                                        trade_id,
                                        e
                                    );
                                }
                            }
                        }
                    }
                }

                tracing::info!("[Stage 3: 结算] 完成 seq={} settled={}", seq, settlement_count);

                // Consumer-as-Producer: 发布到 Stage 4
                let push_event = PushEvent {
                    order_cmd: event.order_cmd.clone(),
                    acquiring_balance_log: event.acquiring_balance_log.clone(),
                    acquiring_order_log: event.acquiring_order_log.clone(),
                    matching_order_logs: event.order_change_logs.clone(),
                    matching_trade_logs: event.trade_change_logs.clone(),
                    matching_trade_count: event.trade_count,
                    settlement_balance_logs: settlement_logs,
                    settlement_count,
                    sequence: seq,
                };

                push_prod_for_settlement.clone().publish(|e| {
                    *e = push_event;
                });
            })
            .build();

        // Stage 2: 撮合阶段
        let matching_behavior = trade_behavior.clone();
        let settlement_prod_for_matching = settlement_producer.clone();

        let matching_producer = build_multi_producer(1024, MatchingEvent::default, BusySpin)
            .handle_events_with(move |event, seq, _| {
                tracing::debug!("[Stage 2: 撮合] 开始 seq={}", seq);

                let order_log = match &event.order_change_log {
                    Some(log) => log,
                    None => {
                        tracing::error!("[Stage 2] 缺少 order_change_log");
                        return;
                    }
                };

                match matching_behavior.handle_match3(order_log.clone()) {
                    Ok((order_logs, trade_logs)) => {
                        let trade_count = trade_logs.as_ref().map(|v| v.len()).unwrap_or(0) as u32;

                        tracing::info!("[Stage 2: 撮合] 完成 seq={} trades={}", seq, trade_count);

                        // Consumer-as-Producer: 发布到 Stage 3
                        let settlement_event = SettlementEvent {
                            order_cmd: event.order_cmd.clone(),
                            acquiring_balance_log: event.balance_change_log.clone(),
                            acquiring_order_log: event.order_change_log.clone(),
                            order_change_logs: order_logs,
                            trade_change_logs: trade_logs,
                            trade_count,
                            sequence: seq,
                        };

                        settlement_prod_for_matching.clone().publish(|e| {
                            *e = settlement_event;
                        });
                    }
                    Err(e) => {
                        tracing::error!("[Stage 2] 撮合失败 seq={}: {:?}", seq, e);
                    }
                }
            })
            .build();

        // Stage 1: 收单阶段（外部输入入口）
        let acquiring_behavior = trade_behavior.clone();
        let matching_prod_for_acquiring = matching_producer.clone();

        let acquiring_producer = build_multi_producer(1024, AcquiringEvent::default, BusySpin)
            .handle_events_with(move |event, seq, _| {
                tracing::debug!("[Stage 1: 收单] 开始 seq={}", seq);

                let cmd = match &event.cmd {
                    Some(cmd) => cmd,
                    None => {
                        tracing::error!("[Stage 1] 缺少 cmd");
                        return;
                    }
                };

                match acquiring_behavior.handle_acquiring2(cmd.clone()) {
                    Ok((balance_log, order_log)) => {
                        tracing::info!("[Stage 1: 收单] 完成 seq={}", seq);

                        // Consumer-as-Producer: 发布到 Stage 2
                        let matching_event = MatchingEvent {
                            order_cmd: Some(cmd.clone()),
                            balance_change_log: Some(balance_log),
                            order_change_log: Some(order_log),
                            sequence: seq,
                        };

                        matching_prod_for_acquiring.clone().publish(|e| {
                            *e = matching_event;
                        });
                    }
                    Err(e) => {
                        tracing::error!("[Stage 1] 收单失败 seq={}: {:?}", seq, e);
                    }
                }
            })
            .build();

        acquiring_producer
    }
}

// =============================================================================
// 测试
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试事件类型
    #[derive(Debug, Clone, Default)]
    struct Stage1Event {
        id: u64,
    }

    #[derive(Debug, Clone, Default)]
    struct Stage2Event {
        stage1_id: u64,
        stage1_data: String,
    }

    #[derive(Debug, Clone, Default)]
    struct Stage3Event {
        stage1_id: u64,
        stage1_data: String,
        stage2_data: String,
    }

    /// 测试 Consumer-as-Producer 模式
    #[test]
    fn test_consumer_as_producer() {
        // Stage 3
        let stage3_count = Arc::new(AtomicUsize::new(0));
        let c3 = stage3_count.clone();

        let stage3_producer = build_multi_producer(64, Stage3Event::default, BusySpin)
            .handle_events_with(move |event, seq, _| {
                tracing::info!(
                    "[Stage 3] 收到 stage1_id={} stage1_data={} stage2_data={}",
                    event.stage1_id,
                    event.stage1_data,
                    event.stage2_data
                );
                c3.fetch_add(1, Ordering::SeqCst);
            })
            .build();

        // Stage 2
        let stage2_count = Arc::new(AtomicUsize::new(0));
        let c2 = stage2_count.clone();
        let stage3_prod = stage3_producer.clone();

        let stage2_producer = build_multi_producer(64, Stage2Event::default, BusySpin)
            .handle_events_with(move |event, seq, _| {
                tracing::debug!("[Stage 2] 处理 stage1_id={}", event.stage1_id);

                // Consumer-as-Producer: 转发到 Stage 3
                let stage3_event = Stage3Event {
                    stage1_id: event.stage1_id,
                    stage1_data: event.stage1_data.clone(),
                    stage2_data: format!("processed-{}", seq),
                };

                stage3_prod.clone().publish(|e| {
                    *e = stage3_event;
                });

                c2.fetch_add(1, Ordering::SeqCst);
            })
            .build();

        // Stage 1
        let stage1_count = Arc::new(AtomicUsize::new(0));
        let c1 = stage1_count.clone();
        let stage2_prod = stage2_producer.clone();

        let mut stage1_producer = build_multi_producer(64, Stage1Event::default, BusySpin)
            .handle_events_with(move |event, seq, _| {
                tracing::debug!("[Stage 1] 处理 id={}", event.id);

                // Consumer-as-Producer: 转发到 Stage 2
                let stage2_event =
                    Stage2Event { stage1_id: event.id, stage1_data: format!("original-{}", seq) };

                stage2_prod.clone().publish(|e| {
                    *e = stage2_event;
                });

                c1.fetch_add(1, Ordering::SeqCst);
            })
            .build();

        // 发布测试事件
        for i in 1..=10 {
            stage1_producer.publish(|e| {
                e.id = i;
            });
        }

        // 等待处理完成
        thread::sleep(Duration::from_millis(100));

        // 验证
        assert_eq!(stage1_count.load(Ordering::SeqCst), 10, "Stage 1 应该处理 10 个事件");
        assert_eq!(stage2_count.load(Ordering::SeqCst), 10, "Stage 2 应该处理 10 个事件");
        assert_eq!(stage3_count.load(Ordering::SeqCst), 10, "Stage 3 应该处理 10 个事件");

        tracing::info!("Consumer-as-Producer 测试成功！");
    }
}
