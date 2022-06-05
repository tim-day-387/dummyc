// Errors module
#![forbid(unsafe_code)]

// Produce an error without outputting state
pub fn stateless_error(artifacts:Vec<String>, artifact_names:Vec<String>, function_name:String, message:String) {
    error_header(function_name, message);

    for i in 0..artifacts.len() {
	println!("=> {}: {}", artifact_names[i], artifacts[i]); 
    }
    
    error_footer();
    
    quit::with_code(1);
}

// Generic unhandled error function
pub fn unhandled_error() {
    let artifacts = [].to_vec();
    let artifact_names = [].to_vec();
    let function_name = "unknown".to_string();
    let message = "There is an unhandled error. Sorry.".to_string();
    stateless_error(artifacts, artifact_names, function_name, message);    
}

// Error header
fn error_header(function_name:String, message:String) {
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error_found ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Function Name: {}", function_name);
    println!("Message: {}", message);
}

// Error footer
fn error_footer() {
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}
