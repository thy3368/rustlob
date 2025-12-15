pub mod proc;

// 重导出 Diff derive 宏，让用户可以直接使用
pub use diff_derive::Diff;

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
