use scanner::Scanner;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, Write};
use std::process;
use tokens::TokenType;

#[derive(Debug)]
pub struct Rox {
    pub had_error: bool,
    pub keywords: HashMap<&'static str, TokenType>,
}

impl Rox {
    pub fn new(had_error: bool) -> Rox {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("or", TokenType::Or);
        keywords.insert("class", TokenType::Class);
        keywords.insert("if", TokenType::If);
        keywords.insert("else", TokenType::Else);
        keywords.insert("true", TokenType::True);
        keywords.insert("false", TokenType::False);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        Rox {
            had_error: had_error,
            keywords: keywords,
        }
    }
    pub fn run_file(&mut self, file_name: &String) {
        println!("source: {}", file_name);
        let mut f = File::open(file_name).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("could not parse file");
        self.run(&contents);
        if self.had_error {
            process::exit(65);
        }
    }
    pub fn run_prompt(&mut self) {
        let mut line = String::new();
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            stdin.lock().read_line(&mut line).expect("syntax error");
            if line.trim() == "exit" {
                break;
            } else {
                self.run(&line.trim().to_string());
                line.clear();
                self.had_error = false;
            }
        }
        println!("done");
    }
    fn run(&mut self, source: &String) {
        let mut scanner: Scanner = Scanner::new(source.to_string());
        scanner.scan_tokens(self); // uses Rox::error()
        scanner.print_tokens();
    }
    pub fn error(line: usize, message: String, rox: &mut Rox) {
        Rox::report(line, "".to_string(), message, rox);
    }
    fn report(line: usize, _where: String, message: String, rox: &mut Rox) {
        println!("[line {}] Error {}: {}", line, _where, message);
        rox.had_error = true;
    }
}
