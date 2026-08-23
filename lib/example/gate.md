# 1
cargo clippy -p example_core_entity -p example_core_use_case -- -D warnings

# 2 
测试覆盖率

cargo llvm-cov --package example_core_use_case --text --output-dir /tmp/example_core_use_case_cov



cargo llvm-cov --package example_core_use_case --html
open target/llvm-cov/html/index.html


