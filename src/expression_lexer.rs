// Expression_lexer module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

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
