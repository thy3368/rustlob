use super::shard::Shard;

/// 内存中的有序日志条目，携带完整负载。
///
/// 当前类型不定义持久化或传输格式；日志成为唯一真相源取决于后续写入链路。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry<P> {
    pub(crate) t: u8,
    pub(crate) p: u8,
    /// 全局单调递增的序列号，是全序的依据。
    lsn: u64,
    /// 由定序器分配的逻辑时间戳。
    logical_ts: u64,
    /// 用于幂等去重的客户端标识。
    client_id: u64,
    /// 在客户端标识内唯一的请求序号。
    request_id: u64,
    /// 完整的业务命令或其他日志负载。
    payload: P,
}

/// 提供状态机 executor 投影事件时所需的日志元数据。
pub trait LogEntryMetadata: Shard {
    fn lsn(&self) -> u64;
    fn logical_ts(&self) -> u64;
}

impl<P> LogEntryMetadata for LogEntry<P> {
    fn lsn(&self) -> u64 {
        self.lsn
    }
    fn logical_ts(&self) -> u64 {
        self.logical_ts
    }
}

impl<P> Shard for LogEntry<P> {
    fn topic(&self) -> u8 {
        self.t
    }
    fn partition(&self) -> u8 {
        self.p
    }
}

impl<P> std::ops::Deref for LogEntry<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

impl<P> LogEntry<P> {
    pub(crate) fn new_with_topic(
        t: u8,
        p: u8,
        lsn: u64,
        logical_ts: u64,
        client_id: u64,
        request_id: u64,
        payload: P,
    ) -> Self {
        Self { t, p, lsn, logical_ts, client_id, request_id, payload }
    }
    pub(crate) fn new(
        lsn: u64,
        logical_ts: u64,
        client_id: u64,
        request_id: u64,
        payload: P,
    ) -> Self {
        Self { t: 0, p: 0, lsn, logical_ts, client_id, request_id, payload }
    }

    pub const fn lsn(&self) -> u64 {
        self.lsn
    }
    pub const fn logical_ts(&self) -> u64 {
        self.logical_ts
    }
    pub const fn client_id(&self) -> u64 {
        self.client_id
    }
    pub const fn request_id(&self) -> u64 {
        self.request_id
    }
    pub const fn topic(&self) -> u8 {
        self.t
    }
    pub const fn partition(&self) -> u8 {
        self.p
    }
    pub const fn payload(&self) -> &P {
        &self.payload
    }
    pub fn into_payload(self) -> P {
        self.payload
    }
}
