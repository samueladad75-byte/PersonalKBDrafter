CREATE TABLE kb_articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ticket_key TEXT,
    title TEXT NOT NULL,
    problem TEXT NOT NULL,
    solution TEXT NOT NULL,
    expected_result TEXT,
    prerequisites TEXT,
    additional_notes TEXT,
    tags TEXT NOT NULL DEFAULT '[]',         -- JSON array
    content_markdown TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',    -- 'draft' | 'published'
    confluence_page_id TEXT,
    confluence_url TEXT,
    confluence_space_key TEXT,
    quality_score INTEGER,
    template_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE kb_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    system_prompt TEXT NOT NULL,
    output_structure TEXT NOT NULL,
    is_builtin INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE flagged_publishes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    article_id INTEGER NOT NULL REFERENCES kb_articles(id),
    pattern_type TEXT NOT NULL,
    matched_text TEXT NOT NULL,
    user_override INTEGER NOT NULL DEFAULT 0,
    flagged_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_articles_ticket ON kb_articles(ticket_key);
CREATE INDEX idx_articles_status ON kb_articles(status);

-- Built-in templates
INSERT INTO kb_templates (id, name, slug, description, system_prompt, output_structure, is_builtin)
VALUES
('tpl-troubleshoot', 'Troubleshooting', 'troubleshooting',
 'Problem/cause/resolution format for resolved support tickets',
 'You are a technical writer creating internal Knowledge Base articles from Jira support tickets.

OUTPUT FORMAT: Markdown with these sections:
# [Descriptive Title]
## Problem
[1-2 sentences describing the user-reported issue]
## Environment
[Bullet list: OS, software version, relevant config]
## Cause
[Root cause explanation, if known from the ticket]
## Resolution
[Numbered step-by-step instructions to resolve]
## Expected Result
[What the user should see after following the steps]
## Additional Notes
[Optional: workarounds, prevention tips, related issues]

RULES:
- Write for IT support audience (technical but not developer-level)
- Use active voice, present tense for instructions
- One action per numbered step
- Include exact menu paths, commands, or config values when in the ticket
- If info is missing from the ticket, write "[Not available in ticket - please add]"
- Do NOT invent information not in the source ticket
- Do NOT include customer names, emails, or identifying info
- Target: 200-500 words',
 '# {title}

## Problem

## Environment

## Cause

## Resolution

1.
2.
3.

## Expected Result

## Additional Notes
',
 1),

('tpl-howto', 'How-To Guide', 'how-to',
 'Step-by-step procedural guide for common tasks',
 'You are a technical writer creating a How-To guide from Jira ticket context.

OUTPUT FORMAT: Markdown with these sections:
# How to [Task Description]
## Overview
[1-2 sentences: what this guide covers and when to use it]
## Prerequisites
[Bullet list of requirements before starting]
## Steps
[Numbered instructions, one action per step]
## Verification
[How to confirm the task was completed successfully]
## Troubleshooting
[Common issues and their solutions]

RULES:
- Write for IT support audience
- Active voice, present tense
- Include exact commands, paths, and config values
- Each step should be independently verifiable
- Do NOT invent information not in the source
- Target: 200-500 words',
 '# How to {title}

## Overview

## Prerequisites

## Steps

1.
2.
3.

## Verification

## Troubleshooting
',
 1),

('tpl-known-issue', 'Known Issue', 'known-issue',
 'Document a known bug with workarounds',
 'You are a technical writer documenting a known issue from a Jira ticket.

OUTPUT FORMAT: Markdown with these sections:
# Known Issue: [Brief Description]
## Symptoms
[What the user observes]
## Affected Systems
[Which systems/versions are impacted]
## Workaround
[Steps to work around the issue until fixed]
## Status
[Current fix status: investigating / fix planned / fixed in version X]
## Related Tickets
[Reference to Jira ticket(s)]

RULES:
- Be specific about symptoms and affected versions
- Workaround steps must be actionable
- Do NOT speculate about fixes or timelines not in the ticket
- Target: 150-400 words',
 '# Known Issue: {title}

## Symptoms

## Affected Systems

## Workaround

## Status

## Related Tickets
',
 1),

('tpl-faq', 'FAQ Entry', 'faq',
 'Question and answer format for simple tickets',
 'You are a technical writer creating an FAQ entry from a Jira ticket.

OUTPUT FORMAT: Markdown with these sections:
# Q: [Question phrased from user perspective]
## Answer
[Clear, concise answer - 2-4 sentences]
## Details
[Optional: expanded explanation if needed]
## See Also
[Related articles or resources]

RULES:
- Question should be how a user would naturally ask it
- Answer should be self-contained (no "see above" references)
- Keep it brief - FAQs should be scannable
- Target: 50-200 words',
 '# Q: {title}

## Answer

## Details

## See Also
',
 1);
