// Lexer module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

// General Imports
use lazy_static::lazy_static;
use regex::Regex;

// File Imports
use expression_lexer::*;

// Constants
const RESERVED:[&'static str; 24] = ["FUNCTION", "RESTORE", "RETURN", "GOSUB", "PRINT", "INPUT", "READ", "DATA", "STOP",
				     "GOTO", "THEN", "NEXT", "STEP", "FOR", "REM", "LET", "DIM", "END", "DEF",
				     "IF", "TO", "ON", ";", ","];
lazy_static! {
    static ref SHEBANG:Regex = Regex::new(r"^(#!.*)$").unwrap();
}

// Perform all lexer commands
pub fn perform_lexing(file_string:String) -> Vec<String> {
    return verify(tokenize(remove_spaces(file_string)));
}

// Create an error if the command is not formed properly
fn verify(tokens:Vec<String>) -> Vec<String> {
    if !is_int(tokens[0].clone()) && !is_shebang(tokens[0].clone()) {
	panic!("LEXER: verify: Line either has no line number or has no reserved token");
    } else if tokens.len() > 1 && !RESERVED.contains(&&tokens[1].clone().to_uppercase().as_str()) {
	panic!("LEXER: verify: Line either has no line number or has no reserved token");
    } else {
	return tokens;
    }
}

// Function to remove spaces
pub fn remove_spaces(file_string:String) -> String {
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
fn tokenize(line_string:String) -> Vec<String> {
    let (line_number, rest) = split_line_number(line_string);
    let mut output = rest_tokenize(rest);

    output.insert(0, line_number);

    return output;
}

// Get only line numer
fn split_line_number(line_string:String) -> (String, String) {
    let mut line_number:String = "".to_string();
    let mut rest:String = "".to_string();
    let mut done = false;

    for c in line_string.chars() {
	if c.is_digit(10) && !done {
	    line_number.push(c);
	} else {
	    rest.push(c);
	    done = true;
	}
    }

    return (line_number, rest);
}

// Create a vector of tokens
fn rest_tokenize(file_string:String) -> Vec<String> {
    let mut output:Vec<String> = Vec::new();
    let mut cur:String = file_string.trim().to_string().clone();
    let mut offset = 0;
    let locations = find_res_tokens(file_string);

    for i in locations {
	let (chunk, rest) = cur.split_at(i - offset);
	offset = i;
	output.push(chunk.to_string());
	cur = rest.to_string();
    }

    if cur != "".to_string() {output.push(cur);}

    output.retain(|x| x != &"".to_string());

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
	if c == '"' {in_string = !in_string;}
	if in_string {i_in_string_or_res.push(iter);}
	
	if (c == '(' || c == ')') && !i_in_string_or_res.contains(&iter) {in_brack = !in_brack;}
	if in_brack {i_in_string_or_res.push(iter);}

	iter +=1;
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

// Check if a shebang token
pub fn is_shebang(token:String) -> bool {return SHEBANG.is_match(&token);}
