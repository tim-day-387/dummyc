// Types module
#![forbid(unsafe_code)]

// Testing methods
#[cfg(test)]
mod tests;

// General Imports
use regex::Regex;
use lazy_static::lazy_static;

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
pub fn find_type(token:String) -> i64 {
    if is_string(token.clone()) {return 3000;}             // string
    else if is_float(token.clone()) {return 4002;}         // float
    else if is_int(token.clone()) {return 4001;}           // int
    else if is_sci_float(token.clone()) {return 4003;}     // sci_float
    else if is_function(token.clone()) {return 2000;}      // function
    else if is_expression(token.clone()) {return 0;}       // expresssion
    else {return 1000;}                                    // symbol
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
fn is_int(token:String) -> bool {
    let mut output = INTEGER.is_match(&token);

    match token.clone().parse::<f64>() {
	Ok(i) => {
	    let signif;
	    
	    if i.abs() < 1.0 {
		signif = i.abs().to_string().replace("0.", "").len();
	    } else {
		signif = i.abs().to_string().replace(".", "").len();
	    }
	    
	    if signif > 6 {
		output = false && output; // sci_float
	    }
	},
	Err(_e) => output = false && output,
    }

    return output;
}

// Check if string
fn is_string(token:String) -> bool {return STRING.is_match(&token);}

// Check if expression
fn is_expression(token:String) -> bool {return EXPRESSION.is_match(&token) && !STRING.is_match(&token);}

// Check if function call
fn is_function(token:String) -> bool {return FUNCTION.is_match(&token);}
