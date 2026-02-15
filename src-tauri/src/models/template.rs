use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct Template {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub system_prompt: String,
    pub output_structure: String,
    pub is_builtin: bool,
    pub created_at: String,
}
