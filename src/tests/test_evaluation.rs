use log::debug;

use crate::interpret::{ast::ast::Label, parser::parse_source, runtime::Evaluator};

fn run_test_on_evaluator(sc : &str, fn_idx : u32, params: Vec<i32>) -> Option<i32>{
  let  (module, blk_table)  = parse_source(sc);
  let mut evaluator = Evaluator::new(module, blk_table);
  evaluator.add_parameters(params);
  evaluator.call(&Label::U32(0));
  evaluator.run()
}


#[test]
fn test_fac(){
let source_code = r#"
(module 
(func $fac (param i32) (result i32)
  block $if (result i32)
    block $else (result i32)
  		i32.const 1 
        local.get 0
        i32.const 2
        i32.lt_s
        br_if $if
  		  drop
        local.get 0
        local.get 0
        i32.const 1
        i32.sub
        call $fac
        i32.mul
        br $if
      end
    end  
  )
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![5]), Some(120));
}

#[test]
fn test_multiple_gcd(){
let source_code = r#" ;; source code adapted from wasmtime rust
(module
  (func $gcd (param i32) (param i32) (result i32) (local i32) 
    block $y
        block $x
        local.get 0
        br_if 0 
        local.get 1
        local.set 2
        br 1
      end
      loop $l1
        local.get 1
        local.get 0
        local.tee 2
        i32.rem_u
        local.set 0
        local.get 2
        local.set 1
        local.get 0
        br_if 0 
      end
    end
    local.get 2
  )
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![27, 6]), Some(3));
}

#[test]
fn test_loop(){
let source_code = r#"
(module
  (func
    (result i32) (local $i i32) 
    loop $loop0 
      ;; increment i
      local.get $i
      i32.const 1
      i32.add
      local.set $i
      ;; local.get $i
      ;; i32.const 5
      ;; local.set $i
      local.get $i ;; lhs
      i32.const 10 ;; rhs
      i32.lt_s
      br_if $loop0
    end
    drop
    local.get $i
    i32.const 5
    i32.add
  )
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(15));
}

#[test]
fn test_nested_loop(){
let source_code = r#"
(module
  (func
    (local $i i32) (local $j i32) 
    loop $loop0
      i32.const 0
      local.set $i
        loop $loop1 
            local.get $i
            i32.const 1
            i32.add
            local.set $i

            local.get $i
            i32.const 10
            i32.lt_s
            br_if $loop1
        end
        local.get $j
        i32.const 1
        i32.add
        local.set $j

        local.get $j 
        i32.const 3
        i32.lt_s 
        br_if $loop0
    end
    local.get 1
  )
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(3));
}

#[test]
fn test_calls(){
let source_code = r#"
(module
    (func $run (result i32)
        i32.const 2
        call $f1
        i32.mul
    )
    (func $f1 (result i32) 
        i32.const 5
        i32.const 5
        i32.mul
    )
)  
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(50));
}
#[test]
fn test_one_local(){
let source_code = r#"
(module 
    (func $sq2 (result i32) (local $x i32) 
        i32.const 3
        local.set $x
        local.get 0
        local.get 0
        i32.mul
    )     
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(9));
}

#[test]
fn test_multiple_locals(){
let source_code = r#"
(module 
    (func $sq2 (result i32) (local $x i32) (local $y i32)
        i32.const 3
        local.set $y
        i32.const 6
        local.set $x
        local.get 0
        local.get 1
        i32.mul
    )     
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(18));
}

#[test]
fn test_global_get_no_id(){
let source_code = r#"
(module 
    (global i32 (i32.const 4))
    (func $run (result i32) 
      global.get 0
    )     
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(4));
}

#[test]
#[should_panic]
fn test_global_set_no_id(){
let source_code = r#"
(module 
    (global i32 (i32.const 4))
    (func $run (result i32) 
      i32.const 2
      global.set 0
      global.get 0
    )     
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(2));
}

#[test]
fn test_global2(){
  let source = r#"(module 
  (global $currentTurn (mut i32) (i32.const 1))
  (global $X i32 (i32.const 1))
  (global $Y i32 (i32.const 2))
  (global $EMPTY i32 (i32.const 0))
  (func (result i32)
    global.get $EMPTY
  )
  )"#;
  assert_eq!(run_test_on_evaluator(source, 0, vec![]), Some(0));
}


#[test]
fn test_global_get_id(){
let source_code = r#"
(module 
    (global $x i32 (i32.const 4))
    (func $run (result i32) 
      global.get $x
    )     
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(4));
}

#[test]
#[should_panic]
fn test_global_set_id(){
let source_code = r#"
(module 
    (global $x i32 (i32.const 4))
    (func $run (result i32) 
      i32.const 3
      global.set $x
      global.get $x
    )     
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(3));
}

#[test]
fn test_multiple_global(){
let source_code = r#"
(module 
    (global $x i32 (i32.const 4))
    (global $y i32 (i32.const 5))
    (func $run (result i32) 
      global.get $x
      global.get $y
      i32.add
    )     
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(9));
}
#[test]
fn test_mut_global_set_id(){
let source_code = r#"
(module 
    (global $x (mut i32) (i32.const 4))
    (func $run (result i32) 
      i32.const 3
      global.set $x
      global.get $x
    )     
)
"#;
  assert_eq!(run_test_on_evaluator(source_code, 0, vec![]), Some(3));
}

#[test]
fn test_print(){
  let source_code = r#"
(module 
    (func $sq2 
      i32.const 2
      call $print
    )     
)
"#;
  let result = run_test_on_evaluator(source_code, 0, vec![]);
}