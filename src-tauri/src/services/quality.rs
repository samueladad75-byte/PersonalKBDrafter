use crate::models::{NewArticle, QualityScore};

pub fn score(article: &NewArticle) -> QualityScore {
    let mut overall: u8 = 0;
    let mut warnings = Vec::new();

    // Title check (20 points)
    let has_title = !article.title.trim().is_empty() && article.title.len() > 5;
    if has_title {
        overall += 20;
    }
    if article.title.len() > 200 {
        warnings.push("Title is very long (>200 chars)".to_string());
    }

    // Problem check (20 points)
    let has_problem = !article.problem.trim().is_empty() && article.problem.len() > 20;
    if has_problem {
        overall += 20;
    }

    // Solution check (25 points)
    let has_solution = !article.solution.trim().is_empty() && article.solution.len() > 50;
    if has_solution {
        overall += 25;
    }
    if article.solution.len() < 100 {
        warnings.push("Solution is very short (<100 chars)".to_string());
    }

    // Expected result check (15 points)
    let has_expected_result = article
        .expected_result
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    if has_expected_result {
        overall += 15;
    }

    // Prerequisites check (10 points)
    let has_prerequisites = article
        .prerequisites
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    if has_prerequisites {
        overall += 10;
    }

    // Additional notes check (5 points)
    if article
        .additional_notes
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
    {
        overall += 5;
    }

    // Bonus: Count solution steps (numbered lists)
    let solution_step_count = count_numbered_steps(&article.solution);
    if solution_step_count >= 3 {
        overall = overall.saturating_add(5).min(100);
    }

    // Check for code blocks
    if !article.content_markdown.contains("```") && !article.content_markdown.contains("`") {
        warnings.push("No code blocks detected".to_string());
    }

    // Word count
    let word_count = article.content_markdown.split_whitespace().count();

    QualityScore {
        overall,
        has_title,
        has_problem,
        has_solution,
        has_expected_result,
        has_prerequisites,
        solution_step_count,
        word_count,
        warnings,
    }
}

fn count_numbered_steps(text: &str) -> usize {
    use regex::Regex;
    let re = Regex::new(r"^\s*\d+\.").unwrap();
    text.lines()
        .filter(|line| re.is_match(line))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_complete_article() {
        let article = NewArticle {
            ticket_key: Some("TEST-123".to_string()),
            title: "Fix Login Issue".to_string(),
            problem: "Users cannot log in due to timeout error".to_string(),
            solution: "1. Clear browser cache\n2. Restart the application\n3. Try logging in again\n\nThe solution involves multiple steps to resolve the timeout.".to_string(),
            expected_result: Some("User can log in successfully".to_string()),
            prerequisites: Some("Admin access required".to_string()),
            additional_notes: Some("This is a known issue".to_string()),
            tags: vec!["login".to_string(), "timeout".to_string()],
            content_markdown: "# Fix Login Issue\n\n## Problem\nUsers cannot log in\n\n## Solution\n1. Clear cache\n2. Restart\n3. Login\n\n```bash\nrm -rf ~/.cache\n```".to_string(),
            template_id: Some("tpl-troubleshoot".to_string()),
        };

        let score = score(&article);
        assert_eq!(score.overall, 100); // Should get max score
        assert!(score.has_title);
        assert!(score.has_problem);
        assert!(score.has_solution);
        assert!(score.has_expected_result);
        assert!(score.has_prerequisites);
        assert_eq!(score.solution_step_count, 3);
    }

    #[test]
    fn test_score_minimal_article() {
        let article = NewArticle {
            ticket_key: None,
            title: "Short".to_string(),
            problem: "Prob".to_string(),
            solution: "Sol".to_string(),
            expected_result: None,
            prerequisites: None,
            additional_notes: None,
            tags: vec![],
            content_markdown: "Short".to_string(),
            template_id: None,
        };

        let score = score(&article);
        assert!(score.overall < 50); // Should get low score
        assert!(!score.has_title); // Title too short
        assert!(!score.has_problem); // Problem too short
        assert!(!score.has_solution); // Solution too short
    }
}
