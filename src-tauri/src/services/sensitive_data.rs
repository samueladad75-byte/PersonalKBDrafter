use crate::models::FlaggedSection;
use regex::Regex;

pub fn scan(text: &str) -> Vec<FlaggedSection> {
    let mut flags = Vec::new();

    // Pattern definitions with severity
    let patterns = vec![
        (
            r"AKIA[0-9A-Z]{16}",
            "aws_key",
            "high",
            "AWS Access Key detected",
        ),
        (
            r"(?i)(password|passwd|pwd|secret|api[_-]?key|token)\s*[:=]\s*\S+",
            "credentials",
            "high",
            "Password or secret detected",
        ),
        (
            r"\b(10\.\d{1,3}\.\d{1,3}\.\d{1,3}|172\.(1[6-9]|2\d|3[01])\.\d{1,3}\.\d{1,3}|192\.168\.\d{1,3}\.\d{1,3})\b",
            "internal_ip",
            "medium",
            "Internal IP address detected",
        ),
        (
            r"-----BEGIN (RSA|DSA|EC|OPENSSH) PRIVATE KEY-----",
            "ssh_key",
            "high",
            "SSH private key detected",
        ),
        (
            r"(?i)(jdbc|mongodb|postgres|mysql)://[^\s]+",
            "connection_string",
            "high",
            "Database connection string detected",
        ),
    ];

    for (pattern_str, pattern_type, severity, description) in patterns {
        let re = Regex::new(pattern_str).unwrap();
        for (line_num, line) in text.lines().enumerate() {
            if let Some(mat) = re.find(line) {
                let matched_text = mat.as_str();
                // Truncate to 50 chars
                let truncated = if matched_text.len() > 50 {
                    format!("{}...", &matched_text[..47])
                } else {
                    matched_text.to_string()
                };

                // Log description for debugging
                log::debug!("Sensitive data detected: {} at line {}", description, line_num + 1);

                flags.push(FlaggedSection {
                    pattern_type: pattern_type.to_string(),
                    severity: severity.to_string(),
                    matched_text: truncated,
                    line_number: line_num + 1, // 1-indexed for user display
                    start_col: mat.start(),
                    end_col: mat.end(),
                });
            }
        }
    }

    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_key_detection() {
        let text = "My key is AKIAIOSFODNN7EXAMPLE and here it is.";
        let flags = scan(text);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0].pattern_type, "aws_key");
        assert_eq!(flags[0].severity, "high");
    }

    #[test]
    fn test_password_detection() {
        let text = "password: mySecretPass123";
        let flags = scan(text);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0].pattern_type, "credentials");
    }

    #[test]
    fn test_internal_ip_detection() {
        let text = "Connect to 192.168.1.100 for access";
        let flags = scan(text);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0].pattern_type, "internal_ip");
        assert_eq!(flags[0].severity, "medium");
    }

    #[test]
    fn test_ssh_key_detection() {
        let text = "-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQEA...";
        let flags = scan(text);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0].pattern_type, "ssh_key");
    }

    #[test]
    fn test_no_false_positives_on_normal_text() {
        let text = "This is a normal article about troubleshooting login issues.";
        let flags = scan(text);
        assert_eq!(flags.len(), 0);
    }

    #[test]
    fn test_truncation_of_long_matches() {
        let long_text = format!("password: {}", "a".repeat(100));
        let flags = scan(&long_text);
        assert_eq!(flags.len(), 1);
        assert!(flags[0].matched_text.len() <= 53); // 50 + "..."
        assert!(flags[0].matched_text.ends_with("..."));
    }
}
