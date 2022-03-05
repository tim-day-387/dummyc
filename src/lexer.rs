// Lexer modulen
#![forbid(unsafe_code)]

// Perform all lexer commands
pub fn perform_lexing(file_string:String) -> Vec<(String, String)> {
    return classify(tokenize(file_string));
}

// Get line number
pub fn get_line_num(line:String) -> i64 {
    let tokens = perform_lexing(line.clone());

    return tokens[0].0.clone().parse::<i64>().unwrap()
}

// Create a vector of tokens
fn tokenize(file_string:String) -> Vec<String> {
    let file_bytes = file_string.as_bytes();
    let mut token: Vec<u8> = Vec::new();
    let mut output: Vec<String> = Vec::new();
    let mut in_string = false;

    // Step through each char
    for i in 0..file_string.len() {
	// Check if in string 
	if file_bytes[i] == b'"' {
	    in_string = !in_string;
	}

	// Add to token or finish token
	if ((file_bytes[i] == b';') | (file_bytes[i] == b',')) && !in_string {
	    // If we hit ; make a token and move on
	    output.push(String::from_utf8_lossy(&token).to_string());
	    token = Vec::new();
	} else if (file_bytes[i] != b' ' && file_bytes[i] != b'\n') | in_string {
	    // Push char to token
	    token.push(file_bytes[i])
	} else if (token.len() > 0) && !in_string {
	    // Push token to vector and make new token
	    output.push(String::from_utf8_lossy(&token).to_string());
	    token = Vec::new();
	}
    }

    // If we have a stray token, push it
    if token.len() > 0 {
	output.push(String::from_utf8_lossy(&token).to_string());
    }

    return output;
}

// Create a vector of tokens
fn classify(tokens:Vec<String>) -> Vec<(String, String)> {
    let mut output: Vec<(String, String)> = Vec::new();
    let mut line_set = false;

    // Find each token
    for t in tokens {
	// Check if the line number has been seen
	if !line_set {
	    // Classify line_num
	    output.push((t.clone(), "line_num".to_string()));
	    line_set = true;
	} else {
	    // Identify non-line number tokens
	    output.push((t.clone(), find_token(t.clone())));
	}
    }

    return output;
}

// Classify token
pub fn find_token(token:String) -> String {
    let output:String;

    // Find what find of token we're looking at
    if is_int(token.clone()) {
	output = "int".to_string();
    } else if is_string(token.clone()) {
	output = "string".to_string();
    } else if is_res(token.clone()) {
	output = "res".to_string();
    } else if is_float(token.clone()) {
	output = "float".to_string();	
    } else {
	output = "eval".to_string();
    }

    return output;
}

// Check if float
fn is_float(token:String) -> bool {
    let char_vec:Vec<char> = token.chars().collect();
    let mut output = true;
    let mut seen_point = false;

    // If every char is a digit, or a decimal point
    for c in char_vec {
	// Check if char is digit
	if !c.is_digit(10) {
	    // Check if char is a point
	    if c != '.' {
		// Not a point, not a float
		output = false;
	    } else if seen_point {
		// Already had a point, not a float
		output = false;
	    } else {
		// First point, might be a float
		seen_point = true;
	    }
	}    
    }

    // Must have seen a point 
    output = output & seen_point;
    
    return output;
}

// Check if integer
fn is_int(token:String) -> bool {
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
    if char_vec.get(0).expect("First char missing!") == &'"' &&
	char_vec.get(last).expect("First char missing!") == &'"' {
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

// Testing methods
#[cfg(test)]
mod test {
    // File Imports
    use super::*;

    // Testing tokenize()
    #[test]
    fn token_1() {
	let given:String = "001 GOTO 001".to_string();
	let answer:Vec<String> = vec!["001".to_string(),"GOTO".to_string(),
					     "001".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_2() {
	let given:String = "000 PRINT \"This ismy program\"".to_string();
	let answer:Vec<String> = vec!["000".to_string(),"PRINT".to_string(),
					     "\"This ismy program\"".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_3() {
	let given:String = "1010 PRINT \"A HAS\";N;\"ELEMENTS\"".to_string();
	let answer:Vec<String> = vec!["1010".to_string(),"PRINT".to_string(),
				             "\"A HAS\"".to_string(),
					     "N".to_string(),"\"ELEMENTS\"".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_4() {
	let given:String = "9000 DATA 9,1,5,5".to_string();
	let answer:Vec<String> = vec!["9000".to_string(),"DATA".to_string(),
				             "9".to_string(),"1".to_string(),
					     "5".to_string(),"5".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing classify()
    #[test]
    fn class_1() {
	let given:Vec<String> = vec!["80".to_string(),"IF".to_string(),
				            "F=1".to_string(),"THEN".to_string(),
					    "110".to_string()];
	let answer:Vec<(String, String)> = vec![("80".to_string(), "line_num".to_string()),
						     ("IF".to_string(), "res".to_string()),
						     ("F=1".to_string(), "eval".to_string()),
						     ("THEN".to_string(), "res".to_string()),
						     ("110".to_string(), "int".to_string())];

	assert_eq!(answer, classify(given));
    }

    // Testing classify()
    #[test]
    fn class_2() {
	let given:Vec<String> = vec!["9000".to_string(),"DATA".to_string(),
				            "9".to_string(),"1".to_string(),
					    "5".to_string(),"5".to_string()]; 
	let answer:Vec<(String, String)> = vec![("9000".to_string(), "line_num".to_string()),
						     ("DATA".to_string(), "res".to_string()),
						     ("9".to_string(), "int".to_string()),
						     ("1".to_string(), "int".to_string()),
						     ("5".to_string(), "int".to_string()),
						     ("5".to_string(), "int".to_string())];

	assert_eq!(answer, classify(given));
    }
    
    // Testing perform_lexing()
    #[test]
    fn lex_1() {
	let given:String = "001 GOTO 001".to_string();
	let answer:Vec<(String, String)> = vec![("001".to_string(), "line_num".to_string()),
						     ("GOTO".to_string(), "res".to_string()),
					             ("001".to_string(), "int".to_string())];
	
	assert_eq!(answer, perform_lexing(given));
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

    // Testing find_token()
    #[test]
    fn find_4() {
	let given:String = ".1326546".to_string();
	let answer:String = "float".to_string();

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
    
    // Testing is_float()
    #[test]
    fn is_f_1() {
	let given:String = "0F1".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_float(given));
    }

    // Testing is_float()
    #[test]
    fn is_f_2() {
	let given:String = "387".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_float(given));
    }

    // Testing is_number()
    #[test]
    fn is_f_3() {
	let given:String = "38.7".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_float(given));
    }
    
    // Testing is_int()
    #[test]
    fn is_i_1() {
	let given:String = "0F1".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_int(given));
    }

    // Testing is_int()
    #[test]
    fn is_i_2() {
	let given:String = "LET".to_string();
	let answer:bool = false;

	assert_eq!(answer, is_int(given));
    }

    // Testing is_int()
    #[test]
    fn is_i_3() {
	let given:String = "387".to_string();
	let answer:bool = true;

	assert_eq!(answer, is_int(given));
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
