#![feature(portable_simd)]
#![allow(
    unused_features,
    clippy::large_stack_arrays,
    clippy::large_stack_frames,
    clippy::iter_over_hash_type,
    clippy::redundant_clone,
    clippy::undocumented_unsafe_blocks,
    reason = "push K-line SIMD 路径含既有大栈缓存与 unsafe 优化，后续需专项 perf/安全审计。"
)]

pub mod k_line;
pub mod push;
pub mod queue;
