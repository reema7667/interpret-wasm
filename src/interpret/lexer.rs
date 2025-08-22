
use logos::{ Logos, Span};
use std::ops::Deref;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r#";;[ a-zA-z0-9!#$%&`*+-./:<>=?@\^_'()|~]+"#)]
#[logos(skip r#";[ a-zA-z0-9!#$%&`*+-./:<>=?@\^_'()|~\n]+;"#)]
#[logos(skip r"[\s\t\n\f]+")]
pub enum Token {
    #[regex(r#"[a-z][a-zA-z0-9!#$%&`*+-./:<>=?@\^_'|~]+"#,  |lex| lex.slice().to_owned() )]
    Kwd(String),

    #[regex(r#"\$[a-zA-z0-9!#$%&`*+-./:<->=?@\^_'\+|~]+"#,  |lex| lex.slice()[1..].to_owned() )]
    Id(String),

    #[regex(r#"-?[0-9]+"#, |lex| lex.slice().parse::<i32>().unwrap())]
    Integer(i32),

    #[regex(r#""[a-zA-z0-9!#$%&`*+-./:<>=?@\^_'|~]+""#, |lex| lex.slice()[1..lex.slice().len()-1].to_owned() )]
    String(String),

    #[token("(")]
    LParan,

    #[token(")")]
    RParan,
}

impl Deref for Token {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Token::String(s) => s,
            Token::Kwd(s) => s,
            Token::Id(s) => s,
            _ => panic!("Cannot deref non-string token"),
        }
    }
}

type Error = (String, Span);

type Result<T> = std::result::Result<T, Error>;

pub fn get_tokens(program: &str) -> Vec<Token> {
    let mut lexer = Token::lexer(program);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(tok) => tokens.push(tok),
            Err(_) => {
                panic!(
                    "syntax error because of an unexpcted token at  {:?}",
                    lexer.span()
                );
            }
        }
    }
    tokens
}

