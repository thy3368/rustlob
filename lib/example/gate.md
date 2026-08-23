# 1
cargo clippy -p example_core -- -D warnings

# 2 
测试覆盖率

cargo llvm-cov --package example_core --text --output-dir /tmp/example_core_cov

