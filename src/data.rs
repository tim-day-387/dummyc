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
	
    }

    // Find text to be printed out
    fn get_print_out)&mut self) {

    }
}
