use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLVMAnalysisResult {
    pub vectorization: VectorizationAnalysis,
    pub inlining: InliningAnalysis,
    pub loop_optimizations: LoopOptimizations,
    pub optimization_remarks: Vec<OptimizationRemark>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizationAnalysis {
    pub vectorized_loops: usize,
    pub missed_loops: usize,
    pub vectorization_rate: f32,
    pub barriers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InliningAnalysis {
    pub inlined_functions: usize,
    pub not_inlined: usize,
    pub inlining_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopOptimizations {
    pub unrolled: usize,
    pub interchanged: usize,
    pub fused: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRemark {
    pub remark_type: String,
    pub message: String,
    pub location: Option<String>,
}

pub struct LLVMAnalyzer {
    project_path: PathBuf,
}

impl LLVMAnalyzer {
    pub fn new(project_path: PathBuf) -> Result<Self> {
        Ok(Self { project_path })
    }

    pub fn generate_and_analyze(&self) -> Result<LLVMAnalysisResult> {
        let output_dir = self.project_path.join("llvm_analysis");
        fs::create_dir_all(&output_dir)?;

        self.generate_ir(3, &output_dir)?;
        self.analyze_ir(&output_dir)
    }

    pub fn generate_ir(&self, opt_level: u8, output_dir: &Path) -> Result<()> {
        println!("🔨 生成LLVM IR (优化级别: {})...", opt_level);

        // 生成LLVM IR
        let status = Command::new("cargo")
            .current_dir(&self.project_path)
            .env("RUSTFLAGS", format!("-C opt-level={} -C debuginfo=0 --emit=llvm-ir", opt_level))
            .args(&["rustc", "--release", "--", "-o"])
            .arg(output_dir.join(format!("output_O{}.ll", opt_level)))
            .status()
            .context("执行cargo rustc失败")?;

        if !status.success() {
            anyhow::bail!("生成LLVM IR失败");
        }

        println!("✅ LLVM IR已生成");
        Ok(())
    }

    pub fn analyze_ir(&self, output_dir: &Path) -> Result<LLVMAnalysisResult> {
        println!("🔍 分析LLVM IR...");

        // 查找生成的.ll文件
        let ll_files: Vec<_> = fs::read_dir(output_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "ll"))
            .collect();

        if ll_files.is_empty() {
            anyhow::bail!("未找到LLVM IR文件");
        }

        let ll_file = ll_files[0].path();
        let content = fs::read_to_string(&ll_file).context("读取LLVM IR文件失败")?;

        // 分析IR内容
        let vectorization = self.analyze_vectorization(&content);
        let inlining = self.analyze_inlining(&content);
        let loop_optimizations = self.analyze_loops(&content);
        let optimization_remarks = self.extract_remarks(&content);

        Ok(LLVMAnalysisResult { vectorization, inlining, loop_optimizations, optimization_remarks })
    }

    fn analyze_vectorization(&self, ir_content: &str) -> VectorizationAnalysis {
        let vectorized = ir_content.matches("vector.body").count();
        let total_loops =
            ir_content.matches("for.body").count() + ir_content.matches("while.body").count();

        let missed = total_loops.saturating_sub(vectorized);
        let rate = if total_loops > 0 { vectorized as f32 / total_loops as f32 } else { 0.0 };

        let mut barriers = Vec::new();
        if ir_content.contains("store volatile") || ir_content.contains("load volatile") {
            barriers.push("volatile内存访问阻止向量化".to_string());
        }
        if ir_content.contains("call") && ir_content.matches("call").count() > 50 {
            barriers.push("过多函数调用可能阻止向量化".to_string());
        }

        VectorizationAnalysis {
            vectorized_loops: vectorized,
            missed_loops: missed,
            vectorization_rate: rate,
            barriers,
        }
    }

    fn analyze_inlining(&self, ir_content: &str) -> InliningAnalysis {
        // 统计define和declare的函数
        let total_defines = ir_content.matches("define").count();
        let external_calls = ir_content.matches("declare").count();

        // 简化估算：假设小函数已被内联
        let inlined = total_defines.saturating_sub(external_calls / 2);
        let not_inlined = external_calls;

        let rate = if total_defines > 0 { inlined as f32 / total_defines as f32 } else { 0.0 };

        InliningAnalysis { inlined_functions: inlined, not_inlined, inlining_rate: rate }
    }

    fn analyze_loops(&self, ir_content: &str) -> LoopOptimizations {
        LoopOptimizations {
            unrolled: ir_content.matches("unroll").count(),
            interchanged: ir_content.matches("interchange").count(),
            fused: ir_content.matches("fusion").count(),
        }
    }

    fn extract_remarks(&self, _ir_content: &str) -> Vec<OptimizationRemark> {
        // 简化实现：返回空列表
        // 实际应该解析LLVM优化备注
        Vec::new()
    }

    pub fn compare_optimization_levels(&self) -> Result<()> {
        let output_dir = self.project_path.join("llvm_comparison");
        fs::create_dir_all(&output_dir)?;

        println!("📊 比较不同优化级别...\n");

        for opt_level in [0, 1, 2, 3] {
            println!("--- 优化级别 {} ---", opt_level);
            self.generate_ir(opt_level, &output_dir)?;

            let result = self.analyze_ir(&output_dir)?;

            println!("向量化率: {:.1}%", result.vectorization.vectorization_rate * 100.0);
            println!("内联率: {:.1}%", result.inlining.inlining_rate * 100.0);
            println!("循环展开: {}", result.loop_optimizations.unrolled);
            println!();
        }

        Ok(())
    }
}
