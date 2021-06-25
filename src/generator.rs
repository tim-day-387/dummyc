// Generator module
#![forbid(unsafe_code)]
pub mod generator {
    // General Imports
    extern crate trees;
    use self::trees::{Tree};

    // Create Rust code from the abstract syntax tree
    pub fn generate(input:Tree<(String, String)>) -> String {
	let mut output:String;
	let mut next_tree:Tree<(String, String)>;
	let mut next_line:String;
	let mut next_option;
	let mut i = 0;

	output = create_main(input.clone());

	// Iter through tree while constructing output
	loop {
	    // Get next child in tree
	    next_option = input.iter().nth(i);
	    match next_option {
		Some(next_option) => next_tree = next_option.to_owned(),
		None => break,
	    }

	    // Get next child in tree
	    next_option = input.iter().nth(i+1);
	    match next_option {
		Some(next_option) => next_line = next_option.data.1.to_string(),
		None => next_line = "".to_string(),
	    }
	    
	    // Iterate and concatenate
	    i = i + 1;
	    output = [output, create_function(next_tree.clone(), next_line)].concat();
	}
	
	return output;
    }

    // Create main body of the Rust code
    fn create_main(input:Tree<(String, String)>) -> String {
	let mut output = "use std::collections::HashMap;\nfn main() {\n".to_string();
	let mut next_line:String;
	let next_token:String;
	let next_option;

	// Get next child in tree
	next_option = input.iter().nth(0);
	match next_option {
	    Some(next_option) => next_token = next_option.data.1.to_string(),
	    None => return [output, "}\n".to_string()].concat(),
	}

	// Define hash map
        next_line = ["  let vars:HashMap<String,(String,String)> 
                     = HashMap::new();\n".to_string()].concat();
	output = [output, next_line].concat();
	
	// Function call
	next_line = ["  line".to_string(), remove_children(next_token),
		     "(vars.clone());\n".to_string()].concat();
	output = [output, next_line].concat();

	// Add last bracket
	output = [output, "}\n".to_string()].concat();
	
	return output;
    }

    // Create Rust from given subtree
    fn create_function(subtree:Tree<(String, String)>, line_num:String) -> String {
	let mut output = ["fn line".to_string(),
			  remove_children(subtree.root().data.1.to_string()),
			  "(vars:HashMap<String,(String,String)>) {\n".to_string()].concat();
	let mut next_token:(String, String);
	let next_line:String;
	let mut next_line_num:String = line_num;
	let mut next_option;
	let mut children:Vec<String> = Vec::new();
	let mut children_type:Vec<String> = Vec::new();
	let mut i = 0;

	// Iter through tree while constructing vector
	loop {
	    // Get next child in tree
	    next_option = subtree.iter().nth(i);
	    match next_option {
		Some(next_option) => next_token = next_option.data.clone(),
		None => break,
	    }
	    
	    // Iterate and concatenate
	    i = i + 1;

	    // Push each token
	    children.push(next_token.1.to_string());
	    children_type.push(next_token.0.to_string());
	}

	// Decide on function
	if children.len() == 0 {
	} else if children.get(0).expect("DNE!").to_string() == "PRINT".to_string() {
	    if children_type.get(1).expect("DNE!").to_string() == "string" {
		output = [output, "  println!(".to_string(),
		      children.get(1).expect("DNE!").to_string(), ");\n".to_string()].concat(); 
            } else if children_type.get(1).expect("DNE!").to_string() == "string" {
		output = [output, "  println!(vars.get(".to_string(),
		      children.get(1).expect("DNE!").to_string(), ".1));\n".to_string()].concat(); 
	    }
	} else if children.get(0).expect("DNE!").to_string() == "GOTO".to_string() {
	    next_line_num = children.get(1).expect("DNE!").to_string();
	} else if children.get(0).expect("DNE!").to_string() == "LET".to_string() {
	    output = [output, "  vars.insert(\"".to_string(), children.get(1).expect("DNE!").to_string(), "\".to_string(),(\"string\".to_string(),".to_string(), children.get(3).expect("DNE!").to_string(), ".to_string()));\n".to_string()].concat();   
	}    

	// Function call
	if next_line_num != "".to_string() {
	    next_line = ["  line".to_string(), next_line_num,
			 "(vars.clone());\n".to_string(), "}\n".to_string()].concat();
	} else {
	    next_line = "}\n".to_string();
	}
	output = [output, next_line].concat();
	
	return output;
    }
    
    // Remove all comments (enclosed within ##)
    fn remove_children(tree_string:String) -> String {
	let tree_bytes = tree_string.as_bytes();
	let mut to_del: Vec<usize> = Vec::new();
	let mut in_child = false;

	// Find chars to del
	for i in 0..tree_string.len() {
	    // Check if in comment, then mark for delete
	    if (tree_bytes[i] == b'(') | in_child {
		in_child = true;
		to_del.push(i);
	    }
	}

	// Del chars
	let mut output: Vec<u8> = Vec::new();
	let mut counter = 0;
	for i in 0..tree_string.len() {
	    // If slated for del, delete
	    if !to_del.contains(&i) {
		output.push(tree_bytes[i]);
		counter = counter + 1;
	    }
	}

	// Return cleaned string
	return String::from_utf8_lossy(&output).to_string();
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
	let func1 = "fn line001(vars:HashMap<String,(String,String)>) {\n  line002(vars.clone());\n}\n".to_string();
	let answer = [main, func1].concat();
	
	assert_eq!(answer, generate(given));
    }
    
    // Testing create_function()
    #[test]
    fn func_1() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "PRINT".to_string()))
	    /tr(("string".to_string(), "\"This is a sample\"".to_string()));
	let answer = "fn line001(vars:HashMap<String,(String,String)>) {\n  println!(\"This is a sample\");\n  line002(vars.clone());\n}\n".to_string();
	
	assert_eq!(answer, create_function(given, "002".to_string()));
    }

    // Testing create_function()
    #[test]
    fn func_2() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "GOTO".to_string()))
	    /tr(("line_num".to_string(), "002".to_string()));
	let answer = "fn line001(vars:HashMap<String,(String,String)>) {\n  line002(vars.clone());\n}\n".to_string();
	
	assert_eq!(answer, create_function(given, "345".to_string()));
    }

    // Testing create_function()
    #[test]
    fn func_3() {
	use self::trees::{tr};
	let given:Tree<(String, String)> = tr(("line_num".to_string(), "001".to_string()))
	    /tr(("res".to_string(), "PRINT".to_string()))
	    /tr(("string".to_string(), "\"This is a sample\"".to_string()));
	let answer = "fn line001(vars:HashMap<String,(String,String)>) {\n  println!(\"This is a sample\");\n}\n".to_string();
	
	assert_eq!(answer, create_function(given, "".to_string()));
    }

    // Testing remove_children()
    #[test]
    fn rm_ch_1() {
	let given:String = "045( 234 234 234 234 234)".to_string();
	let answer:String = "045".to_string();

	assert_eq!(answer, remove_children(given));
    }

    // Testing remove_children()
    #[test]
    fn rm_ch_2() {
	let given:String = "df}d}f]e}k)doe( 234 2asdf;lkj34 234 234 234)".to_string();
	let answer:String = "df}d}f]e}k)doe".to_string();

	assert_eq!(answer, remove_children(given));
    }

    // Testing remove_children()
    #[test]
    fn rm_ch_3() {
	let given:String = "ASDKFJ333ed383838".to_string();
	let answer:String = "ASDKFJ333ed383838".to_string();

	assert_eq!(answer, remove_children(given));
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
