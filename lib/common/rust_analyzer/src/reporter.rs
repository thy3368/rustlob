use anyhow::Result;
use colored::Colorize;
use std::path::Path;

use crate::analyzer::{AnalysisResult, IssueCategory, Severity};
use crate::llvm_analyzer::LLVMAnalysisResult;

pub struct Reporter {
    analysis_result: AnalysisResult,
    llvm_result: Option<LLVMAnalysisResult>,
}

impl Reporter {
    pub fn new(analysis_result: AnalysisResult, llvm_result: Option<LLVMAnalysisResult>) -> Self {
        Self {
            analysis_result,
            llvm_result,
        }
    }

    pub fn generate_report(&self, format: &str, output_file: Option<&Path>) -> Result<()> {
        match format {
            "json" => self.generate_json(output_file),
            "yaml" => self.generate_yaml(output_file),
            "html" => self.generate_html(output_file),
            _ => self.generate_terminal(),
        }
    }

    fn generate_terminal(&self) -> Result<()> {
        println!("\n{}", "╔══════════════════════════════════════════════════════════╗".cyan());
        println!("{}", "║       Rust 代码优化潜力分析报告                         ║".cyan().bold());
        println!("{}", "╚══════════════════════════════════════════════════════════╝".cyan());

        // 基本统计
        println!("\n{}", "📊 基本统计:".yellow().bold());
        println!("  • 分析文件数: {}", self.analysis_result.files_analyzed.to_string().green());
        println!("  • 总代码行数: {}", self.analysis_result.total_lines.to_string().green());
        println!("  • 发现问题数: {}", self.analysis_result.issues.len().to_string().yellow());
        println!("  • 总函数数量: {}", self.analysis_result.statistics.total_functions.to_string().green());

        // 优化分数
        println!("\n{}", "🎯 优化分数:".yellow().bold());
        let score = &self.analysis_result.score;
        let grade_color = match score.overall {
            x if x >= 80.0 => "green",
            x if x >= 60.0 => "yellow",
            _ => "red",
        };

        println!("  • 总体评分: {} ({})",
            self.colorize(&format!("{:.1}/100", score.overall), grade_color),
            self.colorize(score.grade(), grade_color)
        );
        println!("  • 向量化得分: {:.1}/100", score.vectorization);
        println!("  • 内存管理得分: {:.1}/100", score.memory);
        println!("  • 内联优化得分: {:.1}/100", score.inlining);
        println!("  • 缓存对齐得分: {:.1}/100", score.cache);
        println!("\n  • 预估加速潜力: {:.2}x", score.estimated_speedup());
        println!("  • 优化潜力空间: {:.1}%", score.optimization_potential());

        // LLVM分析结果
        if let Some(llvm) = &self.llvm_result {
            println!("\n{}", "🔬 LLVM深度分析:".cyan().bold());
            println!("  • 向量化循环: {}/{} ({:.1}%)",
                llvm.vectorization.vectorized_loops,
                llvm.vectorization.vectorized_loops + llvm.vectorization.missed_loops,
                llvm.vectorization.vectorization_rate * 100.0
            );
            println!("  • 内联函数: {}/{} ({:.1}%)",
                llvm.inlining.inlined_functions,
                llvm.inlining.inlined_functions + llvm.inlining.not_inlined,
                llvm.inlining.inlining_rate * 100.0
            );
            println!("  • 循环展开: {}", llvm.loop_optimizations.unrolled);

            if !llvm.vectorization.barriers.is_empty() {
                println!("\n  向量化障碍:");
                for barrier in &llvm.vectorization.barriers {
                    println!("    ⚠️  {}", barrier.yellow());
                }
            }
        }

        // 问题详情
        println!("\n{}", "🔍 发现的问题 (按严重程度):".yellow().bold());

        let mut issues_by_severity = vec![
            (Severity::Critical, Vec::new()),
            (Severity::High, Vec::new()),
            (Severity::Medium, Vec::new()),
            (Severity::Low, Vec::new()),
            (Severity::Info, Vec::new()),
        ];

        for issue in &self.analysis_result.issues {
            for (sev, issues) in &mut issues_by_severity {
                if issue.severity == *sev {
                    issues.push(issue);
                    break;
                }
            }
        }

        for (severity, issues) in issues_by_severity {
            if issues.is_empty() {
                continue;
            }

            let (icon, color) = match severity {
                Severity::Critical => ("🔴", "red"),
                Severity::High => ("🟠", "yellow"),
                Severity::Medium => ("🟡", "blue"),
                Severity::Low => ("🟢", "green"),
                Severity::Info => ("ℹ️", "cyan"),
            };

            println!("\n{} {:?} 级别 ({} 个):", icon, severity, issues.len());

            for (i, issue) in issues.iter().enumerate().take(5) {
                println!("\n  {}. {}", i + 1, self.colorize(&issue.message, color));
                println!("     📁 文件: {:?}", issue.file);
                if let Some(line) = issue.line {
                    println!("     📍 行号: {}", line);
                }
                println!("     💡 建议: {}", issue.suggestion.italic());
                println!("     📈 影响: {:.0}%", issue.estimated_impact * 100.0);
            }

            if issues.len() > 5 {
                println!("\n  ... 还有 {} 个类似问题", issues.len() - 5);
            }
        }

        // 优化建议
        println!("\n{}", "💡 优化建议:".green().bold());
        let suggestions = self.generate_suggestions();
        for (i, suggestion) in suggestions.iter().enumerate() {
            println!("  {}. {}", i + 1, suggestion);
        }

        // 统计摘要
        println!("\n{}", "📈 优化统计:".yellow().bold());
        println!("  • 堆分配次数: {}", self.analysis_result.statistics.heap_allocations);
        println!("  • 克隆操作次数: {}", self.analysis_result.statistics.clone_operations);
        println!("  • 循环总数: {}", self.analysis_result.statistics.loop_count);

        println!("\n{}", "═══════════════════════════════════════════════════════════".cyan());

        Ok(())
    }

    fn generate_json(&self, output_file: Option<&Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.analysis_result)?;

        if let Some(path) = output_file {
            std::fs::write(path, json)?;
            println!("✅ JSON报告已保存到: {:?}", path);
        } else {
            println!("{}", json);
        }

        Ok(())
    }

    fn generate_yaml(&self, output_file: Option<&Path>) -> Result<()> {
        let yaml = serde_yaml::to_string(&self.analysis_result)?;

        if let Some(path) = output_file {
            std::fs::write(path, yaml)?;
            println!("✅ YAML报告已保存到: {:?}", path);
        } else {
            println!("{}", yaml);
        }

        Ok(())
    }

    fn generate_html(&self, output_file: Option<&Path>) -> Result<()> {
        let html = self.build_html();
        let output_path = output_file.unwrap_or(Path::new("optimization_report.html"));

        std::fs::write(output_path, html)?;
        println!("✅ HTML报告已保存到: {:?}", output_path);

        Ok(())
    }

    fn build_html(&self) -> String {
        let score = &self.analysis_result.score;

        format!(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust 代码优化分析报告</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: 'Segoe UI', Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 20px;
            line-height: 1.6;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 10px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
            padding: 40px;
        }}
        h1 {{
            color: #333;
            text-align: center;
            margin-bottom: 30px;
            font-size: 2.5em;
        }}
        .score-container {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin: 30px 0;
        }}
        .score-card {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 25px;
            border-radius: 10px;
            text-align: center;
            box-shadow: 0 4px 15px rgba(0,0,0,0.1);
        }}
        .score-value {{
            font-size: 3em;
            font-weight: bold;
            margin: 10px 0;
        }}
        .score-label {{
            font-size: 1.1em;
            opacity: 0.9;
        }}
        .stats {{
            background: #f8f9fa;
            padding: 20px;
            border-radius: 10px;
            margin: 20px 0;
        }}
        .stat-item {{
            padding: 10px;
            border-left: 4px solid #667eea;
            margin: 10px 0;
            background: white;
            border-radius: 5px;
        }}
        .issue {{
            border-left: 4px solid #ffc107;
            padding: 15px;
            margin: 10px 0;
            background: #fff9e6;
            border-radius: 5px;
        }}
        .issue.critical {{ border-left-color: #dc3545; background: #ffe6e6; }}
        .issue.high {{ border-left-color: #ff6b6b; background: #fff0f0; }}
        .issue.medium {{ border-left-color: #ffc107; background: #fff9e6; }}
        .issue.low {{ border-left-color: #28a745; background: #e6ffe6; }}
        .suggestion {{
            color: #666;
            font-style: italic;
            margin-top: 5px;
        }}
        .progress-bar {{
            width: 100%;
            height: 30px;
            background: #e9ecef;
            border-radius: 15px;
            overflow: hidden;
            margin: 10px 0;
        }}
        .progress-fill {{
            height: 100%;
            background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: bold;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Rust 代码优化分析报告</h1>

        <div class="score-container">
            <div class="score-card">
                <div class="score-label">总体评分</div>
                <div class="score-value">{:.1}</div>
                <div class="score-label">{}</div>
            </div>
            <div class="score-card">
                <div class="score-label">向量化</div>
                <div class="score-value">{:.1}</div>
            </div>
            <div class="score-card">
                <div class="score-label">内存管理</div>
                <div class="score-value">{:.1}</div>
            </div>
            <div class="score-card">
                <div class="score-label">内联优化</div>
                <div class="score-value">{:.1}</div>
            </div>
            <div class="score-card">
                <div class="score-label">缓存对齐</div>
                <div class="score-value">{:.1}</div>
            </div>
        </div>

        <div class="stats">
            <h2>📊 基本统计</h2>
            <div class="stat-item">📁 分析文件数: <strong>{}</strong></div>
            <div class="stat-item">📝 总代码行数: <strong>{}</strong></div>
            <div class="stat-item">⚠️ 发现问题数: <strong>{}</strong></div>
            <div class="stat-item">🎯 总函数数量: <strong>{}</strong></div>
            <div class="stat-item">📦 堆分配次数: <strong>{}</strong></div>
            <div class="stat-item">📋 克隆操作: <strong>{}</strong></div>
        </div>

        <div class="stats">
            <h2>💡 优化潜力</h2>
            <div class="stat-item">
                预估加速比: <strong>{:.2}x</strong>
            </div>
            <div class="stat-item">
                优化空间: <strong>{:.1}%</strong>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: {:.1}%">{:.1}%</div>
                </div>
            </div>
        </div>

        <div class="stats">
            <h2>🔍 发现的问题</h2>
            {}
        </div>
    </div>
</body>
</html>"#,
            score.overall,
            score.grade(),
            score.vectorization,
            score.memory,
            score.inlining,
            score.cache,
            self.analysis_result.files_analyzed,
            self.analysis_result.total_lines,
            self.analysis_result.issues.len(),
            self.analysis_result.statistics.total_functions,
            self.analysis_result.statistics.heap_allocations,
            self.analysis_result.statistics.clone_operations,
            score.estimated_speedup(),
            score.optimization_potential(),
            score.optimization_potential(),
            score.optimization_potential(),
            self.build_issues_html()
        )
    }

    fn build_issues_html(&self) -> String {
        let mut html = String::new();

        for issue in self.analysis_result.issues.iter().take(20) {
            let class = match issue.severity {
                Severity::Critical => "critical",
                Severity::High => "high",
                Severity::Medium => "medium",
                Severity::Low => "low",
                Severity::Info => "info",
            };

            html.push_str(&format!(
                r#"<div class="issue {}">
                    <strong>{:?}</strong>: {}
                    <div class="suggestion">💡 {}</div>
                    <div style="margin-top: 5px; font-size: 0.9em; color: #666;">
                        📁 {:?} | 📈 影响: {:.0}%
                    </div>
                </div>"#,
                class,
                issue.category,
                issue.message,
                issue.suggestion,
                issue.file,
                issue.estimated_impact * 100.0
            ));
        }

        html
    }

    fn generate_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        let score = &self.analysis_result.score;

        if score.memory < 70.0 {
            suggestions.push("考虑使用对象池或预分配内存减少堆分配".to_string());
            suggestions.push("检查是否可以使用引用或移动语义替代克隆".to_string());
        }

        if score.vectorization < 70.0 {
            suggestions.push("使用迭代器方法或显式SIMD指令优化循环".to_string());
            suggestions.push("确保内存访问模式连续，减少循环依赖".to_string());
        }

        if score.inlining < 70.0 {
            suggestions.push("为小函数添加 #[inline] 属性".to_string());
            suggestions.push("减少函数体大小以提高内联概率".to_string());
        }

        if score.cache < 70.0 {
            suggestions.push("使用 #[repr(align(64))] 对齐关键数据结构".to_string());
            suggestions.push("避免false sharing，分离频繁访问的数据".to_string());
        }

        if self.analysis_result.statistics.heap_allocations > 100 {
            suggestions.push("热路径中检测到大量堆分配，考虑使用栈分配或复用".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("代码优化程度良好！继续保持。".to_string());
        }

        suggestions
    }

    fn colorize(&self, text: &str, color: &str) -> colored::ColoredString {
        match color {
            "red" => text.red(),
            "green" => text.green(),
            "yellow" => text.yellow(),
            "blue" => text.blue(),
            "cyan" => text.cyan(),
            _ => text.normal(),
        }
    }
}
