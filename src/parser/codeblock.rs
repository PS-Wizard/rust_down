use std::iter::Peekable;

use super::ast::Node;

#[allow(dead_code)]
pub fn parse_inline_code(line: &str) -> Option<Node> {
    if line.trim().starts_with("`") && line.trim().ends_with("`") {
        return Some(Node::Code {
            language: "inline".to_string(),
            content: vec![Node::Text(line[2..line.len() - 1].to_string())],
        });
    }

    None
}

pub fn parse_multiline_code<I>(lines: &mut Peekable<I>) -> Option<Node>
where
    I: Iterator<Item = std::io::Result<String>>,
{
    let first_line = lines.next()?.ok()?;
    if !first_line.trim().starts_with("```") {
        return None;
    }

    let lang = first_line.trim().trim_start_matches("```").to_string();
    let mut content = vec![];

    while let Some(Ok(peek_line)) = lines.peek() {
        if peek_line.trim().starts_with("```") {
            lines.next(); 
            break;
        }

        let line = lines.next().unwrap().unwrap();
        content.push(Node::Text(line));
    }

    Some(Node::Code {
        language: if lang.is_empty() {
            "unknown".to_string()
        } else {
            lang
        },
        content,
    })
}
