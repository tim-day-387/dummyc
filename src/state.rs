// State module
#![forbid(unsafe_code)]

// General Imports
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::cmp;
use std::io::{self, BufRead};

// File Imports
use lexer::*;
use expression_lexer::*;
use data::*;

// State struct
#[derive(PartialEq, Clone)]
pub struct State {
    pub input_args:Vec<Data>,
    pub return_val:Data,
    pub variables:HashMap<String, Data>,
    pub prev_code:Vec<(i64, String)>,
    pub next_line:i64,
    pub prev_line:i64,
    pub return_to_line:Vec<i64>,
    pub for_return_to_line:HashMap<String, i64>,
    pub print_location:i64,
    pub print_zone:i64,
}

// State implementation
impl State {
    // Constructor
    pub fn new() -> State {
	State {
	    input_args:Vec::new(),
	    return_val:Data::new("".to_string()),
	    variables:HashMap::new(),
	    prev_code:Vec::new(),
	    next_line:-1,
	    prev_line:-1,
	    return_to_line:Vec::new(),
	    for_return_to_line:HashMap::new(),
	    print_location:0,
	    print_zone:1,
	}
    }

    // Add line for prev code
    pub fn add_prev(&mut self, line:String) {
	// Lex command
	let text:Vec<String> = perform_lexing(line.clone());
	let prev_line;
	
	// Record line
	prev_line = text[0].clone().parse::<i64>().unwrap();
	self.prev_code.push((prev_line, line.clone()));
    }
    
    // Load previous commmands from a file
    pub fn load_prev(&mut self, file_path:&Path) {
	// Useful variables 
	let mut first_token;

	// Add all lines in the code to prev_code
	if let Ok(lines) = read_lines(file_path) {
            for line in lines {
		if let Ok(ip) = line {
		    first_token = perform_lexing(ip.clone())[0].clone();
		    if !is_shebang(first_token.clone()) {
			self.prev_code.push((first_token.parse::<i64>().unwrap(), ip.clone()));
		    }
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
		self.exec_command(command.clone());
            }
	}
    }

    // Execute the given command
    fn exec_command(&mut self, line:String) {
	// Lex command
	let text:Vec<String> = perform_lexing(line.clone());

	// Check for shebang, and do nothing
	if text[0].clone() == "#!/usr/bin/dummyc" {
	    return;
	}

	// Add line to previous code
	self.prev_line = text[0].clone().parse::<i64>().unwrap();

	// Execute command specific method
	self.find_subcommand(text);
    }

    // Find subcommand to execute
    fn find_subcommand(&mut self, text:Vec<String>) {
	// Check if command is present, else do nothing
	if text.len() <= 1 {return;}

	// Set keyword
	let keyword = text[1].clone().to_uppercase();

	// Execute given command
	if keyword == "FUNCTION".to_string() {self.function_cmd(text);}
	else if keyword == "INPUT".to_string() {self.input_cmd(text);}
	else if keyword == "PRINT".to_string() {self.print_cmd(text);}
	else if keyword == "GOTO".to_string() {self.goto_cmd(text);}
	else if keyword == "LET".to_string() {self.let_cmd(text);}
	else if keyword == "IF".to_string() {self.if_cmd(text);}
	else if keyword == "GOSUB".to_string() {self.gosub_cmd(text);}
	else if keyword == "RETURN".to_string() {self.return_cmd(text);}
	else if keyword == "FOR".to_string() {self.for_cmd(text);}
	else if keyword == "NEXT".to_string() {self.next_cmd(text);}
	else if keyword == "REM".to_string() {self.rem_cmd(text);}
	else if keyword == "STOP".to_string() {self.stop_cmd(text);}
	else if keyword == "END".to_string() {self.end_cmd(text);}
	else {self.next_line = i64::MAX;}
    }

    // Implementation of the INPUT command
    fn input_cmd(&mut self, text:Vec<String>) {
	let mut line = "".to_string();	
	let mut counter = 2;
	let needed = (((text.len() - 2) - 1) / 2) + 1;
	
	// Collect input
	std::io::stdin().read_line(&mut line).unwrap();
	line = line.to_string().replace("\n", "");

	// Parse input
	let mut input:Vec<String> = split_arguments(line).into_iter().rev().collect();
	let given = input.len();
	
	loop {
	    // End if we run out of tokens
	    if counter == text.len() {
		break;
	    }

	    // Check if we have a punc token
	    if text[counter].clone() == ",".to_string() {
		counter = counter + 1;
		continue;
	    }

	    // Generate data object
	    let data_string:String;
	    
	    match input.pop() {
		Some(value)=> data_string = value,
		_=> panic!("STATE: function_cmd: Not enough input arguments, have {} and need {}", given, needed),
	    }

	    let data:Data = new_simplified(data_string, self.clone());

	    // Insert name and type
	    self.variables.insert(text[counter].clone(), data);

	    // Iterate token
	    counter = counter + 1;
	}

	// Check if we had too many args
	if self.input_args.len() != 0 {
	    panic!("STATE: function_cmd: Too many input arguments, have {} and need {}", given, needed);
	}

	// Update state
	self.next_line = -1;
    }
	
    // Implmentation of the FUNCTION command
    fn function_cmd(&mut self, text:Vec<String>) {
	let given = self.input_args.len();
	let needed = (((text.len() - 2) - 1) / 2) + 1;

	if text[2] == "RETURN".to_string() || text[2] == "return".to_string() {
	    let var_value:&Data;
	    
	    match self.variables.get(&text[3]) {
		Some(value)=> var_value = value,
		_=> panic!("STATE: function_cmd: Variable does not exist"),
	    }

	    self.return_val = var_value.clone();
	} else {
	    let mut counter = 2;

	    loop {
		// End if we run out of tokens
		if counter == text.len() {
		    break;
		}

		// Check if we have a punc token
		if text[counter].clone() == ",".to_string() {
		    counter = counter + 1;
		    continue;
		}

		// Generate data object
		let arg:Data;
		
		match self.input_args.pop() {
		    Some(value)=> arg = value,
		    _=> panic!("STATE: function_cmd: Not enough input arguments, have {} and need {}", given, needed),
		}

		// Insert name and type
		self.variables.insert(text[counter].clone(), arg.clone());

		// Iterate token
		counter = counter + 1;
	    }	    
	}

	// Check if we had too many args
	if self.input_args.len() != 0 {
	    panic!("STATE: function_cmd: Too many input arguments, have {} and need {}", given, needed);
	}

	// Update state
	self.next_line = -1;
    }
    
    // Implmentation of the PRINT command
    fn print_cmd(&mut self, text:Vec<String>) {
	let mut counter = 2;
	let zone_len = 15;
	let num_zones = 4;
	let width = 60;

	loop {
	    // End if we run out of tokens
	    if counter == text.len() && text[counter - 1].clone() == ";".to_string() {
		break;
	    } else if counter == text.len() {
		self.print_location = 0;
		println!("");
		break;
            }

	    // Check if we have a punc token
	    if text[counter].clone() == ";".to_string() {
		counter += 1;
		continue;
            } else if text[counter].clone() == ",".to_string() {
		self.print_zone = ((self.print_location - (self.print_location % zone_len)) / zone_len) % num_zones + 1;

		for _i in 0..cmp::max((self.print_zone * zone_len) - self.print_location, 0) {
		    print!(" ");
		    self.print_location = (self.print_location + 1) % width;
		}
		
		counter += 1;
		continue;
            }
	    
	    // Generate data object
	    let object = new_simplified(text[counter].clone(), self.clone());

	    // Print out text
	    print!("{}", object.print_out_text);
	    self.print_location = (self.print_location + object.print_out_text.len() as i64) % width;

	    // Iterate token
	    counter = counter + 1;
	}
	
	// Update state
	self.next_line = -1;
    }

    // Implmentation of the GOTO command
    fn goto_cmd(&mut self, text:Vec<String>) {
	// Update state
	self.next_line = text[2].clone().parse::<i64>().unwrap();
    }

    // Implmentation of the LET command
    fn let_cmd(&mut self, text:Vec<String>) {
	// Split statement
	let (var_name, _relational, data) = split(text[2].clone(), true);

	// Generate data object
	let object = new_simplified(data, self.clone());

	// Insert name and type
	self.variables.insert(var_name.clone(), object.clone());

	// Update state
	self.next_line = -1;
    }

    // Implmentation of the IF command
    fn if_cmd(&mut self, text:Vec<String>) {
	let mut goto = -1;

	// Split statement
	let (dataa, relational, datab) = split(text[2].clone(), true);

	// Generate data objects
	let objecta = new_simplified(dataa, self.clone());
	let objectb = new_simplified(datab, self.clone());

	// Check if equivalent
	if relational == "=".to_string() && objecta.eq(&objectb) {
	    goto = text[4].clone().parse::<i64>().unwrap();
	} else if relational == "<>".to_string() && !objecta.eq(&objectb) {
	    goto = text[4].clone().parse::<i64>().unwrap();
	} else if objecta.compare(objectb, relational) {
	    goto = text[4].clone().parse::<i64>().unwrap();
	}

	// Update state
	self.next_line = goto;
    }

    // Implmentation of the GOSUB command
    fn gosub_cmd(&mut self, text:Vec<String>) {
	// Update state
	self.return_to_line.push(text[0].clone().parse::<i64>().unwrap());
	self.next_line = text[2].clone().parse::<i64>().unwrap();
    }

    // Implmentation of the RETURN command
    fn return_cmd(&mut self, _text:Vec<String>) {
	// Update state
	match self.return_to_line.pop() {
	    None => panic!("STATE: return_cmd: Nowhere to return to"),
	    Some(line_to_return_to) => self.prev_line = line_to_return_to,
	}
	
	self.next_line = -1;
    }

    // Implmentation of the FOR command
    fn for_cmd(&mut self, text:Vec<String>) {
	let cur_value:Data;
	
	// Parse step
	let step:Data = new_simplified("1".to_string(), self.clone());
	
	// Split statement
	let (var_name, _relational, data) = split(text[2].clone(), true);

	// Check if exists, then add if not
	match self.variables.get(&var_name) {
	    Some(value)=> {
		// Advance counter by one step
		let mut var_value = value.clone();
		var_value.operation(step, "+".to_string());
		var_value.simplify(self.clone());
		cur_value = var_value.clone();
		self.variables.insert(var_name.clone(), var_value);
	    },
	    _=> {
		// Create counter for the first time
		let object = new_simplified(data, self.clone());
		cur_value = object.clone();
		self.variables.insert(var_name.clone(), object);
	    },
	}

	// Final allowed value
	let limit = new_simplified(text[4].clone(), self.clone());

	if !cur_value.eq(&limit) {
	    self.for_return_to_line.insert(var_name.clone(), text[0].clone().parse::<i64>().unwrap());
	}
	
	// Update state
	self.next_line = -1;
    }

    // Implmentation of the NEXT command
    fn next_cmd(&mut self, text:Vec<String>) {
	let var_name = text[2].clone();
	
	// Check if exists, and set next_line
	match self.for_return_to_line.get(&var_name) {
	    Some(value)=> {
		// Return to FOR
		self.next_line = value.clone();
	    },
	    _=> {
		// Keep going, remove variable
		self.variables.remove(&var_name);
		self.next_line = -1;
	    },
	}

	// Remove line to return to
	self.for_return_to_line.remove(&var_name);
    }
    
    // Implmentation of the REM command
    fn rem_cmd(&mut self, _text:Vec<String>) {
	// Do nothing, this is just a placeholder
	self.next_line = -1;
    }

    // Implmentation of the STOP command
    fn stop_cmd(&mut self, _text:Vec<String>) {
	// Update state
	self.next_line = i64::MAX;
    }
    
    // Implmentation of the END command
    fn end_cmd(&mut self, _text:Vec<String>) {
	// Update state
	self.next_line = i64::MAX;
    }
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


