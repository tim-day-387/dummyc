// Expression_lexer module
#![forbid(unsafe_code)]


// Testing methods
#[cfg(test)]
mod tests;


// File Imports
use errors::stateless_error;
use lexer::remove_spaces;


// Constants
const RELS:[char; 4] = ['=', '<', '>', '!'];
const OPS1:[char; 2] = ['+', '-'];
const OPS2:[char; 2] = ['/', '*'];
const OPS3:[char; 2] = ['^', ' '];


// Split an expression across the relational
pub fn split(token:String, rels_or_ops:bool, strict:bool) -> (String, String, String) {
    let output;

    if rels_or_ops {
	output = split_priority(token.clone(), rels_or_ops, -1);
    } else {
	let first = split_priority(token.clone(), rels_or_ops, 1);
	let second = split_priority(token.clone(), rels_or_ops, 2);
	let third = split_priority(token.clone(), rels_or_ops, 3);

	if first.1 != *"" {output = first;}
	else if second.1 != *"" {output = second;}
	else {output = third;}
    }

    if strict && (output.0 == *"" || output.1 == *"" || output.2 == *"") {
	stateless_error([token].to_vec(),
			["token".to_string()].to_vec(),
			"split".to_string(),
			"Tried to create empty split.".to_string());
    }

    output
}


// Split an expression across the relational
pub fn split_priority(mut token:String, rels_or_ops:bool, priority:i64) -> (String, String, String) {
    let mut first_part_string:String = "".to_string();
    let mut operation_string:String = "".to_string();
    let mut second_part_string:String = "".to_string();
    let mut in_exp:bool = false;
    let mut in_string:bool = false;
    let mut seen_op:bool = false;
    let mut last_char:char = ' ';
    let mut paran_diff = 0;
    let mut ops = OPS1;
    let rels = RELS;

    if priority == 1 {ops = OPS1;}
    else if priority == 2 {ops = OPS2;}
    else if priority == 3 {ops = OPS3;}

    if has_outer_parans(token.clone()) {
	token = remove_outer_parans(token.clone());
    }
    
    // Splits expression based on operation
    for c in token.chars().rev() {
	if c == '"' {in_string = !in_string;}

	if rels_or_ops {
	    if rels.contains(&c) && (paran_diff == 0) && !in_string {
		operation_string.insert(0, c);
		in_exp = true;
		continue;
	    }
	} else {
	    if seen_op && !in_exp {
		if OPS1.contains(&c) || OPS2.contains(&c) || OPS3.contains(&c) {
		    second_part_string.insert(0, last_char);

		    if ops.contains(&c) {operation_string.insert(0, c);}
		    else {second_part_string.insert(0, c);}
		} else if c == 'E' {
		    second_part_string.insert(0, last_char);
		    second_part_string.insert(0, c);
		    seen_op = false;
		    continue;
		} else {
		    operation_string.insert(0, last_char);
		    first_part_string.insert(0, c);
	        }
		in_exp = true;
		continue;
	    }
	    if ops.contains(&c) && (paran_diff == 0) && !in_exp && !in_string {
                last_char = c;
		seen_op = true;
		continue;
	    }
	}

	if c == '(' {paran_diff += 1;}
        if c == ')' {paran_diff += -1;}

	if !in_exp {second_part_string.insert(0, c);}
	if in_exp {first_part_string.insert(0, c);}
    }

    (first_part_string, operation_string, second_part_string)
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

    (name, arguments)
}


// Split items over commas
pub fn split_arguments(unclean_token:String) -> Vec<String> {
    let token = remove_spaces(unclean_token);
    let mut current:String = "".to_string();
    let mut in_string:bool = false;
    let mut paran_diff = 0;
    let mut output:Vec<String> = Vec::new();

    // Check if empty string
    if token == *"" {
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

    output
}


// Remove outer parans
fn remove_outer_parans(token:String) -> String {    
    let mut copy_token = token;
    copy_token.pop();
    copy_token.remove(0);

    copy_token
}


// Check if has outer parans
fn has_outer_parans(mut token:String) -> bool {
    let mut in_string:bool = false;

    if !token.starts_with('(') || token.chars().nth(token.len() - 1) != Some(')') {
	false
    } else {
	token.pop();
	token.remove(0);

	for c in token.chars() {
	    if c == '"' {in_string = !in_string;}
	    if c == ')' && !in_string {return false;}
	    if c == '(' && !in_string {return true;}
	}

	true
    }
}
