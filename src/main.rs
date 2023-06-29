mod cerror;
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
  // println!("\n");
  let args: Vec<_> = env::args().collect();
  let mut lox = Lox::new();
  if args.len() > 2 {
  	println!("Usage: rlox [script]");
  	process::exit(64);
  } else if args.len() == 2 {
  	lox.run_file(&args[1]);
  } else {
  	match lox.run_prompt() {
  		Ok(_) => println!("Tot ziens"),
  		Err(_) => eprintln!("Something went wrong!")
  	}
  }
}
