mod rox;
mod scanner;
mod token;
mod token_type;

use rox::Rox;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rox = Rox::new(false);
    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        rox.run_file(&args[1]);
    } else {
        rox.run_prompt();
    };
}
