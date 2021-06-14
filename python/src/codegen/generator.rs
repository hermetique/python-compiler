#![allow(unused)]
#![allow(clippy::let_and_return, clippy::single_match)]

use super::buffer::Buffer;
use crate::ast::{Instruction, VarType};
use std::collections::HashMap;

pub trait Generator {
    fn new() -> Self;
    fn generate(&mut self, p: crate::ast::Program) -> String;
    fn function_map() -> HashMap<String, String>;
}

pub struct JSTarget {
    module: Module,
}

impl Generator for JSTarget {
    fn new() -> Self {
        let mut r = Self {
            module: Module::default(),
        };

        r
    }

    fn function_map() -> HashMap<String, String> {
        let mut r = HashMap::new();

        r.insert(String::from("print"), String::from("console.log"));

        r
    }

    fn generate(&mut self, p: crate::ast::Program) -> String {
        for instr in p.into_iter() {
            match instr {
                Instruction::VarDec { name, data } => {
                    let v = match data {
                        VarType::Int(i) => i.to_str_radix(10),
                    };

                    self.module.declare_var(&name, v.as_str());
                }

                Instruction::FuncCall { name, args } => {
                    self.module.call_func(&name, &args.join(", "));
                }

                _ => {}
            }
        }
        self.module.buffer.to_string()
    }
}

struct Module {
    buffer: Buffer,
}

impl Module {
    fn start_block(&mut self, start: &str) {
        self.buffer.write(start);
        self.buffer.indent();
    }

    fn end_block(&mut self, end: &str) {
        self.buffer.write(end);
        self.buffer.dedent();
    }

    fn declare_func(&mut self, t: &str, name: &str, args: &str) {
        let r = format!("function {}({}) {{", name, args);
        self.start_block(&r);
    }

    fn declare_var(&mut self, name: &str, v: &str) {
        self.buffer.write(format!("let {} = {};", name, v).as_str());
    }

    fn call_func(&mut self, name: &str, args: &str) {
        let name = match name {
            "print" => "console.log",
            _ => name,
        };

        self.buffer.write(format!("{}({});", name, args).as_str())
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
        }
    }
}
