// Lexer module
#![forbid(unsafe_code)]
pub mod lexer {
    // Perform all lexer commands
    pub fn perform_lexing(file_string:String) -> Vec<(u32, String)> {
	return tokenize(remove_comments(file_string));
    }

    // Remove all comments (enclosed within ##)
    fn remove_comments(file_string:String) -> String {
	let mut in_comment = false;
	let file_bytes = file_string.as_bytes();
	let mut to_del: Vec<usize> = Vec::new();

	// Find chars to del
	for i in 0..file_string.len() {
	    // Check if in comment, then mark for delete
	    if file_bytes[i] == b'#' {
		in_comment = !in_comment;
		to_del.push(i);
	    } else if in_comment == true {
		to_del.push(i);
	    }
	}

	// Del chars
	let mut output: Vec<u8> = Vec::new();
	for i in 0..file_string.len() {
	    // If slated for del, delete
	    if !to_del.contains(&i) {
		output.push(file_bytes[i]);
	    }
	}

	// Return cleaned string
	return String::from_utf8_lossy(&output).to_string();
    }

    // Create a vector of tokens
    fn tokenize(file_string:String) -> Vec<(u32, String)> {
	let file_bytes = file_string.as_bytes();
	let mut token: Vec<u8> = Vec::new();
	let mut output: Vec<(u32, String)> = Vec::new();
	let mut in_string = false;
	let mut line_num = 1;

	// Step through each char
	for i in 0..file_string.len() {
	    // Check if in string 
	    if file_bytes[i] == b'"' {
		in_string = !in_string;
	    }

	    // Add to token or finish token
	    if (file_bytes[i] != b' ' && file_bytes[i] != b'\n') | in_string {
		token.push(file_bytes[i])
	    } else if (token.len() > 0) && !in_string {
		output.push((line_num, String::from_utf8_lossy(&token).to_string()));
		token = Vec::new();
	    }

	    // Update line number
	    if file_bytes[i] == b'\n' {
		line_num = line_num + 1;
	    }
	}

	// If we have a stray token, push it
	if token.len() > 0 {
	    output.push((line_num, String::from_utf8_lossy(&token).to_string()));
	}

	return output;
    }

    // Testing remove_comments()
    #[test]
    fn rm_cmts_1() {
	let given:String = "001 GOTO 001 #This is an example comment#".to_string();
	let answer:String = "001 GOTO 001 ".to_string();

	assert_eq!(answer, remove_comments(given));
    }

    // Testing remove_comments()
    #[test]
    fn rm_cmts_2() {
	let given:String = "00#yuh#0 PRINT \"This is a Dummy program\"
                            001 LET hat = \"the\"
                            002
                            003 #This is a test comment#
                            004
                            005 IF hat = \"the\" THEN GOTO 0#yuh#08 ELSE GOTO 010 #This is an#
                            0#yuh#06
                            007
                            008 PRINT#yuh# \"Hat is 7\"
                            009 END
                            010 EN#yuh#D".to_string();
	let answer:String = "000 PRINT \"This is a Dummy program\"
                            001 LET hat = \"the\"
                            002
                            003 
                            004
                            005 IF hat = \"the\" THEN GOTO 008 ELSE GOTO 010 
                            006
                            007
                            008 PRINT \"Hat is 7\"
                            009 END
                            010 END".to_string();

	assert_eq!(answer, remove_comments(given));
    }

    // Testing remove_comments()
    #[test]
    fn rm_cmts_3() {
	let given:String = "00##0 PR##INT \"This is# a Dum#my program\"
                            001 L##ET hat = \"the\"
                            002 LET## BaBa########## = ##\"booey\"
                            003 ##GOTO 0##00".to_string();
	let answer:String = "000 PRINT \"This ismy program\"
                            001 LET hat = \"the\"
                            002 LET BaBa = \"booey\"
                            003 GOTO 000".to_string();

	assert_eq!(answer, remove_comments(given));
    }

    // Testing tokenize()
    #[test]
    fn token_1() {
	let given:String = "001 GOTO 001".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "GOTO".to_string()),
				      (1, "001".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_2() {
	let given:String = "000 PRINT \"This ismy program\"
                            001 LET hat = \"the\"
                            002 LET BaBa = \"booey\"
                            003 GOTO 000".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "000".to_string()),(1, "PRINT".to_string()),
				      (1, "\"This ismy program\"".to_string()),
				      (2, "001".to_string()),
	                              (2, "LET".to_string()), (2, "hat".to_string()),
				      (2, "=".to_string()), (2, "\"the\"".to_string()),
				      (3, "002".to_string()), (3, "LET".to_string()),
	                              (3, "BaBa".to_string()), (3, "=".to_string()),
				      (3, "\"booey\"".to_string()),
	                              (4, "003".to_string()), (4, "GOTO".to_string()),
				      (4, "000".to_string())];

	assert_eq!(answer, tokenize(given));
    }

    // Testing tokenize()
    #[test]
    fn token_3() {
	let given:String = "001 002 
                           345 #yuh#
                           CAR     HAT".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "002".to_string()),
				      (2, "345".to_string()),(2, "#yuh#".to_string()),
				      (3, "CAR".to_string()),(3, "HAT".to_string())];

	assert_eq!(answer, tokenize(given));
    }
}

// Testing public methods
#[cfg(test)]
mod test {
    // File Imports
    use super::lexer::*;

    // Testing perform_lexing()
    #[test]
    fn lex_1() {
	let given:String = "001 GOTO 001 #This is an example comment#".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "GOTO".to_string()),
					     (1, "001".to_string())];
	
	assert_eq!(answer, perform_lexing(given));
    }

    // Testing perform_lexing()
    #[test]
    fn lex_2() {
	let given:String = "00##0 PR##INT \"This is# a Dum#my program\"
                            001 L##ET hat = \"the\"
                            002 LET## BaBa########## = ##\"booey\"
                            003 ##GOTO 0##00".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "000".to_string()),(1, "PRINT".to_string()),
				      (1, "\"This ismy program\"".to_string()),
				      (2, "001".to_string()),
	                              (2, "LET".to_string()), (2, "hat".to_string()),
				      (2, "=".to_string()), (2, "\"the\"".to_string()),
				      (3, "002".to_string()), (3, "LET".to_string()),
	                              (3, "BaBa".to_string()), (3, "=".to_string()),
				      (3, "\"booey\"".to_string()),
	                              (4, "003".to_string()), (4, "GOTO".to_string()),
					     (4, "000".to_string())];
	
	assert_eq!(answer, perform_lexing(given));
    }

    // Testing perform_lexing()
    #[test]
    fn lex_3() {
	let given:String = "001 002 
                           345 #yuh#
                           CAR     HAT".to_string();
	let answer:Vec<(u32, String)> = vec![(1, "001".to_string()),(1, "002".to_string()),
					     (2, "345".to_string()),(3, "CAR".to_string()),
					     (3, "HAT".to_string())];

	assert_eq!(answer, perform_lexing(given));
    }
}
