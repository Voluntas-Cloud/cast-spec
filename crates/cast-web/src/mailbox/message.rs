//! Mailbox `Message` shape.
//!
//! Each numbered markdown file is one message: a YAML frontmatter
//! header delimited by `---` lines, followed by a freeform markdown
//! body. The body may embed structured payloads as fenced code
//! blocks with a non-standard info-string of the form `json <tag>`,
//! e.g. ```` ```json draft ```` — `tag` is open-ended; renderers
//! fall back to plain JSON if they don't know it.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub id: String,
    pub role: Role,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,
    pub created: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct JsonPayload {
    pub tag: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    #[serde(flatten)]
    pub frontmatter: Frontmatter,
    pub body: String,
    pub payloads: Vec<JsonPayload>,
}

impl Message {
    /// Parse a single mailbox file's contents.
    pub fn parse(text: &str) -> Result<Self> {
        let (fm_yaml, body) = split_frontmatter(text)?;
        let frontmatter: Frontmatter =
            serde_yaml::from_str(fm_yaml).context("parse frontmatter")?;
        let payloads = extract_json_payloads(body);
        Ok(Self {
            frontmatter,
            body: body.to_string(),
            payloads,
        })
    }

    /// Render to the on-disk format. Inverse of [`parse`].
    pub fn render(&self) -> Result<String> {
        let fm = serde_yaml::to_string(&self.frontmatter).context("emit frontmatter")?;
        Ok(format!("---\n{fm}---\n\n{}", self.body))
    }
}

fn split_frontmatter(text: &str) -> Result<(&str, &str)> {
    let rest = text.strip_prefix("---\n").or_else(|| text.strip_prefix("---\r\n"));
    let Some(rest) = rest else {
        bail!("missing leading `---` frontmatter delimiter");
    };
    // Find a closing `---` on its own line.
    let Some(end) = find_closing_delimiter(rest) else {
        bail!("missing closing `---` frontmatter delimiter");
    };
    let yaml = &rest[..end.start];
    let body = rest[end.end..].trim_start_matches('\n').trim_start_matches("\r\n");
    Ok((yaml, body))
}

struct Span {
    start: usize,
    end: usize,
}

fn find_closing_delimiter(s: &str) -> Option<Span> {
    // Scan for "\n---\n" or "\n---\r\n" or end-anchored "---\n".
    for (i, _) in s.match_indices('\n') {
        let after = &s[i + 1..];
        if let Some(rest) = after.strip_prefix("---") {
            if rest.starts_with('\n') {
                return Some(Span { start: i + 1, end: i + 1 + 3 + 1 });
            }
            if rest.starts_with("\r\n") {
                return Some(Span { start: i + 1, end: i + 1 + 3 + 2 });
            }
        }
    }
    None
}

/// Find ```json <tag> ... ``` blocks in the body. Tag is the second
/// info-string token; if missing, the payload is tagged as "json".
fn extract_json_payloads(body: &str) -> Vec<JsonPayload> {
    let mut out = Vec::new();
    let mut lines = body.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("```") else {
            continue;
        };
        let info = rest.trim();
        let mut tokens = info.split_whitespace();
        let Some(first) = tokens.next() else {
            continue;
        };
        if !first.eq_ignore_ascii_case("json") {
            continue;
        }
        let tag = tokens.next().unwrap_or("json").to_string();

        let mut payload = String::new();
        let mut closed = false;
        for inner in lines.by_ref() {
            if inner.trim_start().starts_with("```") {
                closed = true;
                break;
            }
            payload.push_str(inner);
            payload.push('\n');
        }
        if !closed {
            break;
        }
        if let Ok(value) = serde_json::from_str::<Value>(&payload) {
            out.push(JsonPayload { tag, value });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_minimal() {
        let src = indoc! {r#"
            ---
            id: 001-prompt
            role: user
            created: 2026-05-01T00:00:00Z
            ---

            Hello, world.
        "#};
        let m = Message::parse(src).unwrap();
        assert_eq!(m.frontmatter.id, "001-prompt");
        assert_eq!(m.frontmatter.role, Role::User);
        assert!(m.body.contains("Hello, world."));
        assert!(m.payloads.is_empty());
    }

    #[test]
    fn parse_with_draft_payload() {
        let src = indoc! {r#"
            ---
            id: 003-draft
            role: user
            in_reply_to: 002-reply
            created: 2026-05-01T00:00:00Z
            ---

            Here is a draft:

            ```json draft
            { "intent": "x", "payload": {"a": 1} }
            ```

            And some trailing prose.
        "#};
        let m = Message::parse(src).unwrap();
        assert_eq!(m.payloads.len(), 1);
        assert_eq!(m.payloads[0].tag, "draft");
        assert_eq!(m.payloads[0].value["intent"], "x");
    }

    #[test]
    fn render_round_trips() {
        let src = indoc! {r#"
            ---
            id: 010
            role: assistant
            created: 2026-05-01T00:00:00Z
            ---

            body.
        "#};
        let m = Message::parse(src).unwrap();
        let rendered = m.render().unwrap();
        let m2 = Message::parse(&rendered).unwrap();
        assert_eq!(m.frontmatter.id, m2.frontmatter.id);
        assert_eq!(m.frontmatter.role, m2.frontmatter.role);
        assert_eq!(m.body.trim(), m2.body.trim());
    }
}
