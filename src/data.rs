// Data module
#![forbid(unsafe_code)]

// Data struct
pub struct Data {
    plain_text:String,
    output_type:String,
    print_out_text:String,
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

    }
    
    // Resolve any unresolved operations in the expression
    fn resolve(&mut self) {

    }

    // Determine output type
    fn output_type(&mut self) {
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

    // Find text to be printed out
    fn get_print_out(&mut self) {

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
	given.output_type();

	assert_eq!("string".to_string(), given.output_type);
    }

    // Testing output_type()
    #[test]
    fn type_2() {
	let mut given:Data = Data::new("This is another, different test.".to_string());
	given.output_type();

	assert_eq!("unresolved".to_string(), given.output_type);
    }
}
