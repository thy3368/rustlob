use spot_behavior::proc::behavior::{
    spot_trade_behavior::{CmdResp, CommonError, SpotCmdErrorAny},
    v2::spot_market_data_sse_behavior::{
        MarketDataSubscriptionCmdAny, SpotMarketDataSubscriptionBehavior, SubscriptionResponse,
        SubscriptionResult
    }
};
use base_types::cqrs::cqrs_types::ResMetadata;

/// Spot 市场数据 SSE 实现
#[derive(Clone)]
pub struct SpotMarketDataSSEImpl {
    /// 已订阅的流列表
    subscriptions: Vec<String>,
    /// 是否启用复合流属性
    combined_stream_enabled: bool
}

impl SpotMarketDataSSEImpl {
    /// 创建新的 SpotMarketDataSSEImpl 实例
    pub fn new() -> Self {
        Self {
            subscriptions: Vec::new(),
            combined_stream_enabled: false
        }
    }
}

impl SpotMarketDataSubscriptionBehavior for SpotMarketDataSSEImpl {
    fn handle_subscription(
        &mut self, cmd: MarketDataSubscriptionCmdAny
    ) -> Result<CmdResp<SubscriptionResponse>, SpotCmdErrorAny> {
        let nonce = 0; // 临时值，实际应该从 metadata 中获取或生成

        match cmd {
            MarketDataSubscriptionCmdAny::Subscribe {
                metadata,
                streams
            } => {
                // 添加新的订阅流
                for stream in streams {
                    if !self.subscriptions.contains(&stream) {
                        self.subscriptions.push(stream);
                    }
                }
                // 返回成功响应
                Ok(CmdResp::new(ResMetadata::new(nonce, false, 0), SubscriptionResponse {
                    result: Some(SubscriptionResult::Success),
                    id: None
                }))
            }

            MarketDataSubscriptionCmdAny::Unsubscribe {
                metadata,
                streams
            } => {
                // 移除订阅流
                for stream in streams {
                    self.subscriptions.retain(|s| s != &stream);
                }
                // 返回成功响应
                Ok(CmdResp::new(ResMetadata::new(nonce, false, 0), SubscriptionResponse {
                    result: Some(SubscriptionResult::Success),
                    id: None
                }))
            }

            MarketDataSubscriptionCmdAny::ListSubscriptions {
                metadata
            } => {
                // 返回当前订阅列表
                Ok(CmdResp::new(ResMetadata::new(nonce, false, 0), SubscriptionResponse {
                    result: Some(SubscriptionResult::Subscriptions(self.subscriptions.clone())),
                    id: None
                }))
            }

            MarketDataSubscriptionCmdAny::SetProperty {
                metadata,
                property,
                value
            } => {
                // 设置属性（目前仅支持 combined 属性）
                if property == "combined" {
                    self.combined_stream_enabled = value;
                    Ok(CmdResp::new(ResMetadata::new(nonce, false, 0), SubscriptionResponse {
                        result: Some(SubscriptionResult::Property(value)),
                        id: None
                    }))
                } else {
                    Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "property",
                        reason: "Unknown property"
                    }))
                }
            }

            MarketDataSubscriptionCmdAny::GetProperty {
                metadata,
                property
            } => {
                // 获取属性值
                if property == "combined" {
                    Ok(CmdResp::new(ResMetadata::new(nonce, false, 0), SubscriptionResponse {
                        result: Some(SubscriptionResult::Property(self.combined_stream_enabled)),
                        id: None
                    }))
                } else {
                    Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "property",
                        reason: "Unknown property"
                    }))
                }
            }
        }
    }
}
