// Expression module
#![forbid(unsafe_code)]

// Expression struct
pub struct Expression {
    plain_text:String,
    declare_or_query:bool, // true for declare, false for query
    relation_or_exp:bool,  // true for relation, false for exp
    output_type:String,    // Null for 'declare', boolean for 'query', needs resolution for 'exp'
    relational:String,     // Type of relational used in expression
}

// Notes:
// Need something to compute output type, value
// Going to be used as main means of data storage
// Etc.

// Expression implementation
impl Expression {
    // Constructor
    pub fn new(expression_text:String, d_or_q:bool) -> Expression {
	let parse = parse_expression(expression_text.clone());
	
	Expression {
	    plain_text:expression_text,
	    declare_or_query:d_or_q,
	    relation_or_exp:parse.3,
	    output_type:String::new(),
	    relational:parse.1,
	}
    }
}

// Static methods
fn parse_expression(plain_text:String) -> (String, String, String, bool) {
    let char_vec:Vec<char> = plain_text.chars().collect();
    let mut first_part:Vec<char> = Vec::new();
    let mut relational:Vec<char> = Vec::new();
    let mut second_part:Vec<char> = Vec::new();
    let mut in_rel:bool = false;
        
    // Split expression
    for c in char_vec {
	if c == '=' || c == '<' || c == '>' {
	    relational.push(c);
	    in_rel = true;
	} else if !in_rel {
	    first_part.push(c);
	} else {
	    second_part.push(c);
        }
    }

    return (first_part.into_iter().collect(), relational.into_iter().collect(), second_part.into_iter().collect(), in_rel);
}
