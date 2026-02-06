use std::time::Duration;

use diff::{ChangeLogEntry, Entity};

use crate::RepoError;

pub struct RecordMetadata;
pub trait ChangeLogQueueRepo {
    /// 仓储中存储的实体类型
    // type E: Entity;

    fn send(&self, event: &ChangeLogEntry) -> Result<RecordMetadata, RepoError>;

    fn send_batch(&self, records: &Vec<ChangeLogEntry>) -> Result<Vec<RecordMetadata>, RepoError>;

    /// 拉取消息
    fn poll(&self, timeout: Duration) -> Result<Vec<ChangeLogEntry>, RepoError>;

    // todo 定义接口 poll with callback 条件过lu
}
