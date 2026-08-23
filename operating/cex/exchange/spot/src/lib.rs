#![feature(portable_simd)]
#![allow(
    unused_must_use,
    unused_features,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    unused_imports,
    unused_variables,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::empty_line_after_outer_attr,
    clippy::expect_used,
    clippy::large_enum_variant,
    clippy::print_stdout,
    clippy::todo,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unwrap_used,
    reason = "spot_behavior 复刻交易所 API 原始枚举命名，且仍有旧版 handler/test 草稿；先用具体 lint 豁免替代 allow(warnings)。"
)]

pub mod proc;

pub mod entity;

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
