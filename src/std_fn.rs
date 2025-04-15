use crate::env::{Env, Value};
use crate::error::{ErrorType, error};
use std::io::{Write, stdout};
use std::process;

pub fn std_fn(env: &mut Env) {
    print_fn(env);
    println_fn(env);
    input_fn(env);
    int_fn(env);
    str_fn(env);
}

fn str_fn(env: &mut Env) {
    let name = "str".to_string();
    fn to_str(args: Vec<Value>) -> Value {
        if args.len() != 1 {
            error(
                ErrorType::RuntimeError,
                "str() takes exactly one argument".to_string(),
            );
            process::exit(1);
        }
        match &args[0] {
            Value::Number(n) => Value::String(n.to_string()),
            Value::Bool(b) => Value::String(b.to_string()),
            Value::String(s) => Value::String(s.clone()),
            _ => {
                error(
                    ErrorType::RuntimeError,
                    "str() argument must be a number".to_string(),
                );
                process::exit(1);
            }
        }
    }
    let func = Value::FuncBuiltIn {
        name: name.clone(),
        body: to_str,
    };
    env.define(name, func);
}

fn int_fn(env: &mut Env) {
    let name = "int".to_string();
    fn to_int(args: Vec<Value>) -> Value {
        if args.len() != 1 {
            error(
                ErrorType::RuntimeError,
                "int() takes exactly one argument".to_string(),
            );
            process::exit(1);
        }
        match &args[0] {
            Value::Number(n) => Value::Number(n.floor()),
            Value::Bool(b) => Value::Number(if *b { 1.0 } else { 0.0 }),
            Value::String(s) => {
                if let Ok(n) = s.parse::<f64>() {
                    Value::Number(n)
                } else {
                    error(
                        ErrorType::RuntimeError,
                        format!("int() argument must be a number, not `{}`", s),
                    );
                    process::exit(1);
                }
            }
            _ => {
                error(
                    ErrorType::RuntimeError,
                    "int() argument must be a number".to_string(),
                );
                process::exit(1);
            }
        }
    }
    let func = Value::FuncBuiltIn {
        name: name.clone(),
        body: to_int,
    };
    env.define(name, func);
}

fn println_fn(env: &mut Env) {
    let name = "println".to_string();
    let func = Value::FuncBuiltIn {
        name: name.clone(),
        body: |args| {
            for arg in args.iter() {
                print!("{}", arg);
            }
            println!();
            Value::Number(args.len() as f64)
        },
    };
    env.define(name, func);
}

fn print_fn(env: &mut Env) {
    let name = "print".to_string();
    let func = Value::FuncBuiltIn {
        name: name.clone(),
        body: |args| {
            for arg in args.iter() {
                print!("{}", arg);
                stdout().flush().unwrap();
            }
            Value::Number(args.len() as f64)
        },
    };
    env.define(name, func);
}

fn input_fn(env: &mut Env) {
    let name = "input".to_string();
    let func = Value::FuncBuiltIn {
        name: name.clone(),
        body: |args| {
            for arg in args.iter() {
                print!("{}", arg);
                stdout().flush().unwrap();
            }
            let mut input = String::new();
            stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            Value::String(input.trim().to_string())
        },
    };
    env.define(name, func);
}
