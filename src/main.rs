use std::{fs::File, io::Write};

use config::configuration::Config;
use parser::reader::read_file;

mod config;
mod parser;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let ast = read_file(&config);
    let html = ast.to_html(&config.stylesheet); // get the HTML string from your AST

    let mut output = File::create("./output.html").expect("Failed to create output.html");

    output
        .write_all(html.as_bytes())
        .expect("Failed to write HTML to file");

    println!("{:#?}", html);
}
