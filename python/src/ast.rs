#![allow(unused, clippy::single_match, clippy::collapsible_match)]

use num_bigint::BigUint;
use python_parser::ast::*;

use std::ops::Deref;

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
    VarDec {
        name: String,
        data: VarType,
    },

    FuncDef {
        is_async: bool,
        name: String,
        parameters: String,
        body: Program,
    },

    FuncCall {
        name: String,
        args: Vec<String>,
    },
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

        // Generate IR for each statement.
        for statement in t {
            match statement {
                // For some reason, both function calls and variable declarations are assignments.
                Statement::Assignment(t, v) => match &t[0] {
                    Expression::Call(b, a) => instrs.push(handle_call(b.deref(), a.to_vec())),
                    Expression::Name(_) => instrs.push(handle_assignment(t, v)),
                    _ => {}
                },

                Statement::Compound(s) => match s.deref() {
                    CompoundStatement::Funcdef(f) => instrs.push(Instruction::FuncDef {
                        is_async: f.r#async,
                        name: f.name.clone(),
                        parameters: process_parameters(&f.parameters),
                        body: Program::from(f.code.to_vec()), // Function body should be handled recursively.
                    }),
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

fn process_parameters(params: &TypedArgsList) -> String {
    "".to_string()
}
