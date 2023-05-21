use std::{error::Error, fs::{self, File}, io::Write};

use tokenizer::tokenizer::TokenList;


mod parser;
mod tokenizer;

pub struct Config {
    input_file_path: String,
    output_file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input_file_path = match args.next() {
            Some(value) => value,
            None => return Err("no input file path provided"),
        };

        let output_file_path = match args.next() {
            Some(value) => value,
            None => return Err("no output file path provided"),
        };

        Ok(Config {
            input_file_path,
            output_file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let md_contents = fs::read_to_string(config.input_file_path)?;
    let mut tokens = TokenList {
        list: vec![],
    };
    
    md_contents.lines().for_each(|md_line| tokens.tokenize(&md_line));

    let mut out_file = File::create(config.output_file_path)?;
    out_file.write(tokens.parse().as_bytes())?;

    Ok(())
}



