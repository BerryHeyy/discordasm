use std::collections::HashMap;
use std::vec::Vec;

use crate::program_environment;
use crate::program_environment::ProgramEnvironment;
use crate::utility;

pub fn comp_inst_add(line: &str, 
        compilePointer: &mut u16, 
        tokens: Vec<&str>, 
        labelReferences: HashMap<&str, u16>, 
        environment: &ProgramEnvironment) -> bool {

    if utility::map_count(program_environment::REGISTER_ENCODING, tokens[1]) > 0 { // Destination exists
        if utility::valid_hex_str(tokens[2]) {
            let regBits = utility::get_register_size(program_environment::REGISTER_ENCODING[tokens[1]]);
        }
    };
}