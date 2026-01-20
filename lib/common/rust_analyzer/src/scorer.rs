use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationScore {
    pub overall: f32,
    pub vectorization: f32,
    pub memory: f32,
    pub inlining: f32,
    pub cache: f32,
}

impl OptimizationScore {
    pub fn new() -> Self {
        Self {
            overall: 0.0,
            vectorization: 0.0,
            memory: 0.0,
            inlining: 0.0,
            cache: 0.0,
        }
    }

    pub fn grade(&self) -> &'static str {
        match self.overall {
            x if x >= 90.0 => "A (优秀)",
            x if x >= 80.0 => "B (良好)",
            x if x >= 70.0 => "C (中等)",
            x if x >= 60.0 => "D (及格)",
            _ => "F (需改进)",
        }
    }

    pub fn estimated_speedup(&self) -> f32 {
        // 简化的加速比估算模型
        // 假设每10分对应10%的性能提升
        1.0 + ((100.0 - self.overall) / 100.0)
    }

    pub fn optimization_potential(&self) -> f32 {
        // 优化潜力：当前与最佳状态的差距
        100.0 - self.overall
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedScore {
    pub score: OptimizationScore,
    pub categories: Vec<CategoryScore>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScore {
    pub name: String,
    pub score: f32,
    pub weight: f32,
    pub details: Vec<String>,
}

impl CategoryScore {
    pub fn new(name: impl Into<String>, score: f32, weight: f32) -> Self {
        Self {
            name: name.into(),
            score,
            weight,
            details: Vec::new(),
        }
    }

    pub fn add_detail(&mut self, detail: impl Into<String>) {
        self.details.push(detail.into());
    }
}
