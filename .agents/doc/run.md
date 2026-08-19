cargo fmt --all --check
cargo clippy --message-format short --workspace --all-targets -- -D warnings
cargo llvm-cov

# 在 workspace 根目录跑，统计所有 crate

cargo llvm-cov --workspace

# 只统计某个 crate

cargo llvm-cov -p my-crate
cargo llvm-cov --test integration_test
cargo llvm-cov --html

# 漏洞统计

cargo audit

# unsafe统计

cargo geiger

# 对比当前代码和 main 分支（最常用，MR 场景）

cargo semver-checks check-release --baseline-rev main
