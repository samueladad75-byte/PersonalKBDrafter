use crate::models::Template;
use rusqlite::{Connection, Result as SqliteResult};

pub fn list_templates(conn: &Connection) -> SqliteResult<Vec<Template>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, slug, description, system_prompt, output_structure,
                is_builtin, created_at
         FROM kb_templates ORDER BY is_builtin DESC, name ASC",
    )?;

    let templates = stmt.query_map([], |row| {
        Ok(Template {
            id: row.get(0)?,
            name: row.get(1)?,
            slug: row.get(2)?,
            description: row.get(3)?,
            system_prompt: row.get(4)?,
            output_structure: row.get(5)?,
            is_builtin: row.get::<_, i32>(6)? != 0,
            created_at: row.get(7)?,
        })
    })?;

    templates.collect()
}

pub fn get_template(conn: &Connection, id: &str) -> SqliteResult<Template> {
    let mut stmt = conn.prepare(
        "SELECT id, name, slug, description, system_prompt, output_structure,
                is_builtin, created_at
         FROM kb_templates WHERE id = ?1",
    )?;

    stmt.query_row([id], |row| {
        Ok(Template {
            id: row.get(0)?,
            name: row.get(1)?,
            slug: row.get(2)?,
            description: row.get(3)?,
            system_prompt: row.get(4)?,
            output_structure: row.get(5)?,
            is_builtin: row.get::<_, i32>(6)? != 0,
            created_at: row.get(7)?,
        })
    })
}
