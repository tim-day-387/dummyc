// File Imports
use lexer::*;

// Testing tokenize()
#[test]
fn tokenize_1() {
    let given:String = remove_spaces("30000 REM This is just some random test lol".to_string());
    let answer:Vec<String> = vec!["30000".to_string(), "REM".to_string(), "Thisisjustsomerandomtestlol".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_2() {
    let given:String = remove_spaces("10 STOP".to_string());
    let answer:Vec<String> = vec!["10".to_string(), "STOP".to_string()];

    assert_eq!(answer, tokenize(given));
}    

// Testing tokenize()
#[test]
fn tokenize_3() {
    let given:String = remove_spaces("100 LET A=\"Fuh\"".to_string());
    let answer:Vec<String> = vec!["100".to_string(), "LET".to_string(), "A=\"Fuh\"".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_4() {
    let given:String = remove_spaces("2000 PRINT A;B;C;".to_string());
    let answer:Vec<String> = vec!["2000".to_string(), "PRINT".to_string(), "A".to_string(), ";".to_string(), "B".to_string(), ";".to_string(), "C".to_string(), ";".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_5() {
    let given:String = remove_spaces("100 LET A=\"Fuh RETURN TO LET STOP GOTO GOSUB\"".to_string());
    let answer:Vec<String> = vec!["100".to_string(), "LET".to_string(), "A=\"Fuh RETURN TO LET STOP GOTO GOSUB\"".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_6() {
    let given:String = remove_spaces("100".to_string());
    let answer:Vec<String> = vec!["100".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_7() {
    let given:String = remove_spaces("2334 DEF FNF(X) = X^4 - 1".to_string());
    let answer:Vec<String> = vec!["2334".to_string(), "DEF".to_string(), "FNF(X)=X^4-1".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_8() {
    let given:String = remove_spaces("1232 ON L+1 GO TO 300,400,500".to_string());
    let answer:Vec<String> = vec!["1232".to_string(), "ON".to_string(), "L+1".to_string(), "GOTO".to_string(), "300".to_string(), ",".to_string(), "400".to_string(), ",".to_string(), "500".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_9() {
    let given:String = remove_spaces("1 FOR I = A TO B STEP -1".to_string());
    let answer:Vec<String> = vec!["1".to_string(), "FOR".to_string(), "I=A".to_string(), "TO".to_string(), "B".to_string(), "STEP".to_string(), "-1".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_10() {
    let given:String = remove_spaces("1232343 NEXT I".to_string());
    let answer:Vec<String> = vec!["1232343".to_string(), "NEXT".to_string(), "I".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_11() {
    let given:String = remove_spaces("0 INPUT X, A$, Y(2)".to_string());
    let answer:Vec<String> = vec!["0".to_string(), "INPUT".to_string(), "X".to_string(), ",".to_string(), "A$".to_string(), ",".to_string(), "Y(2)".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_12() {
    let given:String = remove_spaces("0 READ X(1), A$, C".to_string());
    let answer:Vec<String> = vec!["0".to_string(), "READ".to_string(), "X(1)".to_string(), ",".to_string(), "A$".to_string(), ",".to_string(), "C".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_13() {
    let given:String = remove_spaces("0101 DATA 3.14159, PI, 5E-10, \",\"".to_string());
    let answer:Vec<String> = vec!["0101".to_string(), "DATA".to_string(), "3.14159".to_string(), ",".to_string(), "PI".to_string(), ",".to_string(), "5E-10".to_string(), ",".to_string(), "\",\"".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_14() {
    let given:String = remove_spaces("9 DIM A (6), B(10,10)".to_string());
    let answer:Vec<String> = vec!["9".to_string(), "DIM".to_string(), "A(6)".to_string(), ",".to_string(), "B(10,10)".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing tokenize()
#[test]
fn tokenize_15() {
    let given:String = remove_spaces("9 RESTORE".to_string());
    let answer:Vec<String> = vec!["9".to_string(), "RESTORE".to_string()];

    assert_eq!(answer, tokenize(given));
}

// Testing remove_spaces()
#[test]
fn remove_spaces_1() {
    let given:String = "A =\"Fuh\"".to_string();
    let answer:String = "A=\"Fuh\"".to_string();

    assert_eq!(answer, remove_spaces(given));
}

// Testing remove_spaces()
#[test]
fn remove_spaces_2() {
    let given:String = "                  ".to_string();
    let answer:String = "".to_string();

    assert_eq!(answer, remove_spaces(given));
}

// Testing remove_spaces()
#[test]
fn remove_spaces_3() {
    let given:String = "     1 0 0 P R  I NT\" Fuh Foo  Fin \"".to_string();
    let answer:String = "100PRINT\" Fuh Foo  Fin \"".to_string();

    assert_eq!(answer, remove_spaces(given));
}
