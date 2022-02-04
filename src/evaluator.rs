// Evaluator module
#![forbid(unsafe_code)]

// Evaluate the given token and return a tuple
pub fn evaluate(token:String) -> (String, String, String, String) {
    let char_vec:Vec<char> = token.chars().collect();
    let mut output:(String, String, String, String) =
	("".to_string(), "".to_string(), "".to_string(), "".to_string());
    let output0:String;
    let output1:String;
    let output2:String;
    let output3:String;
    let mut variable:Vec<char> = Vec::new();
    let mut relational:Vec<char> = Vec::new();
    let mut expression:Vec<char> = Vec::new();
    let mut in_exp:bool = false;
        
    // If every char is a digit, we have a number
    for c in char_vec {
	if c == '=' {
	    relational.push(c);
	    in_exp = true;
	} else if !in_exp {
	    variable.push(c);
	} else {
	    expression.push(c);
        }
    }

    // If relational.len() == 0, we're not in a relation 
    if relational.len() == 0 {
	output0 = "".to_string();
	output1 = "".to_string();
	output2 = variable.into_iter().collect();
	output3 = return_type(output2.clone());
    } else {
	let calculation = calculate(expression.into_iter().collect());
	output0 = variable.into_iter().collect();
	output1 = relational.into_iter().collect();
	output2 = calculation.0;
	output3 = calculation.1;
    }

    // Set output
    output.0 = output0;
    output.1 = output1;
    output.2 = output2;
    output.3 = output3;
    
    return output;
}

// Give a Rust statement to calc the expression, give type it'll return
fn calculate(expression:String) -> (String, String) {
    let output:(String, String);

    // If the expression is a string, return thus
    output = (expression.clone(), "string".to_string());

    return output
}

// Determine the return type of an expression
fn return_type(expression:String) -> String {
    let expression_bytes = expression.as_bytes();
    let mut output:String = "variable".to_string();

    // Step through each char
    for i in 0..expression.len() {
	// Check if expression has strings 
	if expression_bytes[i] == b'"' {
	    output = "string".to_string();
	} 
    }

    return output;
}

// Testing methods
#[cfg(test)]
mod test {
    // File Imports
    use evaluator::*;
    
    // Testing evaluate()
    #[test]
    fn eval_1() {
	let given:String = "T=\"Test\"".to_string();
	let answer = ("T".to_string(), "=".to_string(), "\"Test\"".to_string(),
		      "string".to_string());

	assert_eq!(answer, evaluate(given));
    }

    // Testing evaluate()
    #[test]
    fn eval_2() {
	let given:String = "\"Test\"".to_string();
	let answer = ("".to_string(), "".to_string(), "\"Test\"".to_string(),
		      "string".to_string());

	assert_eq!(answer, evaluate(given));
    }
}
