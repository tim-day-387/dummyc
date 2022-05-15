// Data module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

// General Imports
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
    pub fn new(given_text:String) -> Data {
	Data {
	    plain_text:given_text,
	    output_type:"".to_string(),
	    print_out_text:"".to_string(),
	}
    }

    // Simplify data output to one which can be stored and printed out
    pub fn simplify(&mut self, state:State) {
	self.find_output_type();
	
	if self.output_type == "function".to_string() {
	    self.function(state);
	} else if self.output_type == "unresolved".to_string() {
	    self.resolve(state);
	}
	
	self.get_print_out();
    }

    // Execute the given function call
    fn function(&mut self, state:State) {
	let name = split_function(self.plain_text.clone()).0.to_lowercase();
	let arguments = split_arguments(split_function(self.plain_text.clone()).1);
	let location = "./std/".to_string();
	let string_path = format!("{}{}{}", location, name, ".bas".to_string());
	let file_path = Path::new(&string_path);

	// Useful variables
	let mut lim_state = State::new();
	lim_state.print_location = state.print_location;

	// Add arguments
	for args in arguments.clone() {
	    let data = new_simplified(args, state.clone());

	    lim_state.input_args.insert(0, data);
	}

	// Add all lines in the code to prev_code
	lim_state.load_prev(file_path);
	
	// Execute commands given state
	lim_state.exec_prev();

	// Replace self with return value
	*self = lim_state.return_val.clone();
    }
    
    // Resolve any unresolved operations in the expression
    fn resolve(&mut self, state:State) {
	// Split the expression over the operation
	let (first_part, operation, second_part) = split(self.plain_text.clone(), false);

	// If there is no operation, check if there is a variable
	if operation == "".to_string() {
	    self.get_var_value(state);
	    return;
	}

	let mut first_obj:Data = new_simplified(first_part, state.clone());
	let second_obj:Data = new_simplified(second_part, state.clone());

	first_obj.operation(second_obj, operation);

	*self = first_obj.clone();
    }

    // Perform the compare
    pub fn compare(self, other:Data, operation_string:String) -> bool {
	let output_type = self.clone().find_operation_output_type(other.clone());

	if output_type == "int".to_string() || output_type == "float".to_string() {
	    let a = match self.plain_text.parse::<f32>() {
		Ok(i) => i,
		Err(_e) => panic!("DATA: compare: Invalid float"),
	    };
	    let b = match other.plain_text.parse::<f32>() {
		Ok(i) => i,
		Err(_e) => panic!("DATA: compare: Invalid float"),
	    };

	    if operation_string == "<".to_string() {
		return a < b;
	    } else if operation_string == ">".to_string() {
		return a > b;
	    } else {
		return false;
            }
	} else {
	    return false;
	}
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
	} else if output_type == "float".to_string() {
	    let a = match self.plain_text.parse::<f32>() {
		Ok(i) => i,
		Err(_e) => panic!("DATA: operation: Invalid float"),
	    };
	    let b = match other.plain_text.parse::<f32>() {
		Ok(i) => i,
		Err(_e) => panic!("DATA: operation: Invalid float"),
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
	} else {
	    panic!("DATA: operation: Unsupport type");
	}
    }

    // Find output type of an binary operation
    fn find_operation_output_type(self, other:Data) -> String {
	let self_num:bool = self.output_type == "float".to_string() || self.output_type == "int".to_string();
	let other_num:bool = other.output_type == "float".to_string() || other.output_type == "int".to_string();
	
	if self.output_type == other.output_type {
	    return self.output_type;
	} else if self_num && other_num {
	    return "float".to_string();
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
	} else if is_float(self.plain_text.clone()) {
	    self.output_type = "float".to_string();
	} else if is_function(self.plain_text.clone()) {
	    self.output_type = "function".to_string();
	} else {
	    self.output_type = "unresolved".to_string();
	}
    }

    // Get variable value
    fn get_var_value(&mut self, state:State) {
	let var_value:&Data;

	if self.plain_text == "pr_loc".to_string() {
	    self.plain_text = state.print_location.to_string();
	    self.simplify(state.clone());
	    return;
	}
	
	match state.variables.get(&self.plain_text) {
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
	} else if self.output_type == "int".to_string() {
	    match self.plain_text.clone().parse::<i32>() {
		Ok(i) => if i < 0 {
		    self.print_out_text = format!("{}{}", i.to_string(), " ".to_string());
		} else {
		    self.print_out_text = format!("{}{}{}", " ".to_string(), i.to_string(), " ".to_string());
		},
		Err(_e) => panic!("DATA: get_print_out: Invalid integer"),
	    };
	} else if self.output_type == "float".to_string() {
	    match self.plain_text.clone().parse::<f32>() {
		Ok(i) => if i < -1.0 {
		    self.print_out_text = format!("{}{}", i.to_string(), " ".to_string());
		} else if i < 0.0 {
		    self.print_out_text = format!("{}{}", i.to_string(), " ".to_string());
		    self.print_out_text.remove(1);
		} else if i < 1.0 {
		    self.print_out_text = format!("{}{}{}", " ".to_string(), i.to_string(), " ".to_string());
		    self.print_out_text.remove(1);
		} else {
		    self.print_out_text = format!("{}{}{}", " ".to_string(), i.to_string(), " ".to_string());
		},
		Err(_e) => panic!("DATA: get_print_out: Invalid float"),
	    };
	} else {
	    // Just use the plain text if nothing else
	    self.print_out_text = self.plain_text.clone();
	}
    }
}

// Constructor with simplification
pub fn new_simplified(given_text:String, state:State) -> Data {
    let mut output = Data::new(given_text);

    output.simplify(state);

    return output;
}
