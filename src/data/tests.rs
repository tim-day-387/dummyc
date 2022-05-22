// File Imports
use data::*;

// Testing output_type()
#[test]
fn type_1() {
    let mut given:Data = Data::new("\"This is a test\"".to_string());
    given.find_output_type();

    assert_eq!(3000, given.output_type);
}

// Testing output_type()
#[test]
fn type_2() {
    let mut given:Data = Data::new("This is another, different test.".to_string());
    given.find_output_type();

    assert_eq!(1000, given.output_type);
}
