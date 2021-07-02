// Generator module
#![forbid(unsafe_code)]
pub mod generator {
    // General Imports
    extern crate trees;
    use self::trees::{Tree};

    // File Imports
    use crate::evaluator::evaluator::evaluate;

    // Create Rust code from the abstract syntax tree
    pub fn generate(input:Tree<(String, String)>) -> String {
	let mut output:String;
	let mut next_leaf:Tree<(String, String)>;
	let mut next_line:String;
	let mut next_leaf_unchecked;
	let mut i = 0;

	// Create the beginning of the Rust code
	output = create_main(input.clone());

	// Iter through tree while constructing output
	loop {
	    // Get next child in tree
	    next_leaf_unchecked = input.iter().nth(i);
	    match next_leaf_unchecked {
		Some(next_leaf_unchecked) => next_leaf = next_leaf_unchecked.to_owned(),
		None => break,
	    }

	    // Get next line number in tree
	    next_leaf_unchecked = input.iter().nth(i+1);
	    match next_leaf_unchecked {
		Some(next_leaf_unchecked) => next_line = next_leaf_unchecked.data.1.to_string(),
		None => next_line = "".to_string(),
	    }
	    
	    // Iterate and concatenate with the function in the next leaf
	    i = i + 1;
	    println!("{:?}", next_leaf);
	    output = [output, create_function(next_leaf.clone(), next_line)].concat();
	}
	
	return output;
    }

    // Create main body of the Rust code
    fn create_main(input:Tree<(String, String)>) -> String {
	let mut output = "use std::collections::HashMap;\nfn main() {\n".to_string();
	let mut next_line_of_code:String;
	let next_line:String;
	let next_leaf_unchecked;

	// Get next child in tree
	next_leaf_unchecked = input.iter().nth(0);
	match next_leaf_unchecked {
	    Some(next_leaf_unchecked) => next_line = next_leaf_unchecked.data.1.to_string(),
	    None => return [output, "}\n".to_string()].concat(),
	}

	// Define hash map (create code)
        next_line_of_code = ["  let vars:HashMap<String,(String,String)> 
                     = HashMap::new();\n".to_string()].concat();
	output = [output, next_line_of_code].concat();
	
	// Function call (create code)
	next_line_of_code = ["  line".to_string(), next_line,
		     "(vars.clone());\n".to_string()].concat();
	output = [output, next_line_of_code].concat();

	// Add last bracket (create code)
	output = [output, "}\n".to_string()].concat();
	
	return output;
    }

    // Create Rust from given subtree
    fn create_function(subtree:Tree<(String, String)>, line_num:String) -> String {
	let mut output = ["fn line".to_string(),
			  subtree.root().data.1.to_string(),
			  "(mut vars:HashMap<String,(String,String)>) {\n".to_string()].concat();
	let mut next_token:(String, String);
	let next_line_of_code:String;
	let mut next_line_num:String = line_num;
	let mut next_leaf_unchecked;
	let mut children:Vec<String> = Vec::new();
	let mut children_type:Vec<String> = Vec::new();
	let mut i = 0;

	// Iter through tree to create vectors
	loop {
	    // Get next child in tree
	    next_leaf_unchecked = subtree.iter().nth(i);
	    match next_leaf_unchecked {
		Some(next_leaf_unchecked) => next_token = next_leaf_unchecked.data.clone(),
		None => break,
	    }
	    
	    // Iterate and concatenate
	    i = i + 1;

	    // Push each token
	    children.push(next_token.1.to_string());
	    children_type.push(next_token.0.to_string());
	}

	// Figure out correct function, so Rust code can be made
	if children.len() == 0 {
	    // If there is no function, do nothing
	} else if children.get(0).expect("DNE!").to_string() == "PRINT".to_string() {
	    // Create PRINT code
	    output = [output, print_statement(children.get(1).expect("DNE!").to_string())].concat();
	} else if children.get(0).expect("DNE!").to_string() == "GOTO".to_string() {
	    // Create GOTO code
	    next_line_num = children.get(1).expect("DNE!").to_string();
	} else if children.get(0).expect("DNE!").to_string() == "LET".to_string() {
	    // Create LET code
	    output = [output, let_statement(children.get(1).expect("DNE!").to_string())].concat();
	} else if children.get(0).expect("DNE!").to_string() == "IF".to_string() {
	    // Create IF code
	    output = [output, if_statement(children.get(1).expect("DNE!").to_string(),
					   children.get(3).expect("DNE!").to_string(),
	                                   next_line_num.clone())].concat();
	    next_line_num = "".to_string();
	} else if children.get(0).expect("DNE!").to_string() == "END".to_string() {
	    // Create END code
	    next_line_num = "".to_string();
	}    

	// Create code to call next method and add to end
	if next_line_num != "".to_string() {
	    next_line_of_code = ["  line".to_string(), next_line_num,
			 "(vars.clone());\n".to_string(), "}\n".to_string()].concat();
	} else {
	    next_line_of_code = "}\n".to_string();
	}
	output = [output, next_line_of_code].concat();
	
	return output;
    }

    // Create PRINT statement
    fn print_statement(token:String) -> String {
	let mut output:String = "".to_string();
	
	// Perform evaluation on token
	let eval:(String, String, String, String) = evaluate(token);

	// Choose what to output based on evaluation
	if eval.3 == "string" {
	    // Code for printing strings
	    output = ["  println!(".to_string(),
		      eval.2, ");\n".to_string()].concat(); 
        } else if eval.3 == "eval" {
	    // Code for print variables
	    output = ["  println!(\"{}\", vars.get(\"".to_string(),
		      eval.2, "\").expect(\"DNE!\").1);\n".to_string()].concat();
	}

	return output;
    }

    // Create LET statement
    fn let_statement(token:String) -> String {
	let output:String;
	
	// Perform evaluation on token
	let eval:(String, String, String, String) = evaluate(token);

	// Create ouput based on evaluation
	output = ["  vars.insert(\"".to_string(),
	          eval.0, "\".to_string(),(\"".to_string(), eval.3,
	          "\".to_string(),".to_string(),
	          eval.2, "));\n".to_string()].concat();

	return output;
    }

    // Create IF statement
    fn if_statement(expression:String, goto:String, next_line:String) -> String {
	let output:String;
	
	// Perform evaluation on token
	let eval:(String, String, String, String) = evaluate(expression);

	// Create ouput based on evaluation
	output = ["if ".to_string(), "vars.get(\"".to_string(),
		  eval.0, "\").expect(\"DNE!\").1 ".to_string(),
	          eval.1, "= ".to_string(), eval.2,
		  " {\n".to_string(), "  line".to_string(), goto,
		  "(vars.clone());\n".to_string(), "} else {\n".to_string(),
		  "  line".to_string(), next_line,
		  "(vars.clone());\n".to_string(), "}\n".to_string()].concat();

	return output;
    }

    // Testing if_statement()
    #[test]
    fn if_1() {
	let given_exp = "F=\"Baba\"".to_string();
	let given_goto = "001".to_string();
	let given_next_line = "002".to_string();

	let answer = "if vars.get(\"F\").expect(\"DNE!\").1 == \"Baba\".to_string() {
  line001(vars.clone());
} else {
  line002(vars.clone());
}
".to_string();

	assert_eq!(answer, if_statement(given_exp, given_goto, given_next_line));
    }
    
    // Testing generate()
    #[test]
    fn gen_1() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "002".to_string())));
	let main = "use std::collections::HashMap;\nfn main() {\n  let vars:HashMap<String,(String,String)> 
                     = HashMap::new();\n  line001(vars.clone());\n}\n".to_string();
	let func1 = "fn line001(mut vars:HashMap<String,(String,String)>) {\n  line002(vars.clone());\n}\n".to_string();
	let answer = [main, func1].concat();
	
	assert_eq!(answer, generate(given));
    }
    
    // Testing create_function()
    #[test]
    fn func_1() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "PRINT".to_string()))
	    /tr(("eval".to_string(), "\"This is a sample\"".to_string()));
	let answer = "fn line001(mut vars:HashMap<String,(String,String)>) {\n  println!(\"This is a sample\");\n  line002(vars.clone());\n}\n".to_string();
	
	assert_eq!(answer, create_function(given, "002".to_string()));
    }

    // Testing create_function()
    #[test]
    fn func_2() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("int".to_string(), "002".to_string()));
	let answer = "fn line001(mut vars:HashMap<String,(String,String)>) {\n  line002(vars.clone());\n}\n".to_string();
	
	assert_eq!(answer, create_function(given, "345".to_string()));
    }

    // Testing create_function()
    #[test]
    fn func_3() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "PRINT".to_string()))
	    /tr(("eval".to_string(), "\"This is a sample\"".to_string()));
	let answer = "fn line001(mut vars:HashMap<String,(String,String)>) {\n  println!(\"This is a sample\");\n}\n".to_string();
	
	assert_eq!(answer, create_function(given, "".to_string()));
    }

    // Testing create_function()
    #[test]
    fn func_4() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "PRINT".to_string()))
	    /tr(("eval".to_string(), "Baba".to_string()));
	let answer = "fn line001(mut vars:HashMap<String,(String,String)>) {\n  println!(\"{}\", vars.get(\"Baba\").expect(\"DNE!\").1);\n}\n".to_string();
	
	assert_eq!(answer, create_function(given, "".to_string()));
    }

    // Testing create_main()
    #[test]
    fn main_1() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "002".to_string())))
	    /(tr(("line_num".to_string(), "002".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "001".to_string())));
	let answer = "use std::collections::HashMap;\nfn main() {\n  let vars:HashMap<String,(String,String)> 
                     = HashMap::new();\n  line001(vars.clone());\n}\n";
	
	assert_eq!(answer, create_main(given));
    }

    // Testing create_main()
    #[test]
    fn main_2() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "002".to_string())));
	let answer = "use std::collections::HashMap;\nfn main() {\n  let vars:HashMap<String,(String,String)> 
                     = HashMap::new();\n  line001(vars.clone());\n}\n";
	
	assert_eq!(answer, create_main(given));
    }

    // Testing create_main()
    #[test]
    fn main_3() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("start".to_string(), "MAIN".to_string()))
	    /(tr(("line_num".to_string(), "10".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "20".to_string())))
	    /(tr(("line_num".to_string(), "20".to_string())))
	    /(tr(("line_num".to_string(), "30".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "10".to_string())))
	    /(tr(("line_num".to_string(), "40".to_string())));
	let answer = "use std::collections::HashMap;\nfn main() {\n  let vars:HashMap<String,(String,String)> 
                     = HashMap::new();\n  line10(vars.clone());\n}\n";
	
	assert_eq!(answer, create_main(given));
    }
}

// Testing public methods
#[cfg(test)]
mod test {
    // General Imports
    extern crate trees;
    // use self::trees::{tr,Tree,Forest,Node};

    // File Imports
    // use super::generator::*;

}
