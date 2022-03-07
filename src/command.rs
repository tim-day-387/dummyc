// Command module
#![forbid(unsafe_code)]

// General Imports
use std::io::Write;

// File Imports
use lexer::perform_lexing;
use evaluator::evaluate;
use state::*;

// Execute all previous commands, given state
pub fn exec_prev(prev_code:Vec<(i64, String)>, mut state:State) -> State {    
    // Execute any previous commands
    loop {
	let mut command:String = "".to_string();
	
	// Find next command to execute
	for items in &prev_code {
	    if state.next_line == -1 && state.prev_line < items.0 {
		command = items.1.clone();
		break;
	    } else if state.next_line != -1 && state.next_line <= items.0 {
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
	    state = exec_command(command.clone(), true, state);
        }
    }

    // Return state
    return state; 
}

// Execute the given command
pub fn exec_command(line:String, silence:bool, state:State) -> State {
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

    return find_subcommand(text, class, state);
}

// Find subcommand to execute
fn find_subcommand(text:Vec<String>, class:Vec<String>, mut state:State) -> State {
    // Check if command is present
    if text.len() == 1 {
	// Update state
	state.next_line = -1;
	state.prev_line = text[0].clone().parse::<i64>().unwrap();
	
	// Return updated state
	return state;
    } else if text.len() == 0 {
	// Update state
	state.next_line = -1;
	state.prev_line = -1;

	// Return updated state
	return state;
    }

    // Set keyword
    let keyword = text[1].clone();

    // Execute given command
    if keyword == "PRINT".to_string() {
	return print_cmd(text, class, state);
    } else if keyword == "GOTO".to_string() {
	return goto_cmd(text, class, state);
    } else if keyword == "LET".to_string() {
	return let_cmd(text, class, state);
    } else if keyword == "IF".to_string() {
	return if_cmd(text, class, state);
    } else if keyword == "END".to_string() {
	return end_cmd(text, class, state);
    } else {
	// Update state
	state.next_line = i64::MAX;
	state.prev_line = text[0].clone().parse::<i64>().unwrap();

	// Return updated state
	return state;
    }
}

// Implmentation of the PRINT command
fn print_cmd(text:Vec<String>, class:Vec<String>, mut state:State) -> State {
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
	    match state.types.get(&text[counter]) {
		Some(kind)=> {
		    // Get and print value
		    if kind == &"string".to_string() {
			match state.strings.get(&text[counter]) {
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

    // Update state
    state.next_line = -1;
    state.prev_line = text[0].clone().parse::<i64>().unwrap();

    // Return updated state
    return state;
}

// Implmentation of the GOTO command
fn goto_cmd(text:Vec<String>, _class:Vec<String>, mut state:State) -> State {
    // Update state
    state.next_line = text[2].clone().parse::<i64>().unwrap();
    state.prev_line = text[0].clone().parse::<i64>().unwrap();

    // Return updated state
    return state;
}

// Implmentation of the LET command
fn let_cmd(text:Vec<String>, _class:Vec<String>, mut state:State) -> State {
    // Use evaluator
    let eval_output = evaluate(text[2].clone());
    let var_name = eval_output.0;
    let _rel = eval_output.1;
    let val = eval_output.2;
    let kind = eval_output.3;

    // Insert name and type
    state.types.insert(var_name.clone(), kind.clone());

    // Where to store variable
    if kind.clone() == "string".to_string() {
	let mut string = val.clone();
	string.pop();   
	string.remove(0);
	state.strings.insert(var_name.clone(), string);
    }	

    // Update state
    state.next_line = -1;
    state.prev_line = text[0].clone().parse::<i64>().unwrap();

    // Return updated state
    return state;
}

// Implmentation of the IF command
fn if_cmd(text:Vec<String>, _class:Vec<String>, mut state:State) -> State {
    let mut goto = -1;

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
    match state.types.get(&var_name) {
	Some(kind)=> {
	    // Get and print value
	    if kind == &"string".to_string() {
		match state.strings.get(&var_name) {
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

    // Update state
    state.next_line = goto;
    state.prev_line = text[0].clone().parse::<i64>().unwrap();

    // Return updated state
    return state;
}

// Implmentation of the END command
fn end_cmd(text:Vec<String>, _class:Vec<String>, mut state:State) -> State {
    // Update state
    state.next_line = i64::MAX;
    state.prev_line = text[0].clone().parse::<i64>().unwrap();
    
    // Return updated state
    return state;
}
