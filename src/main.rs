mod builtins;
mod callable;
mod environment;
mod expr;
mod function;
mod interpreter;
mod lox;
mod lox_error;
mod object;
mod parser;
mod scanner;
mod stmt;
mod token;

use lox::Lox;

use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut lox = Lox::new();
    let _ = match args.len() {
        l if l > 2 => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
        2 => lox.run_file(&args[1]),
        _ => lox.run_repl(),
    };
}
