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
	let mut output:Forest<String> = -tr("".to_string());
	let mut line_num:String = "".to_string();
	let mut line_num_got:bool = true;

	for i in 0..tokens.len() {
	    if is_line_number(tokens.get(i).expect("DNE!").to_string()) == true && line_num_got {
		line_num = tokens.get(i).expect("DNE!").to_string();
		line_num_got == false;
	    } else {
		if tokens.get(i).expect("DNE!").to_string() == "GOTO".to_string() {
		    output = -(tr(line_num.clone()) /tr(tokens.get(i).expect("DNE!").to_string())
			                            /tr(tokens.get(i+1).expect("DNE!").to_string()));
		} else if tokens.get(i).expect("DNE!").to_string() == "LET".to_string() {
		    output = -(tr(line_num.clone()) /tr(tokens.get(i).expect("DNE!").to_string())
			                            /tr(tokens.get(i+1).expect("DNE!").to_string())
			                            /tr(tokens.get(i+2).expect("DNE!").to_string())
		                                    /tr(tokens.get(i+3).expect("DNE!").to_string()));

		} else if tokens.get(i).expect("DNE!").to_string() == "PRINT".to_string() {
		    output = -(tr(line_num.clone()) /tr(tokens.get(i).expect("DNE!").to_string())
			                            /tr(tokens.get(i+1).expect("DNE!").to_string()));
		}
	    }
	}
	
	return output;
    }

    // Check if line number
    fn is_line_number(token:String) -> bool {
	let char_vec:Vec<char> = token.chars().collect();
	let mut num_cnt = 0;
	let mut output = true;

	for c in char_vec {
	    if (num_cnt % 3 != 0) | (num_cnt == 0) {
		output = output && c.is_digit(10);
		num_cnt = num_cnt + 1; 
	    }
        }

	return output;
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_1() {
	let given:Vec<String> = vec!["001".to_string(),"GOTO".to_string(),"001".to_string()];
	let answer = -(tr("001".to_string()) /tr("GOTO".to_string()) /tr("001".to_string()));

	assert_eq!(answer, construct_leaf(given));
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_2() {
	let given:Vec<String> = vec!["345".to_string(),"LET".to_string(),"Bababooey".to_string(),
	                             "=".to_string(),"\"Sandpaper\"".to_string()];
	let answer = -(tr("345".to_string()) /tr("LET".to_string()) /tr("Bababooey".to_string())
	                                     /tr("=".to_string()) /tr("\"Sandpaper\"".to_string()));

	assert_eq!(answer, construct_leaf(given));
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_3() {
	let given:Vec<String> = vec!["045".to_string(),"PRINT".to_string(),
				     "\"Yuh Lord\"".to_string()];
	let answer = -(tr("045".to_string()) /tr("PRINT".to_string())
		                             /tr("\"Yuh Lord\"".to_string()));

	assert_eq!(answer, construct_leaf(given));
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

    // Testing is_line_number()
    #[test]
    fn is_ln_4() {
	let given:String = "387_098".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_line_number(given));
    }

    // Testing is_line_number()
    #[test]
    fn is_ln_5() {
	let given:String = "345_123_890".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_line_number(given));
    }

    // Testing is_line_number()
    #[test]
    fn is_ln_6() {
	let given:String = "3f7_g98".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_line_number(given));
    }
}

// Testing public methods
#[cfg(test)]
mod test {

}
