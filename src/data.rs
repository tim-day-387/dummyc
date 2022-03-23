// Data module
#![forbid(unsafe_code)]

// General Imports
use std::collections::HashMap;

// File Imports
use lexer::*;

// Data struct
#[derive(Clone)]
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

    // Check if two data objects are equal
    pub fn equals(self, other:Data) -> bool {
	let first:bool = self.plain_text == other.plain_text;
	let second:bool = self.output_type == other.output_type;
	let third:bool = self.print_out_text == other.print_out_text;

	return first && second && third;
    }

    // Simplify data output to one which can be stored and printed out
    pub fn simplify(&mut self, vars:HashMap<String, Data>) {
	self.find_output_type();
	
	if self.output_type == "unresolved".to_string() {
	    self.resolve(vars);
	}
	
	self.get_print_out();
    }
    
    // Resolve any unresolved operations in the expression
    fn resolve(&mut self, vars:HashMap<String, Data>) {
	let split = split_over_op(self.plain_text.clone());
	let first_part_string:String = split.0;
	let operation_string:String = split.1;
	let second_part_string:String = split.2;

	if operation_string == "".to_string() {
	    self.get_var_value(vars);
	    return;
	}

	let mut first_obj:Data = Data::new(first_part_string);
	let mut second_obj:Data = Data::new(second_part_string);
	
	first_obj.simplify(vars.clone());
	second_obj.simplify(vars.clone());

	first_obj.operation(second_obj);

	// Simplify later
	self.plain_text = first_obj.plain_text.clone();
	self.output_type = first_obj.output_type.clone();
	self.print_out_text = first_obj.print_out_text.clone();	
    }

    // Perform the operation
    fn operation(&mut self, other:Data) {
	self.plain_text = format!("{}{}{}{}", "\"".to_string(), self.print_out_text.clone(), other.print_out_text.clone(), "\"".to_string());
    }

    // Determine output type
    fn find_output_type(&mut self) {
	// Series of cases to find type
	if is_string(self.plain_text.clone()) {
	    self.output_type = "string".to_string();
	} else if is_float(self.plain_text.clone()) {
	    self.output_type = "float".to_string();
	} else {
	    self.output_type = "unresolved".to_string();
	}
    }

    // Get variable value
    fn get_var_value(&mut self, vars:HashMap<String, Data>) {
	let mut var_value:&Data = &Data::new("".to_string());
	
	match vars.get(&self.plain_text) {
	    Some(value)=> var_value = value,
	    _=> println!("ERROR VAL"),
	}

	// Simplify later
	self.plain_text = var_value.plain_text.clone();
	self.output_type = var_value.output_type.clone();
	self.print_out_text = var_value.print_out_text.clone();
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

// Testing methods
#[cfg(test)]
mod test {
    // File Imports
    use data::*;
    
    // Testing output_type()
    #[test]
    fn type_1() {
	let mut given:Data = Data::new("\"This is a test\"".to_string());
	given.find_output_type();

	assert_eq!("string".to_string(), given.output_type);
    }

    // Testing output_type()
    #[test]
    fn type_2() {
	let mut given:Data = Data::new("This is another, different test.".to_string());
	given.find_output_type();

	assert_eq!("unresolved".to_string(), given.output_type);
    }
}
