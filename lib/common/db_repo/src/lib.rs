pub mod adapter;
pub mod core;

// 导出核心仓储接口和分页类型
pub use core::db_repo::{CmdRepo, PageRequest, PageResult, QueryRepo, RepoError};

// 导出适配器实现
pub use adapter::mysql_db_repo::MySqlDbRepo;

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
