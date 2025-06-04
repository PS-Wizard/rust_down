use std::{env, path::Path};

pub struct Config {
    pub markdown_file: String,
    pub stylesheet: String,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let args: Vec<String> = env::args().collect();

        let markdown_file = args
            .get(1)
            .ok_or("Please provide a markdown file.".to_string())?;

        if !Path::new(markdown_file).exists() {
            return Err("The markdown file doesn't exist.".to_string());
        }

        let stylesheet = args.get(2).map(|s| s.as_str()).unwrap_or("default.css");

        if !Path::new(stylesheet).exists() {
            return Err("The stylesheet file doesn't exist.".to_string());
        }

        println!("Got File: {}", markdown_file);
        println!("Using Stylesheet: {}", stylesheet);

        Ok(Config {
            markdown_file: markdown_file.clone(),
            stylesheet: stylesheet.to_string(),
        })
    }
}

