// Types module
#![forbid(unsafe_code)]


// General Imports
use std::fmt;


// Type enum
#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    String,
    Float,
    Int,
    SciFloat,
    Function,
    Expression,
    Symbol,
    Undefined
}


// Type implementation
impl Type {
    // Is this enum a number?
    pub fn check_if_number(self) -> bool {
	(self == Type::Int) ||
	    (self == Type::Float) ||
	    (self == Type::SciFloat)
    }


    // Is this enum a float?
    pub fn check_if_float(self) -> bool {
	(self == Type::Float) ||
	    (self == Type::SciFloat)
    }


    // Are these types compatible?
    pub fn check_if_compatible(self, other:Type) -> bool {
	(self == other) ||
	    (self.check_if_number() && other.check_if_number())
    }


    // Which type takes precendence?
    pub fn precedence(self, other:Type) -> Type {
	if self == other {
	    return self;
	} else if self.clone().check_if_number() && other.clone().check_if_number() {
	    if (self == Type::SciFloat) || (other == Type::SciFloat) {
		return Type::SciFloat;
	    } else if (self == Type::Float) || (other == Type::Float) {
		return Type::Float;
	    } else if (self == Type::Int) || (other == Type::Int) {
		return Type::Int;
	    }
	}

	Type::Undefined
    }
}


// Display implementation for Type
impl fmt::Display for Type {
    // Format
    fn fmt(&self, formatter:&mut fmt::Formatter) -> fmt::Result {
	write!(formatter, "{:?}", self)
    }
}
