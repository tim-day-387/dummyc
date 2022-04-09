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

// Testing is_float()
#[test]
fn is_float_1() {
    let given:String = "3.14159".to_string();
    let answer = true;

    assert_eq!(answer, _is_float(given));
}

// Testing is_float()
#[test]
fn is_float_2() {
    let given:String = "3.1sdfsdf4159".to_string();
    let answer = false;

    assert_eq!(answer, _is_float(given));
}

// Testing is_int()
#[test]
fn is_int_1() {
    let given:String = "387".to_string();
    let answer = true;

    assert_eq!(answer, is_int(given));
}

// Testing is_int()
#[test]
fn is_int_2() {
    let given:String = "stringlmao".to_string();
    let answer = false;

    assert_eq!(answer, is_int(given));
}

// Testing is_string()
#[test]
fn is_string_1() {
    let given:String = "\"This is a test\"".to_string();
    let answer = true;

    assert_eq!(answer, is_string(given));
}

// Testing is_string()
#[test]
fn is_string_2() {
    let given:String = "\"This is \"\" a test\"".to_string();
    let answer = false;

    assert_eq!(answer, is_string(given));
}
