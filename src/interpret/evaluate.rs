use log::debug;

use crate::interpret::{
    op::{
        OP::{self, *},
    },
};

use super::{
    ast::ast::Label,
    runtime::{self, set_fn_variables, Evaluator, FnFrame, ValueStack},
};

impl Evaluator {
    pub fn run(&mut self) -> Option<i32> {
        while self.calls.len() != 0 {
            self.step();
        }
        let result = self.stack.pop();
        if result.is_some() {
            return result;
        }
        return None;
    }
    pub fn step(&mut self) {
        let next_op = self.next_opcode().clone();
        self.pc += 1;

        self.evaluate_bytecode(next_op);
    }
    pub fn evaluate_bytecode(&mut self, opcode: OP) {
        match opcode {
            I32ADD => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        self.stack.push(lhs + rhs);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to add")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to add")
                }
            },
            I32MUL => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = lhs * rhs;
                        self.stack.push(val);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to multiply")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to multiply")
                }
            },
            I32DIVS => {
                if let Some(rhs) = self.stack.pop() {
                    if rhs == 0{
                        panic!("rhs is 0 cannot divide by zero")
                    }
                    if let Some(lhs) = self.stack.pop() {
                        let val = lhs / rhs;
                        self.stack.push(val);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to divide")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to divide")
                }
            },
            I32CONST(n) => {
                self.stack.push(n);
            },
            I32SUB => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        self.stack.push(lhs - rhs);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to subtract")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to subtract")
                }
            },
            I32AND => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = (lhs != 0 && rhs !=0);
                        self.stack.push(val as i32);
                    } else {
                        panic!("top of stack isn't type i32, need lhs for logical and")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs for logical and")
                }
            },
            I32OR => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = (lhs != 0 || rhs !=0);
                        self.stack.push(val as i32);
                    } else {
                        panic!("top of stack isn't type i32, need lhs for logical and")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs for logical and")
                }
            },
            I32LTS => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = (lhs < rhs) as i32;
                        self.stack.push(val);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to compare less than")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to compare less than")
                }
            },
            I32EQ => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = (lhs == rhs) as i32;
                        self.stack.push(val);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to compare equal")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to compare equal")
                }
            },
            I32GES => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = (lhs >= rhs) as i32;
                        self.stack.push(val);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to compare greater equal")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to compare greater equal")
                }
            },
            I32GTS => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = (lhs > rhs) as i32;
                        self.stack.push(val);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to compare greater ")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to compare greater ")
                }
            },
            I32LES => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = (lhs <= rhs) as i32;
                        self.stack.push(val);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to compare less equal")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to compare less equal")
                }
            },
            I32EQZ => {
                if let Some(n) = self.stack.pop() {
                    let val = (n == 0) as i32;
                    self.stack.push(val)
                } else {
                    panic!("top of stack isn't type i32, need rhs to compare equal")
                }
            },
            I32REMU => {
                if let Some(rhs) = self.stack.pop() {
                    if let Some(lhs) = self.stack.pop() {
                        let val = lhs % rhs;
                        self.stack.push(val);
                    } else {
                        panic!("top of stack isn't type i32, need lhs to find remainder")
                    }
                } else {
                    panic!("top of stack isn't type i32, need rhs to find remainder")
                }
            },
            LOCGET(idx) => {
                let n: i32 = self.calls[self.calls.len() - 1].locals[idx];
                self.stack.push(n);
            },
            LOCSET(idx) => {
                if let Some(n) = self.stack.pop() {
                    // TODO other types
                    let this_fn_idx = self.calls.len() - 1;
                    self.calls[this_fn_idx].locals[idx] = n;
                } else {
                    panic!("need to see a constant to set global")
                }
            },
            LOCTEE(idx) => {
                if let Some(n) = self.stack.pop() {
                    self.stack.push(n);
                    let this_fn_idx = self.calls.len() - 1;
                    self.calls[this_fn_idx].locals[idx] = n;
                } else {
                    panic!("need to see a constant to set global")
                }
            },
            GLOGET(label) => match label {
                Label::REF(id) => match self.module.globals_map.get(&id) {
                    Some(idx) => self.stack.push(self.globals[*idx]),
                    None => panic!("no such global with idx {}", id),
                },
                Label::U32(idx) => match self.globals.get(idx) {
                    Some(n) => self.stack.push(*n),
                    None => panic!("no such global with idx {}", idx),
                },
            },
            GLOSET(label) => {
                if let Some(n) = self.stack.pop() {
                    // globals
                    match label {
                        Label::REF(id) => match self.module.globals_map.get(&id) {
                            Some(idx) => {
                                if !self.module.globals[*idx].mutable {
                                    panic!("global of id {} is not mutable", id)
                                }
                                self.globals[*idx] = n;
                            }
                            None => panic!("no such global with idx {}", id),
                        },
                        Label::U32(idx) => {
                            if idx < self.globals.len() {
                                if !self.module.globals[idx].mutable {
                                    panic!("global of idx {} is not mutable", idx)
                                }
                                self.globals[idx] = n;
                            } else {
                                panic!("global index is out of bounds")
                            }
                        }
                    }
                } else {
                    panic!("need to see a constant to set global")
                }
            },
            BLK(blk_idx) => {
            },
            LOOP(blk_idx) => {
            },
            BR(blk_idx) => {
                let fn_idx = self.calls[self.calls.len() - 1].fn_idx;
                self.pc = self.blks_table[fn_idx][blk_idx].next_pc;
            },
            BRIF(blk_idx) => {
                if let Some(condition) = self.stack.pop() {
                    if condition <= 0 {
                        return;
                    }
                    let fn_idx = self.calls[self.calls.len() - 1].fn_idx;
                    self.pc = self.blks_table[fn_idx][blk_idx].next_pc;
                } else {
                    panic!("should have seen a condition for br_if");
                }
            },
            END => {
            },
            CALL(fn_label) => {
                let mut new_fn_idx;
                match fn_label {
                    Label::REF(id) => {
                        if id.as_str() == "print" {
                            let val = self.stack.pop();
                            match val {
                                Some(val) => {
                                    println!("{}", val);
                                    return;
                                }
                                None => {}
                            }
                        }
                        new_fn_idx = self.module.get_fn_idx(&id)
                    }
                    Label::U32(fn_idx) => new_fn_idx = fn_idx,
                }
                let new_fn = { self.module.funcs.get(new_fn_idx).unwrap() };
                let new_locals = set_fn_variables(new_fn, &mut self.stack);
                let new_fn_frame = FnFrame::new(new_fn_idx, new_locals, self.pc);
                self.pc = new_fn.code_addr;
                self.calls.push(new_fn_frame);
            },
            RET => {
                let this_fn_frame = self.calls.pop().unwrap();
                self.pc = this_fn_frame.ret;
            },
            DROP => {
                self.stack.pop();
            },
            I32LOAD => {
                if let Some(offset) = self.stack.pop() {
                    let end = offset as usize + 4;
                    let slice = &self.memory.bytes[offset as usize..end];
                    let n = i32::from_le_bytes(slice.try_into().unwrap());
                    self.stack.push(n);
                } else {
                    panic!("stack should have an index to access memory from which to load")
                }
            },
            I32STORE => {
                if let Some(val) = self.stack.pop() {
                    if let Some(offset) = self.stack.pop() {
                        let end = (offset + 4) as usize;
                        self.memory
                            .bytes
                            .splice(offset as usize..end, val.to_le_bytes());
                    } else {
                        panic!("stack should have an index to access memory for which to store")
                    }
                } else {
                    panic!("Should have seen a value to store in memory")
                }
            },
            _ => panic!("opcode not yet implemented in evaluator"),
        }
    }

    pub fn call(&mut self, label: &Label) {
        let mut fn_idx;
        match label {
            Label::REF(fn_name) => fn_idx = self.module.get_fn_idx(&fn_name),
            Label::U32(idx) => {
                if *idx < self.module.funcs.len() {
                    fn_idx = *idx
                } else {
                    panic!("funcion index out of range")
                }
            }
        }
        let new_fn = { self.module.funcs.get(fn_idx).unwrap() };
        let new_locals = set_fn_variables(new_fn, &mut self.stack);
        let new_fn_frame = FnFrame::new(fn_idx, new_locals, self.pc);
        self.calls.push(new_fn_frame);
        self.pc = new_fn.code_addr;
    }
}
