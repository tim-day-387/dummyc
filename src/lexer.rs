// Lexer modulen
#![forbid(unsafe_code)]

// Constants
const RESERVED:[&'static str; 23] = ["RESTORE", "RETURN", "GOSUB", "PRINT", "INPUT", "READ", "DATA", "STOP",
				     "GOTO", "THEN", "NEXT", "STEP", "FOR", "REM", "LET", "DIM", "END", "DEF",
				     "IF", "TO", "ON", ";", ","];

// Perform all lexer commands
pub fn perform_lexing(file_string:String) -> Vec<String> {
    return verify(tokenize(remove_spaces(file_string)));
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
    let mut left_paran = 0;
    let mut right_paran = 0;
    
    // Splits expression based on operation
    for c in char_vec {
	if (c == '+' || c == '/' || c == '*' || c == '-') && (left_paran == right_paran) && !in_exp {
	    operation_string.push(c);
	    in_exp = true;
	} else if c == '(' {
	    left_paran = left_paran + 1;
	} else if c == ')' {
	    right_paran = right_paran + 1;
	} else if !in_exp {
	    first_part_string.push(c);
	} else {
	    second_part_string.push(c);
        }
    }

    return (first_part_string, operation_string, second_part_string);
}

// Create an error if the command is not formed properly
fn verify(tokens:Vec<String>) -> Vec<String> {
    if !is_int(tokens[0].clone()) {
	panic!("LEXER: verify: Line does not have a line number");
    } else if tokens.len() > 1 && !is_res(tokens[1].clone()) {
	panic!("LEXER: verify: Line has malformed command");
    } else {
	return tokens;
    }
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

// Find the beginnings and ends of all matching reserved tokens
fn find_res_tokens(file_string:String) -> Vec<usize> {
    let mut locations:Vec<usize> = Vec::new();
    let mut i_in_string_or_res = Vec::new();
    let mut in_string = false;
    let mut in_brack = false;
    let mut iter = 0;
    let char_vec:Vec<char> = file_string.chars().collect();
    
    for c in char_vec.clone() {
	if c == '"' {
	    in_string = !in_string;
	}

	if in_string {
	    i_in_string_or_res.push(iter);
        }

	iter = iter + 1;
    }

    iter = 0;

    for c in char_vec.clone() {
	if c == '(' && !i_in_string_or_res.contains(&iter) {
	    in_brack = !in_brack;
	} else if c == ')' && !i_in_string_or_res.contains(&iter) {
	    in_brack = !in_brack;
	}

	if in_brack {
	    i_in_string_or_res.push(iter);
        }

	iter = iter + 1;
    }
    
    for i in &RESERVED {
	let mut value:Vec<usize> = file_string.match_indices(i).map(|(j, _)|j).collect();
	let mut lower_value:Vec<usize> = file_string.match_indices(&i.to_lowercase()).map(|(j, _)|j).collect();

	value.append(&mut lower_value);
	
	for loc in value {
	    if !i_in_string_or_res.contains(&loc) {
		locations.push(loc);
		locations.push(loc + i.len());

		for k in loc..(loc + i.len()) {
		    i_in_string_or_res.push(k);
		}
	    }
	}
    }

    locations.sort();
    
    return locations;
}

// Check if float
pub fn _is_float(token:String) -> bool {
    let char_vec:Vec<char> = token.chars().collect();
    let mut output = true;
    let mut seen_point = false;

    // Check if string is empty
    if token.len() == 0 {
	output = false;
    }
    
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
pub fn is_int(token:String) -> bool {
    let char_vec:Vec<char> = token.chars().collect();
    let mut output = true;

    // Check if string is empty
    if token.len() == 0 {
	output = false;
    }

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

    // Check if string is empty
    if token.len() == 0 {
	output = false;
    }

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
pub fn is_res(token:String) -> bool {
    let output;

    // Check if token is one of the reserved_tokens
    if RESERVED.contains(&token.as_str()) || RESERVED.contains(&token.to_uppercase().as_str()) {
	output = true;
    } else {
	output = false;
    }

    return output;
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

    // Testing tokenize()
    #[test]
    fn tokenize_6() {
	let given:String = remove_spaces("100".to_string());
	let answer:Vec<String> = vec!["100".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_7() {
	let given:String = remove_spaces("2334 DEF FNF(X) = X^4 - 1".to_string());
	let answer:Vec<String> = vec!["2334".to_string(), "DEF".to_string(), "FNF(X)=X^4-1".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_8() {
	let given:String = remove_spaces("1232 ON L+1 GO TO 300,400,500".to_string());
	let answer:Vec<String> = vec!["1232".to_string(), "ON".to_string(), "L+1".to_string(), "GOTO".to_string(), "300".to_string(), ",".to_string(), "400".to_string(), ",".to_string(), "500".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_9() {
	let given:String = remove_spaces("1 FOR I = A TO B STEP -1".to_string());
	let answer:Vec<String> = vec!["1".to_string(), "FOR".to_string(), "I=A".to_string(), "TO".to_string(), "B".to_string(), "STEP".to_string(), "-1".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_10() {
	let given:String = remove_spaces("1232343 NEXT I".to_string());
	let answer:Vec<String> = vec!["1232343".to_string(), "NEXT".to_string(), "I".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_11() {
	let given:String = remove_spaces("0 INPUT X, A$, Y(2)".to_string());
	let answer:Vec<String> = vec!["0".to_string(), "INPUT".to_string(), "X".to_string(), ",".to_string(), "A$".to_string(), ",".to_string(), "Y(2)".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_12() {
	let given:String = remove_spaces("0 READ X(1), A$, C".to_string());
	let answer:Vec<String> = vec!["0".to_string(), "READ".to_string(), "X(1)".to_string(), ",".to_string(), "A$".to_string(), ",".to_string(), "C".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_13() {
	let given:String = remove_spaces("0101 DATA 3.14159, PI, 5E-10, \",\"".to_string());
	let answer:Vec<String> = vec!["0101".to_string(), "DATA".to_string(), "3.14159".to_string(), ",".to_string(), "PI".to_string(), ",".to_string(), "5E-10".to_string(), ",".to_string(), "\",\"".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_14() {
	let given:String = remove_spaces("9 DIM A (6), B(10,10)".to_string());
	let answer:Vec<String> = vec!["9".to_string(), "DIM".to_string(), "A(6)".to_string(), ",".to_string(), "B(10,10)".to_string()];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn tokenize_15() {
	let given:String = remove_spaces("9 RESTORE".to_string());
	let answer:Vec<String> = vec!["9".to_string(), "RESTORE".to_string()];

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
