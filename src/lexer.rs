// Lexer modulen
#![forbid(unsafe_code)]

// Perform all lexer commands
pub fn perform_lexing(file_string:String) -> (Vec<String>, Vec<String>) {
    return classify(tokenize(file_string));
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
fn classify(tokens:Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut text:Vec<String> = Vec::new();
    let mut class:Vec<String> = Vec::new();
    let mut line_set = false;

    // Find each token
    for t in tokens {
	// Check if the line number has been seen
	if !line_set {
	    // Classify line_num
	    text.push(t.clone());
	    class.push("line_num".to_string());
	    line_set = true;
	} else {
	    // Identify non-line number
	    text.push(t.clone());
	    class.push(find_token(t.clone()));
	}
    }

    return (text, class);
}

// Classify token
fn find_token(token:String) -> String {
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
