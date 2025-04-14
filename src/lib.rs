use std::io::Write;

use parser::Parser;
use source::Source;

pub mod env;
pub mod error;
mod expr;
pub mod parser;
pub mod source;
mod stmt;
mod token;

pub fn run_file(source: &str) {
    let contents = std::fs::read_to_string(source).expect("Unable to read file");
    let mut source = Source::new(contents);
    source.tokenize();
    // dbg!(source.get_tokens());
    let mut parser = Parser::new(source.get_tokens());
    parser.parse();
    // dbg!(parser.get_stmts());
    let mut env = env::Env::new();
    for stmt in parser.get_stmts() {
        stmt.eval(&mut env);
    }
}

pub fn run_cli() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    let mut stdout = std::io::stdout();
    let mut env = env::Env::new();
    println!("Running in cli mode");

    loop {
        print!("-> ");
        stdout.flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).unwrap();

        if input.trim() == "exit()" {
            break;
        }

        let mut source = Source::new(input.clone());
        source.tokenize();
        let mut parser = Parser::new(source.get_tokens());
        parser.parse();
        for stmt in parser.get_stmts() {
            if let Some(res) = stmt.eval(&mut env) {
                println!("{}", res);
            }
        }
    }
}
