// State module
#![forbid(unsafe_code)]

// General Imports
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp;

// File Imports
use data::Data;
use types::find_type;
use errors::stateless_error;
use lexer::{is_shebang, perform_multi_lexing, split_line_number};
use expression_lexer::{split, split_function, split_arguments};

// Constants
const ZONE_LEN:i64 = 15;
const NUM_ZONES:i64 = 4;
const WIDTH:i64 = 60;

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
    pub array_offset:i64
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
	    array_offset:0
	}
    }

    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
    }
    
    // Add line for prev code
    pub fn add_prev(&mut self, line:String) {
	let prev_line = split_line_number(line.clone()).0.parse::<i64>().unwrap();
	
	self.prev_code.push((prev_line, line.clone()));
    }
    
    // Load previous commmands from a file
    pub fn load_prev(&mut self, file_path:&Path) {
	// Useful variables 
	let mut first_token;

	// Add all lines in the code to prev_code
	if let Ok(lines) = State::read_lines(file_path) {
            for line in lines {
		if let Ok(ip) = line {
		    first_token = split_line_number(ip.clone()).0;
		    if first_token.clone() != "".to_string() {
			self.prev_code.push((first_token.parse::<i64>().unwrap(), ip.clone()));
		    } else if !is_shebang(ip.clone()) {
			let artifacts = [].to_vec();
			let artifact_names = [].to_vec();
			let function_name = "load_prev".to_string();
			let message = "Line has no line number.".to_string();
			stateless_error(artifacts, artifact_names, function_name, message);
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
	let commands:Vec<Vec<String>> = perform_multi_lexing(line.clone());

	// Check for shebang, and do nothing
	if is_shebang(commands[0][0].clone()) {return;}

	// Add line to previous code, set next line
	self.prev_line = commands[0][0].clone().parse::<i64>().unwrap();
	self.next_line = -1;

	// Save first line redirect
	let mut found_next_line = false;
	let mut next_line = -1;

	// Execute command specific method
	for command in commands {
	    self.find_subcommand(command);

	    if self.next_line != -1 && !found_next_line {
		found_next_line = true;
		next_line = self.next_line;
	    }
	}

	// Set next line
	self.next_line = next_line;
    }

    // Find subcommand to execute
    fn find_subcommand(&mut self, text:Vec<String>) {
	// Check if command is present, else do nothing
	if text.len() <= 1 {return;}

	// Set keyword
	let keyword = text[1].clone().to_uppercase();

	// Execute given command
	if keyword == "FUNCTION".to_string() {self.function_cmd(text);}
	else if keyword == "DIM".to_string() {self.dim_cmd(text);}
	else if keyword == "OPTION".to_string() {self.option_cmd(text);}
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

    // Move to next print zone
    fn pt_next_print_zone(&mut self) {
	self.print_zone = ((self.print_location - (self.print_location % ZONE_LEN)) / ZONE_LEN) % NUM_ZONES + 1;

	for _i in 0..cmp::max((self.print_zone * ZONE_LEN) - self.print_location, 0) {
	    print!(" ");
	    self.print_location = (self.print_location + 1) % WIDTH;
	}
    }

    // Go to next line
    fn pt_next_line(&mut self) {
	self.print_location = 0;
	println!("");
    }

    // Conditionally go to next line
    fn pt_cond_next_line(&mut self) {
	if self.print_location != 0 {
	    self.print_location = 0;
	    println!("");
	}
    }

    // Print out
    fn pt_output_text(&mut self, text:String) {
	print!("{}", text);
	self.print_location = (self.print_location + text.len() as i64) % WIDTH;
    }

    // Implementation of the OPTION comand
    fn option_cmd(&mut self, text:Vec<String>) {
	let offset:Data = Data::new_simplified(text[3].clone(), self.clone());

	self.array_offset = match offset.plain_text.parse::<i64>() {
	    Ok(i) => i,
	    Err(_e) => {
		let artifacts = [].to_vec();
		let artifact_names = [].to_vec();
		let function_name = "option_cmd".to_string();
		let message = "Base is not an integer.".to_string();
		stateless_error(artifacts, artifact_names, function_name, message);
		-1
	    }
	};

	// Update state
	self.next_line = -1;
    }

    // Implementation of the DIM command
    fn dim_cmd(&mut self, text:Vec<String>) {
	// Split statement
	let name = split_function(text[2].clone()).0;
	let arguments = split_arguments(split_function(text[2].clone()).1);

	if arguments.len() != 1 {
	    let artifacts = [].to_vec();
	    let artifact_names = [].to_vec();
	    let function_name = "dim_cmd".to_string();
	    let message = "Wrong number of arguments.".to_string();
	    stateless_error(artifacts, artifact_names, function_name, message);
	}

	let size_string = Data::new_simplified(arguments[0].clone(), self.clone()).plain_text;
	let size = match size_string.parse::<i64>() {
	    Ok(i) => i,
	    Err(_e) => {
		let artifacts = [].to_vec();
		let artifact_names = [].to_vec();
		let function_name = "dim_cmd".to_string();
		let message = "Invalid integer.".to_string();
		stateless_error(artifacts, artifact_names, function_name, message);
		return;
            }
	};

	for i in 0..size {
	    let data_dummy = Data::new("".to_string());

	    // Insert name and type
	    self.variables.insert(format!("{}{}{}{}", name, "(", (i + self.array_offset), ")"), data_dummy.clone());
	}

	// Update state
	self.next_line = -1;
    }

    // Implementation of the INPUT command
    fn input_cmd(&mut self, text:Vec<String>) {
        let mut line = "".to_string();

	self.pt_cond_next_line();

	for counter in 2..text.len() {
	    if text[counter].clone() == ",".to_string() || text[counter].clone() == ";".to_string() {
		continue;
	    } else if find_type(text[counter].clone()) == 3000 {
                let object = Data::new_simplified(text[counter].clone(), self.clone());

		self.pt_output_text(object.print_out_text);
		self.pt_next_line();
            } else {
		std::io::stdin().read_line(&mut line).unwrap();
		line = line.to_string().replace("\n", "");

		let data:Data = Data::new_simplified(line.clone(), self.clone());

		self.variables.insert(text[counter].clone(), data);
		line = "".to_string();
	    }
	}

	// Check if we had too many args
	if self.input_args.len() != 0 {
	    let artifacts = [].to_vec();
	    let artifact_names = [].to_vec();
	    let function_name = "function_cmd".to_string();
	    let message = "Too many input arguments.".to_string();
	    stateless_error(artifacts, artifact_names, function_name, message);
	}

	// Update state
	self.next_line = -1;
    }
	
    // Implmentation of the FUNCTION command
    fn function_cmd(&mut self, text:Vec<String>) {
	let given = self.input_args.len();
	let needed = (((text.len() - 2) - 1) / 2) + 1;

	if text[2] == "RETURN".to_string() || text[2] == "return".to_string() {
	    let mut var_value:&Data = &Data::new("".to_string());
	    
	    match self.variables.get(&text[3]) {
		Some(value)=> var_value = value,
		_=> {
		    let artifacts = [].to_vec();
		    let artifact_names = [].to_vec();
		    let function_name = "function_cmd".to_string();
		    let message = "Variable does not exist.".to_string();
		    stateless_error(artifacts, artifact_names, function_name, message);
		}
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
		let mut arg:Data = Data::new("".to_string());
		
		match self.input_args.pop() {
		    Some(value)=> arg = value,
		    _=> {
			let artifacts = [given.to_string(), needed.to_string()].to_vec();
			let artifact_names = ["given".to_string(), "needed".to_string()].to_vec();
			let function_name = "function_cmd".to_string();
			let message = "Not enough input arguments.".to_string();
			stateless_error(artifacts, artifact_names, function_name, message);
		    }
		}

		// Insert name and type
		self.variables.insert(text[counter].clone(), arg.clone());

		// Iterate token
		counter = counter + 1;
	    }	    
	}

	// Check if we had too many args
	if self.input_args.len() != 0 {
	    let artifacts = [given.to_string(), needed.to_string()].to_vec();
	    let artifact_names = ["given".to_string(), "needed".to_string()].to_vec();
	    let function_name = "function_cmd".to_string();
	    let message = "Too many input arguments.".to_string();
	    stateless_error(artifacts, artifact_names, function_name, message);
	}

	// Update state
	self.next_line = -1;
    }

    // Implmentation of the PRINT command
    fn print_cmd(&mut self, text:Vec<String>) {
	for counter in 2..text.len() {
	    if text[counter].clone() == ";".to_string() {
		continue;
            } else if text[counter].clone() == ",".to_string() {
		self.pt_next_print_zone();
		continue;
            }
	    
	    let object = Data::new_simplified(text[counter].clone(), self.clone());

	    self.pt_output_text(object.print_out_text);
	}

	if text[text.len() - 1].clone() != ";".to_string() {
	    self.pt_next_line();
	}
	
	// Update state
	self.next_line = -1;
    }

    // Implmentation of the LET command
    fn let_cmd(&mut self, text:Vec<String>) {
	// Split statement
	let (var_name, _relational, data) = split(text[2].clone(), true, true);

	// Generate data object
	let object = Data::new_simplified(data, self.clone());

	if find_type(var_name.clone()) == 2000 {
            let array_ref = Data::get_array_reference(var_name.clone(), self.clone());
	    self.variables.insert(array_ref.clone(), object.clone());
	} else {
	    self.variables.insert(var_name.clone(), object.clone());
	}

	// Update state
	self.next_line = -1;
    }

    // Implmentation of the IF command
    fn if_cmd(&mut self, text:Vec<String>) {
	let mut goto = -1;

	// Split statement
	let (dataa, relational, datab) = split(text[2].clone(), true, true);

	// Generate data objects
	let objecta = Data::new_simplified(dataa, self.clone());
	let objectb = Data::new_simplified(datab, self.clone());

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
	    None => {
		let artifacts = [].to_vec();
		let artifact_names = [].to_vec();
		let function_name = "return_cmd".to_string();
		let message = "Nowhere to return to.".to_string();
		stateless_error(artifacts, artifact_names, function_name, message);
	    },
	    Some(line_to_return_to) => self.prev_line = line_to_return_to,
	}
	
	self.next_line = -1;
    }

    // Implmentation of the FOR command
    fn for_cmd(&mut self, text:Vec<String>) {
	let zero:Data = Data::new_simplified("0".to_string(), self.clone());
	let cur_value:Data;
	let step:Data;
	
	// Parse step
	if text.len() == 5 {
	    step = Data::new_simplified("1".to_string(), self.clone());
	} else {
	    step = Data::new_simplified(text[6].clone(), self.clone());
	}
	
	// Split statement
	let (var_name, _relational, data) = split(text[2].clone(), true, true);

	// Check if exists, then add if not
	match self.variables.get(&var_name) {
	    Some(value)=> {
		// Advance counter by one step
		let mut var_value = value.clone();
		var_value.operation(step.clone(), "+".to_string());
		var_value.simplify(self.clone());
		cur_value = var_value.clone();
		self.variables.insert(var_name.clone(), var_value);
	    },
	    _=> {
		// Create counter for the first time
		let object = Data::new_simplified(data, self.clone());
		cur_value = object.clone();
		self.variables.insert(var_name.clone(), object);
	    },
	}

	// Final allowed value
	let limit = Data::new_simplified(text[4].clone(), self.clone());
	let negative = step.clone().compare(zero.clone(), "<".to_string());

	if cur_value.clone().compare(limit.clone(), "<".to_string()) && !negative {
	    self.for_return_to_line.insert(var_name.clone(), text[0].clone().parse::<i64>().unwrap());
	} else if cur_value.clone().compare(limit.clone(), ">".to_string()) && negative {
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

    // Implmentation of the GOTO command
    fn goto_cmd(&mut self, text:Vec<String>) {self.next_line = text[2].clone().parse::<i64>().unwrap();}
    
    // Implmentation of the REM command
    fn rem_cmd(&mut self, _text:Vec<String>) {self.next_line = -1;}

    // Implmentation of the STOP command
    fn stop_cmd(&mut self, _text:Vec<String>) {self.next_line = i64::MAX;}
    
    // Implmentation of the END command
    fn end_cmd(&mut self, _text:Vec<String>) {self.next_line = i64::MAX;}
}
