// 诱饵文件：如果 l1_core 开始依赖具体 VM/基础设施实现，这个文件会意外编译通过，
// 从而让 architecture test 失败。
use revm as infra_runtime;

fn main() {
    let _ = core::any::type_name::<infra_runtime::primitives::Address>();
}
