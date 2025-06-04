use crate::{
    config::configuration::Config,
    parser::{ast::Node, codeblock::*, etc::*, inline, tables::parse_tables},
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{ast, headings::parse_headings};

pub fn read_file(config: &Config) -> Node {
    let mut document = ast::Node::new_document();
    let file = File::open(&config.markdown_file).expect("couldn't open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().peekable();

    while let Some(Ok(line)) = lines.peek() {
        if line.is_empty() {
            lines.next();
            continue;
        }

        if let Some(heading) = parse_headings(&line) {
            lines.next();
            document.push_child(heading);
        } else if let Some(underline) = parse_etc(&line) {
            lines.next();
            document.push_child(underline);
        } else if line.trim().starts_with("```") {
            if let Some(code_block) = parse_multiline_code(&mut lines) {
                document.push_child(code_block);
            }
        } else if line.trim().starts_with(":::table") {
            if let Some(table_stuff) = parse_tables(&mut lines) {
                document.push_child(table_stuff);
            }
        } else {
            if let Some(paragraph) = inline::parse_inline_stuff(&line) {
                document.push_child(paragraph);
            };
            lines.next();
        }
    }

    println!("{:#?}", document);
    document
}
