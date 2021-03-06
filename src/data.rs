// Data module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

// General Imports
use Path;
use rand::Rng;
use std::cmp;

// File Imports
use state::State;
use types::find_type;
use errors::stateless_error;
use expression_lexer::{split, split_function, split_arguments};

// Data struct
#[derive(PartialEq, Clone)]
pub struct Data {
    pub plain_text:String,
    pub output_type:i64,
    pub print_out_text:String,
}

// Data implementation
impl Data {
    // Constructor with simplification
    pub fn new_simplified(given_text:String, state:State) -> Data {
	let mut output = Data::new(given_text.clone());

	output.simplify(state);

	if given_text.clone() != "".to_string() && output.plain_text.clone() == "".to_string() {
	    let artifacts = [given_text.clone()].to_vec();
	    let artifact_names = ["token".to_string()].to_vec();
	    let function_name = "new_simplified".to_string();
	    let message = "Cannot reduce to empty data object.".to_string();
	    stateless_error(artifacts, artifact_names, function_name, message);
	}

	return output;
    }

    // Constructor
    pub fn new(given_text:String) -> Data {
	Data {
	    plain_text:given_text,
	    output_type:-1, // -1 - null; 0 - expression; 1000 - symbol; 2000 symbol_callable; 3000 - string; 4001-3 - int, float, sci_float
	    print_out_text:"".to_string(),
	}
    }

    // Invalid float error
    fn error_invalid_float(function_name:String) {
	let artifacts = [].to_vec();
	let artifact_names = [].to_vec();
	let message = "Invalid float.".to_string();
	stateless_error(artifacts, artifact_names, function_name, message);    
    }

    // Invalid int error
    fn error_invalid_int(function_name:String) {
	let artifacts = [].to_vec();
	let artifact_names = [].to_vec();
	let message = "Invalid integer.".to_string();
	stateless_error(artifacts, artifact_names, function_name, message);    
    }

    // Divide by zero error
    fn error_divide_zero(function_name:String) {
	let artifacts = [].to_vec();
	let artifact_names = [].to_vec();
	let message = "Attempted to divide by zero.".to_string();
	stateless_error(artifacts, artifact_names, function_name, message);
    }

    // Simplify data output to one which can be stored and printed out
    pub fn simplify(&mut self, state:State) {
	self.find_output_type();
	
	if self.output_type == 2000 { // symbol_callable
	    self.resolve_callable(state.clone());
	} else if self.output_type == 1000 { // symbol
	    self.resolve_symbol(state);
	} else if self.output_type == 0 { // expression
	    self.resolve_expression(state.clone());
	}
	
	self.get_print_out();
    }

    // Resolve array reference in actual var name
    pub fn get_array_reference(given:String, state:State) -> String {
	let name = split_function(given.clone()).0.to_lowercase();
	let arguments = split_arguments(split_function(given.clone()).1);
	let location;
	let mut text = "".to_string();

	if arguments.len() == 1 {
	    let location_string = Data::new_simplified(arguments[0].clone(), state.clone()).plain_text;
	    
	    location = match location_string.parse::<i64>() {
		Ok(i) => i,
		Err(_e) => -1,
	    };
	    
	    text = format!("{}{}{}{}", name, "(", location, ")");
	}

	return text;
    }

    // Resolve symbol_callable type data
    fn resolve_callable(&mut self, state:State) {
	let name = split_function(self.plain_text.clone()).0.to_lowercase();
	let arguments = split_arguments(split_function(self.plain_text.clone()).1);
	let array_ref = Data::get_array_reference(self.plain_text.clone(), state.clone());

	if name == "int".to_string() && arguments.len() == 1 {
	    let arg_data = Data::new_simplified(arguments[0].clone(), state.clone());
	    let mut number:i64 = -1;

	    match arg_data.plain_text.parse::<f64>() {
		Ok(i) => number = i.round() as i64,
		Err(_e) => Data::error_invalid_float("resolve_callable".to_string()),
	    };

	    *self = Data::new_simplified(number.to_string(), state);
	} else if Data::does_var_exist(array_ref.clone(), state.clone()) {
	    self.plain_text = array_ref.clone();
	    self.get_var_value(state.clone());
	} else {
	    self.function(state, name, arguments);
	}
    }

    // Resolve symbol type data
    fn resolve_symbol(&mut self, state:State) {
	if self.plain_text.to_lowercase() == "rnd".to_string() {
	    let mut rng = rand::thread_rng();
	    let number:f64 = rng.gen();
	    *self = Data::new_simplified(number.to_string(), state);
	} else if self.plain_text.to_lowercase() == "prloc".to_string() {
	    self.plain_text = state.print_location.to_string();
	    self.simplify(state.clone());
	} else {
	    self.get_var_value(state);
	}
    }

    // Execute the given function call
    fn function(&mut self, state:State, name:String, arguments:Vec<String>) {
	let location_a = "./std/".to_string();
	let location_b = "/usr/lib/dummyc/std/".to_string();
	let string_path_a = format!("{}{}{}", location_a, name, ".bas".to_string());
	let string_path_b = format!("{}{}{}", location_b, name, ".bas".to_string());
	let file_path_a = Path::new(&string_path_a);
	let file_path_b = Path::new(&string_path_b);
	let file_path:&Path;

	if file_path_a.exists() {file_path = file_path_a;}
	else {file_path = file_path_b;}

	// Useful variables
	let mut lim_state = State::new();
	lim_state.print_location = state.print_location;

	// Add arguments
	for args in arguments.clone() {
	    let data = Data::new_simplified(args, state.clone());

	    lim_state.input_args.insert(0, data);
	}

	// Add all lines in the code to prev_code
	lim_state.load_prev(file_path);
	
	// Execute commands given state
	lim_state.exec_all_scans();

	// Replace self with return value
	*self = lim_state.return_val.clone();
    }
    
    // Resolve any unresolved operations in the expression
    fn resolve_expression(&mut self, state:State) {
	let (first_part, operation, second_part) = split(self.plain_text.clone(), false, true);

	let mut first_obj:Data = Data::new_simplified(first_part, state.clone());
	let second_obj:Data = Data::new_simplified(second_part, state.clone());

	first_obj.operation(second_obj, operation);
	first_obj.simplify(state);

	*self = first_obj.clone();
    }

    // Find output type from an operation
    fn find_operation_output_type(self, other:Data) -> i64 {
	if (self.output_type - other.output_type).abs() > 500 {
	    let artifacts = [].to_vec();
	    let artifact_names = [].to_vec();
	    let function_name = "find_operation_output_type".to_string();
	    let message = "Incompatible types.".to_string();
	    stateless_error(artifacts, artifact_names, function_name, message);
	    return -1;
	} else {
	    return cmp::max(self.output_type, other.output_type);
	}
    }
    
    // Perform the compare
    pub fn compare(self, other:Data, operation_string:String) -> bool {
	let output_type = self.clone().find_operation_output_type(other.clone());

	if output_type == 4001 || output_type == 4002 || output_type == 4003 { // int or float or sci_float
	    let a = match self.plain_text.parse::<f64>() {Ok(i) => i, Err(_e) => {Data::error_invalid_float("compare".to_string()); return false;}};
	    let b = match other.plain_text.parse::<f64>() {Ok(i) => i, Err(_e) => {Data::error_invalid_float("compare".to_string()); return false;}};

	    if operation_string == "<".to_string() {
		return a < b;
	    } else if operation_string == ">".to_string() {
		return a > b;
	    } else if operation_string == ">=".to_string() {
		return a >= b;
	    } else if operation_string == "<=".to_string() {
		return a <= b;
	    } else {
		return false;
            }
	} else {
	    return false;
	}
    }
    
    // Perform the operation
    pub fn operation(&mut self, other:Data, operation_string:String) {
	let output_type:i64 = self.clone().find_operation_output_type(other.clone());

	if output_type == 3000 { // string
	    self.plain_text = format!("{}{}{}{}", "\"".to_string(), self.print_out_text.clone(), other.print_out_text.clone(), "\"".to_string());
	} else if output_type == 4001 { // int
	    let a = match self.plain_text.parse::<i64>() {Ok(i) => i, Err(_e) => {Data::error_invalid_int("operation".to_string()); return;}};
	    let b = match other.plain_text.parse::<i64>() {Ok(i) => i, Err(_e) => {Data::error_invalid_int("operation".to_string()); return;}};

	    if operation_string == "+".to_string() {
		self.plain_text = (a+b).to_string();
	    } else if operation_string == "*".to_string() {
		self.plain_text = (a*b).to_string();
	    } else if operation_string == "-".to_string() {
		self.plain_text = (a-b).to_string();
	    } else if operation_string == "/".to_string() {
		if b == 0 {
		    Data::error_divide_zero("operation".to_string());
		} else {
		    self.plain_text = (a/b).to_string();
		}
	    } else if operation_string == "^".to_string() {
		self.plain_text = (a as f64).powf(b as f64).to_string();
	    } else {
		let artifacts = [].to_vec();
		let artifact_names = [].to_vec();
		let function_name = "operation".to_string();
		let message = "Invalid operation.".to_string();
		stateless_error(artifacts, artifact_names, function_name, message);
            } 
	} else if output_type == 4002 || output_type == 4003 { // float or sci_float
	    let a = match self.plain_text.parse::<f64>() {Ok(i) => i, Err(_e) => {Data::error_invalid_float("operation".to_string()); return;}};
	    let b = match other.plain_text.parse::<f64>() {Ok(i) => i, Err(_e) => {Data::error_invalid_float("operation".to_string()); return;}};

	    if operation_string == "+".to_string() {
		self.plain_text = (a+b).to_string();
	    } else if operation_string == "*".to_string() {
		self.plain_text = (a*b).to_string();
	    } else if operation_string == "-".to_string() {
		self.plain_text = (a-b).to_string();
	    } else if operation_string == "/".to_string() {
		if b == 0.0 {
		    Data::error_divide_zero("operation".to_string());
		} else {
		    self.plain_text = (a/b).to_string();
		}
	    } else if operation_string == "^".to_string() {
		self.plain_text = a.powf(b).to_string();
	    } else {
		let artifacts = [].to_vec();
		let artifact_names = [].to_vec();
		let function_name = "operation".to_string();
		let message = "Invalid operation.".to_string();
		stateless_error(artifacts, artifact_names, function_name, message);
            }
	} else {
	    let artifacts = [].to_vec();
	    let artifact_names = [].to_vec();
	    let function_name = "operation".to_string();
	    let message = "Unsupported type.".to_string();
	    stateless_error(artifacts, artifact_names, function_name, message);
	}
    }

    // Determine output type
    fn find_output_type(&mut self) {
	self.output_type = find_type(self.plain_text.clone());
    }

    // Get variable value
    fn get_var_value(&mut self, state:State) {
	let mut var_value:&Data = &Data::new("".to_string());
	
	match state.variables.get(&self.plain_text) {
	    Some(value)=> var_value = value,
	    _=> {
		let artifacts = [self.plain_text.clone()].to_vec();
		let artifact_names = ["variable".to_string()].to_vec();
		let function_name = "get_var_value".to_string();
		let message = "Variable does not exist.".to_string();
		stateless_error(artifacts, artifact_names, function_name, message);
	    }
	}

	*self = var_value.clone();
    }

    // Does a variable by that name exist?
    fn does_var_exist(name:String, state:State) -> bool {
	match state.variables.get(&name) {
	    Some(_value)=> return true,
	    _=> return false,
	}
    }

    // Find text to be printed out, handle formatting
    fn get_print_out(&mut self) {
	if self.output_type == 3000 { // string
	    self.print_out_text = self.plain_text.clone();
	    self.print_out_text.pop();
	    self.print_out_text.remove(0);
	} else if self.output_type == 4001 { // int
	    match self.plain_text.clone().parse::<i64>() {
		Ok(i) => if i < 0 {
		    self.print_out_text = format!("{}{}", i, " ".to_string());
		} else {
		    self.print_out_text = format!("{}{}{}", " ".to_string(), i, " ".to_string());
		},
		Err(_e) => Data::error_invalid_int("get_print_out".to_string()),
	    };
	} else if self.output_type == 4002 { // float
	    match self.plain_text.clone().parse::<f64>() {
		Ok(i) => if i < -1.0 {
		    self.print_out_text = format!("{}{}", i, " ".to_string());
		} else if i < 0.0 {
		    self.print_out_text = format!("{}{}", i, " ".to_string());
		    self.print_out_text.remove(1);
		} else if i == 0.0 {
		    self.print_out_text = format!("{}{}{}", " ".to_string(), i.abs(), " ".to_string());
		} else if i < 1.0 {
		    self.print_out_text = format!("{}{}{}", " ".to_string(), i, " ".to_string());
		    self.print_out_text.remove(1);
		} else {
		    self.print_out_text = format!("{}{}{}", " ".to_string(), i, " ".to_string());
		},
		Err(_e) => Data::error_invalid_float("get_print_out".to_string()),
	    };
	} else if self.output_type == 4003 { // sci_float
	    let mut output:String = "".to_string();
	    let mut last = ' ';
	    let mut point = false;
	    let mut temp = "".to_string();
	    
	    match self.plain_text.clone().parse::<f64>() {
		Ok(i) => if i < 0.0 {
		    temp = format!("{:.E}{}", i, " ".to_string());
		} else {
		    temp = format!("{}{:.E}{}", " ".to_string(), i, " ".to_string());
		},
		Err(_e) => Data::error_invalid_float("get_print_out".to_string()),
	    };
	    for c in temp.chars() {
		if c == '.' {point = true;}
		if c == 'E' && !point {output.push('.');}
		if last == 'E' && c != '-' {output.push('+');}

		output.push(c);
		last = c;
	    }

	    self.print_out_text = output;
	} else {
	    self.print_out_text = self.plain_text.clone();
	}
    }
}
