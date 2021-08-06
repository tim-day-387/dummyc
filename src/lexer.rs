// Lexer module
#![forbid(unsafe_code)]

// Perform all lexer commands
pub fn perform_lexing(file_string:String) -> Vec<(u32, String, String)> {
    return classify(tokenize(remove_comments(file_string)));
}

// Remove all comments (enclosed within ##)
fn remove_comments(file_string:String) -> String {
    let mut in_comment = false;
    let file_bytes = file_string.as_bytes();
    let mut to_del: Vec<usize> = Vec::new();

    // Find chars to del
    for i in 0..file_string.len() {
	// Check if in comment, then mark for delete
	if file_bytes[i] == b'#' {
	    in_comment = !in_comment;
	    to_del.push(i);
	} else if in_comment == true {
	    to_del.push(i);
	}
    }

    // Del chars
    let mut output: Vec<u8> = Vec::new();
    for i in 0..file_string.len() {
	// If slated for del, delete
	if !to_del.contains(&i) {
	    output.push(file_bytes[i]);
	}
    }

    // Return cleaned string
    return String::from_utf8_lossy(&output).to_string();
}

// Create a vector of tokens
fn tokenize(file_string:String) -> Vec<(u32, String)> {
    let file_bytes = file_string.as_bytes();
    let mut token: Vec<u8> = Vec::new();
    let mut output: Vec<(u32, String)> = Vec::new();
    let mut in_string = false;
    let mut line_num = 1;

    // Step through each char
    for i in 0..file_string.len() {
	// Check if in string 
	if file_bytes[i] == b'"' {
	    in_string = !in_string;
	}

	// Add to token or finish token
	if ((file_bytes[i] == b';') | (file_bytes[i] == b',')) && !in_string {
	    // If we hit ; make a token and move on
	    output.push((line_num, String::from_utf8_lossy(&token).to_string()));
	    token = Vec::new();
	} else if (file_bytes[i] != b' ' && file_bytes[i] != b'\n') | in_string {
	    // Push char to token
	    token.push(file_bytes[i])
	} else if (token.len() > 0) && !in_string {
	    // Push token to vector and make new token
	    output.push((line_num, String::from_utf8_lossy(&token).to_string()));
	    token = Vec::new();
	}

	// Update line number
	if file_bytes[i] == b'\n' {
	    line_num = line_num + 1;
	}
    }

    // If we have a stray token, push it
    if token.len() > 0 {
	output.push((line_num, String::from_utf8_lossy(&token).to_string()));
    }

    return output;
}

// Create a vector of tokens
fn classify(tokens:Vec<(u32, String)>) -> Vec<(u32, String, String)> {
    let mut output: Vec<(u32, String, String)> = Vec::new();
    let mut line_num = 0;

    // Find each token
    for t in tokens {
	// Update line number, identify line number token
	if line_num != t.0.clone() {
	    line_num = t.0.clone();
	    output.push((t.0.clone(), t.1.clone(), "line_num".to_string()));
	    continue;
        }

	// Identify non-line number tokens
	output.push((t.0.clone(), t.1.clone(), find_token(t.1.clone())));
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

    // Testing remove_comments()
    #[test]
    fn rm_cmts_1() {
	let given:String = "001 GOTO 001 #This is an example comment#".to_string();
	let answer:String = "001 GOTO 001 ".to_string();

	assert_eq!(answer, remove_comments(given));
    }

    // Testing remove_comments()
    #[test]
    fn rm_cmts_2() {
	let given:String = "00#yuh#0 PRINT \"This is a Dummy program\"
                            001 LET hat = \"the\"
                            002
                            003 #This is a test comment#
                            004
                            005 IF hat = \"the\" THEN GOTO 0#yuh#08 ELSE GOTO 010 #This is an#
                            0#yuh#06
                            007
                            008 PRINT#yuh# \"Hat is 7\"
                            009 END
                            010 EN#yuh#D".to_string();
	let answer:String = "000 PRINT \"This is a Dummy program\"
                            001 LET hat = \"the\"
                            002
                            003 
                            004
                            005 IF hat = \"the\" THEN GOTO 008 ELSE GOTO 010 
                            006
                            007
                            008 PRINT \"Hat is 7\"
                            009 END
                            010 END".to_string();

	assert_eq!(answer, remove_comments(given));
    }

    // Testing remove_comments()
    #[test]
    fn rm_cmts_3() {
	let given:String = "00##0 PR##INT \"This is# a Dum#my program\"
                            001 L##ET hat = \"the\"
                            002 LET## BaBa########## = ##\"booey\"
                            003 ##GOTO 0##00".to_string();
	let answer:String = "000 PRINT \"This ismy program\"
                            001 LET hat = \"the\"
                            002 LET BaBa = \"booey\"
                            003 GOTO 000".to_string();

	assert_eq!(answer, remove_comments(given));
    }

    // Testing tokenize()
    #[test]
    fn token_1() {
	let given:String = "001 GOTO 001".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "GOTO".to_string()),
					     (1, "001".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_2() {
	let given:String = "000 PRINT \"This ismy program\"
                            001 LET hat=\"the\"
                            002 LET BaBa=\"booey\"
                            003 GOTO 000".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "000".to_string()),(1, "PRINT".to_string()),
					     (1, "\"This ismy program\"".to_string()),
					     (2, "001".to_string()),
					     (2, "LET".to_string()), (2, "hat=\"the\"".to_string()),
					     (3, "002".to_string()), (3, "LET".to_string()),
					     (3, "BaBa=\"booey\"".to_string()),
					     (4, "003".to_string()), (4, "GOTO".to_string()),
					     (4, "000".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_3() {
	let given:String = "001 002 
                           345 #yuh#
                           CAR     HAT".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "002".to_string()),
					     (2, "345".to_string()),(2, "#yuh#".to_string()),
					     (3, "CAR".to_string()),(3, "HAT".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_4() {
	let given:String = "1010 PRINT \"A HAS\";N;\"ELEMENTS\"
                            1030 PRINT \"A(\";I;\")=\";A(I)".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "1010".to_string()),(1, "PRINT".to_string()),
				             (1, "\"A HAS\"".to_string()),
					     (1, "N".to_string()),(1, "\"ELEMENTS\"".to_string()),
					     (2, "1030".to_string()),(2, "PRINT".to_string()),
					     (2, "\"A(\"".to_string()),(2, "I".to_string()),
	                                     (2, "\")=\"".to_string()),(2, "A(I)".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_5() {
	let given:String = "1010 PRINT \"A HAS\";N;\"ELEMENTS\"".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "1010".to_string()),(1, "PRINT".to_string()),
				             (1, "\"A HAS\"".to_string()),(1, "N".to_string()),
					     (1, "\"ELEMENTS\"".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_6() {
	let given:String = "9000 DATA 9,1,5,5".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "9000".to_string()),(1, "DATA".to_string()),
				             (1, "9".to_string()),(1, "1".to_string()),
					     (1, "5".to_string()),(1, "5".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_7() {
	let given:String = "80 IF F=1 THEN 110
                            90 PRINT X;\"N,;,;,OUND\"".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "80".to_string()),(1, "IF".to_string()),
				             (1, "F=1".to_string()),(1, "THEN".to_string()),
					     (1, "110".to_string()),(2, "90".to_string()),
	                                     (2, "PRINT".to_string()),(2, "X".to_string()),
	                                     (2, "\"N,;,;,OUND\"".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing classify()
    #[test]
    fn class_1() {
	let given:Vec<(u32, String)> = vec![(1, "80".to_string()),(1, "IF".to_string()),
				            (1, "F=1".to_string()),(1, "THEN".to_string()),
					    (1, "110".to_string()),(2, "90".to_string()),
	                                    (2, "PRINT".to_string()),(2, "X".to_string()),
	                                    (2, "\"N,;,;,OUND\"".to_string())];
	let answer:Vec<(u32, String, String)> = vec![(1, "80".to_string(), "line_num".to_string()),
						     (1, "IF".to_string(), "res".to_string()),
						     (1, "F=1".to_string(), "eval".to_string()),
						     (1, "THEN".to_string(), "res".to_string()),
						     (1, "110".to_string(), "int".to_string()),
						     (2, "90".to_string(), "line_num".to_string()),
						     (2, "PRINT".to_string(), "res".to_string()),
						     (2, "X".to_string(), "eval".to_string()),
						     (2, "\"N,;,;,OUND\"".to_string(),
						      "string".to_string())];

	assert_eq!(answer, classify(given));
    }

    // Testing classify()
    #[test]
    fn class_2() {
	let given:Vec<(u32, String)> = vec![(1, "9000".to_string()),(1, "DATA".to_string()),
				            (1, "9".to_string()),(1, "1".to_string()),
					    (1, "5".to_string()),(1, "5".to_string())]; 
	let answer:Vec<(u32, String, String)> = vec![(1, "9000".to_string(), "line_num".to_string()),
						     (1, "DATA".to_string(), "res".to_string()),
						     (1, "9".to_string(), "int".to_string()),
						     (1, "1".to_string(), "int".to_string()),
						     (1, "5".to_string(), "int".to_string()),
						     (1, "5".to_string(), "int".to_string())];

	assert_eq!(answer, classify(given));
    }

    // Testing classify()
    #[test]
    fn class_3() {
	let given:Vec<(u32, String)> = vec![(1, "1010".to_string()),(1, "PRINT".to_string()),
				            (1, "\"A HAS\"".to_string()),
					    (1, "N".to_string()),(1, "\"ELEMENTS\"".to_string()),
					    (2, "1030".to_string()),(2, "PRINT".to_string()),
					    (2, "\"A(\"".to_string()),(2, "I".to_string()),
	                                    (2, "\")=\"".to_string()),(2, "A(I)".to_string())];
	let answer:Vec<(u32, String, String)> = vec![(1, "1010".to_string(), "line_num".to_string()),
						     (1, "PRINT".to_string(), "res".to_string()),
						     (1, "\"A HAS\"".to_string(), "string".to_string()),
						     (1, "N".to_string(), "eval".to_string()),
						     (1, "\"ELEMENTS\"".to_string(), "string".to_string()),
						     (2, "1030".to_string(), "line_num".to_string()),
						     (2, "PRINT".to_string(), "res".to_string()),
						     (2, "\"A(\"".to_string(), "string".to_string()),
						     (2, "I".to_string(), "eval".to_string()),
						     (2, "\")=\"".to_string(), "string".to_string()),
						     (2, "A(I)".to_string(), "eval".to_string())];

	assert_eq!(answer, classify(given));
    }
    
    // Testing perform_lexing()
    #[test]
    fn lex_1() {
	let given:String = "001 GOTO 001 #This is an example comment#".to_string();
	let answer:Vec<(u32, String, String)> = vec![(1, "001".to_string(), "line_num".to_string()),
						     (1, "GOTO".to_string(), "res".to_string()),
					             (1, "001".to_string(), "int".to_string())];
	
	assert_eq!(answer, perform_lexing(given));
    }

    // Testing perform_lexing()
    #[test]
    fn lex_2() {
	let given:String = "00##0 PR##INT \"This is# a Dum#my program\"
                            001 L##ET hat=\"the\"
                            002 LET## BaBa##########=##\"booey\"
                            003 ##GOTO 0##00".to_string();
	let answer:Vec<(u32, String, String)> = vec![(1, "000".to_string(), "line_num".to_string()),
						     (1, "PRINT".to_string(), "res".to_string()),
						     (1, "\"This ismy program\"".to_string(), "string".to_string()),
						     (2, "001".to_string(), "line_num".to_string()),
						     (2, "LET".to_string(), "res".to_string()),
						     (2, "hat=\"the\"".to_string(), "eval".to_string()),
						     (3, "002".to_string(), "line_num".to_string()),
						     (3, "LET".to_string(), "res".to_string()),
						     (3, "BaBa=\"booey\"".to_string(), "eval".to_string()),
						     (4, "003".to_string(), "line_num".to_string()),
						     (4, "GOTO".to_string(), "res".to_string()),
						     (4, "000".to_string(), "int".to_string())];
	
	assert_eq!(answer, perform_lexing(given));
    }

    // Testing perform_lexing()
    #[test]
    fn lex_3() {
	let given:String = "001 002 
                           345 #yuh#
                           387     HAT".to_string();
	let answer:Vec<(u32, String, String)> = vec![(1, "001".to_string(), "line_num".to_string()),
						     (1, "002".to_string(), "int".to_string()),
						     (2, "345".to_string(), "line_num".to_string()),
						     (3, "387".to_string(), "line_num".to_string()),
						     (3, "HAT".to_string(), "eval".to_string())];

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
