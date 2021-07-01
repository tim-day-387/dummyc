// Evaluator module
#![forbid(unsafe_code)]
pub mod evaluator { 
    // Evaluate the given token and return a tuple
    pub fn evaluate(token:String) -> (String, String, String, String) {
	let char_vec:Vec<char> = token.chars().collect();
	let mut output:(String, String, String, String) =
	    ("".to_string(), "".to_string(), "".to_string(), "".to_string());
	let output0:String;
	let output1:String;
	let output2:String;
	let output3:String;
	let mut variable:Vec<char> = Vec::new();
	let mut relational:Vec<char> = Vec::new();
	let mut expression:Vec<char> = Vec::new();
	let mut in_exp:bool = false;


	
	// If every char is a digit, we have a number
	for c in char_vec {
	    if c == '=' {
		relational.push(c);
		in_exp = true;
	    } else if !in_exp {
		variable.push(c);
	    } else {
		expression.push(c);
            }
        }

	// If relational == 0, we're not in a relation 
	if relational.len() == 0 {
	    output0 = "".to_string();
	    output1 = "".to_string();
	    output2 = variable.into_iter().collect();
	    output3 = find_token(output2.clone());
	} else {
	    output0 = variable.into_iter().collect();
	    output1 = relational.into_iter().collect();
	    output2 = [expression.into_iter().collect(),
		    ".to_string()".to_string()].concat();
	    output3 = "string".to_string();
	}

	// Set output
	output.0 = output0;
	output.1 = output1;
	output.2 = output2;
	output.3 = output3;
	
	return output;
    }

    // Classify token
    pub fn find_token(token:String) -> String {
	let output:String;

	// Find what find of token we're looking at
	if is_number(token.clone()) {
	    output = "int".to_string();
	} else if is_string(token.clone()) {
	    output = "string".to_string();
	} else if is_res(token.clone()) {
	    output = "res".to_string();
	} else {
	    output = "eval".to_string();
	}

	return output;
    }
    
    // Check if number
    fn is_number(token:String) -> bool {
	let char_vec:Vec<char> = token.chars().collect();
	let mut output = true;

	// If every char is a digit, we have a number
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

	// If the first and last chars are ", we have a string
	if char_vec.get(0).expect("DNE!") == &'"' && char_vec.get(last).expect("DNE!") == &'"' {
	    output = true;
	}

	return output;
    }

    // Check if a reserved token
    fn is_res(token:String) -> bool {
	let reserved_tokens:Vec<String> = vec!["IF".to_string(), "THEN".to_string(),
					       "GOTO".to_string(), "FOR".to_string(),
					       "TO".to_string(), "NEXT".to_string(),
					       "RETURN".to_string(), "GOSUB".to_string(),
					       "PRINT".to_string(), "LET".to_string(),
					       "DIM".to_string(), "INPUT".to_string(),
					       "READ".to_string(), "DATA".to_string(),
					       "END".to_string()];

	// Check if token is one of the reserved_tokens
	if reserved_tokens.contains(&token) {
	    return true;
	} else {
	    return false;
	}
    }
        
    // Testing evaluate()
    #[test]
    fn eval_1() {
	let given:String = "T=\"Test\"".to_string();
	let answer = ("T".to_string(), "=".to_string(), "\"Test\".to_string()".to_string(),
		      "string".to_string());

	assert_eq!(answer, evaluate(given));
    }

    // Testing evaluate()
    #[test]
    fn eval_2() {
	let given:String = "\"Test\"".to_string();
	let answer = ("".to_string(), "".to_string(), "\"Test\"".to_string(),
		      "string".to_string());

	assert_eq!(answer, evaluate(given));
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
	let answer:String = "eval".to_string();

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

    // Testing is_res()
    #[test]
    fn is_r_1() {
	let given:String = "3".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_res(given));
    }

    // Testing is_res()
    #[test]
    fn is_r_2() {
	let given:String = "LET".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_res(given));
    }

    // Testing is_res()
    #[test]
    fn is_r_3() {
	let given:String = "IFAND".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_res(given));
    }    
}

// Testing public methods
#[cfg(test)]
mod test {
    // File Imports
    use super::evaluator::*;
}
