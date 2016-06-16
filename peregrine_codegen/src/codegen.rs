use std::collections::BTreeMap;

use code_writer::CodeWriter;
use group_parser::load_instruction_groups;
use loader::{load_instruction_set, filter_instruction_forms, is_avx512};
use types::*;

fn is_branch(ins: &String) -> bool {
    match ins.as_str() {
        "JA" => true,
        "JNA" => true,
        "JAE" => true,
        "JNAE" => true,
        "JB" => true,
        "JNB" => true,
        "JBE" => true,
        "JNBE" => true,
        "JC" => true,
        "JNC" => true,
        "JE" => true,
        "JNE" => true,
        "JG" => true,
        "JNG" => true,
        "JGE" => true,
        "JNGE" => true,
        "JL" => true,
        "JNL" => true,
        "JLE" => true,
        "JNLE" => true,
        "JO" => true,
        "JNO" => true,
        "JP" => true,
        "JNP" => true,
        "JS" => true,
        "JNS" => true,
        "JZ" => true,
        "JNZ" => true,
        "JPE" => true,
        "JPO" => true,
        "JECXZ" => true,
        "JRCXZ" => true,
        "JMP" => true,
        _ => false,
    }
}

fn is_hard_coded_op(ins: &String) -> bool {
    match ins.as_str() {
        "1" => true,
        "3" => true,
        "al" => true,
        "ax" => true,
        "eax" => true,
        "rax" => true,
        "cl" => true,
        "xmm0" => true,
        _ => false,
    }
}

pub fn write_form(writer: &mut CodeWriter, name: &String, form: &InstructionForm) {
    match form.operands.len() {
        0 => writer.code(format!("impl Ins0x for {} {{", name).as_str()),
        1 => writer.code(format!("impl Ins1x<{:?}> for {} {{", form.operands[0].id, name).as_str()),
        2 => {
            writer.code(format!("impl Ins2x<{:?},{:?}> for {} {{",
                                form.operands[0].id,
                                form.operands[1].id,
                                name)
                .as_str())
        }
        _ => panic!("Unrecognized operand count!"), 
    }



    writer.code("}");
}

pub fn generate() {
    let mut instructions = load_instruction_set();
    let (mut groups, ins_group_map) = load_instruction_groups();

    let mut writer = CodeWriter::new("test.rs");


    for ins in instructions {

        if ins.name != "ADDSS" {
            continue;
        }

        let mut forms = filter_instruction_forms(&ins.forms);

        if !(forms.len() > 0) {
            continue;
        }

        let is_branch = is_branch(&ins.name);

        writer.doc("Generated struct and trait implementations for:");
        writer.doc(ins.name.as_str());
        writer.doc(ins.summary.as_str());
        writer.code(format!("pub struct {} {{}}", ins.name).as_str());

        for form in &forms {

            write_form(&mut writer, &ins.name, form);
        }


        break;

    }

}
