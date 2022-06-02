// Types module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

// General Imports
use lazy_static::lazy_static;
use regex::Regex;

// Constants
lazy_static! {
    static ref FLOAT:Regex = Regex::new(r"^(|\+|-)([0-9]*)(\.[0-9]+)$").unwrap();
    static ref SCI_FLOAT:Regex = Regex::new(r"^(|\+|-)([0-9]*)(?:\.[0-9]*)?(([0-9]|[0-9]\.)(e|E))((|\+|-)[0-9]+)$").unwrap();
    static ref STRING:Regex = Regex::new(r#"^(".*")$"#).unwrap();
    static ref INTEGER:Regex = Regex::new(r"^(|\+|-)([0-9]+)$").unwrap();
    static ref FUNCTION:Regex = Regex::new(r"^([a-z|A-Z]+)(\(.*\))$").unwrap();
    static ref EXPRESSION:Regex = Regex::new(r"^.*(=|<|>|!|\+|/|\*|-|\^).*$").unwrap();
}

// Determine output type
pub fn find_output_type(token:String) -> i64 {
    // Series of cases to find type
    if is_string(token.clone()) {
	return 3000; // string
    } else if is_float(token.clone()) || is_int(token.clone()) {
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
			return 4001; // int
		    } else {
			return 4002; // float
		    }
		} else {
		    return 4003; // sci_float
		}
	    },
	    Err(_e) => panic!("DATA: find_output_type: Invalid float"),
	};
    } else if is_function(token.clone()) {
	return 2000; // symbol_callable
    } else {
	if !is_expression(token.clone()) {
	    return 1000; // symbol
	} else {
	    return 0; // expression
        }
    }
}

// Check if float or sci_float
pub fn is_float(token:String) -> bool {return FLOAT.is_match(&token) || SCI_FLOAT.is_match(&token);}

// Check if integer
pub fn is_int(token:String) -> bool {return INTEGER.is_match(&token);}

// Check if string
pub fn is_string(token:String) -> bool {return STRING.is_match(&token);}

// Check if expression
pub fn is_expression(token:String) -> bool {return EXPRESSION.is_match(&token) && !STRING.is_match(&token);}

// Check if function call
pub fn is_function(token:String) -> bool {return FUNCTION.is_match(&token);}
