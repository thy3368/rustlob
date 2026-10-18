/// 内存中的有序日志条目，携带完整负载。
///
/// 当前类型不定义持久化或传输格式；日志成为唯一真相源取决于后续写入链路。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry<P> {
    /// 全局单调递增的序列号，是全序的依据。
    pub lsn: u64,
    /// 由定序器分配的逻辑时间戳。
    pub logical_ts: u64,
    /// 用于幂等去重的客户端标识。
    pub client_id: u64,
    /// 在客户端标识内唯一的请求序号。
    pub request_id: u64,
    /// 完整的业务命令或其他日志负载。
    pub payload: P,
}
