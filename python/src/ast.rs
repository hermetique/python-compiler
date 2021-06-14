#![allow(unused, clippy::single_match)]

use num_bigint::BigUint;
use python_parser::ast::*;

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
}

#[derive(Debug)]
pub struct Program(Vec<Instruction>);

impl From<Tree> for Program {
    fn from(t: Tree) -> Program {
        let mut instrs: Vec<Instruction> = vec![];

        for statement in t {
            match statement {
                Statement::Assignment(t, v) => instrs.push(handle_assignment(t, v)),
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
