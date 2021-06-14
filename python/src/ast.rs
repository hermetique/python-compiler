#![allow(unused, clippy::single_match, clippy::collapsible_match)]

use num_bigint::BigUint;
use python_parser::ast::*;

use std::ops::Deref;

pub type Tree = Vec<Statement>;

pub fn parse_str(i: &str) -> Tree {
    python_parser::file_input(python_parser::make_strspan(i))
        .unwrap()
        .1
}

#[derive(Debug)]
pub enum VarType {
    Int(num_bigint::BigUint),
}

#[derive(Debug)]
pub enum Instruction {
    VarDec { name: String, data: VarType },
    FuncCall { name: String, args: Vec<String> },
}

#[derive(Debug)]
pub struct Program(Vec<Instruction>);

impl From<Tree> for Program {
    fn from(t: Tree) -> Program {
        let mut instrs: Vec<Instruction> = vec![];

        for statement in t {
            match statement {
                Statement::Assignment(t, v) => match &t[0] {
                    Expression::Call(b, a) => instrs.push(handle_call(b.deref(), a.to_vec())),
                    Expression::Name(_) => instrs.push(handle_assignment(t, v)),
                    _ => {}
                },
                _ => {}
            }
        }

        Program(instrs)
    }
}

impl IntoIterator for Program {
    type Item = Instruction;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn handle_assignment(t: Vec<Expression>, v: Vec<Vec<Expression>>) -> Instruction {
    let name: Name;
    match &t[0] {
        Expression::Name(n) => name = n.to_string(),
        _ => {
            panic!("")
        }
    }

    match &v[0][0] {
        Expression::Int(i) => Instruction::VarDec {
            name,
            data: VarType::Int(i.clone()),
        },

        _ => panic!("Unimplemented value"),
    }
}

pub fn handle_call(name: &Expression, args: Vec<Argument>) -> Instruction {
    // Extract the name from the expression.
    let name = match name {
        Expression::Name(n) => n,
        _ => panic!("Error"),
    };

    let mut func_args: Vec<String> = Vec::new();

    for arg in args {
        match arg {
            Argument::Positional(e) => match e {
                Expression::String(p) => {
                    for string in &p {
                        func_args.push(format!(
                            "\"{}\"",
                            string.content.as_str().unwrap().to_owned()
                        ));
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Instruction::FuncCall {
        name: String::from(name),
        args: func_args,
    }
}
