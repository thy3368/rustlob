use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;

use crate::analyzer::{IssueCategory, OptimizationIssue, Severity};

lazy_static! {
    // 内存分配反模式
    static ref HOT_PATH_ALLOCATION: Regex = Regex::new(
        r"(?m)^\s*(let|mut)\s+\w+\s*=\s*(Vec::new|String::new|Box::new|HashMap::new)"
    ).unwrap();

    // 不必要的克隆
    static ref UNNECESSARY_CLONE: Regex = Regex::new(r"\.clone\(\)").unwrap();

    // 非对齐结构体
    static ref UNALIGNED_STRUCT: Regex = Regex::new(
        r"(?m)^pub struct \w+\s*\{(?![\s\S]*#\[repr\(align)"
    ).unwrap();

    // 使用unwrap而非?
    static ref UNWRAP_PATTERN: Regex = Regex::new(r"\.unwrap\(\)").unwrap();

    // 字符串拼接在循环中
    static ref STRING_CONCAT_LOOP: Regex = Regex::new(
        r#"for\s+.*\{[^}]*\+\s*"[^"]*"|"[^"]*"\s*\+[^}]*\}"#
    ).unwrap();

    // 未使用的Result
    static ref UNUSED_RESULT: Regex = Regex::new(
        r"(?m)^\s+\w+\([^)]*\);(?!\s*\.)"
    ).unwrap();

    // 缓存行大小非64/128字节
    static ref WRONG_ALIGNMENT: Regex = Regex::new(
        r#"#\[repr\(align\((\d+)\)\)\]"#
    ).unwrap();

    // volatile访问
    static ref VOLATILE_ACCESS: Regex = Regex::new(
        r"(ptr::read_volatile|ptr::write_volatile)"
    ).unwrap();

    // 过度使用Mutex
    static ref MUTEX_OVERUSE: Regex = Regex::new(
        r"Mutex::new|RwLock::new"
    ).unwrap();

    // 递归函数没有尾调用优化
    static ref RECURSIVE_FUNCTION: Regex = Regex::new(
        r"fn\s+(\w+)[^{]*\{[^}]*\1\s*\([^}]*\}"
    ).unwrap();
}

pub struct PatternDetector;

impl PatternDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_patterns(&self, content: &str, file_path: &Path) -> Vec<OptimizationIssue> {
        let mut issues = Vec::new();

        // 检测热路径内存分配
        for cap in HOT_PATH_ALLOCATION.captures_iter(content) {
            issues.push(OptimizationIssue {
                file: file_path.to_path_buf(),
                line: Self::get_line_number(content, cap.get(0).unwrap().start()),
                category: IssueCategory::MemoryAllocation,
                severity: Severity::High,
                message: "检测到可能在热路径中的堆分配".to_string(),
                suggestion: "考虑预分配或使用对象池模式，避免频繁分配".to_string(),
                estimated_impact: 0.6,
            });
        }

        // 检测不必要的克隆
        let clone_count = UNNECESSARY_CLONE.find_iter(content).count();
        if clone_count > 5 {
            issues.push(OptimizationIssue {
                file: file_path.to_path_buf(),
                line: None,
                category: IssueCategory::MemoryAllocation,
                severity: Severity::Medium,
                message: format!("发现 {} 次 .clone() 调用", clone_count),
                suggestion: "检查是否可以使用引用或移动语义避免克隆".to_string(),
                estimated_impact: 0.4,
            });
        }

        // 检测未对齐的结构体
        for cap in UNALIGNED_STRUCT.captures_iter(content) {
            issues.push(OptimizationIssue {
                file: file_path.to_path_buf(),
                line: Self::get_line_number(content, cap.get(0).unwrap().start()),
                category: IssueCategory::CacheAlignment,
                severity: Severity::Medium,
                message: "结构体未指定对齐方式".to_string(),
                suggestion: "考虑添加 #[repr(align(64))] 或 #[repr(align(128))] 以优化缓存性能".to_string(),
                estimated_impact: 0.5,
            });
        }

        // 检测错误的对齐值
        for cap in WRONG_ALIGNMENT.captures_iter(content) {
            if let Some(align_val) = cap.get(1) {
                let alignment: u32 = align_val.as_str().parse().unwrap_or(0);
                if alignment != 64 && alignment != 128 && alignment > 8 {
                    issues.push(OptimizationIssue {
                        file: file_path.to_path_buf(),
                        line: Self::get_line_number(content, cap.get(0).unwrap().start()),
                        category: IssueCategory::CacheAlignment,
                        severity: Severity::Low,
                        message: format!("对齐值 {} 可能不是最优的", alignment),
                        suggestion: "现代CPU缓存行通常为64或128字节，考虑使用这些值".to_string(),
                        estimated_impact: 0.3,
                    });
                }
            }
        }

        // 检测循环中的字符串拼接
        if STRING_CONCAT_LOOP.is_match(content) {
            issues.push(OptimizationIssue {
                file: file_path.to_path_buf(),
                line: None,
                category: IssueCategory::Algorithmic,
                severity: Severity::High,
                message: "检测到循环中的字符串拼接".to_string(),
                suggestion: "使用 String::with_capacity() 预分配或 format! 宏".to_string(),
                estimated_impact: 0.7,
            });
        }

        // 检测volatile访问
        if VOLATILE_ACCESS.is_match(content) {
            issues.push(OptimizationIssue {
                file: file_path.to_path_buf(),
                line: None,
                category: IssueCategory::Vectorization,
                severity: Severity::Info,
                message: "检测到volatile内存访问".to_string(),
                suggestion: "volatile访问会阻止编译器优化，确保必要时才使用".to_string(),
                estimated_impact: 0.4,
            });
        }

        // 检测Mutex过度使用
        let mutex_count = MUTEX_OVERUSE.find_iter(content).count();
        if mutex_count > 10 {
            issues.push(OptimizationIssue {
                file: file_path.to_path_buf(),
                line: None,
                category: IssueCategory::Concurrency,
                severity: Severity::Medium,
                message: format!("检测到 {} 个Mutex/RwLock", mutex_count),
                suggestion: "考虑使用无锁数据结构或原子操作减少锁竞争".to_string(),
                estimated_impact: 0.6,
            });
        }

        // 检测递归函数
        for cap in RECURSIVE_FUNCTION.captures_iter(content) {
            if let Some(fn_name) = cap.get(1) {
                issues.push(OptimizationIssue {
                    file: file_path.to_path_buf(),
                    line: Self::get_line_number(content, cap.get(0).unwrap().start()),
                    category: IssueCategory::Algorithmic,
                    severity: Severity::Medium,
                    message: format!("检测到递归函数 '{}'", fn_name.as_str()),
                    suggestion: "考虑改写为迭代版本或确保尾调用优化生效".to_string(),
                    estimated_impact: 0.5,
                });
            }
        }

        // 检测unwrap过度使用
        let unwrap_count = UNWRAP_PATTERN.find_iter(content).count();
        if unwrap_count > 20 {
            issues.push(OptimizationIssue {
                file: file_path.to_path_buf(),
                line: None,
                category: IssueCategory::BranchPrediction,
                severity: Severity::Low,
                message: format!("发现 {} 次 unwrap() 调用", unwrap_count),
                suggestion: "过多的unwrap可能影响分支预测，考虑使用?操作符或match".to_string(),
                estimated_impact: 0.2,
            });
        }

        issues
    }

    fn get_line_number(content: &str, pos: usize) -> Option<usize> {
        Some(content[..pos].lines().count())
    }
}
