// use argparse::{ArgumentParser, StoreTrue};
use std::env;
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
  	run_prompt();
  }
}

fn run_file(arg: &str) {
	println!("{:?}", arg);
}

fn run_prompt() {
	println!("> ");
}
