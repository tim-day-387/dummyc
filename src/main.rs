// General Imports
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Write};

// File Imports
mod evaluator;
mod expression;
mod state;
use state::*;
mod lexer;
use lexer::*;
mod command;
use command::*;

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

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// File interpreter
fn script(file_path:&Path) {
    // Useful variables
    let state = State::new();
    let mut line_num;
    let mut prev_code:Vec<(i64, String)> = Vec::new();

    // Add all lines in the code to prev_code
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                line_num = get_line_num(ip.clone());
		prev_code.push((line_num, ip.clone()));
            }
        }
    }

    // Execute commands given state
    exec_prev(prev_code, state);
}

// Interactive prompt for the BASIC interpreter
fn interactive() {
    // Useful variables
    let mut line:String;
    let mut state = State::new();
    let mut silence = true;
    let mut prev_code:Vec<(i64, String)> = Vec::new();

    // Starting Message
    std::io::stdout().write("Start Prompt!\n".as_bytes()).unwrap();
    let _ = std::io::stdout().flush();

    // Interactive prompt main loop
    loop {		
	// Reset line variable
	line = String::new();

	// Execute commands given state
	state = exec_prev(prev_code.clone(), state);

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
	state = exec_command(line.clone(), silence, state);

	// Add line to previous code
	prev_code.push((state.prev_line, line.clone()));
    }
}
