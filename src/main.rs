// General Imports
use std::io::Write;
use std::env;
//use std::fs;
//use std::fs::File;
use std::path::Path;

// File Imports
mod lexer;
use lexer::*;
//mod parser;
//use parser::*;
//mod generator;
//use generator::*;
//mod evaluator;

// Main function code
fn main() {
    // Define flags
    let mut path_set = false;
    
    // Collect args of program
    let args: Vec<String> = env::args().collect();

    // Parse arguements
    let mut _basic_path;
    for n in 1..args.len() {
	// Check if path is set
	if path_set == false {
	    _basic_path = Path::new(&args[n]);
	    path_set = true;
	} else {
	    panic!("File path already specified!");
	}
    }

    // Interpret file or run interactive?
    if path_set == true {
	println!("Run file!");
    } else {
	interactive();
    }	

    // Check if there is a file path
    //if path_set == false {
    //	panic!("File path not specified!");
    //}

    // Get file name
    //let filename = basic_path.file_stem().expect("DNE!");
    
    // Read file
    //let contents = fs::read_to_string(basic_path).expect("Something went wrong reading the file!");
}

// Interactive prompt for the BASIC interpreter
fn interactive() {
    // Useful variables
    let mut line = String::new();

    // Starting Message
    std::io::stdout().write("Start Prompt!\n".as_bytes()).unwrap();
    let _ = std::io::stdout().flush();

    // Interactive prompt main loop
    loop {
	// Pointer
	std::io::stdout().write("~~> ".as_bytes()).unwrap();
	let _ = std::io::stdout().flush();

	// Collect input
	std::io::stdin().read_line(&mut line).unwrap();

	// Check exit conditions
	if line == "DEVSTOP\n".to_string() {
	    break;
	}

	// Execute given command
	exec_command(line);
	
	// Reset line variable
	line = String::new();
    }
}

// Execute the given command
fn exec_command(line:String) {
	// Lex command
	let tokens = perform_lexing(line.clone());
	
	// Write out command
	std::io::stdout().write("COMMAND TEXT: ".as_bytes()).unwrap();
	std::io::stdout().write(line.as_bytes()).unwrap();

	// Write out tokens
	for t in tokens {
	    println!("TOKEN: {} {} {}", t.0, t.1, t.2);
	}
}
