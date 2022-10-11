// File Imports
use types::*;


// Testing find_type()
#[test]
fn find_type_1() {
    let given:String = "3.14159".to_string();
    let answer = Type::Float;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_2() {
    let given:String = "sdf$$$4159".to_string();
    let answer = Type::Symbol;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_3() {
    let given:String = "387".to_string();
    let answer = Type::Int;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_4() {
    let given:String = "stringlmao".to_string();
    let answer = Type::Symbol;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_5() {
    let given:String = "1+1".to_string();
    let answer = Type::Expression;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_6() {
    let given:String = "\"This is a test\"".to_string();
    let answer = Type::String;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_7() {
    let given:String = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
    let answer = Type::Symbol;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_8() {
    let given:String = "f999999999999999999999999999999999999$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$".to_string();
    let answer = Type::Symbol;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_9() {
    let given:String = "\"This is \"+\" a test\"".to_string();
    let answer = Type::Expression;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_10() {
    let given:String = "-\"This is \"+\" a test\"".to_string();
    let answer = Type::Expression;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_11() {
    let given:String = "\"This is a test\"".to_string();
    let answer = Type::String;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_12() {
    let given:String = "tab(32)".to_string();
    let answer = Type::Function;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_13() {
    let given:String = "ta+b(32)".to_string();
    let answer = Type::Expression;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_14() {
    let given:String = "tab(adf;laj4;fjjef;f;f;f;f;()()()()00008877yhh)".to_string();
    let answer = Type::Function;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_15() {
    let given:String = "(1+1)".to_string();
    let answer = Type::Expression;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_16() {
    let given:String = "tab(1)+tab(2)".to_string();
    let answer = Type::Expression;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_17() {
    let given:String = "-12".to_string();
    let answer = Type::Int;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_18() {
    let given:String = "+3.14".to_string();
    let answer = Type::Float;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_19() {
    let given:String = "-123E-22".to_string();
    let answer = Type::SciFloat;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_20() {
    let given:String = "F(N-1)".to_string();
    let answer = Type::Function;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_21() {
    let given:String = "SDF$".to_string();
    let answer = Type::Symbol;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_22() {
    let given:String = "F8".to_string();
    let answer = Type::Symbol;

    assert_eq!(answer, find_type(given));
}


// Testing find_type()
#[test]
fn find_type_23() {
    let given:String = "3.".to_string();
    let answer = Type::Float;

    assert_eq!(answer, find_type(given));
}
