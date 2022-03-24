// Lexer modulen
#![forbid(unsafe_code)]

// Perform all lexer commands
pub fn perform_lexing(file_string:String) -> (Vec<String>, Vec<String>) {
    return classify(tokenize(remove_spaces(file_string)));
}

// Split an expression across the relational
pub fn split(token:String) -> (String, String, String) {
    let char_vec:Vec<char> = token.chars().collect();
    let mut first_part_string:String = "".to_string();
    let mut relational_string:String = "".to_string();
    let mut second_part_string:String = "".to_string();
    let mut in_exp:bool = false;
    
    // Splits expression based on relational
    for c in char_vec {
	if c == '=' || c == '<' || c == '>' || c == '!' {
	    relational_string.push(c);
	    in_exp = true;
	} else if !in_exp {
	    first_part_string.push(c);
	} else {
	    second_part_string.push(c);
        }
    }

    return (first_part_string, relational_string, second_part_string);
}

// Split an expression across the relational
pub fn split_over_op(token:String) -> (String, String, String) {
    let char_vec:Vec<char> = token.chars().collect();
    let mut first_part_string:String = "".to_string();
    let mut operation_string:String = "".to_string();
    let mut second_part_string:String = "".to_string();
    let mut in_exp:bool = false;
    
    // Splits expression based on operation
    for c in char_vec {
	if (c == '+' || c == '/' || c == '*' || c == '-') && !in_exp {
	    operation_string.push(c);
	    in_exp = true;
	} else if !in_exp {
	    first_part_string.push(c);
	} else {
	    second_part_string.push(c);
        }
    }

    return (first_part_string, operation_string, second_part_string);
}

// Function to remove spaces
fn remove_spaces(file_string:String) -> String {
    let char_vec:Vec<char> = file_string.chars().collect();
    let mut output_string:String = "".to_string();
    let mut in_string = false;
    
    for c in char_vec {
	if c != ' ' {
	    output_string.push(c);
        } else if in_string {
	    output_string.push(c);
	}

	if c == '"' {
	    in_string = !in_string;
	}
    }

    return output_string;
}

// Create a vector of tokens
fn tokenize(file_string:String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut cur:String = file_string.trim().to_string().clone();
    let mut offset = 0;
    let locations = find_res_tokens(file_string);

    for i in locations {
	let (chunk, rest) = cur.split_at(i - offset);
	offset = i;
	output.push(chunk.to_string());
	cur = rest.to_string();
    }

    if cur != "".to_string() {
	output.push(cur);
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
	if !line_set && find_token(t.clone()) == "int".to_string() {
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

    if class[0] != "line_num".to_string() {
	panic!("LEXER: classify: Line does not have a line number");
    }

    return (text, class);
}

// Find the beginnings and ends of all matching reserved tokens
fn find_res_tokens(file_string:String) -> Vec<usize> {
    let reserved_tokens:Vec<String> = vec!["IF".to_string(), "THEN".to_string(),
					   "GOTO".to_string(), "FOR".to_string(),
					   ";".to_string(), ",".to_string(),
					   "NEXT".to_string(), "REM".to_string(),
					   "RETURN".to_string(), "GOSUB".to_string(),
					   "PRINT".to_string(), "LET".to_string(),
					   "DIM".to_string(), "INPUT".to_string(),
					   "READ".to_string(), "DATA".to_string(),
					   "END".to_string(), "STOP".to_string()];
    let mut locations:Vec<usize> = Vec::new();
    let mut i_in_string = Vec::new();
    let mut in_string = false;
    let mut i = 0;
    let char_vec:Vec<char> = file_string.chars().collect();
    
    for c in char_vec {
	if c == '"' {
	    in_string = !in_string;
	}

	if in_string {
	    i_in_string.push(i);
        }

	i = i + 1;
    }
    
    for i in reserved_tokens {
	let value:Vec<_> = file_string.match_indices(&i).map(|(j, _)|j).collect();
	
	for loc in value {
	    if !i_in_string.contains(&loc) {
		locations.push(loc);
		locations.push(loc + i.len());
	    }
	}
    }

    locations.sort();
    
    return locations;
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
pub fn is_float(token:String) -> bool {
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
pub fn is_string(token:String) -> bool {
    let char_vec:Vec<char> = token.chars().collect();
    let mut in_string = false;
    let mut output = true;

    // Step through each char
    for i in 0..(token.len() - 1) {
	// Check if in string 
	if char_vec[i] == '"' {
	    in_string = !in_string;
	}

	output = output && in_string;
    }

    output = output && (char_vec[token.len() - 1] == '"');
    
    return output;
}

// Check if a reserved token
fn is_res(token:String) -> bool {
    let reserved_tokens:Vec<String> = vec!["IF".to_string(), "THEN".to_string(),
					   "GOTO".to_string(), "FOR".to_string(),
					   ";".to_string(), ",".to_string(),
					   "NEXT".to_string(), "REM".to_string(),
					   "RETURN".to_string(), "GOSUB".to_string(),
					   "PRINT".to_string(), "LET".to_string(),
					   "DIM".to_string(), "INPUT".to_string(),
					   "READ".to_string(), "DATA".to_string(),
					   "END".to_string(), "STOP".to_string()];
    
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
    use lexer::*;

    // Testing tokenize()
    #[test]
    fn tokenize_1() {
	let given:String = remove_spaces("30000 REM This is just some random test lol".to_string());
	let answer:Vec<String> = vec!["30000".to_string(), "REM".to_string(), "Thisisjustsomerandomtestlol".to_string()];

	assert_eq!(answer, tokenize(given));
    }
    
    // Testing tokenize()
    #[test]
    fn tokenize_2() {
	let given:String = remove_spaces("10 STOP".to_string());
	let answer:Vec<String> = vec!["10".to_string(), "STOP".to_string()];

	assert_eq!(answer, tokenize(given));
    }    

    // Testing tokenize()
    #[test]
    fn tokenize_3() {
	let given:String = remove_spaces("100 LET A=\"Fuh\"".to_string());
	let answer:Vec<String> = vec!["100".to_string(), "LET".to_string(), "A=\"Fuh\"".to_string()];

	assert_eq!(answer, tokenize(given));
    }
    
    // Testing tokenize()
    #[test]
    fn tokenize_4() {
	let given:String = remove_spaces("2000 PRINT A;B;C;".to_string());
	let answer:Vec<String> = vec!["2000".to_string(), "PRINT".to_string(), "A".to_string(), ";".to_string(), "B".to_string(), ";".to_string(), "C".to_string(), ";".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_5() {
	let given:String = remove_spaces("100 LET A=\"Fuh RETURN TO LET STOP GOTO GOSUB\"".to_string());
	let answer:Vec<String> = vec!["100".to_string(), "LET".to_string(), "A=\"Fuh RETURN TO LET STOP GOTO GOSUB\"".to_string()];

	assert_eq!(answer, tokenize(given));
    }
        
    // Testing split()
    #[test]
    fn split_1() {
	let given:String = "A=\"Fuh\"".to_string();
	let answer = ("A".to_string(), "=".to_string(), "\"Fuh\"".to_string());

	assert_eq!(answer, split(given));
    }

    // Testing split()
    #[test]
    fn split_2() {
	let given:String = "B<=23423984723fffffjjjdjdj{}||[".to_string();
	let answer = ("B".to_string(), "<=".to_string(), "23423984723fffffjjjdjdj{}||[".to_string());

	assert_eq!(answer, split(given));
    }

    // Testing remove_spaces()
    #[test]
    fn remove_spaces_1() {
	let given:String = "A =\"Fuh\"".to_string();
	let answer:String = "A=\"Fuh\"".to_string();

	assert_eq!(answer, remove_spaces(given));
    }

    // Testing remove_spaces()
    #[test]
    fn remove_spaces_2() {
	let given:String = "                  ".to_string();
	let answer:String = "".to_string();

	assert_eq!(answer, remove_spaces(given));
    }

    // Testing remove_spaces()
    #[test]
    fn remove_spaces_3() {
	let given:String = "     1 0 0 P R  I NT\" Fuh Foo  Fin \"".to_string();
	let answer:String = "100PRINT\" Fuh Foo  Fin \"".to_string();

	assert_eq!(answer, remove_spaces(given));
    }
}
