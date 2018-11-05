use scanner::Scanner;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, Write};
use std::process;
use token_type::TokenType;

#[derive(Debug)]
pub struct Rox {
    pub had_error: bool,
    pub keywords: HashMap<String, TokenType>,
}

impl Rox {
    pub fn new(had_error: bool) -> Rox {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);
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
