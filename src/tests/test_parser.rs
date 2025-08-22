#[cfg(test)]
mod test_parser {
use crate::interpret::op::{OP, OP::*};
use crate::interpret::ast::ast::{self, Export, Global, Label, Mem};
use crate::interpret::parser::parse_source;
#[test]
fn test_empty_module(){
    let source = "(module)";
    parse_source(source);
}
#[test]
fn test_empty_function(){
    let source = r#"(module 
                                (func ))"#;
    parse_source(source);
}
#[test]
fn test_one_param_function(){
    let source = r#"(module 
                                (func (param i32) ))"#;
    parse_source(source);
}
#[test]
fn test_param_id_function(){
    let source = r#"(module 
                                (func (param $p1 i32) ))"#;
    parse_source(source);
}
#[test]
fn test_more_than_one_param_function(){
    let source = r#"(module 
                                (func (param i32) (param i32) (param i32) ))"#;
    parse_source(source);
}

#[test]
fn test_one_result_function(){
    let source = r#"(module 
    (func (result i32) ))"#;
    parse_source(source);
}
#[test]
fn test_param_and_result_function(){
    let source = r#"(module 
    (func (param i32) (result i32) ))"#;
    parse_source(source);
}
#[test]
fn test_id_function(){
    let source = r#"(module 
    (func $id ))"#;
    parse_source(source);
}
#[test]
fn test_id_param_function(){
    let source = r#"(module 
    (func $id (param i32) ))"#;
    parse_source(source);
}
#[test]
fn test_one_local_function(){
    let source = r#"(module 
    (func $id (local i32) ))"#;
    parse_source(source);  
}
#[test]
fn test_return_function(){
    let source = r#"(module 
    (func (result i32) ))"#;
    parse_source(source);  
}
#[test]
fn test_function_empty_sig_instruction(){
    let source = r#"(module 
    (func i32.const 5 drop ))"#;
    parse_source(source);  
}
#[test]
fn test_function_with_sig_and_instruction(){
    let source = r#"(module 
    (func i32.const 5 drop ))"#;
    parse_source(source);  
}

#[test]
fn test_function_with_add_instruction(){
    let source = r#"(module 
    (func i32.add ))"#;
    parse_source(source);  
}
#[test]
fn test_function_with_more_than_two_arith_instruction(){
    let source = r#"(module 
    (func i32.const 5 i32.const 6 i32.const 6))"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![OP::I32CONST(5), OP::I32CONST(6), OP::I32CONST(6), RET]);
}
#[test]
fn test_function_with_const_instruction(){
    let source = r#"(module 
    (func i32.const 5 ))"#;
    parse_source(source);  
}
#[test]
fn test_function_with_var_instruction_n_ref(){
    let source = r#"(module 
    (func local.get 5 ))"#;
    parse_source(source);  
}
#[test]
fn test_function_with_var_instruction_id_ref(){
    let source = r#"(module 
    (func (local $x i32) local.get $x ))"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![OP::LOCGET(0), RET]);
}
#[test]
fn test_function_with_block_instruction_id_ref(){
    let source = r#"(module 
    (func block $x ))"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![OP::BLK(0), RET]);
}
#[test]
fn test_function_with_block_instrution_and_br(){
    let source = r#"(module 
    (func block br 0  ))"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![OP::BLK(0), OP::BR(0), RET]);
}
#[test]
fn test_function_with_block_instruction_with_result(){
    let source = r#"(module 
    (func block (result i32) )  )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![OP::BLK(0), RET]);
}
#[test]
fn test_multiple_empty_functions(){
    let source = r#"(module 
    (func )
    (func)
    )"#; 
    let (module, blk_table) = parse_source(source);  
    // assert_eq!(module.code, vec![OP::BLK(0), OP::BR(0)]);
}
#[test]
fn test_function_with_branch_instruction_id_ref(){
    let source = r#"(module 
    (func block $x br $x ))"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![OP::BLK(0), OP::BR(0), RET]);
}
#[test]
fn test_function_with_multiple_blocks_first_branch(){
    let source = r#"(module 
    (func block $x 
            block $y
                block $z
                    br $x
                end 
            end 
        end
    ))"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![BLK(0), BLK(1), BLK(2), BR(0), END, END, END, RET]);
}
#[test]
fn test_function_with_multiple_blocks_last_branch(){
    let source = r#"(module 
    (func block $x 
            block $y
                block $z
                    br $z
                end 
            end 
        end
    ))"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![BLK(0), BLK(1), BLK(2), BR(2), END, END, END, RET]);
}
#[test]
fn test_function_with_multiple_blocks_middle_branch(){
    let source = r#"(module 
    (func block $x 
            block $y
                block $z
                    br $y
                end 
            end 
        end
    ))"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.code, vec![BLK(0), BLK(1), BLK(2), BR(1), END, END, END, RET]);
}

#[test]
fn test_export_fn(){
    let source = r#"(module 
    (export "fn" (func 0))
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.exports.get("fn"), Some(&Export{export_type: ast::ExportType::FUNCTION, export_ref: Label::U32(0) }) );
}

#[test]
fn test_export_fn_id(){
    let source = r#"(module 
    (export "fn" (func $x))
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.exports.get("fn"), Some(&Export{export_type: ast::ExportType::FUNCTION, export_ref: Label::REF(String::from("x")) }) );
}
#[test]
fn test_export_memory(){
    let source = r#"(module 
    (export "mem" (memory 0))
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.exports.get("mem"), Some(&Export{export_type: ast::ExportType::MEMORY, export_ref: Label::U32(0) }) );
}
#[test]
fn test_export_global(){
    let source = r#"(module 
    (export "g" (global 0))
    (export "fn" (func $x))
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.exports.get("g"), Some(&Export{export_type: ast::ExportType::GLOBAL, export_ref: Label::U32(0) }) );
    assert_eq!(module.exports.get("fn"), Some(&Export{export_type: ast::ExportType::FUNCTION, export_ref: Label::REF(String::from("x")) }) );
}
#[test]
fn test_multiple_exports(){
    let source = r#"(module 
    (export "g" (global 0))
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.exports.get("g"), Some(&Export{export_type: ast::ExportType::GLOBAL, export_ref: Label::U32(0) }) );
}
#[test]
fn test_global_no_id(){
    let source = r#"(module 
      (global i32 (i32.const 4))
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.globals.get(0), Some(&Global{mutable: false, value  :4}) );
}
#[test]
fn test_global_id(){
    let source = r#"(module 
      (global $curr i32 (i32.const 4))
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.globals.get(0), Some(&Global{mutable: false, value  :4}) );
    assert_eq!(module.globals_map.get("curr"), Some(&0))
}
#[test]
fn test_global_mut(){
    let source = r#"(module 
      (global (mut i32) (i32.const 4))
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.globals.get(0), Some(&Global{mutable: true, value  :4}) );
}
#[test]
fn test_memory(){
    let source = r#"(module 
      (memory $mem 1)
    )"#;
    let (module, blk_table) = parse_source(source);  
    assert_eq!(module.memory, Some(Mem{name: Some(String::from("mem")), initial_capacity  :1} ));
}
}
