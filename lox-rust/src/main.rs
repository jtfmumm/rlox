mod cerror;
mod expr;
mod evaluate;
mod interpreter;
mod literal;
mod parser;
mod scanner;
mod token;

use interpreter::interpret;
use parser::Parser;
use scanner::Scanner;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

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
  		Err(_) => eprintln!("Something went wrong!")
  	}
  }
}

fn run_file(arg: &str) {
    let contents = fs::read_to_string(arg)
        .expect("Should have been able to read the file");

	let _ = run(contents);
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
	    let _ = run(user_input);
	}

    Ok(())
}

fn run(source: String) {
	let mut scanner = Scanner::new(source);
	let tokens = scanner.scan_tokens();
	let mut parser = Parser::new(tokens);
	match parser.parse() {
		Ok(expr) => interpret(expr),
		Err(_) => println!("\n\x1b[1;31merror\x1b[0m: could not parse due to previous error") //println!("\nError: {:}", err)
	}
}





