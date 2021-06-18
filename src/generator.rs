// Generator module
#![forbid(unsafe_code)]
pub mod generator {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree,Forest,Node};
    use std::io::Write;

    pub fn generate(input:Tree<String>) {
	
    }
}

// Testing public methods
#[cfg(test)]
mod test {
    // General Imports
    extern crate trees;
    use self::trees::{tr,Tree,Forest,Node};

    // File Imports
    use super::generator::*;

}
