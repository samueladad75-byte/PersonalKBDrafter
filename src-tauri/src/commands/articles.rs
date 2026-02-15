use crate::db::{articles, DbPool};
use crate::error::AppError;
use crate::models::{Article, NewArticle, QualityScore};
use crate::services::{quality, sensitive_data};
use tauri::State;

#[tauri::command]
pub async fn save_draft(article: NewArticle, db: State<'_, DbPool>) -> Result<Article, AppError> {
    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<Article, AppError> {
        let conn = pool.get()?;
        let id = articles::insert_article(&conn, &article)?;
        Ok(articles::get_article(&conn, id)?)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

#[tauri::command]
pub async fn get_article(id: i64, db: State<'_, DbPool>) -> Result<Article, AppError> {
    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<Article, AppError> {
        let conn = pool.get()?;
        Ok(articles::get_article(&conn, id)?)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

#[tauri::command]
pub async fn list_articles(
    status: Option<String>,
    db: State<'_, DbPool>,
) -> Result<Vec<Article>, AppError> {
    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<Vec<Article>, AppError> {
        let conn = pool.get()?;
        Ok(articles::list_articles(&conn, status)?)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

#[tauri::command]
pub async fn delete_draft(id: i64, db: State<'_, DbPool>) -> Result<(), AppError> {
    let pool = db.inner().clone();
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool.get()?;
        Ok(articles::delete_article(&conn, id)?)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

#[tauri::command]
pub async fn export_markdown(
    id: i64,
    path: String,
    db: State<'_, DbPool>,
) -> Result<(), AppError> {
    let pool = db.inner().clone();
    let article = tokio::task::spawn_blocking(move || -> Result<Article, AppError> {
        let conn = pool.get()?;
        Ok(articles::get_article(&conn, id)?)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))??;

    // Write markdown to file
    std::fs::write(&path, &article.content_markdown)
        .map_err(|e| AppError::Internal(format!("Failed to write file: {}", e)))?;

    Ok(())
}

#[tauri::command]
pub async fn score_quality(article: NewArticle) -> Result<QualityScore, AppError> {
    Ok(quality::score(&article))
}

#[tauri::command]
pub async fn scan_sensitive_data(content: String) -> Result<Vec<crate::models::FlaggedSection>, AppError> {
    Ok(sensitive_data::scan(&content))
}
