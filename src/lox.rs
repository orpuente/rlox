use std::{
    fs,
    io::{self, BufRead, Write},
    process::exit,
};

use console::{style, Term};

use crate::{environment::Environment, error::LoxError, parser, scanner::Scanner};

#[derive(Default)]
pub struct Lox;

impl Lox {
    pub fn entry_point(&mut self, args: &[String]) {
        match args.len() {
            1 => self.run_prompt(),
            2 => self.run_file(&args[1]),
            _ => {
                println!("Usage: rlox [script]");
                exit(64);
            }
        }
    }

    fn run_file(&mut self, path: &str) {
        let mut env = Environment::default();
        let contents = fs::read_to_string(path).unwrap();
        match self.run(&contents, &mut env) {
            Ok(_) => (),
            Err(_) => exit(64),
        }
    }

    fn run_prompt(&mut self) {
        let mut env = Environment::default();

        self.set_term_title("Lox");
        let stdin = io::stdin();
        self.prompt_symbol();

        for line in stdin.lock().lines() {
            match line {
                Ok(ref cmd) if cmd == "clear" => self.clear(),
                Ok(ref cmd) if cmd == "exit" => exit(0),
                Ok(ref source) => {
                    match self.run(source, &mut env) {
                        Ok(_) => (),
                        Err(err) => println!("{err}"),
                    }
                },
                _ => break,
            }
            self.prompt_symbol();
        }
    }

    fn prompt_symbol(&self) {
        let mut stdout = io::stdout();
        print!("\n{} ", style("rlox⟩").green());
        stdout.flush().unwrap();
    }

    fn clear(&self) {
        let term = Term::stdout();
        term.clear_screen().unwrap();
    }

    fn set_term_title(&self, title: &str) {
        let term = Term::stdout();
        term.set_title(title)
    }

    fn run(&mut self, source: &str, env: &mut Environment) -> Result<(), LoxError> {
        let scanner = Scanner::new(source);

        match scanner.scan_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens.to_vec());
                let program = parser.parse()?;
                for stmt in program {
                    match stmt.eval(env) {
                        Ok(_) => (),
                        Err(type_err) => println!("ERROR: {}", style(type_err.to_string()).red()),
                    }
                }
                
                Ok(())
            }
            Err(errors) => {
                for err in &errors {
                    println!("{err}");
                }

                Err(LoxError::ScannerError(errors))
            }
        }
    }
}
