// Lexer module
#![forbid(unsafe_code)]


// Testing methods
#[cfg(test)]
mod tests;


// General Imports
use regex::Regex;
use types::enums::Type;
use errors::stateless_error;
use lazy_static::lazy_static;


// File Imports
use types::find_type;


// Constants
const RESERVED:[&str; 27] = ["FUNCTION", "RESTORE", "RETURN", "OPTION", "GOSUB", "PRINT",
				     "INPUT", "READ", "BASE", "DATA", "STOP", "GOTO", "THEN", "NEXT",
				     "STEP", "FOR", "REM", "LET", "DIM", "END", "DEF",
				     "IF", "TO", "ON", ";", ":", ","];
lazy_static! {
    static ref SHEBANG:Regex = Regex::new(r"^(#!.*)$").unwrap();
    static ref LINENUM:Regex = Regex::new(r"[0-9]*").unwrap();
}


// Perform all lexer command for multiple commands per line
pub fn perform_multi_lexing(line_string:String) -> Vec<Vec<String>> {
    let tokens:Vec<String> = tokenize(line_string);
    let mut command:Vec<String> = Vec::new();
    let mut output:Vec<Vec<String>>= Vec::new();
    let line_number:String = tokens[0].clone();
    let mut saw_rem = false;
    let mut first = true;

    command.push(line_number.clone());

    for token in tokens {
	if first {first = false; continue;}
	if token.to_uppercase() == *"REM" {saw_rem = true;}

	if token == *":" && !saw_rem {
	    output.push(verify(command));
	    command = vec![line_number.clone()];
	} else {
	    command.push(token);
	}
    }

    output.push(verify(command));

    output
}


// Create an error if the command is not formed properly, add implied let statements
fn verify(mut tokens:Vec<String>) -> Vec<String> {
    if (find_type(tokens[0].clone()) != Type::Int) && !is_shebang(tokens[0].clone()) {
	stateless_error([].to_vec(),
			[].to_vec(),
			"verify".to_string(),
			"Line has no line number.".to_string());

	Vec::new()
    } else if tokens.len() > 1 && !RESERVED.contains(&tokens[1].clone().to_uppercase().as_str()) {
	tokens.insert(1, "LET".to_string());

	tokens
    } else {
	tokens
    }
}


// Function to remove spaces
pub fn remove_spaces(file_string:String) -> String {
    let char_vec:Vec<char> = file_string.chars().collect();
    let mut output_string:String = "".to_string();
    let mut in_string = false;
    
    for c in char_vec {
	if c != ' ' || in_string {
	    output_string.push(c);
        }

	if c == '"' {
	    in_string = !in_string;
	}
    }

    output_string
}


// Create a vector of tokens
fn tokenize(line_string:String) -> Vec<String> {
    let (line_number, rest) = split_line_number(line_string);
    let mut output = rest_tokenize(rest);

    output.insert(0, line_number);

    output
}


// Get only line number
pub fn split_line_number(unclean_line_string:String) -> (String, String) {
    let clean_line_string:String = remove_spaces(unclean_line_string);
    let matches:Vec<String> = LINENUM
        .find_iter(&clean_line_string)
        .map(|m| m.as_str().to_string())
        .collect();

    (matches[0].clone(), clean_line_string[matches[0].clone().len()..clean_line_string.len()].to_string())
}


// Create a vector of tokens
fn rest_tokenize(file_string:String) -> Vec<String> {
    let mut output:Vec<String> = Vec::new();
    let mut cur:String = file_string.trim().to_string();
    let mut offset = 0;
    let locations = find_res_tokens(file_string);

    for i in locations {
	let (chunk, rest) = cur.split_at(i - offset);
	offset = i;
	output.push(chunk.to_string());
	cur = rest.to_string();
    }

    if cur != *"" {output.push(cur);}

    output.retain(|x| x != &"".to_string());

    output
}


// Find the beginnings and ends of all matching reserved tokens
fn find_res_tokens(file_string:String) -> Vec<usize> {
    let mut locations:Vec<usize> = Vec::new();
    let mut i_in_string_or_res = Vec::new();
    let mut in_string = false;
    let mut in_brack = false;

    for (iter, c) in file_string.chars().enumerate() {
	if c == '"' {in_string = !in_string;}
	if (c == '(' || c == ')') && !in_string {in_brack = !in_brack;}

	if in_string || in_brack {i_in_string_or_res.push(iter);}
    }
    
    for i in &RESERVED {
	let mut value:Vec<usize> = file_string.match_indices(i).map(|(j, _)|j).collect();
	let mut lower_value:Vec<usize> = file_string.match_indices(&i.to_lowercase()).map(|(j, _)|j).collect();

	value.append(&mut lower_value);
	
	value.iter().map(|loc| {
	    if !i_in_string_or_res.contains(loc) {
		locations.extend([*loc, loc + i.len()]);
		i_in_string_or_res.extend(*loc..(*loc + i.len()));
	    }
	}).for_each(drop);
    }

    locations.sort_unstable();
    
    locations
}


// Check if a shebang token
pub fn is_shebang(token:String) -> bool {SHEBANG.is_match(&token)}
