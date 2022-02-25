// General Imports
use std::io::Write;
use std::env;
//use std::fs;
//use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

// File Imports
mod lexer;
use lexer::*;
mod evaluator;
use evaluator::*;

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

	// Check silence conditions
	if line == "DEVTALK\n".to_string() {
	    silence = !silence;
	}

	// Execute given command, update state
	state = exec_command(line, silence, var_types.clone(), string_vals.clone());
	var_types = state.0;
	string_vals = state.1;
	
	// Reset line variable
	line = String::new();
    }
}

// Execute the given command
fn exec_command(line:String, silence:bool, mut types:HashMap<String, String>, mut strings:HashMap<String, String>) -> (HashMap<String, String>, HashMap<String, String>) {
    // Lex command
    let tokens = perform_lexing(line.clone());
    let mut text:Vec<String> = Vec::new();
    let mut class:Vec<String> = Vec::new();

    // Write out command
    if !silence {
	std::io::stdout().write("COMMAND TEXT: ".as_bytes()).unwrap();
	std::io::stdout().write(line.as_bytes()).unwrap();
    }
    
    // Write out tokens
    for t in tokens {
	if !silence {
	    println!("TOKEN: {} {} {}", t.0, t.1, t.2);
	}
	text.push(t.1);
	class.push(t.2);
    }

    // Check if command is present
    if text.len() <= 1 {
	// Return updated state
	return (types, strings)
    }

    // Set keyword
    let keyword = text[1].clone();

    // Execute given command
    if keyword == "PRINT".to_string() {
	// Determine how to print
	if class[2] == "string".to_string() {
	    // Remove parathesis
	    text[2].pop();
	    text[2].remove(0);

	    // Output the string
	    println!("{}", text[2]);
	} else if class[2] == "eval".to_string() {
	    // Get the type of the variable
	    match types.get(&text[2]) {
		Some(kind)=> {
		    // Get and print value
		    if kind == &"string".to_string() {
			match strings.get(&text[2]) {
			    Some(value)=> println!("{}", value),
			    _=> println!("ERROR VAL"),
			}
		    }		    
		},
		_=> {
		    // Error
		    println!("ERROR TYPE");
		},
	    }
	}
    } else if keyword == "GOTO".to_string() {
    } else if keyword == "LET".to_string() {
	// Use evaluator
	let eval_output = evaluate(text[2].clone());
	let var_name = eval_output.0;
	let _rel = eval_output.1;
	let mut val = eval_output.2;
	let kind = eval_output.3;

	// Insert name and type
	types.insert(var_name.clone(), kind.clone());

	// Where to store variable
	if kind.clone() == "string".to_string() {
	    val.pop();   
	    val.remove(0);
	    strings.insert(var_name.clone(), val);
	}	
    } else if keyword == "IF".to_string() {
    } else if keyword == "END".to_string() {
    }    

    // Return updated state
    return (types, strings)
}
