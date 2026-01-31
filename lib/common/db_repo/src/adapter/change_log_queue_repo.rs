use std::time::Duration;

use diff::ChangeLogEntry;
use immutable_derive::immutable;

use crate::{
    core::queue_repo::{ChangeLogQueueRepo, RecordMetadata},
    RepoError
};

// 线程间通道
#[immutable]
pub struct ChangeLogChannelQueueRepo {}



impl ChangeLogQueueRepo for ChangeLogChannelQueueRepo {
    fn send(&self, event: &ChangeLogEntry) -> Result<RecordMetadata, RepoError> { todo!() }

    fn send_batch(&self, records: &Vec<ChangeLogEntry>) -> Result<Vec<RecordMetadata>, RepoError> { todo!() }

    fn poll(&self, timeout: Duration) -> Result<Vec<ChangeLogEntry>, RepoError> { todo!() }
}


// kafka

#[immutable]
pub struct ChangeLogKafkaQueueRepo {}
impl ChangeLogQueueRepo for ChangeLogKafkaQueueRepo {
    fn send(&self, event: &ChangeLogEntry) -> Result<RecordMetadata, RepoError> { todo!() }

    fn send_batch(&self, records: &Vec<ChangeLogEntry>) -> Result<Vec<RecordMetadata>, RepoError> { todo!() }

    fn poll(&self, timeout: Duration) -> Result<Vec<ChangeLogEntry>, RepoError> { todo!() }
}


// 组播
#[immutable]
pub struct ChangeLogMultiCaseQueueRepo {}

impl ChangeLogQueueRepo for ChangeLogMultiCaseQueueRepo {
    fn send(&self, event: &ChangeLogEntry) -> Result<RecordMetadata, RepoError> { todo!() }

    fn send_batch(&self, records: &Vec<ChangeLogEntry>) -> Result<Vec<RecordMetadata>, RepoError> { todo!() }

    fn poll(&self, timeout: Duration) -> Result<Vec<ChangeLogEntry>, RepoError> { todo!() }
}
