// State module
#![forbid(unsafe_code)]

// General Imports
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, Write};


// File Imports
use lexer::*;
use data::*;

// State struct
pub struct State {
    pub variables:HashMap<String, Data>,
    pub prev_code:Vec<(i64, String)>,
    pub next_line:i64,
    pub prev_line:i64,
    pub return_to_line:Vec<i64>,
}

// State implementation
impl State {
    // Constructor
    pub fn new() -> State {
	State {
	    variables:HashMap::new(),
	    prev_code:Vec::new(),
	    next_line:-1,
	    prev_line:-1,
	    return_to_line:Vec::new(),
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
	} else if keyword == "GOSUB".to_string() {
	    self.gosub_cmd(text, class);
	} else if keyword == "RETURN".to_string() {
	    self.return_cmd(text, class);
	} else if keyword == "REM".to_string() {
	    self.rem_cmd(text, class);
	} else if keyword == "STOP".to_string() {
	    self.stop_cmd(text, class);
	} else if keyword == "END".to_string() {
	    self.end_cmd(text, class);
	} else {
	    // Update state
	    self.next_line = i64::MAX;
	}
    }

    // Implmentation of the PRINT command
    fn print_cmd(&mut self, text:Vec<String>, _class:Vec<String>) {
	let mut counter = 2;

	loop {
	    // End if we run out of tokens
	    if counter == text.len() && text[counter - 1].clone() == ";".to_string() {
		break;
	    } else if counter == text.len() {
		println!("");
		break;
            }

	    // Check if we have a punc token
	    if text[counter].clone() == ";".to_string() || text[counter].clone() == ",".to_string() {
		counter = counter + 1;
		continue;
            }
	    
	    // Generate data object
	    let mut object = Data::new(text[counter].clone());

	    // Simplify object
	    object.simplify(self.variables.clone());

	    // Print out text
	    print!("{}", object.print_out_text);			       

	    // Iterate token
	    counter = counter + 1;
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
	// Split statement
	let text_split = split(text[2].clone());
	let var_name = text_split.0;
	let _relational = text_split.1;
	let data = text_split.2;

	// Generate data object
	let mut object = Data::new(data);

	// Simplify object
	object.simplify(self.variables.clone());

	
	// Insert name and type
	self.variables.insert(var_name.clone(), object.clone());

	// Update state
	self.next_line = -1;
    }

    // Implmentation of the IF command
    fn if_cmd(&mut self, text:Vec<String>, _class:Vec<String>) {
	let mut goto = -1;

	// Split statement
	let text_split = split(text[2].clone());
	let dataa = text_split.0;
	let _relational = text_split.1;
	let datab = text_split.2;

	// Generate data objects
	let mut objecta = Data::new(dataa);
	let mut objectb = Data::new(datab);

	// Simplify object
	objecta.simplify(self.variables.clone());
	objectb.simplify(self.variables.clone());

	// Check if equivalent
	if objecta.equals(objectb) {
	    goto = text[4].clone().parse::<i64>().unwrap();
	}

	// Update state
	self.next_line = goto;
    }

    // Implmentation of the GOSUB command
    fn gosub_cmd(&mut self, text:Vec<String>, _class:Vec<String>) {
	// Update state
	self.return_to_line.push(text[0].clone().parse::<i64>().unwrap());
	self.next_line = text[2].clone().parse::<i64>().unwrap();
    }

    // Implmentation of the RETURN command
    fn return_cmd(&mut self, _text:Vec<String>, _class:Vec<String>) {
	// Update state
	match self.return_to_line.pop() {
	    None => println!("Nowhere to return to!"),
	    Some(line_to_return_to) => self.prev_line = line_to_return_to,
	}
	
	self.next_line = -1;
    }
    
    // Implmentation of the REM command
    fn rem_cmd(&mut self, _text:Vec<String>, _class:Vec<String>) {
	// Do nothing, this is just a placeholder
	self.next_line = -1;
    }

    // Implmentation of the STOP command
    fn stop_cmd(&mut self, _text:Vec<String>, _class:Vec<String>) {
	// Update state
	self.next_line = i64::MAX;
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


