use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BddMetadata {
    pub feature: String,
    pub scenario: String,
    pub given: Vec<String>,
    pub when: String,
    pub then: Vec<String>,
    pub tags: Vec<String>,
    pub priority: u8,
}

impl Default for BddMetadata {
    fn default() -> Self {
        Self {
            feature: String::new(),
            scenario: String::new(),
            given: Vec::new(),
            when: String::new(),
            then: Vec::new(),
            tags: Vec::new(),
            priority: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BddResult {
    pub feature: String,
    pub scenario: String,
    pub given: Vec<String>,
    pub when: String,
    pub then: Vec<String>,
    pub passed: bool,
    pub duration_ms: u64,
    pub error: Option<String>,
}

impl BddResult {
    pub fn new(
        given: &[String],
        when: &str,
        then: &[String],
        passed: bool,
        duration_ms: u64,
    ) -> Self {
        Self {
            feature: String::new(),
            scenario: String::new(),
            given: given.to_vec(),
            when: when.to_string(),
            then: then.to_vec(),
            passed,
            duration_ms,
            error: None,
        }
    }

    pub fn with_error(
        given: &[String],
        when: &str,
        then: &[String],
        error: String,
        duration_ms: u64,
    ) -> Self {
        Self {
            feature: String::new(),
            scenario: String::new(),
            given: given.to_vec(),
            when: when.to_string(),
            then: then.to_vec(),
            passed: false,
            duration_ms,
            error: Some(error),
        }
    }

    pub fn set_context(&mut self, feature: &str, scenario: &str) {
        self.feature = feature.to_string();
        self.scenario = scenario.to_string();
    }
}

pub fn generate_report(results: &[BddResult]) -> String {
    let mut report = String::new();
    report.push_str("╔══════════════════════════════════════════════════════╗\n");
    report.push_str("║              BDD Test Report                        ║\n");
    report.push_str("╠══════════════════════════════════════════════════════╣\n");

    let mut passed = 0;
    let mut failed = 0;

    for result in results {
        if result.passed {
            passed += 1;
            report.push_str(&format!(
                "║ ✓ {}: {}                       ║\n",
                result.feature, result.scenario
            ));
        } else {
            failed += 1;
            report.push_str(&format!(
                "║ ✗ {}: {}                       ║\n",
                result.feature, result.scenario
            ));
        }
    }

    report.push_str("╠══════════════════════════════════════════════════════╣\n");
    report.push_str(&format!(
        "║  Total: {} | Passed: {} | Failed: {}               ║\n",
        results.len(),
        passed,
        failed
    ));
    report.push_str("╚══════════════════════════════════════════════════════╝\n");

    report
}
