//! Functionality for modifying and constructing parse trees and IR.
//! 
//! This compiler parses Python input using a Nom parser, which outputs a parse tree. That parse tree is 
//! then converted into another form of IR, to be processed by the code generator. The new form of IR is
//! essentially a more linear AST, composed of `Instructions`. The new IR also contains significantly less 
//! information about the actual source.

#![allow(unused, clippy::single_match)]

use num_bigint::BigUint;
use python_parser::ast::*;

/// A parse tree.
pub type Tree = Vec<Statement>;

/// Parse a string into a Tree.
pub fn parse_str(i: &str) -> Tree {
    python_parser::file_input(python_parser::make_strspan(i))
        .unwrap()
        .1
}

/// Variable types used in IR.
#[derive(Debug)]
pub enum VarType {
    Int(num_bigint::BigUint),
}

/// IR Instructions.
#[derive(Debug)]
pub enum Instruction {
    /// A variable declaration. `name` is the name of the variable
    /// and `data` is the value, which is a `VarType`.
    VarDec { name: String, data: VarType },
}

/// An IR program.
#[derive(Debug)]
pub struct Program(Vec<Instruction>);

impl Program {
    /// Create a new program from a Vec of `Instructions`.
    pub fn new(instrs: Vec<Instruction>) -> Self {
        Self(instrs)
    }
}

impl From<Tree> for Program {
    /// Create a new `Program` from a parse tree.
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

    /// Convert a `Program` into an `Iterator` of `Instruction`s.
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/*** Statement Processors ***/

/// Convert an `Assignement` statement into a `VarDec` instruction.
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
