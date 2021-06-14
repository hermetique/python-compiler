#![allow(unused)]
#![allow(clippy::let_and_return, clippy::single_match)]

use super::buffer::Buffer;
use crate::ast::{Instruction, VarType};
use std::collections::HashMap;

/// All code generators (targets) implement this trait.
pub trait Generator {
    /// Create a new instance of a Generator.
    fn new() -> Self;
    /// Generate code from an IR `Program` (see AST module.)
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

/// An internal module used by the C target.
/// TODO: Make this a trait.
pub struct Module {
    buffer: Buffer,
}

impl Module {
    fn start_block(&mut self, start: &str) {
        self.buffer.write(start);
        self.buffer.indent();
    }

    fn end_block(&mut self, end: &str) {
        self.buffer.dedent();
        self.buffer.write(end);
    }

    fn declare_func(&mut self, name: &str, args: &str) {
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

impl ToString for Module {
    fn to_string(&self) -> String {
        self.buffer.to_string()
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_default() {
        let module = Module::default();
        assert!(module.buffer == Buffer::default());
    }

    #[test]
    fn test_buffer_blocks() {
        let mut module = Module::default();

        module.start_block("test {");
        module.buffer.write("test");
        module.end_block("}");

        assert_eq!(module.buffer.to_string(), "test {\n    test\n}")
    }

    #[test]
    fn test_declare_func() {
        let mut module = Module::default();

        module.declare_func("main", "");
        assert_eq!(module.buffer.to_string(), "function main() {");

        module.buffer.write("printf('hello, world');");
        assert_eq!(
            module.buffer.to_string(),
            "function main() {\n    printf('hello, world');"
        );

        module.end_block("}");
        assert_eq!(
            module.buffer.to_string(),
            "function main() {\n    printf('hello, world');\n}"
        );
    }

    #[test]
    fn test_declare_var() {
        let mut module = Module::default();

        module.declare_var("foo", "100");

        assert_eq!(module.buffer.to_string(), "let foo = 100;")
    }

    #[test]
    fn test_generator_var_dec() {
        use crate::ast::*;

        let input = Instruction::VarDec {
            name: String::from("test"),
            data: VarType::Int(num_bigint::BigUint::from(10 as u64)),
        };

        let mut generator = JSTarget::new();
        let output = generator.generate(Program::new(vec![input]));

        assert_eq!(output, "let test = 10;");
    }
}
