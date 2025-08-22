use std::ops::Deref;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[\s\t\n\f,]+")]

pub enum Token {
    #[regex(r#"exports.functions.[a-zA-z0-9!#$%&`*+-./:<>=?@\^_'|~]+"#,  |lex| lex.slice()[18..].to_owned() )]
    Function(String),

    #[regex(r#"exports.globals.get\([a-zA-z0-9!#$%&`*+-./:<>=?@\^_'|~]+\)"#,  |lex| lex.slice()[20..lex.slice().len()-1].to_owned() )]
    GetGlobal(String), 

    #[regex(r#"exports.memory.get\([a-zA-z0-9!#$%&`*+-./:<>=?@\^_'|~]+\)"#,  |lex| lex.slice()[19..lex.slice().len()-1].to_owned() )]
    GetMemory(String),

    #[regex(r#"[a-zA-Z][a-zA-z0-9!#$%&`*+-./:<>=?@\^_'|~]+"#,  |lex| lex.slice()[..].to_owned() )]
    Keyword(String),

    #[regex(r#"-?[0-9]+"#, |lex| lex.slice().parse::<i32>().unwrap())]
    Integer(i32),

    #[token("(")]
    LParan,

    #[token(")")]
    RParan,
}

