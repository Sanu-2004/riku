use kire::{parser::Parser, source::Source};

fn main() {
    let input = r#"
            !true == false
            1 + 2 * 3
            1 + 2 * 3 == 7
            1 + 2 * 3 == 7 & 1 + 2 * 4 > 7
            1 + 2 * 3 == 7 & 1 + 2 * 4 != 7 | 1 + 2 * 3 == 7
            (1 + 2) * (3 + 4)
            (1 + 2) * (3 + 4) == 3*3 + 3*4
            (1 + 2) * (3 + 4) == 1*3 + 2*4 & 1 + 2 * 3 == 7
            (1 + 2) * (3 + 4) == 1*3 + 2*4 & 1 + 2 * 3 == 7 | 1 + 2 * 3 == 7
            1 + 2 * 3 == 7 & 1 + 2 * 4 != 7 | 1 + 2 * 3 == 7
            (1 + 2) * (3 + 4) == (1+4) * (3 + 2)
            "#;

    let mut source = Source::new(input.to_string());
    source.tokenize();

    let mut parser = Parser::new(source.get_tokens());
    parser.parse();

    // dbg!(source.get_tokens());
    // dbg!(parser.get_stmts());
    for expr in parser.get_stmts() {
        println!("{} = {}", expr, expr.eval());
    }
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
