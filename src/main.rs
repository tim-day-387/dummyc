// Crates
extern crate rand;
extern crate regex;
extern crate lazy_static;
extern crate quit;

// Modules
mod lexer;
mod expression_lexer;
mod state;
mod data;
mod types;
mod errors;

// General Imports
use std::path::Path;
use std::env;

// File Imports
use state::State;

// Main function code
fn main() {
    // Useful variables
    let mut path_set = false;
    let args: Vec<String> = env::args().collect(); 
    let mut basic_path = Path::new(".");

    // Parse arguments
    for n in 1..args.len() {
	// Check if path is set
	if path_set == false {
	    basic_path = Path::new(&args[n]);
	    path_set = true;
	} else {
	    let artifacts = [].to_vec();
	    let artifact_names = [].to_vec();
	    let function_name = "main".to_string();
	    let message = "File path already specified.".to_string();
	    stateless_error(artifacts, artifact_names, function_name, message);
	}
    }

    // Interpret file or run interactive?
    if path_set == true {
	script(basic_path);
    } else {
	interactive();
    }	
}

// File interpreter
fn script(file_path:&Path) {
    // Useful variables
    let mut state = State::new();

    // Add all lines in the code to prev_code
    state.load_prev(file_path);
    
    // Execute commands given state
    state.exec_prev();
}

// Interactive prompt for the BASIC interpreter
fn interactive() {
    // Useful variables
    let mut line:String;
    let mut state = State::new();

    // Starting Message
    println!("START PROMPT");

    // Interactive prompt main loop
    loop {
	println!("READY");
	
	loop {
	    // Reset line variable
	    line = String::new();
	    
	    // Collect input
	    std::io::stdin().read_line(&mut line).unwrap();
	    line = line.to_string().replace("\n", "");

	    // Check if we should execute
	    if line == "RUN" || line == "EXIT" {
		break;
	    }
	    
	    // Add line to state
	    state.add_prev(line.clone());	    
	}

	// Execute commands given state
	state.exec_prev();

	// Reset state
	state = State::new();

	// Check if we should stop the program
	if line == "EXIT" {
	    break;
	}
    }
}
