// Data module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

// General Imports
use std::collections::HashMap;
use Path;

// File Imports
use expression_lexer::*;
use state::*;

// Data struct
#[derive(PartialEq, Clone)]
pub struct Data {
    pub plain_text:String,
    pub output_type:String,
    pub print_out_text:String,
}

// Data implementation
impl Data {
    // Constructor
    fn new(given_text:String) -> Data {
	Data {
	    plain_text:given_text,
	    output_type:"".to_string(),
	    print_out_text:"".to_string(),
	}
    }

    // Simplify data output to one which can be stored and printed out
    pub fn simplify(&mut self, vars:HashMap<String, Data>) {
	self.find_output_type();
	
	if self.output_type == "function".to_string() {
	    self.function();
	} else if self.output_type == "unresolved".to_string() {
	    self.resolve(vars);
	}
	
	self.get_print_out();
    }

    // Execute the given function call
    fn function(&mut self) {
	let name = split_function(self.plain_text.clone());
	let location = "./std/".to_string();
	let string_path = format!("{}{}{}", location, name, ".bas".to_string());
	let file_path = Path::new(&string_path);

	// Useful variables
	let mut state = State::new();

	// Add all lines in the code to prev_code
	state.load_prev(file_path);
	
	// Execute commands given state
	state.exec_prev();
    }
    
    // Resolve any unresolved operations in the expression
    fn resolve(&mut self, vars:HashMap<String, Data>) {
	// Split the expression over the operation
	let split = split(self.plain_text.clone(), false);
	let first_part_string:String = split.0;
	let operation_string:String = split.1;
	let second_part_string:String = split.2;

	// If there is no operation, check if there is a variable
	if operation_string == "".to_string() {
	    self.get_var_value(vars);
	    return;
	}

	let mut first_obj:Data = new_simplified(first_part_string, vars.clone());
	let second_obj:Data = new_simplified(second_part_string, vars.clone());

	first_obj.operation(second_obj, operation_string);

	*self = first_obj.clone();
    }

    // Perform the operation
    pub fn operation(&mut self, other:Data, operation_string:String) {
	let output_type = self.clone().find_operation_output_type(other.clone());

	if output_type == "string".to_string() {
	    self.plain_text = format!("{}{}{}{}", "\"".to_string(), self.print_out_text.clone(), other.print_out_text.clone(), "\"".to_string());
	} else if output_type == "int".to_string() {
	    let a = match self.plain_text.parse::<i32>() {
		Ok(i) => i,
		Err(_e) => panic!("DATA: operation: Invalid integer"),
	    };
	    let b = match other.plain_text.parse::<i32>() {
		Ok(i) => i,
		Err(_e) => panic!("DATA: operation: Invalid integer"),
	    };

	    if operation_string == "+".to_string() {
		self.plain_text = (a+b).to_string();
	    } else if operation_string == "*".to_string() {
		self.plain_text = (a*b).to_string();
	    } else if operation_string == "-".to_string() {
		self.plain_text = (a-b).to_string();
	    } else {
		panic!("DATA: operation: Invalid operation");
            } 
	}
    }

    // Find output type of an binary operation
    fn find_operation_output_type(self, other:Data) -> String {
	if self.output_type == other.output_type {
	    return self.output_type;
	} else {
	    panic!("DATA: find_operation_output_type: Incompatible types");
	}
    }
    
    // Determine output type
    fn find_output_type(&mut self) {
	// Series of cases to find type
	if is_string(self.plain_text.clone()) {
	    self.output_type = "string".to_string();
	} else if is_int(self.plain_text.clone()) {
	    self.output_type = "int".to_string();
	} else if is_function(self.plain_text.clone()) {
	    self.output_type = "function".to_string();
	} else {
	    self.output_type = "unresolved".to_string();
	}
    }

    // Get variable value
    fn get_var_value(&mut self, vars:HashMap<String, Data>) {
	let var_value:&Data;
	
	match vars.get(&self.plain_text) {
	    Some(value)=> var_value = value,
	    _=> panic!("DATA: get_var_value: Variable does not exist"),
	}

	*self = var_value.clone();
    }

    // Find text to be printed out
    fn get_print_out(&mut self) {
	// Series of cases to get output_string
	if self.output_type == "string".to_string() {
	    // Use the plain text but remove parans
	    self.print_out_text = self.plain_text.clone();
	    self.print_out_text.pop();
	    self.print_out_text.remove(0);
	} else {
	    // Just use the plain text if nothing else
	    self.print_out_text = self.plain_text.clone();
	}
    }
}

// Constructor with simplification
pub fn new_simplified(given_text:String, vars:HashMap<String, Data>) -> Data {
    let mut output = Data::new(given_text);

    output.simplify(vars);

    return output;
}
