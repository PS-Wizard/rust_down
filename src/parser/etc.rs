use regex::Regex;

use super::ast::Node;

pub fn parse_etc(line: &str) -> Option<Node> {
    for level in (1..=4).rev() {
        let prefix = ">".repeat(level) + " ";
        if line.starts_with(&prefix) {
            return Some(Node::BlockQuote {
                r#type: level as u8,
                content: line[level..].to_string(),
            });
        }
    }

    let re = Regex::new(r"!\[(?P<alt>.*?)\]\((?P<url>.*?)\)").unwrap();
    if let Some(capture) = re.captures(line) {
        let alt = capture.name("alt").unwrap().as_str();
        let src = capture.name("url").unwrap().as_str();
        return Some(Node::Image {
            alt: alt.to_string(),
            src: src.to_string(),
        });
    }

    match line.trim() {
        "---" => Some(Node::LineBreak()),
        _ => None,
    }
}
