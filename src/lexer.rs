// Lexer module
pub mod lexer {
    // Remove all comments (enclosed within ##)
    pub fn remove_comments(file_string:String) -> String {
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
	let mut counter = 0;
	for i in 0..file_string.len() {
	    // If slated for del, delete
	    if !to_del.contains(&i) {
		output.push(file_bytes[i]);
		counter = counter + 1;
	    }
	}

	// Return cleaned string
	return String::from_utf8_lossy(&output).to_string();
    }

    // Create a vector of tokens
    pub fn tokenize(file_string:String) -> Vec<String> {
	let file_bytes = file_string.as_bytes();
	let mut token: Vec<u8> = Vec::new();
	let mut output: Vec<String> = Vec::new();
	let mut in_string = false;

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
		output.push(String::from_utf8_lossy(&token).to_string());
		token = Vec::new();
	    }
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
                            005 IF hat = \"the\" THEN GOTO 0#yuh#08 ELSE GOTO 010 #This is another comment#
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
}
