#![allow(unused)]
#![allow(clippy::let_and_return, clippy::single_match)]

use super::buffer::Buffer;
use crate::ast::{Instruction, VarType};

pub trait Generator {
    fn new() -> Self;
    fn generate(&mut self, p: crate::ast::Program) -> String;
}

pub struct CTarget {
    module: Module,
}

impl Generator for CTarget {
    fn new() -> Self {
        let mut r = Self {
            module: Module::default(),
        };

        r
    }

    fn generate(&mut self, p: crate::ast::Program) -> String {
        for instr in p.into_iter() {
            match instr {
                Instruction::VarDec { name, data } => {
                    let t: &str;
                    let v: String;

                    match data {
                        VarType::Int(i) => {
                            t = "uint32_t";
                            v = i.to_str_radix(10);
                        }
                    }

                    self.module.declare_var(t, &name, Some(v.as_str()));
                }
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
        self.buffer.dedent();
        self.buffer.write(end);
    }

    fn declare_func(&mut self, t: &str, name: &str, args: &str) -> String {
        let r = format!("{} {}({}) {{", t, name, args);
        self.start_block(&r);
        r
    }

    fn declare_var(&mut self, t: &str, name: &str, value: Option<&str>) -> String {
        let r = match value {
            Some(v) => format!("{} {} = {};", t, name, v),
            None => format!("{} {};", t, name),
        };

        self.buffer.write(&r);
        r
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

        module.declare_func("void", "main", "void");
        assert_eq!(module.buffer.to_string(), "void main(void) {");

        module.buffer.write("printf('hello, world');");
        assert_eq!(
            module.buffer.to_string(),
            "void main(void) {\n    printf('hello, world');"
        );

        module.end_block("}");
        assert_eq!(
            module.buffer.to_string(),
            "void main(void) {\n    printf('hello, world');\n}"
        );
    }

    #[test]
    fn test_declare_var() {
        let mut module = Module::default();

        module.declare_var("uint32_t", "foo", Some("100"));
        module.declare_var("uint32_t", "bar", None);

        assert_eq!(
            module.buffer.to_string(),
            "uint32_t foo = 100;\nuint32_t bar;"
        )
    }
}
