// General Imports
extern crate trees;
use self::trees::{tr,Tree,Forest,Node};

use std::io::{self, Write};
use std::env;
use std::fs;

// File Imports
mod lexer;
use lexer::lexer::*;
mod parser;
use parser::parser::*;
mod generator;
use generator::generator::*;

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
    println!("Compiling {}", args[1]);

    // Perform lexing
    let tokens = perform_lexing(contents.clone());

    // Perform parsing
    let ast = construct_tree(tokens);

    // Perform generation
    let given:Tree<String> = (tr("MAIN".to_string())
		      /(tr("001".to_string()) /tr("GOTO".to_string()) /tr("002".to_string()))
	              /(tr("002".to_string()) /tr("GOTO".to_string()) /tr("001".to_string())));
	
    create_main(given);
    
}
