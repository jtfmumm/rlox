mod builtins;
mod callable;
mod function;
mod lox_error;
mod expr;
mod environment;
mod interpreter;
mod lox;
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
  if args.len() > 2 {
  	println!("Usage: rlox [script]");
  	process::exit(64);
  } else if args.len() == 2 {
  	lox.run_file(&args[1]);
  } else {
  	match lox.run_repl() {
      Ok(()) => {},
      Err(err) => println!("Exited with error: {}", err)
    };
  }
}
