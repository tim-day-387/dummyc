// File Imports
use types::*;

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

// Testing is_int()
#[test]
fn is_int_3() {
    let given:String = "1+1".to_string();
    let answer = false;

    assert_eq!(answer, is_int(given));
}

// Testing is_int()
#[test]
fn is_int_4() {
    let given:String = "-98765500000".to_string();
    let answer = true;

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

// Testing is_string()
#[test]
fn is_string_3() {
    let given:String = "This is \"\" a test\"".to_string();
    let answer = false;

    assert_eq!(answer, is_string(given));
}

// Testing is_string()
#[test]
fn is_string_4() {
    let given:String = "\"This is \"+\" a test\"".to_string();
    let answer = false;

    assert_eq!(answer, is_string(given));
}

// Testing is_expression()
#[test]
fn is_expression_1() {
    let given:String = "\"This is \"+\" a test\"".to_string();
    let answer = true;

    assert_eq!(answer, is_expression(given));
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

// Testing is_function()
#[test]
fn is_function_5() {
    let given:String = "(1+1)".to_string();
    let answer = false;

    assert_eq!(answer, is_function(given));
}

// Testing is_function()
#[test]
fn is_function_6() {
    let given:String = "tab(1)+tab(2)".to_string();
    let answer = false;

    assert_eq!(answer, is_function(given));
}
