// Parser module
#![forbid(unsafe_code)]
pub mod parser {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree,Forest};
    
    // Construct Abstract Syntax Tree (line_num, name, type) -> (type, name)
    pub fn construct_tree(tokens:Vec<(u32, String, String)>) -> Tree<(String, String)> {
	let mut output:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()));
	let mut sub_tokens:Vec<(String, String)> = Vec::new();
	let mut line_num = 1;

	// Make leaves
	for t in tokens {
	    // If the line number changed, make leaf - otherwise, push token
	    if t.0 != line_num {
		// Make leaf, clear subtokens, change line number
		output.root_mut().append(construct_leaf(sub_tokens));
		sub_tokens = Vec::new();
		sub_tokens.push((t.1.clone(), t.2.clone()));
		line_num = line_num + 1;
	    } else {
		// Push token to subtokens
		sub_tokens.push((t.1.clone(), t.2.clone()));
	    }
	}

	// If we have a stray token, push it
	if sub_tokens.len() > 0 {
	    output.root_mut().append(construct_leaf(sub_tokens));
	}
	
	return output;
    }

    // Construct AST Leaf (name, type) -> (type, name)
    fn construct_leaf(tokens:Vec<(String, String)>) -> Forest<(String, String)> {
	let mut output:Forest<(String, String)> = -(tr(("".to_string(), "".to_string())));

	// Trivial case (only line number)
	if tokens.len() == 1 {
	    output = -(tr((tokens.get(0).expect("Token doesn't exist!").1.to_string(),
			   tokens.get(0).expect("Token doesn't exist!").0.to_string())));
	    return output;
	}

	// Save first tokens, create pairs
	let line_num:String = tokens.get(0).expect("Token doesn't exist!").0.to_string();
	let keyword:String = tokens.get(1).expect("Token doesn't exist!").0.to_string();
	let line_num_pair:(String, String) = (tokens.get(0).expect("Token doesn't exist!").1.to_string(),
					      line_num.clone());
	let keyword_pair:(String, String) = (tokens.get(1).expect("Token doesn't exist!").1.to_string(),
					     keyword.clone());
	
	// Parse remaining tokens
	if keyword == "GOTO".to_string() {
	    // Create GOTO leaf
	    output = -(tr(line_num_pair)
		/tr(keyword_pair)
		/tr((tokens.get(2).expect("Token doesn't exist!").1.to_string(),
	             tokens.get(2).expect("Token doesn't exist!").0.to_string())));
	} else if keyword == "LET".to_string() {
	    // Create LET leaf
	    output = -(tr(line_num_pair)
		/tr(keyword_pair)
		/tr((tokens.get(2).expect("Token doesn't exist!").1.to_string(),
		     tokens.get(2).expect("Token doesn't exist!").0.to_string())));
	} else if keyword == "PRINT".to_string() {
	    // Create PRINT leaf
	    output = -(tr(line_num_pair)
		/tr(keyword_pair)
		/tr((tokens.get(2).expect("Token doesn't exist!").1.to_string(),
		     tokens.get(2).expect("Token doesn't exist!").0.to_string())));
	} else if keyword == "IF".to_string() {
	    // Create IF leaf
	    output = -(tr(line_num_pair)
		/tr(keyword_pair)
		/tr((tokens.get(2).expect("Token doesn't exist!").1.to_string(),
		     tokens.get(2).expect("Token doesn't exist!").0.to_string()))
	        /tr((tokens.get(3).expect("Token doesn't exist!").1.to_string(),
		     tokens.get(3).expect("Token doesn't exist!").0.to_string()))
	        /tr((tokens.get(4).expect("Token doesn't exist!").1.to_string(),
		     tokens.get(4).expect("Token doesn't exist!").0.to_string())));
	} else if keyword == "END".to_string() {
	    // Create END leaf
	    output = -(tr(line_num_pair)
		/tr(keyword_pair));
	} 
	
	return output;
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_1() {
	let given:Vec<(String, String)> = vec![("001".to_string(), "line_num".to_string()),
				     ("GOTO".to_string(), "res".to_string()),
				     ("001".to_string(), "int".to_string())];
	let answer:Forest<(String, String)> = -(tr(("line_num".to_string(), "001".to_string()))
				      /tr(("res".to_string(), "GOTO".to_string()))
				      /tr(("int".to_string(), "001".to_string())));

	assert_eq!(answer, construct_leaf(given));
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_2() {
	let given:Vec<(String, String)> = vec![("345".to_string(), "line_num".to_string()),
				     ("LET".to_string(), "res".to_string()),
				     ("Bababooey=\"Sandpaper\"".to_string(), "eval".to_string())];
	let answer:Forest<(String, String)> = -(tr(("line_num".to_string(), "345".to_string()))
				      /tr(("res".to_string(), "LET".to_string()))
				      /tr(("eval".to_string(),
				           "Bababooey=\"Sandpaper\"".to_string())));
	
	assert_eq!(answer, construct_leaf(given));
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_3() {
	let given:Vec<(String, String)> = vec![("045".to_string(), "line_num".to_string()),
				     ("PRINT".to_string(), "res".to_string()),
				     ("\"Yuh Lord\"".to_string(), "string".to_string())];
	let answer:Forest<(String, String)> = -(tr(("line_num".to_string(), "045".to_string()))
				      /tr(("res".to_string(), "PRINT".to_string()))
		                      /tr(("string".to_string(), "\"Yuh Lord\"".to_string())));

	assert_eq!(answer, construct_leaf(given));
    }

    // Testing construct_leaf()
    #[test]
    fn con_leaf_4() {
	let given:Vec<(String, String)> = vec![("045".to_string(), "line_num".to_string())];
	let answer:Forest<(String, String)> = -(tr(("line_num".to_string(), "045".to_string())));

	assert_eq!(answer, construct_leaf(given));
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
	let given:Vec<(u32, String, String)> = vec![(1, "001".to_string(), "line_num".to_string()),
					    (1, "GOTO".to_string(), "res".to_string()),
					    (1, "002".to_string(), "int".to_string())];
	let answer:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("int".to_string(), "002".to_string())));

	assert_eq!(answer, construct_tree(given));
    }

    // Testing construct_tree()
    #[test]
    fn con_tree_2() {
	let given:Vec<(u32, String, String)> = vec![(1, "001".to_string(), "line_num".to_string()),
					    (1, "GOTO".to_string(), "res".to_string()),
					    (1, "002".to_string(), "int".to_string()),
					    (2, "002".to_string(), "line_num".to_string()),
					    (2, "GOTO".to_string(), "res".to_string()),
					    (2, "001".to_string(), "int".to_string())];
	let answer:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("int".to_string(), "002".to_string())))
	    /(tr(("line_num".to_string(), "002".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("int".to_string(), "001".to_string())));

	assert_eq!(answer, construct_tree(given));
    }

    // Testing construct_tree()
    #[test]
    fn con_tree_3() {
	let given:Vec<(u32, String, String)> = vec![(1, "001".to_string(), "line_num".to_string()),
					    (1, "GOTO".to_string(), "res".to_string()),
					    (1, "002".to_string(), "int".to_string()),
					    (2, "002".to_string(), "line_num".to_string()),
					    (3, "003".to_string(), "line_num".to_string()),
					    (3, "GOTO".to_string(), "res".to_string()),
					    (3, "001".to_string(), "int".to_string()),
					    (4, "004".to_string(), "line_num".to_string())];
	let answer:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("int".to_string(), "002".to_string())))
	    /(tr(("line_num".to_string(), "002".to_string())))
	    /(tr(("line_num".to_string(), "003".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("int".to_string(), "001".to_string())))
	    /(tr(("line_num".to_string(), "004".to_string())));

	assert_eq!(answer, construct_tree(given));
    }
}
