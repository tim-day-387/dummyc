// Expression_lexer module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

// General Imports
use lazy_static::lazy_static;
use regex::Regex;

// File Imports
use lexer::*;

// Constants
const RELS:[char; 4] = ['=', '<', '>', '!'];
const OPS:[char; 5] = ['+', '/', '*', '-', '^'];

// Split an expression across the relational
pub fn split(mut token:String, rels_or_ops:bool) -> (String, String, String) {
    let mut first_part_string:String = "".to_string();
    let mut operation_string:String = "".to_string();
    let mut second_part_string:String = "".to_string();
    let mut in_exp:bool = false;
    let mut in_string:bool = false;
    let mut seen_data:bool = false;
    let mut paran_diff = 0;

    if has_outer_parans(token.clone()) {
	token = remove_outer_parans(token.clone());
    }
    
    // Splits expression based on operation
    for c in token.chars() {
	if c == '"' {in_string = !in_string;}
	if c.is_alphanumeric() {seen_data = true;}

	if rels_or_ops {
	    if RELS.contains(&c) && (paran_diff == 0) && !in_string {
		operation_string.push(c);
		in_exp = true;
		continue;
	    }
	} else {
	    if OPS.contains(&c) && (paran_diff == 0) && !in_exp && !in_string && seen_data {
		operation_string.push(c);
		in_exp = true;
		continue;
	    }
	}

	if rels_or_ops {
	    if c == '(' {paran_diff += 1;}
	    if c == ')' {paran_diff += -1;}
	} else {
	    if c == '(' {paran_diff += 1; continue;}
	    if c == ')' {paran_diff += -1; continue;}
	}

	if !in_exp {first_part_string.push(c); continue;}
	if in_exp {second_part_string.push(c); continue;}
    }

    if first_part_string == "".to_string() || second_part_string == "".to_string() {
	panic!("EXPRESSION_LEXER: split: Tried to create empty split from {}", token.clone());
    }

    return (first_part_string, operation_string, second_part_string);
}

// Get function name
pub fn split_function(token:String) -> (String, String) {
    let mut name = "".to_string();
    let mut arguments = "".to_string();
    let mut in_args = false;
    
    for c in token.chars() {
	if c == '(' || in_args {
	    in_args = true;
	    arguments.push(c);
	} else {
	    name.push(c);
	}    
    }

    arguments.pop();
    arguments.remove(0);

    return (name, arguments);
}

// Split items over commas
pub fn split_arguments(unclean_token:String) -> Vec<String> {
    let token = remove_spaces(unclean_token);
    let mut current:String = "".to_string();
    let mut in_string:bool = false;
    let mut paran_diff = 0;
    let mut output:Vec<String> = Vec::new();

    // Check if empty string
    if token == "".to_string() {
	return output;
    }
    
    // Splits expression based on operation
    for c in token.chars() {
	if c == '"' {in_string = !in_string;}

	if c == '(' {paran_diff += 1;}
	if c == ')' {paran_diff += -1;}

	if c == ',' && !in_string && paran_diff == 0{
	    output.push(current);
	    current = "".to_string();
	} else {
	    current.push(c);
	}
    }

    output.push(current);

    return output;
}

// Remove outer parans
fn remove_outer_parans(token:String) -> String {    
    let mut copy_token = token.clone();
    copy_token.pop();
    copy_token.remove(0);

    return copy_token;
}

// Check if has outer parans
fn has_outer_parans(mut token:String) -> bool {
    let mut in_string:bool = false;

    if token.chars().nth(0) != Some('(') || token.chars().nth(token.len() - 1) != Some(')') {
	return false;
    } else {
	token.pop();
	token.remove(0);

	for c in token.chars() {
	    if c == '"' {in_string = !in_string;}
	    if c == ')' && !in_string {return false;}
	    if c == '(' && !in_string {return true;}
	}

	return true;
    }
}

// Check if float
pub fn is_float(token:String) -> bool {
    lazy_static! {static ref REA:Regex = Regex::new(r"^(|\+|-)([0-9]*)(\.[0-9]+)$").unwrap();}
    lazy_static! {static ref REB:Regex = Regex::new(r"^(|\+|-)([0-9]*)(?:\.[0-9]*)?(([0-9]|[0-9]\.)(e|E))((|\+|-)[0-9]+)$").unwrap();}
    return REA.is_match(&token) || REB.is_match(&token);
}

// Check if integer
pub fn is_int(token:String) -> bool {
    lazy_static! {static ref RE:Regex = Regex::new(r"^(|\+|-)([0-9]+)$").unwrap();}
    return RE.is_match(&token);
}

// Check if string
pub fn is_string(token:String) -> bool {
    lazy_static! {static ref RE:Regex = Regex::new(r#"^(".*")$"#).unwrap();}
    return RE.is_match(&token);
}

// Check if expression
pub fn is_expression(token:String) -> bool {
    let mut output = false;
    
    for c in token.chars() {
	if RELS.contains(&c) || OPS.contains(&c) {
	    output = true;
	}
    }

    return !is_string(token.clone()) && output;
}

// Check if function call
pub fn is_function(token:String) -> bool {
    let mut output = false;
    let mut func_name_empty = true;
    let char_vec:Vec<char> = token.chars().collect();
    
    for c in char_vec.clone() {
	if RELS.contains(&c) || OPS.contains(&c) {
	    output = false;
	    break;
	}

	if c == '(' && char_vec[token.len() - 1] == ')' && !func_name_empty {
	    output = true;
	    break;
	}

	func_name_empty = false;
    }

    return output;
}

