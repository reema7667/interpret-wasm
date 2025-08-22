use crate::interpret::lexer::{get_tokens, Token};
use crate::interpret::ast::ast::{
    Block, Export, ExportType, Fn, Global, Label, Mem, Mod, Parser,
};
use crate::interpret::op::OP;
use crate::interpret::op::OP::*;
use crate::interpret::scanner::Scanner;
use log::debug;
use std::collections::HashMap;

use core::{panic};

impl Parser {
    pub fn parse_fn(&mut self) -> Fn {
        let fn_code_addr = self.code_memory.len();
        let mut name: Option<String> = None;
        let mut locals_map: HashMap<String, usize> = HashMap::new();
        let mut function = Fn::Empty();
        let mut n_locals = 0;
        function.code_addr = fn_code_addr;
        if let Some(Token::Id(id)) = self.scanner.peek1() {
            function.name = Some(id.clone());
            self.scanner.advance();
        }
        while self.parse_var("param", &mut locals_map, n_locals) {
            function.args += 1;
            n_locals += 1;
        }
        function.result = self.parse_result();
        while self.parse_var("local", &mut locals_map, n_locals) {
            function.locals += 1;
            n_locals += 1;
        }
        while self.parse_instruction(&locals_map) {}
        self.code_memory.push(RET);
        //self.parse_instruction2();
        match self.scanner.get_next_token() {
            Some(Token::RParan) => {
            }
            _ => panic!("should see rparan for end of function"),
        }
        return function;
    }

    fn parse_instruction(&mut self, locals_map: &HashMap<String, usize>) -> bool {
        let token = self.scanner.peek1();
        if let Some(Token::Kwd(inst)) = self.scanner.peek1() {
            let instruction = inst.clone();
            self.scanner.advance();
            let new_bytecode = self.parse_to_bytecode(&instruction, locals_map);
            self.code_memory.push(new_bytecode);
            return true;
        }
        return false;
    }
    fn parse_to_bytecode(&mut self, inst: &str, vars_map: &HashMap<String, usize>) -> OP {
        match inst {
            "i32.const" => {
                if let Some(Token::Integer(n)) = self.scanner.get_next_token() {
                    I32CONST(*n)
                } else {
                    panic!("should see constant operand with {} instruciton", inst)
                }
            }
            "i32.add" => I32ADD,
            "i32.sub" => I32SUB,
            "i32.mul" => I32MUL,
            "i32.lt_s" => I32LTS,
            "i32.rem_u" => I32REMU,
            "i32.div_s" => I32DIVS,
            "i32.ne" => I32NE,
            "i32.eq" => I32EQ,
            "i32.eqz" => I32EQZ,
            "i32.load" => I32LOAD,
            "i32.store" => I32STORE,
            "i32.gt_s" => I32GTS,
            "i32.ge_s" => I32GES,
            "i32.le_s" => I32LES,
            "i32.and" => I32AND,
            "i32.or" => I32OR,
            "i32.xor" => I32XOR,
            "local.get" => match self.scanner.get_next_token() {
                Some(Token::Id(id)) => {
                    if let Some(idx) = vars_map.get(id) {
                        LOCGET(*idx)
                    } else {
                        panic!("no such named local variable for {} instructino", inst)
                    }
                }
                Some(Token::Integer(idx)) => LOCGET(*idx as usize),
                _ => panic!(
                    "should see local variable reference for {} instruction",
                    inst
                ),
            },
            "local.set" => match self.scanner.get_next_token() {
                Some(Token::Id(id)) => {
                    if let Some(idx) = vars_map.get(id) {
                        LOCSET(*idx)
                    } else {
                        panic!("no such named local variable for {} instructino", inst)
                    }
                }
                Some(Token::Integer(idx)) => LOCSET(*idx as usize),
                _ => panic!(
                    "should see local variable reference for {} instruction",
                    inst
                ),
            },
            "local.tee" => match self.scanner.get_next_token() {
                Some(Token::Id(id)) => {
                    if let Some(idx) = vars_map.get(id) {
                        LOCTEE(*idx)
                    } else {
                        panic!("no such named local variable for {} instructino", inst)
                    }
                }
                Some(Token::Integer(idx)) => LOCTEE(*idx as usize),
                _ => panic!(
                    "should see local variable reference for {} instruction",
                    inst
                ),
            },
            "global.get" => match self.scanner.get_next_token() {
                Some(Token::Id(id)) => GLOGET(Label::REF(id.clone())),
                Some(Token::Integer(idx)) => GLOGET((Label::U32(*idx as usize))),
                _ => panic!(
                    "should see global variable reference for {} instruction",
                    inst
                ),
            },
            "global.set" => match self.scanner.get_next_token() {
                Some(Token::Id(id)) => GLOSET(Label::REF(id.clone())),
                Some(Token::Integer(idx)) => GLOSET((Label::U32(*idx as usize))),
                _ => panic!(
                    "should see global variable reference for {} instruction",
                    inst
                ),
            },
            "block" => {
                let fn_idx = self.functions.len();
                let mut blk_id = None;
                if let Some(Token::Id(id)) = self.scanner.peek1() {
                    blk_id = Some(id.clone());
                    self.scanner.advance();
                }
                let has_result = self.parse_result();
                let blk = Block {
                    id: blk_id,
                    is_loop: false,
                    next_pc: 0,
                    result: has_result,
                };
                self.blks_table[fn_idx].push(blk);
                let blk_idx: usize = self.blks_table[fn_idx].len() - 1;
                self.blks_stack.push(blk_idx);
                BLK(blk_idx)
            }
            "br" => {
                let fn_idx = self.functions.len(); // not -1 because the current function being parsed hasn't been pushed onto the function stack
                                                    // let token =  self.scanner.get_next_token();
                                                    // debug!("br operand is {:?}", token);
                match self.scanner.get_next_token() {
                    Some(Token::Id(id)) => {
                        let br_blk = Some(id.clone());
                        //for (stack_idx, blk_idx) in self.blks_stack.iter().rev().enumerate(){ // nesting starts from in-to-out
                        for blk_idx in self.blks_stack.iter().rev() {
                            // nesting starts from in-to-out
                            if let Some(blk) = self.blks_table[fn_idx].get(*blk_idx) {
                                if blk.id == br_blk {
                                    return BR(*blk_idx);
                                }
                            }
                        }
                        panic!("no matching variable name {:?}", id);
                    }
                    Some(Token::Integer(nesting)) => {
                        let blk_depth = self.blks_stack.len() - (*nesting as usize) - 1; // nesting is from in-to-out
                        debug!("block depth in stack is {blk_depth}");
                        let blk_idx = self.blks_stack.get(blk_depth);
                        match blk_idx {
                            Some(blk_ref) => {
                                let blk_ref = *blk_ref;
                                BR(blk_ref)
                            }
                            None => panic!("not a valid block"),
                        }
                    }
                    _ => panic!("need to see reference operand with {} instruction", inst),
                }
            }
            "br_if" => {
                let fn_idx = self.functions.len();
                match self.scanner.get_next_token() {
                    Some(Token::Id(id)) => {
                        let br_blk = Some(id.clone());
                        for blk_idx in self.blks_stack.iter().rev() {
                            // nesting starts from in-to-out
                            if let Some(blk) = self.blks_table[fn_idx].get(*blk_idx) {
                                if blk.id == br_blk {
                                    return BRIF(*blk_idx);
                                }
                            }
                        }
                        panic!("no matching variable name {:?}", id);
                    }
                    Some(Token::Integer(nesting)) => {
                        let blk_depth = self.blks_stack.len() - (*nesting as usize) - 1; // nesting is from in-to-out
                        debug!("block depth in stack is {blk_depth}");
                        let blk_idx = self.blks_stack.get(blk_depth);
                        match blk_idx {
                            Some(blk_ref) => {
                                let blk_ref = *blk_ref;
                                BRIF(blk_ref)
                            }
                            None => panic!("not a valid block"),
                        }
                    }
                    _ => panic!("need to see reference operand with {} instruction", inst),
                }
            }
            "loop" => {
                let fn_idx = self.functions.len();
                let mut blk_id = None;
                if let Some(Token::Id(id)) = self.scanner.peek1() {
                    blk_id = Some(id.clone());
                    self.scanner.advance();
                }
                let has_result = self.parse_result();
                let blk = Block {
                    id: blk_id,
                    is_loop: true,
                    next_pc: self.code_memory.len(),
                    result: has_result,
                };
                self.blks_table[fn_idx].push(blk);
                let blk_idx: usize = self.blks_table[fn_idx].len() - 1;
                self.blks_stack.push(blk_idx);
                LOOP(blk_idx)
            }
            "end" => {
                let fn_idx = self.functions.len(); // Check why not -1? Not -1 because function symbol has not been added to the function table yet, parsing needs to be completed
                let blk_idx = self.blks_stack.pop().unwrap(); // assume always valid
                let blk = &mut self.blks_table[fn_idx][blk_idx];
                if !blk.is_loop {
                    blk.next_pc = self.code_memory.len();
                }
                END
            }
            "call" => {
                match self.scanner.get_next_token() {
                    Some(Token::Id(fn_name)) => CALL(Label::REF(fn_name.clone())),
                    Some(Token::Integer(idx)) => CALL(Label::U32(*idx as usize)),
                    _ => panic!("should see seen a function reference with {}", inst),
                }
            }
            "return" => RET,
            "drop" => DROP,
            "nop" => NOP,
            _ => todo!("implement {inst}"),
        }
    }
    fn parse_result(&mut self) -> bool {
        if let Some(Token::LParan) = self.scanner.peek1() {
            if let Some(Token::Kwd(kwd)) = self.scanner.peek2() {
                if kwd.as_str() == "result" {
                    self.scanner.advance();
                    self.scanner.advance();
                    match self.scanner.get_next_token() {
                        Some(Token::Kwd(kwd)) => {
                            if kwd.as_str() == "i32" {
                            } else {
                                panic!("Result can only be of type i32")
                            }
                        }

                        _ => {
                            panic!("should see a type with result keyword")
                        }
                    }
                    match self.scanner.get_next_token() {
                        Some(Token::RParan) => return true,
                        _ => panic!("should see rparan for end of result"),
                    }
                }
            }
        }
        return false;
    }
    fn parse_var(
        &mut self,
        var_kwd: &str,
        locals_map: &mut HashMap<String, usize>,
        n_locals: u32,
    ) -> bool {
        if let Some(Token::LParan) = self.scanner.peek1() {
            if let Some(Token::Kwd(kwd)) = self.scanner.peek2() {
                if kwd.as_str() == var_kwd {
                    self.scanner.advance();
                    self.scanner.advance();
                    if let Some(Token::Id(id)) = self.scanner.peek1() {
                        locals_map.insert(id.clone(), n_locals as usize); 
                        self.scanner.advance();
                    }
                    match self.scanner.get_next_token() {
                        Some(Token::Kwd(kwd)) => {
                            if kwd.as_str() == "i32" {
                            } else {
                                panic!("Param can only be of type i32")
                            }
                        }
                        _ => {
                            panic!("should see a type with var keyword")
                        }
                    }
                    match self.scanner.get_next_token() {
                        Some(Token::RParan) => return true,
                        _ => panic!("should see rparan for end of var"),
                    }
                }
            }
        }
        return false;
    }
    fn parse_export(&mut self) {
        let mut export_type: ExportType;
        let mut export_name: String;
        let mut export_ref: Label;
        if let Some(Token::String(name)) = self.scanner.get_next_token() {
            export_name = name.clone();
        } else {
            panic!("should see export name");
        }
        match self.scanner.get_next_token() {
            Some(Token::LParan) => {}
            _ => panic!("should see lparen for beginning of export type "),
        }
        match self.scanner.get_next_token() {
            Some(Token::Kwd(kwd)) => match kwd.as_str() {
                "func" => export_type = ExportType::FUNCTION,
                "global" => export_type = ExportType::GLOBAL,
                "memory" => export_type = ExportType::MEMORY,
                _ => panic!("can't export this type {:?}", kwd),
            },
            _ => panic!("should see export type after export"),
        }
        match self.scanner.get_next_token() {
            Some(Token::Id(id)) => export_ref = Label::REF(id.clone()),
            Some(Token::Integer(idx)) => export_ref = Label::U32(*idx as usize),
            _ => panic!("should see export reference"),
        }
        match self.scanner.get_next_token() {
            Some(Token::RParan) => {}
            _ => panic!("should see rparen to terminate export type "),
        }
        match self.scanner.get_next_token() {
            Some(Token::RParan) => {}
            _ => panic!("should see rparen to terminate export"),
        }
        self.exports
            .insert(export_name, Export::new(export_type, export_ref));
    }

    fn parse_global(&mut self) {
        let mut value: i32;
        let mut is_mut = false;
        if let Some(Token::Id(id)) = self.scanner.peek1() {
            self.globals_map.insert(id.clone(), self.globals.len());
            self.scanner.advance();
        }
        if let Some(Token::LParan) = self.scanner.peek1() {
            if let Some(Token::Kwd(kwd)) = self.scanner.peek2() {
                if kwd.as_str() == "mut" {
                    is_mut = true;
                    self.scanner.advance();
                    self.scanner.advance();
                    if let Some(Token::Kwd(num_type)) = self.scanner.get_next_token() {
                        if num_type.as_str() != "i32" {
                            panic!(
                                "This implementation only accepts i32 type for globals, saw {}",
                                num_type
                            );
                        }
                    }
                    match self.scanner.get_next_token() {
                        Some(Token::RParan) => {}
                        _ => panic!("should see right parenthesis to terminate global type"),
                    }
                } else {
                    panic!("should see type for global")
                }
            }
        } else {
            match self.scanner.get_next_token() {
                Some(Token::Kwd(num_type)) => {}
                _ => panic!("This implementation only accepts i32 type for globals"),
            }
        }
        match self.scanner.get_next_token() {
            Some(Token::LParan) => {
                match self.scanner.get_next_token() {
                    Some(Token::Kwd(kwd)) => {
                        if kwd.as_str() == "i32.const" {
                            match self.scanner.get_next_token(){
                            Some(Token::Integer(n)) => value = *n,
                            _ =>   panic!("Should integer for i32.const instruction to set initial global value")
                        }
                        } else {
                            panic!("should see ")
                        }
                    }
                    _ => panic!(
                        "Should to see i32.const instruction to set initial global value"
                    ),
                }
                match self.scanner.get_next_token() {
                    Some(Token::RParan) => {}
                    _ => panic!("should see right parenthesis to terminate global value"),
                }
            }
            _ => panic!("should see left parenthesis to start global value"),
        }
        match self.scanner.get_next_token() {
            Some(Token::RParan) => {}
            _ => panic!("should see right parenthesis to terminate global component"),
        }
        self.globals.push(Global {
            mutable: is_mut,
            value,
        })
    }
    fn parse_memory(&mut self) {
        let mut name = None;
        let mut initial_capacity: u32;
        if let Some(Token::Id(id)) = self.scanner.peek1() {
            name = Some(id.clone());
            self.scanner.advance();
        }
        match self.scanner.get_next_token() {
            Some(Token::Integer(n)) => initial_capacity = *n as u32,
            _ => panic!("Should see initial memory capacity"),
        }
        match self.scanner.get_next_token() {
            Some(Token::RParan) => {}
            _ => panic!("should see right parenthesis to terminate memory declaration"),
        }
        self.memory = Some(Mem {
            name,
            initial_capacity,
        })
    }
}
pub fn parse_source(source: &str) -> (Mod, Vec<Vec<Block>>) {
    let mut scanner = Scanner::new(get_tokens(source));
    let mut parser = Parser::new(scanner);

    match parser.scanner.current_debug() {
        Some(Token::LParan) => {}
        _ => panic!("should see lparan for beginning of  Wasm module"),
    }
    if let Some(Token::Kwd(kwd)) = parser.scanner.get_next_token() {
        if "module" != kwd.as_str() {
            panic!("each wat file should start with a module")
        }
    }
    loop {
        match parser.scanner.get_next_token() {
            Some(Token::LParan) => {
            }
            Some(Token::RParan) => break, // end of wasm
            _ => panic!("should see terminator or beginnor symbol ), ("),
        }
        if let Some(Token::Kwd(kwd)) = parser.scanner.peek1() {
            match kwd.as_str() {
                "memory" => {
                    parser.scanner.advance();
                    parser.parse_memory();
                }
                "global" => {
                    parser.scanner.advance();
                    parser.parse_global();
                }
                "export" => {
                    parser.scanner.advance();
                    parser.parse_export();
                }
                "func" => {
                    parser.scanner.advance();
                    parser.blks_table.push(Vec::new());
                    let function = parser.parse_fn();
                    if function.name.is_some() {
                        parser
                            .funcs_refs
                            .insert(function.name.clone().unwrap(), parser.functions.len());
                    }
                    parser.functions.push(function);
                    parser.blks_stack.clear();
                }
                _ => panic!(
                    "unknown statement, have to be one of four: func, export, global, memory"
                ),
            }
        }
    }
    if let Some(x) = parser.scanner.peek1() {
        panic!(
            "module definition terminated shouldn't see anything else, saw {:?}",
            x
        );
    }
    let module = Mod {
        memory: parser.memory,
        exports: parser.exports,
        funcs: parser.functions,
        funcs_refs: parser.funcs_refs,
        globals_map: parser.globals_map,
        globals: parser.globals,
        code: parser.code_memory,
        start: None,
    };
    return (module, parser.blks_table);
}

