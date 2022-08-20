mod string_eater;
mod parse_error;
mod function;
mod variable;
mod scope;
mod runner;
mod parser;
mod instruction;
mod data_type;

use std::env;
use std::fs;

use runner::Runner;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // One argument allowed
    if args.len() < 1 {
        println!("Expected at least 1 argument");
        println!("Usage: file");
        std::process::exit(1);
    }

    // Read input files
    let data = match fs::read_to_string(&args[0]) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to read input file '{}': {}", &args[0], e);
            std::process::exit(1);
        }
    };

    let mut runner = Runner::new();

    // Import given source file into runner environment
    runner.source(data);
}
