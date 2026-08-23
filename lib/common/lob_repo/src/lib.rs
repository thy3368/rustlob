#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::assign_op_pattern,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::todo,
    clippy::type_complexity,
    clippy::unnecessary_map_or,
    clippy::unwrap_used,
    reason = "lob_repo 仍包含未完成 adapter 与多版撮合实现；先显式记录历史债务，替代 allow(warnings)。"
)]

pub mod adapter;
pub mod core;

pub use core::repo_snapshot_support::LobError;

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
