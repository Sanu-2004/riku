use kire::{env::Env, parser::Parser, source::Source};

fn main() {
    let input = r#"
        let a = 1;
        let b = "hello world"
        print("a + 1 is -> ", a +1)
        print("b is -> ", b)
        "#;

    let mut source = Source::new(input.to_string());
    source.tokenize();

    let mut parser = Parser::new(source.get_tokens());
    parser.parse();

    let mut env = Env::new();

    // dbg!(source.get_tokens());
    // dbg!(parser.get_stmts());
    for stmt in parser.get_stmts() {
        stmt.eval(&mut env);
    }
    // dbg!(env);
}

// use kire::{run_cli, run_file};

// fn main() {
//     let args = std::env::args().collect::<Vec<_>>();
//     if args.len() > 2 {
//         eprintln!("Usage: {} <source_file>", args[0]);
//         std::process::exit(1);
//     }
//     if args.len() == 2 {
//         run_file(&args[1]);
//         std::process::exit(1);
//     } else {
//         run_cli();
//     }
// }
