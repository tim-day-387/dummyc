// Imports
use std::io::{self, Write};
use std::env;
use std::fs;

// Main function code
fn main() {
    // Collect args of program and check if right number
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
	panic!("Wrong number of arguements!");
    }

    // Create handle for stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Read file
    let contents = fs::read_to_string(&args[1])
	.expect("Something went wrong reading the file!");

    println!("Running {}", args[1]);

    let clean_contents = remove_comments(contents.clone());
    let tokens = tokenize(clean_contents.clone());
    
    // Test
    handle.write_all(contents.as_bytes());
    println!("\n");
    handle.write_all(clean_contents.as_bytes());
    println!("\n");
    println!("{:?}", tokens)
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
fn tokenize(file_string:String) -> Vec<String> {
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
