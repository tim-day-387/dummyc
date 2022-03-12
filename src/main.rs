// General Imports
use std::env;
use std::path::Path;
use std::io::Write;

// File Imports
mod lexer;
mod evaluator;
mod state;
use state::*;

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
	    panic!("File path already specified!");
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
    let mut silence = true;

    // Starting Message
    std::io::stdout().write("Start Prompt!\n".as_bytes()).unwrap();
    let _ = std::io::stdout().flush();

    // Interactive prompt main loop
    loop {		
	// Reset line variable
	line = String::new();

	// Execute commands given state
	state.exec_prev();

	// Pointer
	std::io::stdout().write("~~> ".as_bytes()).unwrap();
	let _ = std::io::stdout().flush();

	// Collect input
	std::io::stdin().read_line(&mut line).unwrap();

	// Check exit conditions
	if line == "DEVSTOP\n".to_string() {
	    break;
	}

	// Check silence conditions
	if line == "DEVTALK\n".to_string() {
	    silence = !silence;
	    continue;
	}

	// Execute given command, update state
	state.exec_command(line.clone(), silence);
    }
}
