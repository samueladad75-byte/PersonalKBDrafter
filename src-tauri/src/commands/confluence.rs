use crate::db::DbPool;
use crate::error::AppError;
use crate::models::confluence::{ConfluenceSpace, PublishResult};
use crate::services::{confluence::ConfluenceClient, markdown_to_confluence, tokens};
use tauri::State;

/// Test Confluence connection
#[tauri::command]
pub async fn test_confluence_connection(
    base_url: String,
    pat: String,
) -> Result<bool, AppError> {
    let client = ConfluenceClient::new(base_url, pat);
    client.test_connection().await
}

/// Save Confluence configuration
#[tauri::command]
pub async fn save_confluence_config(
    _base_url: String,
    pat: String,
) -> Result<(), AppError> {
    // Store PAT in keychain
    tokens::store_token("confluence", &pat)?;

    // Store base URL in settings (we could use a settings table, but for now just return success)
    // The frontend will persist the base URL in localStorage

    Ok(())
}

/// Disconnect from Confluence
#[tauri::command]
pub async fn disconnect_confluence() -> Result<(), AppError> {
    tokens::delete_token("confluence")?;
    Ok(())
}

/// Get Confluence connection status
#[tauri::command]
pub async fn get_confluence_connection_status() -> Result<bool, AppError> {
    // Check if token exists
    match tokens::get_token("confluence") {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// List available Confluence spaces
#[tauri::command]
pub async fn list_confluence_spaces(confluence_url: String) -> Result<Vec<ConfluenceSpace>, AppError> {
    let pat = tokens::get_token("confluence")?;
    let client = ConfluenceClient::new(confluence_url, pat);
    client.list_spaces().await
}

/// Publish an article to Confluence
#[tauri::command]
pub async fn publish_article(
    article_id: i64,
    space_key: String,
    confluence_url: String,
    db: State<'_, DbPool>,
) -> Result<PublishResult, AppError> {
    let pat = tokens::get_token("confluence")?;

    // Get article from database
    let pool = db.inner().clone();
    let article = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let conn = pool.get()?;
        Ok(crate::db::articles::get_article(&conn, article_id)?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    // Convert markdown to Confluence XHTML
    let conversion_result = markdown_to_confluence::convert(&article.content_markdown)?;

    // Create page in Confluence
    let client = ConfluenceClient::new(confluence_url, pat);
    let publish_result = client
        .create_page(&space_key, &article.title, &conversion_result.xhtml, &article.tags)
        .await?;

    // Update article in database with publish info
    let pool2 = db.inner().clone();
    let page_id = publish_result.page_id.clone();
    let page_url = publish_result.url.clone();
    let space_key_clone = space_key.clone();

    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool2.get()?;
        conn.execute(
            "UPDATE kb_articles SET status = 'published', confluence_page_id = ?1, confluence_url = ?2, confluence_space_key = ?3 WHERE id = ?4",
            (page_id, page_url, space_key_clone, article_id),
        )?;
        Ok(())
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    Ok(publish_result)
}

/// Update an already-published article in Confluence
#[tauri::command]
pub async fn update_published_article(
    article_id: i64,
    confluence_url: String,
    db: State<'_, DbPool>,
) -> Result<PublishResult, AppError> {
    let pat = tokens::get_token("confluence")?;

    // Get article from database
    let pool = db.inner().clone();
    let article = tokio::task::spawn_blocking(move || -> Result<_, AppError> {
        let conn = pool.get()?;
        Ok(crate::db::articles::get_article(&conn, article_id)?)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    // Check if article has been published
    let page_id = article.confluence_page_id.ok_or_else(|| {
        AppError::Internal("Article has not been published yet".to_string())
    })?;

    // Convert markdown to Confluence XHTML
    let conversion_result = markdown_to_confluence::convert(&article.content_markdown)?;

    // Fetch current page version, then update
    let client = ConfluenceClient::new(confluence_url, pat);
    let current_version = client.get_page_version(&page_id).await?;
    let publish_result = client
        .update_page(&page_id, &article.title, &conversion_result.xhtml, current_version)
        .await?;

    // Update article URL in database
    let pool2 = db.inner().clone();
    let page_url = publish_result.url.clone();

    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let conn = pool2.get()?;
        conn.execute(
            "UPDATE kb_articles SET confluence_url = ?1 WHERE id = ?2",
            (page_url, article_id),
        )?;
        Ok(())
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task join error: {}", e)))??;

    Ok(publish_result)
}
