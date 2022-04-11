// Expression_lexer module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

// Constants
const RELS:[char; 4] = ['=', '<', '>', '!'];
const OPS:[char; 4] = ['+', '/', '*', '-'];

// Split an expression across the relational
pub fn split(token:String, rels_or_ops:bool) -> (String, String, String) {
    let mut first_part_string:String = "".to_string();
    let mut operation_string:String = "".to_string();
    let mut second_part_string:String = "".to_string();
    let mut in_exp:bool = false;
    let mut in_string:bool = false;
    let mut paran_diff = 0;
    
    // Splits expression based on operation
    for c in token.chars() {
	if c == '"' {in_string = !in_string;}

	if rels_or_ops {
	    if RELS.contains(&c) && (paran_diff == 0) && !in_string {
		operation_string.push(c);
		in_exp = true;
		continue;
	    }
	} else {
	    if OPS.contains(&c) && (paran_diff == 0) && !in_exp && !in_string {
		operation_string.push(c);
		in_exp = true;
		continue;
	    }
	}
	    
	if c == '(' {paran_diff += 1; continue;}
	if c == ')' {paran_diff += -1; continue;}

	if !in_exp {first_part_string.push(c); continue;}
	if in_exp {second_part_string.push(c); continue;}
    }

    return (first_part_string, operation_string, second_part_string);
}

// Check if float
pub fn _is_float(token:String) -> bool {
    match token.parse::<f32>() {
	Ok(_i) => return true,
	Err(_e) => return false,
    };
}

// Check if integer
pub fn is_int(token:String) -> bool {
    match token.parse::<i32>() {
	Ok(_i) => return true,
	Err(_e) => return false,
    };
}

// Check if string
pub fn is_string(token:String) -> bool {
    let char_vec:Vec<char> = token.chars().collect();
    let num_parans = token.matches("\"").count();

    // Check if string
    if num_parans == 2 && char_vec[0] == '"' && char_vec[token.len() -1] == '"' {
	return true;
    } else {
	return false;
    }
}

// Check if function call
pub fn is_function(token:String) -> bool {
    let mut output = false;
    let char_vec:Vec<char> = token.chars().collect();
    
    for c in char_vec.clone() {
	if RELS.contains(&c) || OPS.contains(&c) {
	    output = false;
	    break;
	}

	if c == '(' && char_vec[token.len() - 1] == ')' {
	    output = true;
	    break;
	}
    }

    return output;
}
