use crate::interpreter::Interpreter;
use crate::lox_error::LoxError;
use crate::object::{stringify_cli_result, Object};
use crate::parser::Parser;
use crate::scanner::Scanner;

use std::fs;
use std::io;
use std::io::Write;
use std::process;
use std::rc::Rc;

const COMPILE_ERROR_CODE: i32 = 65;
const RUNTIME_ERROR_CODE: i32 = 70;

pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, arg: &str) -> io::Result<()> {
        let contents = fs::read_to_string(arg).expect("Should have been able to read the file");

        match self.run(contents) {
            Ok(_) => {}
            Err(err) => match err {
                LoxError::Parse => process::exit(COMPILE_ERROR_CODE),
                LoxError::Runtime => process::exit(RUNTIME_ERROR_CODE),
                LoxError::Scan => process::exit(COMPILE_ERROR_CODE),
            },
        }
        Ok(())
    }

    pub fn run_repl(&mut self) -> io::Result<()> {
        let exit_string = "exit()\n".to_string();
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
            match self.run(user_input) {
                Ok(obj) => println!("val: {}", stringify_cli_result(&obj)),
                Err(err) => println!("Exited with error: {}", err),
            }
        }

        Ok(())
    }

    fn run(&mut self, source: String) -> Result<Rc<Object>, LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse()?;
        self.interpreter.interpret(stmts)
    }
}
