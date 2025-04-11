pub mod error;
mod expr;
pub mod parser;
pub mod source;
mod token;

pub fn run_file(_source: &str) {
    // let contents = std::fs::read_to_string(source).expect("Unable to read file");
    // let mut source = Source::new(contents);
    // source.tokenize();
    // let mut parser = expr::Parser::new(source.get_tokens().clone());
    // parser.parse();
    // dbg!(parser.get_stmts());
}

pub fn run_cli() {
    // let stdin = std::io::stdin();
    // let mut input = String::new();
    // let mut stdout = std::io::stdout();

    // loop {
    //     print!("> ");
    //     stdout.flush().unwrap();
    //     input.clear();
    //     stdin.read_line(&mut input).unwrap();

    //     if input.trim() == "exit()" {
    //         break;
    //     }

    //     let mut source = Source::new(input.clone());
    //     source.tokenize();
    //     let mut parser = expr::Parser::new(source.get_tokens().clone());
    //     parser.parse();
    //     dbg!(parser.get_stmts());
    // }
}
