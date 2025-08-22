use std::{fmt::format, thread::panicking};

use log::debug;
use logos::{Lexer, Logos};

use crate::{
    interpret::{
        ast::ast::{ExportType, Label},
        runtime::Evaluator,
    },
    repl::lexer::Token::{self as ReplToken, *},
};

type FnCall = (String, Vec<i32>);

fn consume_lparan(lexer: &mut Lexer<'_, ReplToken>) -> Result<(), String> {
    if let Some(Ok(LParan)) = lexer.next() {
        debug!("consumed ( ");
        Ok(())
    } else {
        Err(format!(
            "didn't see a left paranthesis at {:?}, saw {}",
            lexer.span(),
            lexer.slice()
        ))
    }
}
fn consume_rparan(lexer: &mut Lexer<'_, ReplToken>) -> Result<(), String> {
    if let Some(Ok(LParan)) = lexer.next() {
        Ok(())
    } else {
        Err(format!(
            "didn't see a left paranthesis at {:?}, saw {}",
            lexer.span(),
            lexer.slice()
        ))
    }
}

pub fn parse_command(line: &str, evaluator: &mut Evaluator) -> Result<(), String> {
    let mut lexer = ReplToken::lexer(&line);
    if let Some(Ok(token)) = lexer.next() {
        match token {
            Function(name) => {
                match parse_function(&name, &mut lexer) {
                    Ok(args) => {
                        evaluator.add_parameters(args); // Fix
                        let export = evaluator.module.exports.get(&name);
                        let mut fn_ref: Label;
                        if let Some(export) = export {
                            if export.export_type == ExportType::FUNCTION {
                                fn_ref = export.export_ref.clone();
                            } else {
                                return Err(format!(
                                    "export is not a function it is of type {:?}",
                                    export.export_type
                                ));
                            }
                        } else {
                            debug!("{:?}", evaluator.module.exports);
                            return Err(format!("no such function export {}", name));
                        }
                        evaluator.call(&fn_ref);
                        let result = evaluator.run();
                        println!("function result is {result:?}");
                        return Ok(());
                    }
                    Err(msg) => return Err(msg),
                }
            }
            GetGlobal(name) => {
                let export = evaluator.module.exports.get(&name);
                let mut global_ref: Label;
                if let Some(export) = export {
                    if export.export_type == ExportType::GLOBAL {
                        global_ref = export.export_ref.clone();
                    } else {
                        return Err(format!(
                            "export is not a global it is of type {:?}",
                            export.export_type
                        ));
                    }
                } else {
                    return Err(format!("no such exported global {}", name));
                }
                let global_idx = evaluator.module.get_global(&global_ref);
                println!("{}", evaluator.globals[global_idx]);
                Ok(())
            }
            GetMemory(name) => {
                let export = evaluator.module.exports.get(&name);
                match export {
                    Some(export) => match export.export_type {
                        ExportType::MEMORY => {
                            println!("Memory in integers:");
                            evaluator.memory.pretty_print_as_integers();
                            Ok(())
                        }
                        _ => Err(format!(
                            "export is not a memory it is of type {:?}",
                            export.export_type
                        )),
                    },
                    None => Err(format!("no such exported memory {}", name)),
                }
            }
            _ => return Err(format!("no such command {:?}", token)),
        }
    } else {
        return Err(String::from("no such command"));
    }
}

fn next_keyword(lexer: &mut Lexer<'_, ReplToken>) -> Result<String, String> {
    if let Keyword(string) = lexer.next().unwrap().unwrap() {
        return Ok(string);
    } else {
        Err(String::from("should have seen a keyword"))
    }
}
pub fn parse_set_global(lexer: &mut Lexer<'_, ReplToken>) -> Result<i32, String> {
    consume_lparan(lexer);
    if let Some(Ok(Integer(n))) = lexer.next() {
        Ok(n)
    } else {
        Err(String::from("should have seen a global identifier"))
    }
}

pub fn parse_get_global(lexer: &mut Lexer<'_, ReplToken>) -> Result<String, String> {
    consume_lparan(lexer);
    if let Some(Ok(Keyword(kwd))) = lexer.next() {
        Ok(kwd)
    } else {
        Err(String::from("should have seen a global identifier"))
    }
}
pub fn parse_function(name: &str, lexer: &mut Lexer<'_, ReplToken>) -> Result<Vec<i32>, String> {
    let mut args: Vec<i32> = Vec::new();
    let mut fn_name = name;
    consume_lparan(lexer);

    while let Some(Ok(Integer(n))) = lexer.next() {
        args.push(n);
    }
    consume_rparan(lexer);
    Ok(args)
}
