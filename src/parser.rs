// Parser module
#![forbid(unsafe_code)]
pub mod parser {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree,Forest};
    
    // Construct Abstract Syntax Tree
    pub fn construct_tree(tokens:Vec<(u32, String)>) -> Tree<(String, String)> {
	let mut output:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()));
	let mut sub_tokens:Vec<String> = Vec::new();
	let mut line_num = 1;

	// Make leaves
	for t in tokens {
	    if t.0 != line_num {
		output.root_mut().append(construct_leaf(sub_tokens));
		sub_tokens = Vec::new();
		sub_tokens.push(t.1.clone());
		line_num = line_num + 1;
	    } else {
		sub_tokens.push(t.1.clone());
	    }
	}

	// If we have a stray token, push it
	if sub_tokens.len() > 0 {
	    output.root_mut().append(construct_leaf(sub_tokens));
	}
	
	return output;
    }

    // Construct AST Leaf
    fn construct_leaf(tokens:Vec<String>) -> Forest<(String, String)> {
	let mut output:Forest<(String, String)> = -(tr(("".to_string(), "".to_string())));

	// Trivial case, or set line_num
	if tokens.len() == 1 {
	    output = -(tr(("line_num".to_string(), tokens.get(0).expect("DNE!").to_string())));
	    return output;
	}

	// Save first tokenss
	let line_num:String = tokens.get(0).expect("DNE!").to_string();
	let keyword:String = tokens.get(1).expect("DNE!").to_string();
	let line_num_pair:(String, String) = ("line_num".to_string(),
					      line_num.clone());
	let keyword_pair:(String, String) = ("res".to_string(),
					     tokens.get(1).expect("DNE!").to_string());
	
	// Parse remaining tokens
	if keyword == "GOTO".to_string() {
	    output = -(tr(line_num_pair)
		/tr(keyword_pair)
		/tr(("line_num".to_string(), tokens.get(1+1).expect("DNE!").to_string())));
	} else if keyword == "LET".to_string() {
	    output = -(tr(line_num_pair)
		/tr(keyword_pair)
		/tr(("var".to_string(), tokens.get(1+1).expect("DNE!").to_string()))
		/tr(("relate".to_string(), tokens.get(1+2).expect("DNE!").to_string()))
		/tr((find_token(tokens.get(1+3).expect("DNE!").to_string()),
		     tokens.get(1+3).expect("DNE!").to_string())));
	} else if keyword == "PRINT".to_string() {
	    output = -(tr(line_num_pair)
		/tr(keyword_pair)
		/tr((find_token(tokens.get(1+1).expect("DNE!").to_string()),
		     tokens.get(1+1).expect("DNE!").to_string())));
	} 
	
	return output;
    }

    // Classify token
    fn find_token(token:String) -> String {
	let mut output:String = "".to_string();

	if is_number(token.clone()) {
	    output = "int".to_string();
	} else if is_string(token.clone()) {
	    output = "string".to_string();
	} else {
	    output = "var".to_string();
	}

	return output;
    }

    
    
    // Check if number
    fn is_number(token:String) -> bool {
	let char_vec:Vec<char> = token.chars().collect();
	let mut output = true;

	for c in char_vec {
	    output = output && c.is_digit(10);
        }

	return output;
    }

    // Check if string
    fn is_string(token:String) -> bool {
	let char_vec:Vec<char> = token.chars().collect();
	let last = char_vec.len()-1;
	let mut output = false;

	if char_vec.get(0).expect("DNE!") == &'"' && char_vec.get(last).expect("DNE!") == &'"' {
	    output = true;
	}

	return output;
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_1() {
	let given:Vec<String> = vec!["001".to_string(),"GOTO".to_string(),"001".to_string()];
	let answer:Forest<(String, String)> = -(tr(("line_num".to_string(), "001".to_string()))
				      /tr(("res".to_string(), "GOTO".to_string()))
				      /tr(("line_num".to_string(), "001".to_string())));

	assert_eq!(answer, construct_leaf(given));
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_2() {
	let given:Vec<String> = vec!["345".to_string(),"LET".to_string(),"Bababooey".to_string(),
	                             "=".to_string(),"\"Sandpaper\"".to_string()];
	let answer:Forest<(String, String)> = -(tr(("line_num".to_string(), "345".to_string()))
				      /tr(("res".to_string(), "LET".to_string()))
				      /tr(("var".to_string(), "Bababooey".to_string()))
				      /tr(("relate".to_string(), "=".to_string()))
				      /tr(("string".to_string(), "\"Sandpaper\"".to_string())));
	
	assert_eq!(answer, construct_leaf(given));
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_3() {
	let given:Vec<String> = vec!["045".to_string(),"PRINT".to_string(),
				     "\"Yuh Lord\"".to_string()];
	let answer:Forest<(String, String)> = -(tr(("line_num".to_string(), "045".to_string()))
				      /tr(("res".to_string(), "PRINT".to_string()))
		                      /tr(("string".to_string(), "\"Yuh Lord\"".to_string())));

	assert_eq!(answer, construct_leaf(given));
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_4() {
	let given:Vec<String> = vec!["045".to_string()];
	let answer:Forest<(String, String)> = -(tr(("line_num".to_string(), "045".to_string())));

	assert_eq!(answer, construct_leaf(given));
    }

    // Testing find_token()
    #[test]
    fn find_1() {
	let given:String = "031".to_string();
	let answer:String = "int".to_string();

	assert_eq!(answer, find_token(given));
    }

    // Testing find_token()
    #[test]
    fn find_2() {
	let given:String = "\"This is a sample\"".to_string();
	let answer:String = "string".to_string();

	assert_eq!(answer, find_token(given));
    }

    // Testing find_token()
    #[test]
    fn find_3() {
	let given:String = "G3gedg444".to_string();
	let answer:String = "".to_string();

	assert_eq!(answer, find_token(given));
    }
    
    // Testing is_string()
    #[test]
    fn is_s_1() {
	let given:String = "0F1".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_string(given));
    }

    // Testing is_string()
    #[test]
    fn is_s_2() {
	let given:String = "\"0F1\"".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_string(given));
    }

    // Testing is_string()
    #[test]
    fn is_s_3() {
	let given:String = "\"This is a sample string\"".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_string(given));
    }
    
    // Testing is_number()
    #[test]
    fn is_n_1() {
	let given:String = "0F1".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_number(given));
    }

    // Testing is_number()
    #[test]
    fn is_n_2() {
	let given:String = "LET".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_number(given));
    }

    // Testing is_number()
    #[test]
    fn is_n_3() {
	let given:String = "387".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_number(given));
    }

    // Testing is_number()
    #[test]
    fn is_n_4() {
	let given:String = "3".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_number(given));
    }

    // Testing is_number()
    #[test]
    fn is_n_5() {
	let given:String = "900".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_number(given));
    }

    // Testing is_number()
    #[test]
    fn is_n_6() {
	let given:String = "3f7_g98".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_number(given));
    }
}

// Testing public methods
#[cfg(test)]
mod test {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree};

    // File Imports
    use super::parser::*;
    
    // Testing construct_tree()
    #[test]
    fn con_tree_1() {
	let given:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "GOTO".to_string()),
					    (1, "002".to_string())];
	let answer:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "002".to_string())));

	assert_eq!(answer, construct_tree(given));
    }

    // Testing construct_tree()
    #[test]
    fn con_tree_2() {
	let given:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "GOTO".to_string()),
					    (1, "002".to_string()),(2, "002".to_string()),
					    (2, "GOTO".to_string()),(2, "001".to_string())];
	let answer:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "002".to_string())))
	    /(tr(("line_num".to_string(), "002".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "001".to_string())));

	assert_eq!(answer, construct_tree(given));
    }

    // Testing construct_tree()
    #[test]
    fn con_tree_3() {
	let given:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "GOTO".to_string()),
					    (1, "002".to_string()),(2, "002".to_string()),
					    (3, "003".to_string()),(3, "GOTO".to_string()),
					    (3, "001".to_string()),(4, "004".to_string())];
	let answer:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "002".to_string())))
	    /(tr(("line_num".to_string(), "002".to_string())))
	    /(tr(("line_num".to_string(), "003".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "001".to_string())))
	    /(tr(("line_num".to_string(), "004".to_string())));

	assert_eq!(answer, construct_tree(given));
    }
}
