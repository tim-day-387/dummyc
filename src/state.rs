// State module
#![forbid(unsafe_code)]

// General Imports
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Write};


// File Imports
use lexer::perform_lexing;
use evaluator::evaluate;

// State struct
pub struct State {
    pub types:HashMap<String, String>,
    pub strings:HashMap<String, String>,
    pub prev_code:Vec<(i64, String)>,
    pub next_line:i64,
    pub prev_line:i64,
}

// State implementation
impl State {
    // Constructor
    pub fn new() -> State {
	State {
	    types:HashMap::new(),
	    strings:HashMap::new(),
	    prev_code:Vec::new(),
	    next_line:-1,
	    prev_line:-1,
	}
    }

    // Load previous commmands from a file
    pub fn load_prev(&mut self, file_path:&Path) {
	// Useful variables 
	let mut line_num;

	// Add all lines in the code to prev_code
	if let Ok(lines) = read_lines(file_path) {
            for line in lines {
		if let Ok(ip) = line {
                    line_num = perform_lexing(ip.clone()).0[0].parse::<i64>().unwrap();
		    self.prev_code.push((line_num, ip.clone()));
		}
            }
	}
    }

    // Execute all previous commands, given state
    pub fn exec_prev(&mut self) {    
	// Execute any previous commands
	loop {
	    let mut command:String = "".to_string();
	    
	    // Find next command to execute
	    for items in &self.prev_code {
		if self.next_line == -1 && self.prev_line < items.0 {
		    command = items.1.clone();
		    break;
		} else if self.next_line != -1 && self.next_line <= items.0 {
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
		self.exec_command(command.clone(), true);
            }
	}
    }

    // Execute the given command
    pub fn exec_command(&mut self, line:String, silence:bool) {
	// Lex command
	let text:Vec<String> = perform_lexing(line.clone()).0;
	let class:Vec<String> = perform_lexing(line.clone()).1;

	// Write out command
	if !silence {
	    std::io::stdout().write("COMMAND TEXT: ".as_bytes()).unwrap();
	    std::io::stdout().write(line.as_bytes()).unwrap();
	}

	// Add line to previous code
	self.prev_line = text[0].clone().parse::<i64>().unwrap();
	self.prev_code.push((self.prev_line, line.clone()));

	// Execute command specific method
	self.find_subcommand(text, class);
    }

    // Find subcommand to execute
    fn find_subcommand(&mut self, text:Vec<String>, class:Vec<String>) {
	// Check if command is present
	if text.len() == 1 {
	    // Update state
	    self.next_line = -1;
	    return;
	} else if text.len() == 0 {
	    // Update state
	    self.next_line = -1;
	    self.prev_line = -1;
	    return;
	}

	// Set keyword
	let keyword = text[1].clone();

	// Execute given command
	if keyword == "PRINT".to_string() {
	    self.print_cmd(text, class);
	} else if keyword == "GOTO".to_string() {
	    self.goto_cmd(text, class);
	} else if keyword == "LET".to_string() {
	    self.let_cmd(text, class);
	} else if keyword == "IF".to_string() {
	    self.if_cmd(text, class);
	} else if keyword == "END".to_string() {
	    self.end_cmd(text, class);
	} else {
	    // Update state
	    self.next_line = i64::MAX;
	}
    }

    // Implmentation of the PRINT command
    fn print_cmd(&mut self, text:Vec<String>, class:Vec<String>) {
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
		match self.types.get(&text[counter]) {
		    Some(kind)=> {
			// Get and print value
			if kind == &"string".to_string() {
			    match self.strings.get(&text[counter]) {
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
	self.next_line = -1;
    }

    // Implmentation of the GOTO command
    fn goto_cmd(&mut self, text:Vec<String>, _class:Vec<String>) {
	// Update state
	self.next_line = text[2].clone().parse::<i64>().unwrap();
    }

    // Implmentation of the LET command
    fn let_cmd(&mut self, text:Vec<String>, _class:Vec<String>) {
	// Use evaluator
	let eval_output = evaluate(text[2].clone());
	let var_name = eval_output.0;
	let _rel = eval_output.1;
	let val = eval_output.2;
	let kind = eval_output.3;

	// Insert name and type
	self.types.insert(var_name.clone(), kind.clone());

	// Where to store variable
	if kind.clone() == "string".to_string() {
	    let mut string = val.clone();
	    string.pop();   
	    string.remove(0);
	    self.strings.insert(var_name.clone(), string);
	}	

	// Update state
	self.next_line = -1;
    }

    // Implmentation of the IF command
    fn if_cmd(&mut self, text:Vec<String>, _class:Vec<String>) {
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
	match self.types.get(&var_name) {
	    Some(kind)=> {
		// Get and print value
		if kind == &"string".to_string() {
		    match self.strings.get(&var_name) {
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
	self.next_line = goto;
    }

    // Implmentation of the END command
    fn end_cmd(&mut self, _text:Vec<String>, _class:Vec<String>) {
	// Update state
	self.next_line = i64::MAX;
    }
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


