use derivatives_behavior::proc::usds_m_future::behavior::user_data_behavior::UsdsMFutureUserDataStreamAny;

/// USDS-M 期货用户数据 SSE 实现
pub struct UsdsMFutureUserDataSSEImpl;

impl UsdsMFutureUserDataSSEImpl {
    /// 创建新的用户数据 SSE 实例
    pub fn new() -> Self {
        Self
    }

    /// 发布用户数据事件
    pub fn publish_event(&mut self, event: UsdsMFutureUserDataStreamAny) {
        println!("👤 Publishing USDS-M Future user data event: {:?}", event);
    }
}
