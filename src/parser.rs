// Parser module
#![forbid(unsafe_code)]
pub mod parser {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree,Forest};
    use std::pin::Pin;
    
    // Construct Abstract Syntax Tree
    fn construct_tree(tokens:Vec<String>) -> Tree<String> {
	let mut output:Tree<String> = tr("MAIN".to_string());
	let mut sub_tokens:Vec<String> = Vec::new();

	// Make leaves
	for t in tokens {
	    if is_line_number(t.clone()) == true {
		output.root_mut().append(construct_leaf(sub_tokens));
		sub_tokens = Vec::new();
	    } else {
		sub_tokens.push(t.clone());
	    }
	}
	
	return output;
    }

    // Construct AST Leaf
    fn construct_leaf(tokens:Vec<String>) -> Forest<String> {
	let output:Forest<String> = -tr("MAIN".to_string());

	return output;
    }

    // Check if line number
    fn is_line_number(token:String) -> bool {
	let char_vec:Vec<char> = token.chars().collect();
	let mut output = true;

	for c in char_vec {
            output = output && c.is_digit(10);
        }

	return output;
    }

    // Testing is_line_number()
    #[test]
    fn is_ln_1() {
	let given:String = "0F1".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_line_number(given));
    }

    // Testing is_line_number()
    #[test]
    fn is_ln_2() {
	let given:String = "LET".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_line_number(given));
    }

    // Testing is_line_number()
    #[test]
    fn is_ln_3() {
	let given:String = "387".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_line_number(given));
    }
}

// Testing public methods
#[cfg(test)]
mod test {

}
