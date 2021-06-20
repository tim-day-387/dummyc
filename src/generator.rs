// Generator module
#![forbid(unsafe_code)]
pub mod generator {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree,Forest,Node};
    use std::io::Write;

    // Create Rust code from the abstract syntax tree
    pub fn generate(input:Tree<String>, name:String) -> String {
	return "".to_string();
    }

    // Create main body of the Rust code
    pub fn create_main(input:Tree<String>) -> String {
	let mut output = "fn main() {\n".to_string();
	let mut next_token;
	let mut next_option; 
	let mut i = 0;

	// Iter through tree while constructing output
	loop {
	    next_option = input.iter().nth(i);
	    match next_option {
		Some(next_option) => next_token = next_option.to_string(),
		None => break,
	    }

	    i = i + 1;
	    println!("{}", next_token);
	}
	
	return output;
    }

    // Testing create_main()
    #[test]
    fn main_1() {
	let given:Tree<String> = (tr("MAIN".to_string())
		      /(tr("001".to_string()) /tr("GOTO".to_string()) /tr("002".to_string()))
	              /(tr("002".to_string()) /tr("GOTO".to_string()) /tr("001".to_string())));
	let answer = "fn main() {\n";
	
	assert_eq!(answer, create_main(given));
    }
}

// Testing public methods
#[cfg(test)]
mod test {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree,Forest,Node};

    // File Imports
    use super::generator::*;

}
