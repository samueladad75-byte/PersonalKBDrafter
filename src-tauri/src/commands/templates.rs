use crate::db::{templates, DbPool};
use crate::error::AppError;
use crate::models::Template;
use tauri::State;

#[tauri::command]
pub async fn list_templates(db: State<'_, DbPool>) -> Result<Vec<Template>, AppError> {
    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<Vec<Template>, AppError> {
        let conn = pool.get()?;
        Ok(templates::list_templates(&conn)?)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

#[tauri::command]
pub async fn get_template(id: String, db: State<'_, DbPool>) -> Result<Template, AppError> {
    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<Template, AppError> {
        let conn = pool.get()?;
        Ok(templates::get_template(&conn, &id)?)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}
