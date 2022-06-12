// Types module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;
#[cfg(test)]
mod find_type_tests;

// General Imports
use regex::Regex;
use lazy_static::lazy_static;

// File Imports
use errors::{stateless_error, unhandled_error};
use expression_lexer::{split, split_function};

// Constants
lazy_static! {
    static ref FLOAT:Regex = Regex::new(r"^(|\+|-)([0-9]*)(\.[0-9]+)$").unwrap();
    static ref SCI_FLOAT:Regex = Regex::new(r"^(|\+|-)([0-9]*)(?:\.[0-9]*)?(([0-9]|[0-9]\.)(e|E))((|\+|-)[0-9]+)$").unwrap();
    static ref STRING:Regex = Regex::new(r#"^("[^"]*")$"#).unwrap();
    static ref INTEGER:Regex = Regex::new(r"^(|\+|-)([0-9]+)$").unwrap();
    static ref FUNCTION:Regex = Regex::new(r"^([a-z|A-Z]+)(\(.*\))$").unwrap();
    static ref EXPRESSION:Regex = Regex::new(r"^.+(=|<|>|!|\+|/|\*|-|\^).+$").unwrap();
    static ref SYMBOL:Regex = Regex::new(r"^([a-z]|[A-Z])+(\$|[0-9])*$").unwrap();
}

// Determine output type
pub fn find_type(token:String) -> i64 {
    let string_test = is_string(token.clone());
    let float_test = is_float(token.clone());
    let int_test = is_int(token.clone());
    let sci_float_test = is_sci_float(token.clone());
    let function_test = is_function(token.clone());
    let expression_test = is_expression(token.clone());
    let symbol_test = is_symbol(token.clone());

    let all = [string_test, float_test, sci_float_test, int_test, function_test, expression_test, symbol_test].to_vec();

    let mut num_true = 0;

    for test in all {
	if test {num_true = num_true + 1;}
    }

    if num_true > 1 || num_true == 0 {
	let artifacts = [string_test.to_string(), float_test.to_string(),
			 sci_float_test.to_string(), int_test.to_string(),
			 function_test.to_string(), expression_test.to_string(),
			 symbol_test.to_string(), token.clone()].to_vec();
	let artifact_names = ["string".to_string(), "float".to_string(),
			      "sci_float".to_string(), "int".to_string(),
			      "function".to_string(), "expression".to_string(),
			      "symbol".to_string(), "token".to_string()].to_vec();
	let function_name = "find_type".to_string();
	let message = "Object does not match any of the types.".to_string();
        stateless_error(artifacts, artifact_names, function_name, message);
    }

    if string_test {return 3000;}             // string
    else if float_test {return 4002;}         // float
    else if int_test {return 4001;}           // int
    else if sci_float_test {return 4003;}     // sci_float
    else if function_test {return 2000;}      // function
    else if expression_test {return 0;}       // expresssion
    else if symbol_test {return 1000;}        // symbol
    else {unhandled_error(); return -1;}
}

// Check if float
fn is_float(token:String) -> bool {
    let mut output = FLOAT.is_match(&token) || SCI_FLOAT.is_match(&token);

    match token.clone().parse::<f64>() {
	Ok(i) => {
	    let signif;
	    
	    if i.abs() < 1.0 {
		signif = i.abs().to_string().replace("0.", "").len();
	    } else {
		signif = i.abs().to_string().replace(".", "").len();
	    }
	    
	    if signif <= 6 {
		if is_int(token.clone()) {
		    output = false && output; // int
		} else {
		    output = true && output; // float
		}
	    } else {
		output = false && output; // sci_float
	    }
	},
	Err(_e) => output = false && output,
    }

    return output;
}

// Check if sci_float
fn is_sci_float(token:String) -> bool {
    let mut output = FLOAT.is_match(&token) || SCI_FLOAT.is_match(&token);

    match token.clone().parse::<f64>() {
	Ok(i) => {
	    let signif;
	    
	    if i.abs() < 1.0 {
		signif = i.abs().to_string().replace("0.", "").len();
	    } else {
		signif = i.abs().to_string().replace(".", "").len();
	    }
	    
	    if signif <= 6 {
		if is_int(token.clone()) {
		    output = false && output; // int
		} else {
		    output = false && output; // float
		}
	    } else {
		output = true && output; // sci_float
	    }
	},
	Err(_e) => output = false && output,
    }

    return output;
}

// Check if integer
fn is_int(token:String) -> bool {return INTEGER.is_match(&token);}

// Check if string
fn is_string(token:String) -> bool {return STRING.is_match(&token);}

// Check if symbol
fn is_symbol(token:String) -> bool {return SYMBOL.is_match(&token);}

// Check if expression
fn is_expression(token:String) -> bool {
    return EXPRESSION.is_match(&token) &&
	!STRING.is_match(&token) &&
	!SCI_FLOAT.is_match(&token) &&
	split(token, false, false).1 != "".to_string();
}

// Check if function call
fn is_function(token:String) -> bool {
    let mut output = FUNCTION.is_match(&token);

    if output {
	let args = split_function(token.clone()).1;

	for c in args.chars() {
	    if c == ')' {
		output = false;
		break;
	    }
	    if c == '(' {
		output = true;
		break;
	    }
	}
    }

    return output;
}
