pub mod ast {
    use std::collections::HashMap;

    use crate::interpret::{op::OP, scanner::Scanner};
    pub const IDX_LIMIT: u8 = 32;
    #[derive(Debug, Clone, PartialEq)]
    pub enum Label {
        REF(String),
        U32(usize),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Mem {
        pub name: Option<String>,
        pub initial_capacity: u32,
    }
    impl Mem {
        pub fn new(name: Option<String>, initial_capacity: u32) -> Self {
            Self {
                name,
                initial_capacity,
            }
        }
    }
    pub type BlockTable = Vec<Vec<Block>>;

    #[derive(Debug, Clone)]
    pub struct Block {
        pub id: Option<String>,
        pub is_loop: bool,
        pub next_pc: usize,
        pub result: bool,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Global {
        pub mutable: bool,
        pub value: i32,
    }

    #[derive(Debug, Clone)]
    pub struct Fn {
        pub name: Option<String>,
        pub args: u32,
        pub locals: u32,
        pub code_addr: usize,
        pub result: bool,
    }
    impl Fn {
        pub fn Empty() -> Self {
            Self {
                name: None,
                args: 0,
                locals: 0,
                code_addr: 0,
                result: false,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct Mod {
        pub memory: Option<Mem>,
        pub exports: HashMap<String, Export>,
        pub funcs: Vec<Fn>,
        pub funcs_refs: HashMap<String, usize>,
        pub globals_map: HashMap<String, usize>,
        pub globals: Vec<Global>, // globals
        pub code: Code,
        pub start: Option<usize>,
    }
    impl Mod {
        pub fn get_fn_idx(&self, fn_id: &str) -> usize {
            let idx_len = self.funcs.len();
            if idx_len == 0 {
                panic!("no functions in module")
            }

            let fn_idx = self.funcs_refs.get(fn_id);
            match fn_idx {
                Some(idx) => *idx,
                None => panic!("no function of the id {:?}", fn_id),
            }
        }
        pub fn get_global(&mut self, label: &Label) -> usize {
            let mut global_idx;
            match label {
                Label::REF(name) => {
                    if let Some(idx) = self.globals_map.get(name) {
                        global_idx = *idx;
                    } else {
                        panic!("no global of such name {}", name);
                    }
                }
                Label::U32(idx) => {
                    if *idx < self.globals.len() {
                        global_idx = *idx
                    } else {
                        panic!("global index out of range")
                    }
                }
            }
            return global_idx;
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ExportType {
        FUNCTION,
        GLOBAL,
        MEMORY,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Export {
        pub export_type: ExportType,
        pub export_ref: Label,
    }
    impl Export {
        pub fn new(export_type: ExportType, export_ref: Label) -> Self {
            Self {
                export_type,
                export_ref,
            }
        }
    }

    type FnSymTable = Vec<Fn>;
    type Code = Vec<OP>;

    #[derive(Debug)]
    pub struct Parser {
        pub memory: Option<Mem>,
        pub scanner: Scanner,
        pub exports: HashMap<String, Export>,
        pub functions: Vec<Fn>,
        pub code_memory: Code,
        pub funcs_refs: HashMap<String, usize>,
        pub globals: Vec<Global>,
        pub globals_map: HashMap<String, usize>,
        pub blks_table: Vec<Vec<Block>>,
        pub blks_stack: Vec<usize>,
    }
    impl Parser {
        pub fn new(scanner: Scanner) -> Self {
            Self {
                memory: None,
                scanner: scanner,
                exports: HashMap::new(),
                functions: Vec::new(),
                code_memory: Vec::new(),
                funcs_refs: HashMap::new(),
                globals: Vec::new(),
                globals_map: HashMap::new(),
                blks_table: Vec::new(),
                blks_stack: Vec::new(),
            }
        }
    }
}
