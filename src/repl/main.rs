use std::io::Error;
use std::{env, fs};

use crate::interpret::lexer::Token as ModToken;
use crate::line_reader;
use crate::interpret::ast::ast::{BlockTable, Label, Mod};
use crate::interpret::parser::parse_source;
use crate::interpret::runtime::Evaluator;
use crate::repl::lexer::{
    Token as ReplToken, Token::Function, Token::GetGlobal, Token::Integer, Token::LParan,
};
use crate::repl::parser::parse_command;
use log::debug;
use logos::{Lexer, Logos};
use regex::Regex;

enum ArgsError {
    TooFew(String),
    TooMany(String),
}
impl From<ArgsError> for String {
    fn from(error: ArgsError) -> Self {
        match error {
            ArgsError::TooFew(error) => error,
            ArgsError::TooMany(error) => error,
        }
    }
}

enum ParseError {
    ArgsError,
    ModuleError,
}
fn get_file_path() -> Result<String, ArgsError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(ArgsError::TooFew("Expected a file name".to_string()));
    } else if args.len() > 2 {
        return Err(ArgsError::TooFew("Expected only a file name".to_string()));
    } else {
        let file_path = args[1].clone();
        return Ok(file_path);
    }
}

fn parse_file() -> Result<(Mod, BlockTable), ArgsError> {
    let file_path = get_file_path()?;
    let source_code: String = fs::read_to_string(&file_path).unwrap();
    Ok(parse_source(&source_code))
}

pub fn run() -> Result<(), String> {
    let (mut module, mut blk_table) = parse_file()?;
    let mut line_reader: line_reader::LineReader =
        line_reader::LineReader::new(".repl-history.txt", ">>> ");
    let mut evaluator: Evaluator;
    evaluator = Evaluator::new(module, blk_table);
    loop {
        match line_reader.readline() {
            line_reader::LineReadStatus::Line(line) => {
                let result = parse_command(&line, &mut evaluator);
                println!("{:?}", result)
            }
            line_reader::LineReadStatus::Done => break,
        }
    }
    Ok(())
}
// used line reader from Thomas Peters
