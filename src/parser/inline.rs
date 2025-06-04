// use super::ast::Node;

use std::vec;

use regex::Regex;

use super::ast::Node;

pub fn parse_inline_stuff(line: &str) -> Option<Node> {
    let mut paragraph = Node::Paragraph(vec![]);
    let mut last_index = 0;

    let re = Regex::new(
        r"(?P<bold>\*\*(.+?)\*\*)|(?P<italic>\*(.+?)\*)|(?P<code>`(.+?)`)|(?P<mark>==(.+?)==)|(?P<strikethrough>~~(.+?)~~)",
    )
    .unwrap();

    for cap in re.captures_iter(line) {
        let m = cap.get(0).unwrap();
        let start = m.start();
        let end = m.end();

        // push text before match if any
        if start > last_index {
            let text = &line[last_index..start];
            paragraph.push_child(Node::Text(text.to_string()));
        }

        // push matched pattern as node
        if let Some(bold) = cap.name("bold") {
            let inner = bold.as_str().trim_matches('*');
            paragraph.push_child(Node::Bold(vec![Node::Text(inner.to_string())]));
        } else if let Some(italic) = cap.name("italic") {
            let inner = italic.as_str().trim_matches('*');
            paragraph.push_child(Node::Italic(vec![Node::Text(inner.to_string())]));
        } else if let Some(code) = cap.name("code") {
            let inner = code.as_str().trim_matches('`');
            paragraph.push_child(Node::Code {
                language: "plaintext".into(),
                content: vec![Node::Text(inner.to_string())],
            });
        } else if let Some(mark) = cap.name("mark") {
            let inner = mark.as_str().trim_matches('=');
            paragraph.push_child(Node::Mark(vec![Node::Text(inner.to_string())])); // change if u want text inside
        }

        last_index = end;
    }

    // push leftover text
    if last_index < line.len() {
        let tail = &line[last_index..];
        paragraph.push_child(Node::Text(tail.to_string()));
    }

    Some(paragraph)
}
