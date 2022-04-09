// File Imports
use expression_lexer::*;

// Testing split()
#[test]
fn split_1() {
    let given:String = "A=\"Fuh\"".to_string();
    let answer = ("A".to_string(), "=".to_string(), "\"Fuh\"".to_string());

    assert_eq!(answer, split(given, true));
}

// Testing split()
#[test]
fn split_2() {
    let given:String = "B<=23423984723fffffjjjdjdj{}||[".to_string();
    let answer = ("B".to_string(), "<=".to_string(), "23423984723fffffjjjdjdj{}||[".to_string());

    assert_eq!(answer, split(given, true));
}
