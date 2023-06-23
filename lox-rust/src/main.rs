mod cerror;
mod expr;
mod parser;
mod scanner;
mod token;

use cerror::error;
use parser::Parser;
use scanner::Scanner;
// use token::{Token, TokenType};

// use argparse::{ArgumentParser, StoreTrue};
// use std::any::Any;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;
// use std::rc::Rc;

fn main() {
  println!("\n");
  let args: Vec<_> = env::args().collect();
  if args.len() > 2 {
  	println!("Usage: jlox [script]");
  	process::exit(64);
  } else if args.len() == 2 {
  	run_file(&args[1]);
  } else {
  	match run_prompt() {
  		Ok(_) => println!("Tot ziens"),
  		Err(_) => error(0, "Something went wrong!")
  	}
  }
}

fn run_file(arg: &str) {
    let contents = fs::read_to_string(arg)
        .expect("Should have been able to read the file");

	run(contents);
}

fn run_prompt() -> io::Result<()> {
    let exit_string = "x\n".to_string();
	loop {
		print!("> ");
		io::stdout().flush()?;
	    let mut user_input = String::new();
	    let stdin = io::stdin();
	    stdin.read_line(&mut user_input)?;
	    match user_input {
	    	s if s == exit_string => break,
	    	_ => {}
	    }
	    run(user_input)
	}

    Ok(())
}

fn run(source: String) {
	let mut scanner = Scanner::new(source);
	let tokens = scanner.scan_tokens();
	// for t in &tokens {
	// 	println!("{:?}", t);
	// }
	let mut parser = Parser::new(tokens);
	println!("{:?}", parser.equality().to_string());
}





