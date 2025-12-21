pub mod core;
pub mod adapter;

// 导出核心仓储接口和分页类型
pub use core::db_repo::{
    DBCmdRepo,
    DBQueryRepo,
    RepoError,
    PageRequest,
    PageResult,
};

pub fn add(left: u64, right: u64) -> u64 { left + right }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

