use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct JiraTicket {
    pub key: String,
    pub summary: String,
    #[ts(optional)]
    pub description: Option<String>,
    pub status: String,
    #[ts(optional)]
    pub priority: Option<String>,
    #[ts(optional)]
    pub resolution: Option<String>,
    pub labels: Vec<String>,
    pub components: Vec<String>,
    pub comments: Vec<JiraComment>,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/bindings/")]
pub struct JiraComment {
    pub author: String,
    pub body: String,
    pub created: String,
}
