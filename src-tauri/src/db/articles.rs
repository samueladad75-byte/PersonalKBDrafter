use crate::models::{Article, ArticleStatus, NewArticle};
use rusqlite::{params, Connection, Result as SqliteResult};

pub fn insert_article(conn: &Connection, article: &NewArticle) -> SqliteResult<i64> {
    let tags_json = serde_json::to_string(&article.tags).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        "INSERT INTO kb_articles (
            ticket_key, title, problem, solution, expected_result,
            prerequisites, additional_notes, tags, content_markdown, template_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            article.ticket_key,
            article.title,
            article.problem,
            article.solution,
            article.expected_result,
            article.prerequisites,
            article.additional_notes,
            tags_json,
            article.content_markdown,
            article.template_id,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_article(conn: &Connection, id: i64) -> SqliteResult<Article> {
    let mut stmt = conn.prepare(
        "SELECT id, ticket_key, title, problem, solution, expected_result,
                prerequisites, additional_notes, tags, content_markdown, status,
                confluence_page_id, confluence_url, confluence_space_key,
                quality_score, template_id, created_at, updated_at
         FROM kb_articles WHERE id = ?1",
    )?;

    stmt.query_row([id], |row| {
        let tags_json: String = row.get(8)?;
        let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
        let status_str: String = row.get(10)?;
        let status = ArticleStatus::from_str(&status_str).unwrap_or(ArticleStatus::Draft);

        Ok(Article {
            id: row.get(0)?,
            ticket_key: row.get(1)?,
            title: row.get(2)?,
            problem: row.get(3)?,
            solution: row.get(4)?,
            expected_result: row.get(5)?,
            prerequisites: row.get(6)?,
            additional_notes: row.get(7)?,
            tags,
            content_markdown: row.get(9)?,
            status,
            confluence_page_id: row.get(11)?,
            confluence_url: row.get(12)?,
            confluence_space_key: row.get(13)?,
            quality_score: row.get(14)?,
            template_id: row.get(15)?,
            created_at: row.get(16)?,
            updated_at: row.get(17)?,
        })
    })
}

pub fn list_articles(
    conn: &Connection,
    status_filter: Option<String>,
) -> SqliteResult<Vec<Article>> {
    let (query, params): (&str, Vec<&dyn rusqlite::ToSql>) = if let Some(ref status) = status_filter {
        (
            "SELECT id, ticket_key, title, problem, solution, expected_result,
                    prerequisites, additional_notes, tags, content_markdown, status,
                    confluence_page_id, confluence_url, confluence_space_key,
                    quality_score, template_id, created_at, updated_at
             FROM kb_articles WHERE status = ?1 ORDER BY updated_at DESC",
            vec![status as &dyn rusqlite::ToSql],
        )
    } else {
        (
            "SELECT id, ticket_key, title, problem, solution, expected_result,
                    prerequisites, additional_notes, tags, content_markdown, status,
                    confluence_page_id, confluence_url, confluence_space_key,
                    quality_score, template_id, created_at, updated_at
             FROM kb_articles ORDER BY updated_at DESC",
            vec![],
        )
    };

    let mut stmt = conn.prepare(query)?;
    let articles = stmt.query_map(params.as_slice(), |row| {
        let tags_json: String = row.get(8)?;
        let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
        let status_str: String = row.get(10)?;
        let status = ArticleStatus::from_str(&status_str).unwrap_or(ArticleStatus::Draft);

        Ok(Article {
            id: row.get(0)?,
            ticket_key: row.get(1)?,
            title: row.get(2)?,
            problem: row.get(3)?,
            solution: row.get(4)?,
            expected_result: row.get(5)?,
            prerequisites: row.get(6)?,
            additional_notes: row.get(7)?,
            tags,
            content_markdown: row.get(9)?,
            status,
            confluence_page_id: row.get(11)?,
            confluence_url: row.get(12)?,
            confluence_space_key: row.get(13)?,
            quality_score: row.get(14)?,
            template_id: row.get(15)?,
            created_at: row.get(16)?,
            updated_at: row.get(17)?,
        })
    })?;

    articles.collect()
}

pub fn update_article(conn: &Connection, id: i64, article: &NewArticle) -> SqliteResult<()> {
    let tags_json = serde_json::to_string(&article.tags).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        "UPDATE kb_articles SET
            ticket_key = ?1, title = ?2, problem = ?3, solution = ?4,
            expected_result = ?5, prerequisites = ?6, additional_notes = ?7,
            tags = ?8, content_markdown = ?9, template_id = ?10,
            updated_at = datetime('now')
         WHERE id = ?11",
        params![
            article.ticket_key,
            article.title,
            article.problem,
            article.solution,
            article.expected_result,
            article.prerequisites,
            article.additional_notes,
            tags_json,
            article.content_markdown,
            article.template_id,
            id,
        ],
    )?;

    Ok(())
}

pub fn delete_article(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM kb_articles WHERE id = ?1", [id])?;
    Ok(())
}

pub fn update_article_quality_score(conn: &Connection, id: i64, score: u8) -> SqliteResult<()> {
    conn.execute(
        "UPDATE kb_articles SET quality_score = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![score, id],
    )?;
    Ok(())
}
