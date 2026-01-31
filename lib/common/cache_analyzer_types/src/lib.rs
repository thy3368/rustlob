pub mod validation;

/// 缓存分析公共类型定义
///
/// 这些类型被所有使用 `CacheAnalyzer` derive 的结构体共享

/// 字段分析信息
#[derive(Debug, Clone)]
pub struct FieldAnalysis {
    /// 字段名称
    pub name: String,
    /// 字段在结构体中的偏移量（字节）
    pub offset: usize,
    /// 字段大小（字节）
    pub size: usize,
    /// 字段对齐要求（字节）
    pub alignment: usize,
    /// 是否为热点字段
    pub is_hot: bool,
}

/// 详细缓存分析报告
#[derive(Debug)]
pub struct CacheAnalysisReport {
    /// 结构体名称
    pub struct_name: String,
    /// 结构体总大小（字节）
    pub total_size: usize,
    /// 结构体对齐要求（字节）
    pub alignment: usize,
    /// 缓存行大小（字节，通常为64）
    pub cache_line_size: usize,
    /// 需要的缓存行数量
    pub cache_lines_needed: usize,
    /// 字段数量
    pub field_count: usize,
    /// 字段分析详情
    pub field_analyses: Vec<FieldAnalysis>,
    /// 填充字节总数
    pub padding_bytes: usize,
    /// 填充字节占总大小的百分比
    pub padding_percentage: f32,
    /// 最优字段顺序（字段索引列表）
    pub optimal_field_order: Vec<usize>,
    /// 当前字段顺序是否为最优
    pub is_current_order_optimal: bool,
    /// 优化建议列表
    pub suggestions: Vec<String>,
}

/// 内存布局信息
#[derive(Debug, Clone)]
pub struct MemoryLayout {
    /// 结构体名称
    pub name: String,
    /// 结构体大小（字节）
    pub size: usize,
    /// 对齐要求（字节）
    pub alignment: usize,
    /// 是否为紧凑布局（packed）
    pub is_packed: bool,
}

impl CacheAnalysisReport {
    /// 计算最优字段顺序
    ///
    /// 按对齐要求和大小降序排列可以最小化填充字节
    pub fn calculate_optimal_field_order(fields: &[FieldAnalysis]) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..fields.len()).collect();

        // 按大小和对齐降序排序
        indices.sort_by(|&a, &b| {
            let field_a = &fields[a];
            let field_b = &fields[b];

            // 首先按对齐降序
            let align_cmp = field_b.alignment.cmp(&field_a.alignment);
            if align_cmp != std::cmp::Ordering::Equal {
                return align_cmp;
            }

            // 然后按大小降序
            field_b.size.cmp(&field_a.size)
        });

        indices
    }

    /// 检查当前顺序是否最优
    pub fn is_order_optimal(current: &[usize], optimal: &[usize], fields: &[FieldAnalysis]) -> bool {
        if current.len() != optimal.len() {
            return false;
        }

        // 比较排序后的字段大小序列
        let mut current_sizes: Vec<usize> = current.iter()
            .map(|&idx| fields[idx].size)
            .collect();
        let mut optimal_sizes: Vec<usize> = optimal.iter()
            .map(|&idx| fields[idx].size)
            .collect();

        current_sizes.sort_by(|a, b| b.cmp(a));
        optimal_sizes.sort_by(|a, b| b.cmp(a));

        current_sizes == optimal_sizes
    }

    /// 计算填充字节
    pub fn calculate_padding(fields: &[FieldAnalysis]) -> usize {
        let mut offset = 0;
        let mut total_padding = 0;

        for field in fields {
            // 计算对齐需要的填充
            let padding = if field.alignment > 0 {
                (field.alignment - (offset % field.alignment)) % field.alignment
            } else {
                0
            };
            total_padding += padding;
            offset += padding + field.size;
        }

        total_padding
    }
}
