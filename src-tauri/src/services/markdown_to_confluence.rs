use crate::error::AppError;
use crate::models::confluence::ConversionResult;
use pulldown_cmark::{Event, Parser, Tag, TagEnd, CodeBlockKind, HeadingLevel};

/// Convert markdown to Confluence storage format (XHTML)
pub fn convert(markdown: &str) -> Result<ConversionResult, AppError> {
    let parser = Parser::new(markdown);
    let mut output = String::new();
    let mut warnings = Vec::new();
    let mut list_stack: Vec<ListType> = Vec::new();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => output.push_str("<p>"),
                Tag::Heading { level, .. } => {
                    let level_num = heading_level_to_number(level);
                    output.push_str(&format!("<h{}>", level_num));
                }
                Tag::BlockQuote(_) => output.push_str("<blockquote>"),
                Tag::CodeBlock(kind) => {
                    let lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => "plain".to_string(),
                    };
                    output.push_str(&format!(
                        r#"<ac:structured-macro ac:name="code"><ac:parameter ac:name="language">{}</ac:parameter><ac:plain-text-body><![CDATA["#,
                        escape_xml(&lang)
                    ));
                }
                Tag::List(None) => {
                    output.push_str("<ul>");
                    list_stack.push(ListType::Unordered);
                }
                Tag::List(Some(_)) => {
                    output.push_str("<ol>");
                    list_stack.push(ListType::Ordered);
                }
                Tag::Item => output.push_str("<li>"),
                Tag::Strong => output.push_str("<strong>"),
                Tag::Emphasis => output.push_str("<em>"),
                Tag::Link { dest_url, .. } => {
                    output.push_str(&format!(r#"<a href="{}">"#, escape_xml(&dest_url)));
                }
                Tag::Image { .. } => {
                    warnings.push("Images are not supported in v1 - will be omitted".to_string());
                }
                Tag::Table(_) => {
                    warnings.push("Tables are not supported in v1 - content will be rendered as text".to_string());
                }
                Tag::Strikethrough => output.push_str("<del>"),
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Paragraph => output.push_str("</p>\n"),
                TagEnd::Heading(level) => {
                    let level_num = heading_level_to_number(level);
                    output.push_str(&format!("</h{}>\n", level_num));
                }
                TagEnd::BlockQuote(_) => output.push_str("</blockquote>\n"),
                TagEnd::CodeBlock => {
                    output.push_str("]]></ac:plain-text-body></ac:structured-macro>\n");
                }
                TagEnd::List(_) => {
                    if let Some(list_type) = list_stack.pop() {
                        match list_type {
                            ListType::Ordered => output.push_str("</ol>\n"),
                            ListType::Unordered => output.push_str("</ul>\n"),
                        }
                    }
                }
                TagEnd::Item => output.push_str("</li>"),
                TagEnd::Strong => output.push_str("</strong>"),
                TagEnd::Emphasis => output.push_str("</em>"),
                TagEnd::Link => output.push_str("</a>"),
                TagEnd::Strikethrough => output.push_str("</del>"),
                _ => {}
            },
            Event::Text(text) => {
                output.push_str(&escape_xml(&text));
            }
            Event::Code(code) => {
                output.push_str(&format!("<code>{}</code>", escape_xml(&code)));
            }
            Event::SoftBreak => output.push(' '),
            Event::HardBreak => output.push_str("<br/>"),
            Event::Rule => output.push_str("<hr/>\n"),
            Event::TaskListMarker(_) => {
                warnings.push("Task lists are not supported in v1 - checkboxes will be omitted".to_string());
            }
            _ => {}
        }
    }

    // Deduplicate warnings
    warnings.sort();
    warnings.dedup();

    Ok(ConversionResult {
        xhtml: output,
        warnings,
    })
}

#[derive(Debug)]
enum ListType {
    Ordered,
    Unordered,
}

/// Convert HeadingLevel to numeric value
fn heading_level_to_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Escape XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headings() {
        let md = "# H1\n## H2\n### H3";
        let result = convert(md).unwrap();
        println!("Generated XHTML:\n{}", result.xhtml);
        assert!(result.xhtml.contains("<h1>H1</h1>"));
        assert!(result.xhtml.contains("<h2>H2</h2>"));
        assert!(result.xhtml.contains("<h3>H3</h3>"));
    }

    #[test]
    fn test_bold_italic() {
        let md = "**bold** and *italic*";
        let result = convert(md).unwrap();
        assert!(result.xhtml.contains("<strong>bold</strong>"));
        assert!(result.xhtml.contains("<em>italic</em>"));
    }

    #[test]
    fn test_inline_code() {
        let md = "Some `inline code` here";
        let result = convert(md).unwrap();
        assert!(result.xhtml.contains("<code>inline code</code>"));
    }

    #[test]
    fn test_code_block() {
        let md = "```python\nprint('hello')\n```";
        let result = convert(md).unwrap();
        println!("Code block XHTML:\n{}", result.xhtml);
        assert!(result.xhtml.contains(r#"<ac:structured-macro ac:name="code">"#));
        assert!(result.xhtml.contains(r#"<ac:parameter ac:name="language">python</ac:parameter>"#));
        assert!(result.xhtml.contains("print('hello')") || result.xhtml.contains("print(&apos;hello&apos;)"));
    }

    #[test]
    fn test_lists() {
        let md = "- Item 1\n- Item 2\n\n1. Numbered 1\n2. Numbered 2";
        let result = convert(md).unwrap();
        assert!(result.xhtml.contains("<ul>"));
        assert!(result.xhtml.contains("<li>Item 1</li>"));
        assert!(result.xhtml.contains("</ul>"));
        assert!(result.xhtml.contains("<ol>"));
        assert!(result.xhtml.contains("<li>Numbered 1</li>"));
        assert!(result.xhtml.contains("</ol>"));
    }

    #[test]
    fn test_links() {
        let md = "[Click here](https://example.com)";
        let result = convert(md).unwrap();
        assert!(result.xhtml.contains(r#"<a href="https://example.com">Click here</a>"#));
    }

    #[test]
    fn test_blockquote() {
        let md = "> This is a quote";
        let result = convert(md).unwrap();
        assert!(result.xhtml.contains("<blockquote>"));
        assert!(result.xhtml.contains("This is a quote"));
        assert!(result.xhtml.contains("</blockquote>"));
    }

    #[test]
    fn test_xml_escaping() {
        // Test escaping in text content and inline code
        let md = "Text with & \"quotes\" and `code <with> tags`";
        let result = convert(md).unwrap();
        println!("XML escaped XHTML:\n{}", result.xhtml);
        // Ampersand should be escaped
        assert!(result.xhtml.contains("&amp;"));
        // Quotes should be escaped
        assert!(result.xhtml.contains("&quot;"));
        // In inline code, < and > should be escaped
        assert!(result.xhtml.contains("&lt;with&gt;"));
    }

    #[test]
    fn test_horizontal_rule() {
        let md = "Before\n\n---\n\nAfter";
        let result = convert(md).unwrap();
        assert!(result.xhtml.contains("<hr/>"));
    }
}
