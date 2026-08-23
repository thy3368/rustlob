#![allow(
    dead_code,
    non_camel_case_types,
    unused_imports,
    clippy::disallowed_macros,
    clippy::large_enum_variant,
    clippy::redundant_clone,
    clippy::todo,
    clippy::too_many_arguments,
    reason = "derivatives_behavior 复刻交易所 API 原始枚举命名，且仍有旧版 handler 草稿；先用具体 lint 豁免替代 allow(warnings)。"
)]

pub mod proc;

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
