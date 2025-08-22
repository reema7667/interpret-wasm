
use crate::{interpret::{self, lexer::Token, ast::ast::Label, parser::{self, parse_source}, runtime::Evaluator}, repl::main::run};


fn run_src(sc : &str,fn_idx : u32,  params: Vec<i32>) -> Option<i32>{ 
    let  (mut module, mut blk_table)  = parse_source(sc);
    let mut evaluator = Evaluator::new(module, blk_table);
    evaluator.add_parameters(params);
    evaluator.call(&Label::U32(0));
    evaluator.run()
}

#[test]
fn test_integer_subtraction_neg_lhs_is_bigger() {
    let src = 
    r#"(module 
        (func (result i32)
        i32.const 10
        i32.const -11
        i32.sub) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(21));
}
#[test]
fn test_integer_addition() {
    let src = 
    r#"(module 
        (func (result i32)
        i32.const 2
        i32.const 5
        i32.add) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(7));

    let src = 
    r#"(module 
        (func (result i32)
        i32.const 2
        i32.const 2
        i32.add) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(4));

    let src = 
    r#"(module 
        (func (result i32)
        i32.const -2
        i32.const -2
        i32.add) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(-4));

    let src = 
    r#"(module 
        (func (result i32)
        i32.const -2
        i32.const 2
        i32.add) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(0));


    let src = 
    r#"(module 
        (func (result i32)
        i32.const -3
        i32.const 2
        i32.add) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(-1));
}

#[test]
fn test_integer_multiplication() {
    let src = 
    r#"(module 
        (func (result i32)
        i32.const 2
        i32.const 5
        i32.mul) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(10));

    let src = 
    r#"(module 
        (func (result i32)
        i32.const 2
        i32.const 2
        i32.mul) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(4));

    let src = 
    r#"(module 
        (func (result i32)
        i32.const -2
        i32.const -2
        i32.mul) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(4));

    let src = 
    r#"(module 
        (func (result i32)
        i32.const -2
        i32.const 2
        i32.mul) 
    )"#;
    let result = run_src(src, 0, vec![]);
    assert_eq!(result, Some(-4));
}

