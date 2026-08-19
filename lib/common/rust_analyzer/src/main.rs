#![allow(warnings)]

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::Colorize;

mod analyzer;
mod llvm_analyzer;
mod optimizer;
mod patterns;
mod reporter;
mod scorer;

use analyzer::RustCodeAnalyzer;
use llvm_analyzer::LLVMAnalyzer;
use reporter::Reporter;

#[derive(Parser)]
#[command(name = "rust-opt-analyzer")]
#[command(about = "Rust代码优化潜力分析工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 分析Rust源代码的优化潜力
    Analyze {
        /// 要分析的项目路径
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// 输出格式 (json, yaml, html, terminal)
        #[arg(short, long, default_value = "terminal")]
        output: String,

        /// 输出文件路径
        #[arg(short = 'f', long)]
        output_file: Option<PathBuf>,

        /// 是否生成LLVM IR进行深度分析
        #[arg(short, long)]
        deep: bool,
    },

    /// 生成并分析LLVM IR
    LlvmAnalyze {
        /// 项目路径
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// 优化级别 (0, 1, 2, 3)
        #[arg(short, long, default_value = "3")]
        opt_level: u8,

        /// 输出目录
        #[arg(short, long, default_value = "llvm_analysis")]
        output_dir: PathBuf,
    },

    /// 比较优化前后的性能差异
    Compare {
        /// 项目路径
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path, output, output_file, deep } => {
            println!("{}", "🔍 开始分析Rust代码...".green().bold());

            let analyzer = RustCodeAnalyzer::new(path.clone())?;
            let analysis_result = analyzer.analyze()?;

            if deep {
                println!("{}", "🔬 执行深度LLVM分析...".cyan().bold());
                let llvm_analyzer = LLVMAnalyzer::new(path)?;
                let llvm_result = llvm_analyzer.generate_and_analyze()?;

                let reporter = Reporter::new(analysis_result, Some(llvm_result));
                reporter.generate_report(&output, output_file.as_ref().map(|v| &**v))?;
            } else {
                let reporter = Reporter::new(analysis_result, None);
                reporter.generate_report(&output, output_file.as_ref().map(|v| &**v))?;
            }

            println!("{}", "✅ 分析完成!".green().bold());
        }

        Commands::LlvmAnalyze { path, opt_level, output_dir } => {
            println!("{}", "🔬 生成LLVM IR并分析...".cyan().bold());

            let llvm_analyzer = LLVMAnalyzer::new(path)?;
            llvm_analyzer.generate_ir(opt_level, &output_dir)?;
            let result = llvm_analyzer.analyze_ir(&output_dir)?;

            println!("\n{}", "=== LLVM分析结果 ===".yellow().bold());
            println!("{}", serde_json::to_string_pretty(&result)?);

            println!("{}", "✅ LLVM分析完成!".green().bold());
        }

        Commands::Compare { path } => {
            println!("{}", "📊 比较优化级别性能差异...".magenta().bold());

            let llvm_analyzer = LLVMAnalyzer::new(path)?;
            llvm_analyzer.compare_optimization_levels()?;

            println!("{}", "✅ 比较完成!".green().bold());
        }
    }

    Ok(())
}
