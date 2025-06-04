use crate::parser::ast::Node;

pub fn parse_headings(line: &str) -> Option<Node> {
    for level in (1..=6).rev() {
        let prefix = "#".repeat(level) + " ";
        if line.starts_with(&prefix) {
            return Some(Node::Heading {
                level: level as u8,
                content: vec![Node::Text(line[level..].to_string())],
            });
        }
    }
    None
}
