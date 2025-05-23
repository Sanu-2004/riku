use riku::{run_cli, run_file};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }
    if args.len() == 2 {
        run_file(&args[1]);
        std::process::exit(1);
    } else {
        run_cli();
    }
}
