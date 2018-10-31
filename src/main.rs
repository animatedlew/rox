use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, Write};
use std::process;

fn run_file(file_name: &String) {
    println!("source: {}", file_name);
    let mut f = File::open(file_name).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("could not parse file");
    run(&contents);
}

fn run_prompt() {
    let mut line = String::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        stdin.lock().read_line(&mut line).expect("syntax error");
        if line.trim() == "exit" {
            break;
        } else {
            run(&line.trim().to_string());
            line.clear();
        }
    }
    println!("done");
}

fn run(input: &String) {
    println!("[{}]", input);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    };
}
