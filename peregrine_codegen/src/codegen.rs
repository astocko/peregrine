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

fn operand_to_struct(id: &OperandId) -> &str {
    match *id { 
        OperandId::rel8 => "RIPRelativeOffset8",
        OperandId::rel32 => "RIPRelativeOffset32",
        OperandId::imm4 => "Imm4",
        OperandId::imm8 => "Imm8",
        OperandId::imm16 => "Imm16",
        OperandId::imm32 => "Imm32",
        OperandId::imm64 => "Imm64",
        OperandId::r8 => "GPRegister8",
        OperandId::r16 => "GPRegister16",
        OperandId::r32 => "GPRegister32",
        OperandId::r64 => "GPRegister64",
        OperandId::mm => "MMXRegister",
        OperandId::xmm => "XMMRegister",
        OperandId::xmm_k_ => "XMMRegisterK",
        OperandId::xmm_k_z_ => "XMMRegisterKZ",
        OperandId::ymm => "YMMRegister",
        OperandId::ymm_k_ => "YMMRegisterK",
        OperandId::ymm_k_z_ => "YMMRegisterKZ",
        OperandId::zmm => "ZMMRegister",
        OperandId::zmm_k_ => "ZMMRegisterK",
        OperandId::zmm_k_z_ => "ZMMRegisterKZ",
        OperandId::k => "KRegister",
        OperandId::k_k_ => "KRegisterK",
        OperandId::m => "MemoryAny",
        OperandId::m8 => "Memory8",
        OperandId::m16 => "Memory16",
        OperandId::m16_k_z_ => "Memory16KZ",
        OperandId::m32 => "Memory32",
        OperandId::m32_k_ => "Memory32K",
        OperandId::m32_k_z_ => "Memory32KZ",
        OperandId::m64 => "Memory64",
        OperandId::m64_k_ => "Memory64K",
        OperandId::m64_k_z_ => "Memory64KZ",
        OperandId::m80 => "Memory80",
        OperandId::m128 => "Memory128",
        OperandId::m128_k_z_ => "Memory128KZ",
        OperandId::m256 => "Memory256",
        OperandId::m256_k_z_ => "Memory256KZ",
        OperandId::m512 => "Memory512",
        OperandId::m512_k_z_ => "Memory512KZ",
        OperandId::m64__m32bcst => "BroadcastM64M32",
        OperandId::m128__m32bcst => "BroadcastM128M32",
        OperandId::m256__m32bcst => "BroadcastM256M32",
        OperandId::m512__m32bcst => "BroadcastM512M32",
        OperandId::m128__m64bcst => "BroadcastM128M64",
        OperandId::m256__m64bcst => "BroadcastM256M64",
        OperandId::m512__m64bcst => "BroadcastM512M64",
        OperandId::moffs32 => panic!("moffs32 should never be encoded!"),
        OperandId::moffs64 => panic!("moffs64 should never be encoded!"),
        OperandId::vm32x => "VMemory32XMM",
        OperandId::vm32x_k_ => "VMemory32XMMK",
        OperandId::vm32y => "VMemory32YMM",
        OperandId::vm32y_k_ => "VMemory32YMMK",
        OperandId::vm32z => "VMemory32ZMM",
        OperandId::vm32z_k_ => "VMemory32ZMMK",
        OperandId::vm64x => "VMemory64XMM",
        OperandId::vm64x_k_ => "VMemory64XMMK",
        OperandId::vm64y => "VMemory64YMM",
        OperandId::vm64y_k_ => "VMemory64YMMK",
        OperandId::vm64z => "VMemory64ZMM",
        OperandId::vm64z_k_ => "VMemory64ZMMK",
        OperandId::_sae_ => "SuppressAllExceptions",
        OperandId::_er_ => "RoundingControl",
        OperandId::NONE => "",
        _ => "",
    }
}

fn is_hard_coded_op(id: &OperandId) -> bool {
    match *id {
        OperandId::_1_ => true,
        OperandId::_3_ => true,
        OperandId::al => true,
        OperandId::ax => true,
        OperandId::eax => true,
        OperandId::rax => true,
        OperandId::cl => true,
        OperandId::xmm0 => true,
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
