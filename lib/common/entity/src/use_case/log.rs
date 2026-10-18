

/// 一条不可变的、全局有序的日志条目
/// 它是系统的唯一真相源（Source of Truth）
#[derive(Debug, Clone, Copy)]
#[repr(C)] // 保证内存布局确定，便于零拷贝和跨节点传输
pub struct LogEntry {
    /// 全局单调递增的序列号
    /// 这是"全序"的唯一依据
    pub seq: u64,

    /// 逻辑时间戳（由定序器分配，非物理时钟）
    /// 用于替代 NTP/PTP，作为系统的逻辑时间
    pub logical_ts: u64,

    /// 客户端标识（用于幂等去重）
    pub client_id: u64,

    /// 客户端请求序号（client_id 内唯一）
    pub request_id: u64,

    /// 负载类型标签（区分 Command / SystemEvent / Control）
    pub payload_type: PayloadType,


    /// CRC32 / XXH3 校验和（完整性保护）
    pub checksum: u64,
}