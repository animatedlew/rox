use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, Write};
use std::process;

struct GlobalState {
    had_error: bool,
}

struct Token {
    name: String,
}

struct Scanner {
    source: String,
}

impl Scanner {
    fn scan_tokens(&self) -> Vec<Token> {
        self.source
            .split(" ")
            .map(|n| Token {
                name: n.to_string(),
            }).collect::<Vec<Token>>()
    }
    fn run_file(file_name: &String, state: &mut GlobalState) {
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
    fn run_prompt(state: &mut GlobalState) {
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
            println!("{}", token.name);
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut state = GlobalState { had_error: true };
    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        Scanner::run_file(&args[1], &mut state);
    } else {
        Scanner::run_prompt(&mut state);
    };
}
