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
    // Define flags
    let mut silent_mode = true;
    let mut current_dir = false;
    let mut path_set = false;
    
    // Collect args of program
    let args: Vec<String> = env::args().collect();

    // Parse arguements
    let mut basic_path = Path::new("/dev/null/");
    for n in 1..args.len() {
	let chars: Vec<char> = args[n].chars().collect();

	// Check if flag
	if chars[0] == '-' {
	    // Check length
	    if chars.len() == 1 {
		panic!("No flag specified!");
            }
	    
	    // Check what flags to enable
	    for c in chars {
		if c == 's' {
		    silent_mode = false;
		} else if c == 'c' {
		    current_dir = true;
		} else {
		    panic!("Not a valid flag!");
		}
	    }
	} else {
	    // Check if path is set
	    if path_set == false {
		basic_path = Path::new(&args[n]);
		path_set = true;
	    } else {
		panic!("File path already specified!");
	    }
	}
    }

    // Check if there is a file path
    if path_set == false {
	panic!("File path not specified!");
    }

    // Get file name
    let filename = basic_path.file_stem().expect("DNE!");
    
    // Read file
    let contents = fs::read_to_string(basic_path)
	.expect("Something went wrong reading the file!");

    // Perform lexing, parsing, generation
    if silent_mode {println!("Lexing {:?} ...", basic_path)};
    let tokens = perform_lexing(contents.clone());
    if silent_mode {println!("Done!")};

    if silent_mode {println!("Parsing {:?} ...", basic_path)};
    let ast = construct_tree(tokens);
    if silent_mode {println!("Done!")};

    if silent_mode {println!("Generating code for {:?} ...", basic_path)};
    let code = generate(ast);
    if silent_mode {println!("Done!")};
    
    // Create file path
    let rust_path = Path::new("/tmp").join(filename).with_extension("rs");
    if current_dir {
	rust_path = Path::new(".").join(filename).with_extension("rs");
    }

    // Open a file in write-only mode
    let mut file = match File::create(&rust_path) {
        Err(_) => panic!("Couldn't create file!"),
        Ok(file) => file,
    };

    // Write the code string
    match file.write_all(code.as_bytes()) {
        Err(_) => panic!("Couldn't create file!"),
        Ok(_) => if silent_mode {println!("Successfully compiled and wrote to {:?} ", rust_path)},
    }
}
