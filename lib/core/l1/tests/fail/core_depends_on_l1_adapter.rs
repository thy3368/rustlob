// 诱饵文件：如果 l1_core 开始依赖 l1_adapter，这个文件会意外编译通过，
// 从而让 architecture test 失败。
use l1_adapter as outbound_adapter;

fn main() {
    let _ = core::any::type_name::<outbound_adapter::ExecuteAndCommitBlockStatePipeline>();
}
