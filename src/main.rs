// General Imports
use std::io::Write;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

// File Imports
mod lexer;
use lexer::lexer::*;
mod parser;
use parser::parser::*;
mod generator;
use generator::generator::*;

// Main function code
fn main() {
    // Collect args of program and check if right number
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
	panic!("Wrong number of arguements!");
    }

    // Read file
    let contents = fs::read_to_string(&args[1])
	.expect("Something went wrong reading the file!");
    println!("Compiling {}", args[1]);

    // Perform lexing
    let tokens = perform_lexing(contents.clone());

    // Perform parsing
    let ast = construct_tree(tokens);

    // Perform generation
    let code = generate(ast);

    // Create file
    let path = Path::new("../sample.rs");

    // Open a file in write-only mode
    let mut file = match File::create(&path) {
        Err(_) => panic!("Couldn't create file!"),
        Ok(file) => file,
    };

    // Write the code string
    match file.write_all(code.as_bytes()) {
        Err(_) => panic!("Couldn't read file!"),
        Ok(_) => println!("Successfully wrote to file!"),
    }
}
