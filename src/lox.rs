use std::{fs, io::{self, BufRead, Write}, process::exit};

use console::{style, Term};

use crate::scanner::Scanner;

#[derive(Default)]
pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn entry_point(&mut self, args: &[String]) {
        if args.len() > 2 {
            println!("Usage: rlox [script]");
            exit(64);
        }
        if args.len() == 2 {
            self.run_file(&args[1]);
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, path: &str) {
        let contents = fs::read_to_string(path).unwrap();
        self.run(&contents);
        if self.had_error { exit(64) }
    }
    
    fn run_prompt(&mut self) {
        self.set_term_title("Lox");
        let stdin = io::stdin();
        self.prompt_symbol();
        for line in stdin.lock().lines() {
            match line {
                Ok(ref cmd) if cmd == "clear" => self.clear(),
                Ok(ref source) => {
                    self.run(source);
                    self.had_error = false;
                }
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
    
    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        
        match scanner.scan_tokens() {
            Ok(tokens) => {
                for token in tokens {
                    println!("{token}");
                }
            },
            Err(errors) => {
                for err in errors {
                    self.error(err.line, &err.message);
                }
            },
        }
    }
    
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
    
    fn report(&mut self, line: usize, loc: &str, message: &str) {
        eprintln!("[line {line}] Error{loc}: {message}");
        self.had_error = true;
    }
}
