#[derive(Debug)]
pub enum ErrorType {
    TypeError,
    SyntaxError,
    RuntimeError,
    UndefinedVariable,
}

pub fn error(error: ErrorType, message: String) {
    eprintln!("{:?}: {}", error, message);
}

pub fn line_error(error: ErrorType, line: usize, message: String) {
    eprintln!("{:?} on line: {}: {}", error, line, message);
}
