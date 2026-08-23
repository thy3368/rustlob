#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::disallowed_types,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::clone_on_ref_ptr,
    clippy::empty_line_after_doc_comments,
    clippy::expect_used,
    clippy::manual_div_ceil,
    clippy::needless_borrow,
    clippy::ptr_arg,
    clippy::todo,
    clippy::unwrap_used,
    reason = "db_repo 仍有未完成内存/PG adapter 与旧 MySQL 连接实现；先以具体 lint 豁免替代 allow(warnings)。"
)]

pub mod adapter;
pub mod core;

// 导出核心仓储接口和分页类型
pub use core::db_repo::{CmdRepo, PageRequest, PageResult, QueryRepo, RepoError};
pub use core::db_repo2::CmdRepo2;
pub use core::event_publish::EventPublisher2;
pub use core::kv_store::{KvStore, RkyvKvStoreExt, StorageError};

// 导出适配器实现
pub use adapter::mysql_db_repo::MySqlDbRepo;
pub use adapter::v2::mysql_repo::MySqlRepo;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
