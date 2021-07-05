// General Imports
use std::io::Write;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

// File Imports
mod lexer;
use lexer::*;
mod parser;
use parser::*;
mod generator;
use generator::*;
mod evaluator;

// Main function code
fn main() {
    // Collect args of program and check if right number
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
	panic!("Wrong number of arguements!");
    }

    // Get file name
    let basic_path = Path::new(&args[1]);
    let filename = basic_path.file_stem().expect("DNE!");
    
    // Read file
    let contents = fs::read_to_string(basic_path)
	.expect("Something went wrong reading the file!");

    // Perform lexing, parsing, generation
    println!("Lexing {:?} ...", basic_path);
    let tokens = perform_lexing(contents.clone());
    println!("Done!");

    println!("Parsing {:?} ...", basic_path);
    let ast = construct_tree(tokens);
    println!("Done!");

    println!("Generating code for {:?} ...", basic_path);
    let code = generate(ast);
    println!("Done!");
    
    // Create file path
    let rust_path = Path::new("/tmp").join(filename).with_extension("rs");

    // Open a file in write-only mode
    let mut file = match File::create(&rust_path) {
        Err(_) => panic!("Couldn't create file!"),
        Ok(file) => file,
    };

    // Write the code string
    match file.write_all(code.as_bytes()) {
        Err(_) => panic!("Couldn't create file!"),
        Ok(_) => println!("Successfully compiled and wrote to {:?} ", rust_path),
    }
}
