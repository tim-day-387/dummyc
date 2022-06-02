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

// Testing split()
#[test]
fn split_5() {
    let given:String = "(1+1)".to_string();
    let answer = ("1".to_string(), "+".to_string(), "1".to_string());

    assert_eq!(answer, split(given, false));
}

// Testing split_arguments()
#[test]
fn split_arguments_1() {
    let given:String = "a=test(a,b),(1+(2+(3+4))),\"This is, a test\"".to_string();
    let answer:Vec<String> = vec!["a=test(a,b)".to_string(), "(1+(2+(3+4)))".to_string(), "\"This is, a test\"".to_string()];

    assert_eq!(answer, split_arguments(given));
}

// Testing split_arguments()
#[test]
fn split_arguments_2() {
    let given:String = "".to_string();
    let answer:Vec<String> = Vec::new();

    assert_eq!(answer, split_arguments(given));
}

// Testing has_outer_parans()
#[test]
fn has_outer_parans_1() {
    let given:String = "(1+1)+(2+2)".to_string();
    let answer = false;

    assert_eq!(answer, has_outer_parans(given));
}

// Testing has_outer_parans()
#[test]
fn has_outer_parans_2() {
    let given:String = "(1\"+1)+(2\"+2)".to_string();
    let answer = true;

    assert_eq!(answer, has_outer_parans(given));
}

// Testing has_outer_parans()
#[test]
fn has_outer_parans_3() {
    let given:String = "(1+(2+3)+4)".to_string();
    let answer = true;

    assert_eq!(answer, has_outer_parans(given));
}

// Testing has_outer_parans()
#[test]
fn has_outer_parans_4() {
    let given:String = "(1+1)".to_string();
    let answer = true;

    assert_eq!(answer, has_outer_parans(given));
}

// Testing has_outer_parans()
#[test]
fn has_outer_parans_5() {
    let given:String = "1".to_string();
    let answer = false;

    assert_eq!(answer, has_outer_parans(given));
}
