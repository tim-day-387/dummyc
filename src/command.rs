// Command module
#![forbid(unsafe_code)]

// General Imports
use std::io::Write;
use std::collections::HashMap;

// File Imports
use lexer::perform_lexing;
use evaluator::evaluate;

// Execute the given command
pub fn exec_command(line:String, silence:bool, types:HashMap<String, String>, strings:HashMap<String, String>) -> (HashMap<String, String>, HashMap<String, String>, i64, i64) {
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

    return find_subcommand(text, class, types, strings);
}

// Find subcommand to execute
fn find_subcommand(text:Vec<String>, class:Vec<String>, mut types:HashMap<String, String>, mut strings:HashMap<String, String>) -> (HashMap<String, String>, HashMap<String, String>, i64, i64) {
    let mut goto:i64 = -1;
    
    // Check if command is present
    if text.len() == 1 {
	// Return updated state
	return (types, strings, goto, text[0].clone().parse::<i64>().unwrap())
    } else if text.len() == 0 {
	// Return updated state
	return (types, strings, goto, -1)
    }

    // Set keyword
    let keyword = text[1].clone();

    // Execute given command
    if keyword == "PRINT".to_string() {
	let mut counter = 2;

	loop {
	    // Determine how to print
	    if class[counter] == "string".to_string() {
		// Remove parathesis
		let mut string = text[counter].clone();
		string.pop();
		string.remove(0);

		// Output the string
		print!("{}", string);
	    } else if class[counter] == "eval".to_string() {
		// Get the type of the variable
		match types.get(&text[counter]) {
		    Some(kind)=> {
			// Get and print value
			if kind == &"string".to_string() {
			    match strings.get(&text[counter]) {
				Some(value)=> print!("{}", value),
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

	    counter = counter + 1;

	    if counter == text.len() {
		println!("");
		break;
	    }
	}
    } else if keyword == "GOTO".to_string() {
	goto = text[2].clone().parse::<i64>().unwrap();
    } else if keyword == "LET".to_string() {
	// Use evaluator
	let eval_output = evaluate(text[2].clone());
	let var_name = eval_output.0;
	let _rel = eval_output.1;
	let val = eval_output.2;
	let kind = eval_output.3;

	// Insert name and type
	types.insert(var_name.clone(), kind.clone());

	// Where to store variable
	if kind.clone() == "string".to_string() {
	    let mut string = val.clone();
	    string.pop();   
	    string.remove(0);
	    strings.insert(var_name.clone(), string);
	}	
    } else if keyword == "IF".to_string() {
	// Use evaluator
	let eval_output = evaluate(text[2].clone());
	let var_name = eval_output.0;
	let _rel = eval_output.1;
	let val = eval_output.2;
	let mut var_val = &"".to_string();

	// Remove parathesis
	let mut string = val.clone();
	string.pop();
	string.remove(0);

	// Get variable value
	match types.get(&var_name) {
	    Some(kind)=> {
		// Get and print value
		if kind == &"string".to_string() {
		    match strings.get(&var_name) {
			Some(value)=> var_val = value,
			_=> println!("ERROR VAL"),
		    }
		}		    
	    },
	    _=> {
		// Error
		println!("ERROR TYPE");
	    },
	}

	// Check if equivalent
	if var_val == &string {
	    goto = text[4].clone().parse::<i64>().unwrap();
	}
    } else if keyword == "END".to_string() {
	goto = i64::MAX;
    }    

    // Return updated state
    return (types, strings, goto, text[0].clone().parse::<i64>().unwrap())
}
