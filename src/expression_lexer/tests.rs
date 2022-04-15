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

// Testing split()
#[test]
fn split_3() {
    let given:String = "a=test(Sample,Sample,Sample)".to_string();
    let answer = ("a".to_string(), "=".to_string(), "test(Sample,Sample,Sample)".to_string());

    assert_eq!(answer, split(given, true));
}

// Testing split()
#[test]
fn split_4() {
    let given:String = "a=test()".to_string();
    let answer = ("a".to_string(), "=".to_string(), "test()".to_string());

    assert_eq!(answer, split(given, true));
}

// Testing split_arguments()
#[test]
fn split_arguments_1() {
    let given:String = "a=test(a, b),(1+(2+(3+4))),\"This is, a test\"".to_string();
    let answer:Vec<String> = vec!["a=test(a, b)".to_string(), "(1+(2+(3+4)))".to_string(), "\"This is, a test\"".to_string()];

    assert_eq!(answer, split_arguments(given));
}

// Testing split_arguments()
#[test]
fn split_arguments_2() {
    let given:String = "".to_string();
    let answer:Vec<String> = Vec::new();

    assert_eq!(answer, split_arguments(given));
}


// Testing is_float()
#[test]
fn is_float_1() {
    let given:String = "3.14159".to_string();
    let answer = true;

    assert_eq!(answer, is_float(given));
}

// Testing is_float()
#[test]
fn is_float_2() {
    let given:String = "3.1sdfsdf4159".to_string();
    let answer = false;

    assert_eq!(answer, is_float(given));
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

// Testing is_function()
#[test]
fn is_function_1() {
    let given:String = "\"This is a test\"".to_string();
    let answer = false;

    assert_eq!(answer, is_function(given));
}

// Testing is_function()
#[test]
fn is_function_2() {
    let given:String = "tab(32)".to_string();
    let answer = true;

    assert_eq!(answer, is_function(given));
}

// Testing is_function()
#[test]
fn is_function_3() {
    let given:String = "ta+b(32)".to_string();
    let answer = false;

    assert_eq!(answer, is_function(given));
}

// Testing is_function()
#[test]
fn is_function_4() {
    let given:String = "tab(adf;laj4;fjjef;f;f;f;f;()()()()00008877yhh)".to_string();
    let answer = true;

    assert_eq!(answer, is_function(given));
}
