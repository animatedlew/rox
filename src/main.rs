mod token_type;
mod token;
mod scanner;
mod global_state;

use std::env;
use std::process;
use scanner::Scanner;
use global_state::GlobalState;

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
