use std::fmt::{self, write};
use log::debug;
use crate::interpret::ast::ast::{BlockTable, Fn, Mod};
use super::op::OP;
pub const PAGE: u32 = 65536;
pub type ValueStack = Vec<i32>;

#[derive(Debug, Clone)]
pub struct FnFrame {
    pub fn_idx: usize,
    pub locals: Vec<i32>,
    pub ret: usize,
}
impl FnFrame {
    pub fn new(fn_idx: usize, locals: Vec<i32>, ret: usize) -> Self {
        Self {
            fn_idx,
            locals,
            ret,
        }
    }
}

pub fn set_fn_variables<'a>(function: &'a Fn, caller_stack: &'a mut ValueStack) -> ValueStack {
    let mut vars_table: Vec<i32> = Vec::new();
    let args = function.args;
    if args > 0 {
        if caller_stack.len() < args as usize {
            panic!(
                "not enough parameters in stack saw {} parameters, function signature needs {}",
                caller_stack.len(),
                args
            )
        }
        for arg in 0..args {
            vars_table.push(caller_stack.pop().unwrap());
        }
    }
    for local in 0..function.locals {
        vars_table.push(0);
    }
    vars_table
}
#[derive(Debug, Clone)]
pub struct Evaluator {
    pub module: Mod,
    pub globals: Vec<i32>,
    pub memory: LinearMemory,
    pub stack: ValueStack,
    pub pc: usize,
    pub calls: Vec<FnFrame>,
    pub blks_table: BlockTable,
}
impl Evaluator {
    pub fn add_parameters(&mut self, params: Vec<i32>) {
        self.stack.extend(params);
    }
    fn set_globals(module: &Mod) -> Vec<i32> {
        let mut globals: Vec<i32> = Vec::new();
        for global in module.globals.iter() {
            globals.push(global.value);
        }
        globals
    }
    pub fn new(module: Mod, blks_table: BlockTable) -> Self {
        let mut new_memory: Vec<u8> = Vec::new();
        debug!(" {:?}", module.memory);
        if let Some(mem) = &module.memory {
            debug!("memory initial is : {}", mem.initial_capacity);
            let pages = PAGE * mem.initial_capacity;
            new_memory = vec![0; pages as usize];
        }
        let globals = Evaluator::set_globals(&module);
        Self {
            module,
            globals,
            memory: LinearMemory { bytes: new_memory },
            stack: Vec::new(),
            pc: 0,
            calls: Vec::new(),
            blks_table,
        }
    }
    pub fn next_opcode(&self) -> &OP {
        &self.module.code[self.pc]
    }
}

#[derive(Debug, Clone)]
pub struct LinearMemory {
    pub bytes: Vec<u8>,
}

impl fmt::Display for LinearMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, byte) in self.bytes.iter().enumerate() {
            write!(f, "{:02x} ", byte);
            if (i + 1) % 4 == 0 {
                writeln!(f);
                if *byte == 0 {
                    break;
                }
            }
        }
        Ok(())
    }
}

impl LinearMemory {
    pub fn pretty_print_as_integers(&self) {
        let mut iter = self.bytes.chunks(4);
        let mut before_is_zero = false;
        let mut address = 0;
        while let Some(chunk) = iter.next() {
            let n = i32::from_le_bytes(chunk.try_into().unwrap()); 
            if n == 0 && before_is_zero {
            } else if n == 0 {
                println!("{address:08x}: 0");
                before_is_zero = true;
            } else {
                println!("{address:08x}: {}", n);
                before_is_zero = false;
            }
            address += 1;
        }
    }
}
