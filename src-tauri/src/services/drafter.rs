use crate::error::AppError;
use crate::models::jira::JiraTicket;
use crate::models::template::Template;
use crate::services::ollama;
use regex::Regex;

/// Build prompts for LLM from ticket and template
pub fn build_prompt(ticket: &JiraTicket, template: &Template) -> (String, String) {
    let system_prompt = template.system_prompt.clone();

    // Get last comment as resolution note
    let resolution_note = if !ticket.comments.is_empty() {
        &ticket.comments[ticket.comments.len() - 1].body
    } else {
        "[No resolution note found]"
    };

    // Build comments list
    let comments_text = if ticket.comments.is_empty() {
        "[No comments in ticket]".to_string()
    } else {
        ticket
            .comments
            .iter()
            .map(|c| format!("[{} - {}]: {}", c.author, c.created, c.body))
            .collect::<Vec<_>>()
            .join("\n\n")
    };

    let user_prompt = format!(
        r#"Convert this Jira ticket into a KB article.

TICKET: {}
SUMMARY: {}
DESCRIPTION:
{}

COMMENTS (chronological):
{}

RESOLUTION: {}
STATUS: {}
LABELS: {}
COMPONENTS: {}
"#,
        ticket.key,
        ticket.summary,
        ticket.description.as_deref().unwrap_or("[No description]"),
        comments_text,
        resolution_note,
        ticket.status,
        ticket.labels.join(", "),
        ticket.components.join(", ")
    );

    (system_prompt, user_prompt)
}

/// Post-process LLM output to clean up common issues
pub fn post_process(raw: &str) -> String {
    let mut cleaned = raw.to_string();

    // Remove common preambles (first line only if it matches)
    let preamble_re = Regex::new(r"(?i)^(here'?s?|i'?ve|sure|certainly|of course).*?\n").unwrap();
    if let Some(mat) = preamble_re.find(&cleaned) {
        if mat.start() == 0 {
            cleaned = cleaned[mat.end()..].to_string();
        }
    }

    // Remove trailing sign-offs
    let signoff_re =
        Regex::new(r"(?i)\n(let me know|feel free|i hope|is there anything).*$").unwrap();
    cleaned = signoff_re.replace(&cleaned, "").to_string();

    // Ensure code blocks are properly closed
    // Count triple backticks by checking if they appear in pairs
    let backtick_positions: Vec<_> = cleaned.match_indices("```").collect();
    if backtick_positions.len() % 2 != 0 {
        // Odd number means unclosed code block
        cleaned.push_str("\n```\n");
    }

    // Trim excess whitespace
    cleaned.trim().to_string()
}

/// Draft an article from a Jira ticket using LLM
pub async fn draft(
    ticket: &JiraTicket,
    template: &Template,
    ollama_url: &str,
    model: &str,
) -> Result<String, AppError> {
    let (system_prompt, user_prompt) = build_prompt(ticket, template);

    let raw_output = ollama::generate(ollama_url, model, &system_prompt, &user_prompt).await?;

    let cleaned = post_process(&raw_output);

    // Sanity check: if output is suspiciously short, it might be a refusal
    if cleaned.len() < 50 {
        return Err(AppError::Internal(
            "Generated article seems incomplete (< 50 chars). Try again or edit manually."
                .to_string(),
        ));
    }

    Ok(cleaned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_prompt() {
        use crate::models::jira::JiraComment;

        let ticket = JiraTicket {
            key: "TEST-123".to_string(),
            summary: "Login fails with 500 error".to_string(),
            description: Some("Users report 500 errors when logging in".to_string()),
            status: "Resolved".to_string(),
            priority: Some("High".to_string()),
            resolution: Some("Fixed".to_string()),
            labels: vec!["authentication".to_string(), "bug".to_string()],
            components: vec!["API".to_string()],
            comments: vec![
                JiraComment {
                    author: "Alice".to_string(),
                    body: "Investigating the logs".to_string(),
                    created: "2024-01-01T10:00:00".to_string(),
                },
                JiraComment {
                    author: "Bob".to_string(),
                    body: "Fixed by updating auth token validation".to_string(),
                    created: "2024-01-01T11:00:00".to_string(),
                },
            ],
            created: "2024-01-01T09:00:00".to_string(),
            updated: "2024-01-01T12:00:00".to_string(),
        };

        let template = Template {
            id: "test".to_string(),
            name: "Test".to_string(),
            slug: "test".to_string(),
            description: "Test template".to_string(),
            system_prompt: "You are a technical writer.".to_string(),
            output_structure: "# Title\n## Problem\n## Solution".to_string(),
            is_builtin: false,
            created_at: "2024-01-01".to_string(),
        };

        let (system, user) = build_prompt(&ticket, &template);

        assert_eq!(system, "You are a technical writer.");
        assert!(user.contains("TEST-123"));
        assert!(user.contains("Login fails with 500 error"));
        assert!(user.contains("Fixed by updating auth token validation"));
        assert!(user.contains("authentication, bug"));
        assert!(user.contains("API"));
    }

    #[test]
    fn test_post_process_removes_preamble() {
        let input = "Here's a draft KB article:\n\n# Title\n\nContent here";
        let output = post_process(input);
        assert!(!output.starts_with("Here's"));
        assert!(output.starts_with("# Title"));
    }

    #[test]
    fn test_post_process_removes_signoff() {
        let input = "# Title\n\nContent here\n\nLet me know if you need anything else!";
        let output = post_process(input);
        assert!(!output.contains("Let me know"));
    }

    #[test]
    fn test_post_process_closes_code_blocks() {
        let input = "# Title\n\n```python\nprint('hello')";
        let output = post_process(input);
        assert!(output.matches("```").count() == 2);
    }

    #[test]
    fn test_post_process_preserves_valid_code_blocks() {
        let input = "# Title\n\n```python\nprint('hello')\n```\n\nMore text";
        let output = post_process(input);
        assert!(output.matches("```").count() == 2);
        assert!(output.contains("More text"));
    }
}
