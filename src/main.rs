// General Imports
use std::io::Write;
use std::env;
//use std::fs;
//use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

// File Imports
mod lexer;
mod evaluator;
mod command;
use command::*;

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
    let mut line:String = String::new();
    let mut var_types:HashMap<String, String> = HashMap::new();
    let mut string_vals:HashMap<String, String> = HashMap::new();
    let mut state;
    let mut silence = true;
    let mut next_line = -1;
    let mut prev_line = -1;
    let mut prev_code:Vec<(i64, String)> = Vec::new();

    // Starting Message
    std::io::stdout().write("Start Prompt!\n".as_bytes()).unwrap();
    let _ = std::io::stdout().flush();

    // Interactive prompt main loop
    loop {		
	// Reset line variable
	line = String::new();
	
	// Execute any previous commands
	loop {
	    let mut command:String = "".to_string();
	    
	    // Find next command to execute
	    for items in &prev_code {
		if next_line == -1 && prev_line < items.0 {
		    command = items.1.clone();
		    break;
		} else if next_line != -1 && next_line <= items.0 {
		    command = items.1.clone();
		    break;
		}
	    }

	    // Check if there is a line
	    if command == "".to_string() {
		// There are no more commands
		break;
	    } else {
		// Execute given command, update state
		state = exec_command(command.clone(), silence, var_types.clone(), string_vals.clone());
		var_types = state.0;
		string_vals = state.1;
		next_line = state.2;
		prev_line = state.3;
            }
	}

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
	state = exec_command(line.clone(), silence, var_types.clone(), string_vals.clone());
	var_types = state.0;
	string_vals = state.1;
	next_line = state.2;
	prev_line = state.3;

	// Add line to previous code
	prev_code.push((prev_line, line.clone()));
    }
}
