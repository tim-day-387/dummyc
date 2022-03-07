// State module
#![forbid(unsafe_code)]

// General Imports
use std::collections::HashMap;

// State struct
pub struct State {
    pub types:HashMap<String, String>,
    pub strings:HashMap<String, String>,
    pub next_line:i64,
    pub prev_line:i64,
}

// State implementation
impl State {
    // Constructor
    pub fn new() -> State {
	State {
	    types:HashMap::new(),
	    strings:HashMap::new(),
	    next_line:-1,
	    prev_line:-1,
	}
    }
}

