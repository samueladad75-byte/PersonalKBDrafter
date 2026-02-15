use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct QualityScore {
    pub overall: u8,
    pub has_title: bool,
    pub has_problem: bool,
    pub has_solution: bool,
    pub has_expected_result: bool,
    pub has_prerequisites: bool,
    pub solution_step_count: usize,
    pub word_count: usize,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct FlaggedSection {
    pub pattern_type: String,
    pub severity: String,
    pub matched_text: String,
    pub line_number: usize,
    pub start_col: usize,
    pub end_col: usize,
}
