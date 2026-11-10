/// 将已通过业务校验的查询结果投影为 adapter 视图。
///
/// 该投影不可失败，不负责业务拒绝、状态校验或数据加载。
pub trait ComputeView<O>: Send + Sync {
    type View;

    fn compute_view(&self, output: O) -> Self::View;
}
