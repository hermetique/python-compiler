extern crate python;
use python::codegen;
use python::codegen::Generator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./python [INPUT FILE]");
        std::process::exit(1);
    }

    println!("Opening file {}", args[1]);
    let content = std::fs::read_to_string(args[1].clone());

    println!("Parsing file");
    let p = python::ast::parse_str(content.unwrap().as_str());

    println!("Compiling file");
    let mut generator = codegen::JSTarget::new();
    println!(
        "Compiled file: \n{}",
        generator.generate(python::ast::Program::from(p))
    );
}
