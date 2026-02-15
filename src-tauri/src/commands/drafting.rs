use crate::db::DbPool;
use crate::error::AppError;
use crate::models::jira::JiraTicket;
use crate::services::{drafter, ollama};
use tauri::State;

/// Check if Ollama is available at the configured URL
#[tauri::command]
pub async fn check_ollama_status(ollama_url: String) -> Result<bool, AppError> {
    ollama::check_health(&ollama_url).await
}

/// Draft an article from a Jira ticket using LLM
#[tauri::command]
pub async fn draft_with_llm(
    ticket: JiraTicket,
    template_id: String,
    ollama_url: String,
    model: String,
    db: State<'_, DbPool>,
) -> Result<String, AppError> {
    // Get the template from the database
    let pool = db.inner().clone();
    let template = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let conn = pool.get()?;
        Ok(crate::db::templates::get_template(&conn, &template_id)?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    // Generate the article
    let markdown = drafter::draft(&ticket, &template, &ollama_url, &model).await?;

    Ok(markdown)
}
