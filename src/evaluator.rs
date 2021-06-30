// Evaluator module
#![forbid(unsafe_code)]
pub mod evaluator {
}

// Testing public methods
#[cfg(test)]
mod test {
    // File Imports
    use super::evaluator::*;

    // Evaluate the given token and return a tuple
    fn evaluate(token:String) -> (String, String, String, String) {
	let char_vec:Vec<char> = token.chars().collect();
	let mut output:(String, String, String, String) =
	    ("".to_string(), "".to_string(), "".to_string(), "".to_string());
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

	let output0:String = variable.into_iter().collect();
	output.0 = output0;
	let output1:String = relational.into_iter().collect();
	output.1 = output1;
	let output2:String = expression.into_iter().collect();
	output.2 = [output2,
		    ".to_string()".to_string()].concat();
	output.3 = "\"string\".to_string()".to_string();
	
	return output;
    }

    // Testing evaluate()
    #[test]
    fn eval_1() {
	let given:String = "T=\"Test\"".to_string();
	let answer = ("T".to_string(), "=".to_string(), "\"Test\".to_string()".to_string(),
		      "\"string\".to_string()".to_string());

	assert_eq!(answer, evaluate(given));
    }
}
