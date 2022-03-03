// Command module
#![forbid(unsafe_code)]

// General Imports
use std::io::Write;
use std::collections::HashMap;

// File Imports
use lexer::perform_lexing;
use evaluator::evaluate;

// Execute the given command
pub fn exec_command(line:String, silence:bool, mut types:HashMap<String, String>, mut strings:HashMap<String, String>) -> (HashMap<String, String>, HashMap<String, String>) {
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
	    println!("TOKEN: {} {}", t.0, t.1);
	}
	text.push(t.0);
	class.push(t.1);
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
