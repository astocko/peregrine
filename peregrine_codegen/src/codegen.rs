use std::collections::BTreeMap;

use loader::{load_instruction_set, filter_instruction_forms, is_avx512};
use group_parser::load_instruction_groups;
use types::*;

pub fn generate() {
    let mut instructions = load_instruction_set();
    let (mut groups, ins_group_map) = load_instruction_groups();

    for ins in instructions {
        let forms = filter_instruction_forms(&ins.forms);

        if !(forms.len() > 0) {
            continue;
        }


    }

}
