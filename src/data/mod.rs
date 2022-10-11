// Data module
#![forbid(unsafe_code)]


// Testing methods
#[cfg(test)]
mod tests;


// General Imports
use Path;
use rand::Rng;


// File Imports
use state::State;
use types::enums::Type;
use types::find_type;
use errors::{stateless_error, parse_int, parse_float, error_divide_zero};
use expression_lexer::{split, split_function, split_arguments};


// Data struct
#[derive(PartialEq, Clone)]
pub struct Data {
    pub plain_text:String,
    pub output_type:Type,
    pub print_out_text:String,
}


// Data implementation
impl Data {
    // Constructor with simplification
    pub fn new_simplified(given_text:String, state:State) -> Data {
	let mut output = Data::new(given_text.clone());

	output.simplify(state);

	if given_text != *"" && output.plain_text.clone() == *"" {
	    stateless_error([given_text].to_vec(),
			    ["token".to_string()].to_vec(),
			    "new_simplified".to_string(),
			    "Cannot reduce to empty data object.".to_string());
	}

	output
    }


    // Constructor
    pub fn new(given_text:String) -> Data {
	Data {
	    plain_text:given_text,
	    output_type:Type::Undefined,
	    print_out_text:"".to_string(),
	}
    }


    // Simplify data output to one which can be stored and printed out
    pub fn simplify(&mut self, state:State) {
	self.find_output_type();

	if self.output_type == Type::Function {
	    self.resolve_callable(state);
	} else if self.output_type == Type::Symbol {
	    self.resolve_symbol(state);
	} else if self.output_type == Type::Expression {
	    self.resolve_expression(state);
	}

	self.get_print_out();
    }


    // Resolve array reference in actual var name
    pub fn get_array_reference(given:String, state:State) -> String {
	let name = split_function(given.clone()).0.to_lowercase();
	let arguments = split_arguments(split_function(given).1);
	let location;
	let mut text = "".to_string();

	if arguments.len() == 1 {
	    let location_string = Data::new_simplified(arguments[0].clone(), state).plain_text;

	    location = match location_string.parse::<i64>() {
		Ok(i) => i,
		Err(_e) => -1,
	    };

	    text = format!("{}{}{}{}", name, "(", location, ")");
	}

	text
    }


    // Resolve symbol_callable type data
    fn resolve_callable(&mut self, state:State) {
	let name = split_function(self.plain_text.clone()).0.to_lowercase();
	let arguments = split_arguments(split_function(self.plain_text.clone()).1);
	let array_ref = Data::get_array_reference(self.plain_text.clone(), state.clone());

	if name == *"int" && arguments.len() == 1 {
	    let arg_data = Data::new_simplified(arguments[0].clone(), state.clone());
	    let number:i64 = parse_float(arg_data.plain_text, "resolve_callable".to_string()).round() as i64;

	    *self = Data::new_simplified(number.to_string(), state);
	} else if Data::does_var_exist(array_ref.clone(), state.clone()) {
	    self.plain_text = array_ref;
	    self.get_var_value(state);
	} else {
	    self.function(state, name, arguments);
	}
    }


    // Resolve symbol type data
    fn resolve_symbol(&mut self, state:State) {
	if self.plain_text.to_lowercase() == *"rnd" {
	    let mut rng = rand::thread_rng();
	    let number:f64 = rng.gen();
	    *self = Data::new_simplified(number.to_string(), state);
	} else if self.plain_text.to_lowercase() == *"prloc" {
	    self.plain_text = state.print_location.to_string();
	    self.simplify(state);
	} else {
	    self.get_var_value(state);
	}
    }


    // Execute the given function call
    fn function(&mut self, state:State, name:String, arguments:Vec<String>) {
	let location_a = "./std/".to_string();
	let location_b = "/usr/lib/dummyc/std/".to_string();
	let string_path_a = format!("{}{}{}", location_a, name, ".bas");
	let string_path_b = format!("{}{}{}", location_b, name, ".bas");
	let file_path_a = Path::new(&string_path_a);
	let file_path_b = Path::new(&string_path_b);

	let file_path:&Path = if file_path_a.exists() {
	    file_path_a
	} else {
	    file_path_b
	};

	// Useful variables
	let mut lim_state = State::new();
	lim_state.print_location = state.print_location;

	// Add arguments
	for args in arguments {
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
    fn find_operation_output_type(self, other:Data) -> Type {
	if !self.clone().output_type.check_if_compatible(other.clone().output_type) {
	    stateless_error([].to_vec(),
			    [].to_vec(),
			    "find_operation_output_type".to_string(),
			    "Incompatible types.".to_string());
	    Type::Undefined
	} else {
	    self.output_type.precedence(other.output_type)
	}
    }


    // Perform the compare
    pub fn compare(self, other:Data, operation_string:String) -> bool {
	let output_type:Type = self.clone().find_operation_output_type(other.clone());

	if output_type.check_if_number() {
	    let a = parse_float(self.plain_text, "compare".to_string());
	    let b = parse_float(other.plain_text, "compare".to_string());

	    if operation_string == *"<" {
		a < b
	    } else if operation_string == *">" {
		a > b
	    } else if operation_string == *">=" {
		a >= b
	    } else if operation_string == *"<=" {
		a <= b
	    } else {
		false
            }
	} else {
	    false
	}
    }


    // Perform the operation
    pub fn operation(&mut self, other:Data, operation_string:String) {
	let output_type:Type = self.clone().find_operation_output_type(other.clone());

	if output_type == Type::String {
	    self.plain_text = format!("{}{}{}{}", "\"", self.print_out_text.clone(), other.print_out_text, "\"");
	} else if output_type == Type::Int {
	    let a = parse_int(self.plain_text.clone(), "operation".to_string());
	    let b = parse_int(other.plain_text, "operation".to_string());

	    if operation_string == *"+" {
		self.plain_text = (a+b).to_string();
	    } else if operation_string == *"*" {
		self.plain_text = (a*b).to_string();
	    } else if operation_string == *"-" {
		self.plain_text = (a-b).to_string();
	    } else if operation_string == *"/" {
		if b == 0 {
		    error_divide_zero("operation".to_string());
		} else {
		    self.plain_text = ((a as f64)/(b as f64)).to_string();
		}
	    } else if operation_string == *"^" {
		self.plain_text = (a as f64).powf(b as f64).to_string();
	    } else {
		stateless_error([].to_vec(),
				[].to_vec(),
				"operation".to_string(),
				"Invalid operation.".to_string());
            }
	} else if output_type.check_if_float() {
	    let a = parse_float(self.plain_text.clone(), "operation".to_string());
	    let b = parse_float(other.plain_text, "operation".to_string());

	    if operation_string == *"+" {
		self.plain_text = (a+b).to_string();
	    } else if operation_string == *"*" {
		self.plain_text = (a*b).to_string();
	    } else if operation_string == *"-" {
		self.plain_text = (a-b).to_string();
	    } else if operation_string == *"/" {
		if b == 0.0 {
		    error_divide_zero("operation".to_string());
		} else {
		    self.plain_text = (a/b).to_string();
		}
	    } else if operation_string == *"^" {
		self.plain_text = a.powf(b).to_string();
	    } else {
		stateless_error([].to_vec(),
				[].to_vec(),
				"operation".to_string(),
				"Invalid operation.".to_string());
            }
	} else {
	    stateless_error([].to_vec(),
			    [].to_vec(),
			    "operation".to_string(),
			    "Unsupported type.".to_string());
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
		stateless_error([self.plain_text.clone()].to_vec(),
				["variable".to_string()].to_vec(),
				"get_var_value".to_string(),
				"Variable does not exist.".to_string());
	    }
	}

	*self = var_value.clone();
    }


    // Does a variable by that name exist?
    fn does_var_exist(name:String, state:State) -> bool {
	matches!(state.variables.get(&name), Some(_value))
    }


    // Find text to be printed out, handle formatting
    fn get_print_out(&mut self) {
	if self.output_type == Type::String {
	    self.print_out_text = self.plain_text.clone();
	    self.print_out_text.pop();
	    self.print_out_text.remove(0);
	} else if self.output_type == Type::Int {
	    let i = parse_int(self.plain_text.clone(), "get_print_out".to_string());

	    if i < 0 {
		self.print_out_text = format!("{}{}", i, " ");
	    } else {
		self.print_out_text = format!("{}{}{}", " ", i, " ");
	    }
	} else if self.output_type == Type::Float {
	    let i = parse_float(self.plain_text.clone(), "get_print_out".to_string());

	    if i < -1.0 {
		self.print_out_text = format!("{}{}", i, " ");
	    } else if i < 0.0 {
		self.print_out_text = format!("{}{}", i, " ");
		self.print_out_text.remove(1);
	    } else if i == 0.0 {
		self.print_out_text = format!("{}{}{}", " ", i.abs(), " ");
	    } else if i < 1.0 {
		self.print_out_text = format!("{}{}{}", " ", i, " ");
		self.print_out_text.remove(1);
	    } else {
		self.print_out_text = format!("{}{}{}", " ", i, " ");
	    }
	} else if self.output_type == Type::SciFloat {
	    let mut output:String = "".to_string();
	    let mut last = ' ';
	    let mut point = false;

	    let i = parse_float(self.plain_text.clone(), "get_print_out".to_string());

	    let temp = if i < 0.0 {
		format!("{:.E}{}", i, " ")
	    } else {
		format!("{}{:.E}{}", " ", i, " ")
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
