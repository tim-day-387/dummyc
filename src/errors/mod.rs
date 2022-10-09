// Errors module
#![forbid(unsafe_code)]


// Parse float or error
pub fn parse_float(string_to_parse:String, function_name:String) -> f64 {
    match string_to_parse.parse::<f64>() {
	Ok(i) => {
	    return i;
	},
	Err(_e) => {
	    stateless_error([].to_vec(),
			    [].to_vec(),
			    function_name,
			    "Invalid float.".to_string());
	    return -1.0;
	}
    }
}


// Parse int or error
pub fn parse_int(string_to_parse:String, function_name:String) -> i64 {
    match string_to_parse.parse::<i64>() {
	Ok(i) => {
	    return i
	},
	Err(_e) => {
	    stateless_error([].to_vec(),
			    [].to_vec(),
			    function_name,
			    "Invalid integer.".to_string());
	    return -1;
	}
    }
}


// Divide by zero error
pub fn error_divide_zero(function_name:String) {
    stateless_error([].to_vec(),
		    [].to_vec(),
		    function_name,
		    "Attempted to divide by zero.".to_string());
}


// Produce an error without outputting state
pub fn stateless_error(artifacts:Vec<String>, artifact_names:Vec<String>, function_name:String, message:String) {
    error_header(function_name, message);

    for i in 0..artifacts.len() {
	eprintln!("=> {}: {}", artifact_names[i], artifacts[i]); 
    }
    
    error_footer();
    
    quit::with_code(1);
}


// Generic unhandled error function
pub fn unhandled_error() {
    stateless_error([].to_vec(),
		    [].to_vec(),
		    "unknown".to_string(),
		    "There is an unhandled error. Sorry.".to_string());
}


// Error header
fn error_header(function_name:String, message:String) {
    eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error_found ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    eprintln!("Function Name: {}", function_name);
    eprintln!("Message: {}", message);
}


// Error footer
fn error_footer() {
    eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}
