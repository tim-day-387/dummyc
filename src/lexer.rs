// Lexer module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

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
    let mut in_string:bool = false;
    
    // Splits expression based on relational
    for c in char_vec {
	if c == '"' {
	    in_string = !in_string;
	}
	
	if (c == '=' || c == '<' || c == '>' || c == '!') && !in_string {
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
    let mut in_string:bool = false;
    let mut left_paran = 0;
    let mut right_paran = 0;
    
    // Splits expression based on operation
    for c in char_vec {
	if c == '"' {
	    in_string = !in_string;
	}
	
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
    if !is_int(tokens[0].clone()) && !is_shebang(tokens[0].clone()) {
	panic!("LEXER: verify: Line either has no line number or has no reserved token");
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
pub fn _is_res(token:String) -> bool {
    let output;

    // Check if token is one of the reserved_tokens
    if RESERVED.contains(&token.as_str()) || RESERVED.contains(&token.to_uppercase().as_str()) {
	output = true;
    } else {
	output = false;
    }

    return output;
}

// Check if a shebang token
pub fn is_shebang(token:String) -> bool {
    let mut output = false;

    // Check if token is shebang
    if token.clone() == "#!/usr/bin/dummyc" {
	output = true;
    }

    return output;
}
