use global_state::GlobalState;
use token::Token;

use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::io::prelude::*;
use std::process;

pub struct Scanner {
    source: String,
}

impl Scanner {
    fn scan_tokens(&self) -> Vec<Token> {
        self.source
            .split(" ")
            .map(|n| Token::new(n.to_string())).collect::<Vec<Token>>()
    }
    pub fn run_file(file_name: &String, state: &mut GlobalState) {
        println!("source: {}", file_name);
        let mut f = File::open(file_name).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("could not parse file");
        Scanner::run(&contents, state);
        if state.had_error {
            process::exit(65);
        }
    }
    pub fn run_prompt(state: &mut GlobalState) {
        let mut line = String::new();
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            stdin.lock().read_line(&mut line).expect("syntax error");
            if line.trim() == "exit" {
                break;
            } else {
                Scanner::run(&line.trim().to_string(), state);
                line.clear();
                state.had_error = false;
            }
        }
        println!("done");
    }
    fn run(source: &String, state: &mut GlobalState) {
        let scanner: Scanner = Scanner { source: source.to_string() };
        let tokens: Vec<Token> = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token._type);
        }
        Scanner::error(2, "I can't believe it's not butter.".to_string(), state);
    }
    fn error(line: u32, message: String, state: &mut GlobalState) {
        Scanner::report(line, "".to_string(), message, state);
    }
    fn report(line: u32, _where: String, message: String, state: &mut GlobalState) {
        println!("[line {}] Error {}: {}", line, _where, message);
        state.had_error = true;
    }
}