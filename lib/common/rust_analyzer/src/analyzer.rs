use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use syn::visit::Visit;
use syn::{File, Item};
use walkdir::WalkDir;

use crate::patterns::PatternDetector;
use crate::scorer::OptimizationScore;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AnalysisResult {
    pub files_analyzed: usize,
    pub total_lines: usize,
    pub issues: Vec<OptimizationIssue>,
    pub score: OptimizationScore,
    pub statistics: Statistics,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct OptimizationIssue {
    pub file: PathBuf,
    pub line: Option<usize>,
    pub category: IssueCategory,
    pub severity: Severity,
    pub message: String,
    pub suggestion: String,
    pub estimated_impact: f32, // 0.0 - 1.0
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum IssueCategory {
    Vectorization,
    MemoryAllocation,
    Inlining,
    BranchPrediction,
    CacheAlignment,
    Concurrency,
    Cloning,
    Algorithmic,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Severity {
    Critical, // 严重性能问题
    High,     // 高优先级
    Medium,   // 中等优先级
    Low,      // 低优先级
    Info,     // 信息提示
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Statistics {
    pub total_functions: usize,
    pub inline_candidates: usize,
    pub heap_allocations: usize,
    pub clone_operations: usize,
    pub loop_count: usize,
    pub vectorizable_loops: usize,
}

pub struct RustCodeAnalyzer {
    root_path: PathBuf,
    pattern_detector: PatternDetector,
}

impl RustCodeAnalyzer {
    pub fn new(root_path: PathBuf) -> Result<Self> {
        Ok(Self { root_path, pattern_detector: PatternDetector::new() })
    }

    pub fn analyze(&self) -> Result<AnalysisResult> {
        let mut issues = Vec::new();
        let mut statistics = Statistics {
            total_functions: 0,
            inline_candidates: 0,
            heap_allocations: 0,
            clone_operations: 0,
            loop_count: 0,
            vectorizable_loops: 0,
        };

        let mut files_analyzed = 0;
        let mut total_lines = 0;

        // 遍历所有Rust文件
        for entry in WalkDir::new(&self.root_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            let path = entry.path();

            // 跳过target目录
            if path.components().any(|c| c.as_os_str() == "target") {
                continue;
            }

            files_analyzed += 1;
            let content =
                fs::read_to_string(path).with_context(|| format!("读取文件失败: {:?}", path))?;

            total_lines += content.lines().count();

            // 解析Rust代码
            if let Ok(ast) = syn::parse_file(&content) {
                let mut visitor = CodeVisitor::new(path.to_path_buf());
                visitor.visit_file(&ast);

                issues.extend(visitor.issues);
                statistics.total_functions += visitor.function_count;
                statistics.heap_allocations += visitor.heap_allocations;
                statistics.clone_operations += visitor.clone_count;
                statistics.loop_count += visitor.loop_count;
            }

            // 使用正则模式检测
            let pattern_issues = self.pattern_detector.detect_patterns(&content, path);
            issues.extend(pattern_issues);
        }

        // 计算优化分数
        let score = self.calculate_score(&issues, &statistics);

        Ok(AnalysisResult { files_analyzed, total_lines, issues, score, statistics })
    }

    fn calculate_score(
        &self,
        issues: &[OptimizationIssue],
        stats: &Statistics,
    ) -> OptimizationScore {
        let mut score = 100.0;

        // 根据问题严重程度扣分
        for issue in issues {
            let deduction = match issue.severity {
                Severity::Critical => 5.0 * issue.estimated_impact,
                Severity::High => 3.0 * issue.estimated_impact,
                Severity::Medium => 2.0 * issue.estimated_impact,
                Severity::Low => 1.0 * issue.estimated_impact,
                Severity::Info => 0.5 * issue.estimated_impact,
            };
            score -= deduction;
        }

        // 基于统计数据调整
        if stats.heap_allocations > 100 {
            score -= (stats.heap_allocations as f32 / 100.0) * 2.0;
        }

        if stats.clone_operations > 50 {
            score -= (stats.clone_operations as f32 / 50.0) * 3.0;
        }

        OptimizationScore {
            overall: score.max(0.0).min(100.0),
            vectorization: self.calculate_category_score(issues, IssueCategory::Vectorization),
            memory: self.calculate_category_score(issues, IssueCategory::MemoryAllocation),
            inlining: self.calculate_category_score(issues, IssueCategory::Inlining),
            cache: self.calculate_category_score(issues, IssueCategory::CacheAlignment),
        }
    }

    fn calculate_category_score(
        &self,
        issues: &[OptimizationIssue],
        category: IssueCategory,
    ) -> f32 {
        let category_issues: Vec<_> = issues.iter().filter(|i| i.category == category).collect();

        if category_issues.is_empty() {
            return 100.0;
        }

        let mut score: f32 = 100.0;
        for issue in category_issues {
            let deduction = match issue.severity {
                Severity::Critical => 10.0,
                Severity::High => 7.0,
                Severity::Medium => 4.0,
                Severity::Low => 2.0,
                Severity::Info => 1.0,
            };
            score -= deduction;
        }

        score.max(0.0).min(100.0)
    }
}

struct CodeVisitor {
    file_path: PathBuf,
    issues: Vec<OptimizationIssue>,
    function_count: usize,
    heap_allocations: usize,
    clone_count: usize,
    loop_count: usize,
}

impl CodeVisitor {
    fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            issues: Vec::new(),
            function_count: 0,
            heap_allocations: 0,
            clone_count: 0,
            loop_count: 0,
        }
    }
}

impl<'ast> Visit<'ast> for CodeVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.function_count += 1;

        // 检查是否应该内联
        let has_inline_attr = node.attrs.iter().any(|attr| attr.path().is_ident("inline"));

        // 简单的内联候选检测：小函数没有inline属性
        let body_tokens = quote::quote!(#node).to_string();
        if body_tokens.len() < 500 && !has_inline_attr {
            self.issues.push(OptimizationIssue {
                file: self.file_path.clone(),
                line: None,
                category: IssueCategory::Inlining,
                severity: Severity::Medium,
                message: format!("函数 '{}' 可能适合内联", node.sig.ident),
                suggestion: "考虑添加 #[inline] 或 #[inline(always)] 属性".to_string(),
                estimated_impact: 0.3,
            });
        }

        syn::visit::visit_item_fn(self, node);
    }

    fn visit_expr(&mut self, node: &'ast syn::Expr) {
        // 检测Vec::new()、Box::new()等堆分配
        if let syn::Expr::Call(call) = node {
            let call_str = quote::quote!(#call).to_string();

            if call_str.contains("Vec :: new")
                || call_str.contains("Box :: new")
                || call_str.contains("String :: new")
            {
                self.heap_allocations += 1;
            }

            if call_str.contains(".clone()") {
                self.clone_count += 1;
            }
        }

        syn::visit::visit_expr(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.loop_count += 1;

        // 检测可能的向量化机会
        self.issues.push(OptimizationIssue {
            file: self.file_path.clone(),
            line: None,
            category: IssueCategory::Vectorization,
            severity: Severity::Low,
            message: "发现循环，考虑是否可以向量化".to_string(),
            suggestion: "检查循环是否可以使用迭代器方法或SIMD优化".to_string(),
            estimated_impact: 0.4,
        });

        syn::visit::visit_expr_for_loop(self, node);
    }
}
