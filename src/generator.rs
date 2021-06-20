// Generator module
#![forbid(unsafe_code)]
pub mod generator {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree,Forest,Node};
    use std::io::Write;

    // Create Rust code from the abstract syntax tree
    pub fn generate(input:Tree<String>) -> String {
	let mut output:String;
	let mut next_tree:Tree<String>;
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
	    
	    // Iterate and concatenate
	    i = i + 1;
	    output = [output, create_function(next_tree.clone())].concat();
	}
	
	return output;
    }

    // Create main body of the Rust code
    fn create_main(input:Tree<String>) -> String {
	let mut output = "fn main() {\n".to_string();
	let mut next_line:String;
	let mut next_token:String;
	let mut next_option;
	let mut i = 0;

	// Iter through tree while constructing output
	loop {
	    // Get next child in tree
	    next_option = input.iter().nth(i);
	    match next_option {
		Some(next_option) => next_token = next_option.to_string(),
		None => break,
	    }
	    
	    // Iterate and concatenate
	    i = i + 1;
	    next_line = ["  line".to_string(), remove_children(next_token), "();\n".to_string()].concat();
	    output = [output, next_line].concat();
	}

	// Add last bracket
	output = [output, "}\n".to_string()].concat();
	
	return output;
    }

    // Create Rust from given subtree
    fn create_function(subtree:Tree<String>) -> String {
	let mut output = ["fn line".to_string(), remove_children(subtree.root().to_string()), "() {\n".to_string()].concat();
	let mut next_token:String;
	let mut next_line:String;
	let mut next_option;
	let mut children:Vec<String> = Vec::new();
	let mut i = 0;

	// Iter through tree while constructing vector
	loop {
	    // Get next child in tree
	    next_option = subtree.iter().nth(i);
	    match next_option {
		Some(next_option) => next_token = next_option.to_string(),
		None => break,
	    }
	    
	    // Iterate and concatenate
	    i = i + 1;

	    // Push each token
	    children.push(next_token);
	}

	// Decide on function
	if children.len() == 0 {
	} else if children.get(0).expect("DNE!").to_string() == "PRINT".to_string() {
	    output = [output, "  println!(".to_string(), children.get(1).expect("DNE!").to_string(), ");\n".to_string()].concat(); 
	} else if children.get(0).expect("DNE!").to_string() == "GOTO".to_string() {
	    output = [output, "  line".to_string(), children.get(1).expect("DNE!").to_string(), "();\n".to_string()].concat();
	}
	    
	// Add last bracket
	output = [output, "}\n".to_string()].concat();
	
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
	let given:Tree<String> = (tr("MAIN".to_string())
		      /(tr("001".to_string()) /tr("GOTO".to_string()) /tr("002".to_string()))
	              /(tr("002".to_string()) /tr("GOTO".to_string()) /tr("001".to_string())));
	let answer = "fn main() {\n  line001();\n  line002();\n}\n";
	
	assert_eq!(answer, create_main(given));
    }

    // Testing create_main()
    #[test]
    fn main_2() {
	let given:Tree<String> = (tr("MAIN".to_string())
		      /(tr("001".to_string()) /tr("GOTO".to_string()) /tr("002".to_string())));
	let answer = "fn main() {\n  line001();\n}\n";
	
	assert_eq!(answer, create_main(given));
    }

    // Testing create_main()
    #[test]
    fn main_3() {
	let given:Tree<String> = (tr("MAIN".to_string())
			/(tr("001".to_string()) /tr("GOTO".to_string()) /tr("002".to_string()))
			/(tr("002".to_string()))
			/(tr("003".to_string()) /tr("GOTO".to_string()) /tr("001".to_string()))
	                /(tr("004".to_string())));
	let answer = "fn main() {\n  line001();\n  line002();\n  line003();\n  line004();\n}\n";
	
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
