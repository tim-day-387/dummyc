// General Imports
use std::io::{self, Write};
use std::env;
use std::fs;

// File Imports
mod lexer;
use lexer::lexer::*;

// Main function code
fn main() {
    // Collect args of program and check if right number
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
	panic!("Wrong number of arguements!");
    }

    // Create handle for stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Read file
    let contents = fs::read_to_string(&args[1])
	.expect("Something went wrong reading the file!");

    println!("Running {}", args[1]);

    let clean_contents = remove_comments(contents.clone());
    let tokens = tokenize(clean_contents.clone());
    
    // Test
    handle.write_all(contents.as_bytes());
    println!("\n");
    handle.write_all(clean_contents.as_bytes());
    println!("\n");
    println!("{:?}", tokens)
}
