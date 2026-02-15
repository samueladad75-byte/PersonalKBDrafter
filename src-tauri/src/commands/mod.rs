pub mod articles;
pub mod confluence;
pub mod drafting;
pub mod jira;
pub mod templates;

#[tauri::command]
pub fn ping() -> String {
    "pong".to_string()
}

// Re-export commands for easy handler registration
pub use articles::*;
pub use confluence::*;
pub use drafting::*;
pub use jira::*;
pub use templates::*;
