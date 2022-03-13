// Data module
#![forbid(unsafe_code)]

// General Imports
use std::collections::HashMap;

// Data struct
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
    pub fn simplify(&mut self) {
	self.find_output_type();
	self.get_print_out();
    }
    
    // Resolve any unresolved operations in the expression
    fn resolve(&mut self) {
	
    }

    // Determine output type
    fn find_output_type(&mut self) {
	let char_vec:Vec<char> = self.plain_text.chars().collect();
	let first = 0;
	let last = char_vec.len()-1;

	// Series of cases to find type
	if char_vec.get(first).expect("First char missing!") == &'"' && char_vec.get(last).expect("First char missing!") == &'"' {
	    self.output_type = "string".to_string();
	} else {
	    self.output_type = "unresolved".to_string();
	}
    }

    // Get variable value
    fn get_var_value(&mut self, mut vars:HashMap<String, Data>) {
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
