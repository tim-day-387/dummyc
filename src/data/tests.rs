// File Imports
use types::enums::Type;
use data::*;


// Testing output_type()
#[test]
fn type_1() {
    let mut given:Data = Data::new("\"This is a test\"".to_string());
    given.find_output_type();

    assert_eq!(Type::String, given.output_type);
}


// Testing output_type()
#[test]
fn type_2() {
    let mut given:Data = Data::new("Thisisanotherdifferenttest".to_string());
    given.find_output_type();

    assert_eq!(Type::Symbol, given.output_type);
}


// Testing simplify()
#[test]
fn simplify_1() {
    let state:State = State::new();
    let given:Data = Data::new_simplified("1+1".to_string(), state.clone());
    let answer:Data = Data::new_simplified("2".to_string(), state.clone());

    if answer.eq(&given) == false {
	println!("Given:");
	println!("{}", given.plain_text);
	println!("{}", given.output_type);
	println!("{}", given.print_out_text);
	println!("Answer:");
	println!("{}", answer.plain_text);
	println!("{}", answer.output_type);
	println!("{}", answer.print_out_text);	
    }
    
    assert_eq!(answer.eq(&given), true);
}


// Testing simplify()
#[test]
fn simplify_2() {
    let state:State = State::new();
    let given:Data = Data::new_simplified("0+(5+5)".to_string(), state.clone());
    let answer:Data = Data::new_simplified("10".to_string(), state.clone());

    if answer.eq(&given) == false {
	println!("Given:");
	println!("{}", given.plain_text);
	println!("{}", given.output_type);
	println!("{}", given.print_out_text);
	println!("Answer:");
	println!("{}", answer.plain_text);
	println!("{}", answer.output_type);
	println!("{}", answer.print_out_text);	
    }
    
    assert_eq!(answer.eq(&given), true);
}


// Testing simplify()
#[test]
fn simplify_3() {
    let state:State = State::new();
    let given:Data = Data::new_simplified("(1+1)".to_string(), state.clone());
    let answer:Data = Data::new_simplified("2".to_string(), state.clone());

    if answer.eq(&given) == false {
	println!("Given:");
	println!("{}", given.plain_text);
	println!("{}", given.output_type);
	println!("{}", given.print_out_text);
	println!("Answer:");
	println!("{}", answer.plain_text);
	println!("{}", answer.output_type);
	println!("{}", answer.print_out_text);	
    }
    
    assert_eq!(answer.eq(&given), true);
}


// Testing simplify()
#[test]
fn simplify_4() {
    let state:State = State::new();
    let given:Data = Data::new_simplified("(1+(2+3)+4)".to_string(), state.clone());
    let answer:Data = Data::new_simplified("10".to_string(), state.clone());

    if answer.eq(&given) == false {
	println!("Given:");
	println!("{}", given.plain_text);
	println!("{}", given.output_type);
	println!("{}", given.print_out_text);
	println!("Answer:");
	println!("{}", answer.plain_text);
	println!("{}", answer.output_type);
	println!("{}", answer.print_out_text);	
    }
    
    assert_eq!(answer.eq(&given), true);
}
