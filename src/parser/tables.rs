use std::iter::Peekable;

use super::ast::Node;

pub fn parse_tables<I>(lines: &mut Peekable<I>) -> Option<Node>
where
    I: Iterator<Item = std::io::Result<String>>,
{
    let _ = lines.next();
    let mut rows = vec![];
    while let Some(Ok(line)) = lines.peek() {
        if line.trim() == ":::" {
            lines.next();
            break;
        }

        let line = lines.next().unwrap().unwrap();
        let cells = line
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();
        rows.push(cells);
    }
    if rows.is_empty() {
        return None;
    }

    let headers = rows[0].clone();
    let data = rows[1..].to_vec();
    Some(Node::Table {
        headers,
        rows: data,
    })
}
