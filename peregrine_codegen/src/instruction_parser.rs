extern crate serde;
extern crate serde_json;

use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;
use std::u8;

use self::serde_json::Value;

use types::*;


const X86_ISET: &'static str = "x86-64";
const X86_INS_FILE: &'static str = "x86_64.json";

fn parse_isa(value: &Value, form: &mut InstructionForm) {
    match *value {
        Value::Array(ref a) => {
            for v in a {
                let data = v.as_object().unwrap();

                for (key, val) in data.iter() {
                    let enum_val: ISA;
                    match key.as_str() {
                        "id" => parse_str_as_enum!(val, ISA, enum_val),
                        _ => panic!("Unexpected key in ISA"),
                    }
                    form.isas.push(enum_val);
                }

            }
        }
        _ => panic!("Expected JSON Array for parsing the ISA"),
    }
}

fn parse_implicit_operands(value: &Value, form: &mut InstructionForm) {
    match *value {
        Value::Array(ref a) => {
            for v in a {
                let data = v.as_object().unwrap();

                let mut imp_op = ImplicitOperand::new();

                for (key, val) in data.iter() {
                    match key.as_str() {
                        "id" => parse_str_as_enum!(val, ImplicitRegister, imp_op.id),
                        "input" => parse_bool_as_bool!(val, imp_op.input),
                        "output" => parse_bool_as_bool!(val, imp_op.output),
                        _ => panic!("Unexpected field in the implicit operand definition!"),
                    }
                }

                form.implicit_operands.push(imp_op);
            }
        }
        _ => panic!("Expected JSON Array for parsing the implicit operands"),
    }
}

fn parse_operands(value: &Value, form: &mut InstructionForm) {
    match *value {
        Value::Array(ref a) => {
            for v in a {
                let data = v.as_object().unwrap();

                let mut op = Operand::new();

                for (key, val) in data.iter() {
                    match key.as_str() {
                        "type" => parse_str_as_enum!(val, OperandId, op.id),
                        "input" => parse_bool_as_bool!(val, op.input),
                        "output" => parse_bool_as_bool!(val, op.output),
                        "extended_size" => parse_num_as_u64!(val, op.extended_size),
                        _ => panic!("Unexpected field in the implicit operand definition!"),
                    }
                }

                form.operands.push(op);
            }
        }
        _ => panic!("Expected JSON Array for parsing the implicit operands"),
    }
}

fn parse_prefix(value: &Value, prefix: &mut Prefix) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "mandatory" => parse_bool_as_bool!(val, prefix.mandatory),
            "byte" => parse_str_as_hex!(val, prefix.byte),
            _ => panic!("Unexpected field in the prefix definition!"),
        }
    }
}

fn parse_rex(value: &Value, rex: &mut REX) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "mandatory" => parse_bool_as_bool!(val, rex.mandatory),
            "W" => parse_str_as_enum!(val, Bit, rex.W),
            "R" => parse_str_as_enum!(val, BitRef, rex.R),
            "B" => parse_str_as_enum!(val, BitRef, rex.B),
            "X" => parse_str_as_enum!(val, BitRef, rex.X),
            _ => panic!("Unexpected field in the prefix definition!"),
        }
    }
}

fn parse_vex(value: &Value, vex: &mut VEX) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "type" => parse_str_as_enum!(val, VEXType, vex.id),
            "mmmmm" => parse_str_as_bin!(val, vex.mmmmm),
            "pp" => parse_str_as_bin!(val, vex.pp),
            "W" => parse_str_as_enum!(val, Bit, vex.W),
            "L" => parse_str_as_enum!(val, Bit, vex.L), 
            "R" => parse_str_as_enum!(val, BitRef, vex.R),
            "B" => parse_str_as_enum!(val, BitRef, vex.B),
            "X" => parse_str_as_enum!(val, BitRef, vex.X),
            "vvvv" => parse_str_as_enum!(val, ZeroRef, vex.vvvv),

            _ => panic!("Unexpected field in the prefix definition!"),
        }
    }
}

fn parse_evex(value: &Value, evex: &mut EVEX) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "mm" => parse_str_as_bin!(val, evex.mm),
            "pp" => parse_str_as_bin!(val, evex.pp),
            "W" => parse_str_as_enum!(val, Bit, evex.W),
            "LL" => parse_str_as_enum!(val, LLBitRef, evex.LL),
            "RR" => parse_str_as_enum!(val, NoneRef, evex.RR),
            "B" => parse_str_as_enum!(val, NoneRef, evex.B),
            "X" => parse_str_as_enum!(val, NoneRef, evex.X),
            "vvvv" => parse_str_as_enum!(val, ZeroRef, evex.vvvv),
            "V" => parse_str_as_enum!(val, ZeroRef, evex.V),
            "b" => parse_str_as_enum!(val, ZeroRef, evex.b),
            "aaa" => parse_str_as_enum!(val, ZeroRef, evex.aaa),
            "z" => parse_str_as_enum!(val, ZeroRef, evex.z),
            "disp8xN" => parse_str_as_pow2!(val, evex.disp8xN, 1, 64),
            _ => panic!("Unexpected field in the prefix definition!"),
        }
    }
}

fn parse_opcode(value: &Value, opcode: &mut Opcode) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "byte" => parse_str_as_hex!(val, opcode.byte),
            "addend" => parse_str_as_enum!(val, NoneRef, opcode.addend),
            _ => panic!("Unexpected field in the opcode definition!"),
        }
    }
}


fn parse_modrm(value: &Value, modrm: &mut ModRM) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "mode" => parse_str_as_enum!(val, AddressMode, modrm.mode),
            "rm" => parse_str_as_enum!(val, NoneRef, modrm.rm),
            "reg" => parse_str_as_enum!(val, IntOrRef, modrm.reg),
            _ => panic!("Unexpected field in the ModRM definition!"),
        }
    }
}

fn parse_register_byte(value: &Value, regbyte: &mut RegisterByte) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "register" => parse_str_as_enum!(val, NoneRef, regbyte.register),
            "payload" => parse_str_as_enum!(val, NoneRef, regbyte.payload),
            _ => panic!("Unexpected field in the RegisterByte definition!"),
        }
    }
}

fn parse_immediate(value: &Value, imm: &mut Immediate) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "size" => parse_num_with_values!(val, imm.size, 1, 2, 4, 8),
            "value" => parse_str_as_enum!(val, IntOrRef, imm.value),
            _ => panic!("Unexpected field in the Immediate definition!"),
        }
    }
}

fn parse_code_offset(value: &Value, codeoff: &mut CodeOffset) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "size" => parse_num_with_values!(val, codeoff.size, 1, 4),
            "value" => parse_str_as_enum!(val, NoneRef, codeoff.value),
            _ => panic!("Unexpected field in the CodeOffset definition!"),
        }
    }
}

fn parse_data_offset(value: &Value, dataoff: &mut DataOffset) {
    let data = value.as_object().unwrap();

    for (key, val) in data.iter() {
        match key.as_str() {
            "size" => parse_num_with_values!(val, dataoff.size, 4, 8),
            "value" => parse_str_as_enum!(val, NoneRef, dataoff.value),
            _ => panic!("Unexpected field in the DataOffset definition!"),
        }
    }
}



fn parse_encodings(value: &Value, form: &mut InstructionForm) {
    match *value {
        Value::Array(ref a) => {
            for v in a {
                let data = v.as_object().unwrap();
                let mut encoding = Encoding::new();

                for (key, val) in data.iter() {
                    match key.as_str() {
                        "prefix" => {
                            let mut prefix = Prefix::new();
                            parse_prefix(val, &mut prefix);
                            encoding.prefix = Some(prefix);
                        }
                        "REX" => {
                            let mut rex = REX::new();
                            parse_rex(val, &mut rex);
                            encoding.rex = Some(rex);
                        }
                        "VEX" => {
                            let mut vex = VEX::new();
                            parse_vex(val, &mut vex);
                            encoding.vex = Some(vex);
                        }
                        "EVEX" => {
                            let mut evex = EVEX::new();
                            parse_evex(val, &mut evex);
                            encoding.evex = Some(evex);
                        }
                        "opcode" => {
                            let mut opcode = Opcode::new();
                            parse_opcode(val, &mut opcode);
                            encoding.opcodes.push(opcode);
                        }
                        "ModRM" => {
                            let mut modrm = ModRM::new();
                            parse_modrm(val, &mut modrm);
                            encoding.modrm = Some(modrm);
                        }
                        "register_byte" => {
                            let mut register_byte = RegisterByte::new();
                            parse_register_byte(val, &mut register_byte);
                            encoding.register_byte = Some(register_byte);
                        }
                        "immediate" => {
                            let mut immediate = Immediate::new();
                            parse_immediate(val, &mut immediate);
                            encoding.immediate = Some(immediate);
                        }
                        "data_offset" => {
                            let mut dataoff = DataOffset::new();
                            parse_data_offset(val, &mut dataoff);
                            encoding.data_offset = Some(dataoff);
                        }
                        "code_offset" => {
                            let mut codeoff = CodeOffset::new();
                            parse_code_offset(val, &mut codeoff);
                            encoding.code_offset = Some(codeoff);
                        }
                        _ => (),
                    }
                }

                form.encodings.push(encoding);
            }
        }
        _ => panic!("Expected JSON Array for parsing the instruction encodings"),
    }
}

fn parse_form(form: &mut InstructionForm, value: &Value) {
    let data = value.as_object().unwrap();
    for (key, value) in data.iter() {
        match key.as_str() {
            "mmx_mode" => parse_str_as_enum!(value, MMXMode, form.mmx_mode),
            "xmm_mode" => parse_str_as_enum!(value, XMMMode, form.xmm_mode),
            "canceling_inputs" => parse_bool_as_bool!(value, form.canceling_inputs),
            "isa" => parse_isa(value, form),
            "implicit_operands" => parse_implicit_operands(value, form), 
            "operands" => parse_operands(value, form),
            "encodings" => parse_encodings(value, form),
            _ => panic!("Unrecognized field in instruction form definition!"),
        }
    }
}

fn parse_forms(ins: &mut Instruction, value: &Value) {
    match *value {
        Value::Array(ref forms) => {
            for v in forms {
                let mut form = InstructionForm::new();
                parse_form(&mut form, v);
                ins.forms.push(form);
            }
        }
        _ => (),
    }
}

fn parse_string(value: &Value) -> String {
    match *value {
        Value::String(ref v) => v.to_owned(),
        _ => String::from(""),
    }
}

fn parse_instruction(mut ins: &mut Instruction, v: &Value) {
    let data = v.as_object().unwrap();
    for (key, value) in data.iter() {
        match key.as_str() {
            "summary" => ins.summary = parse_string(value),
            "forms" => parse_forms(&mut ins, value),
            _ => (),
        }
    }
}

fn parse_instructions(v: &Value, ins_vec: &mut Vec<Instruction>) {
    let instructions = v.as_object().unwrap();
    for (key, value) in instructions.iter() {
        let mut ins = Instruction::new(key);
        parse_instruction(&mut ins, value);
        ins_vec.push(ins);
    }
}

pub fn load_instructions() -> Vec<Instruction> {
    let mut f = File::open(X86_INS_FILE).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let x86_ins: Value = serde_json::from_str(&s).unwrap();
    let x86_ins = x86_ins.as_object().unwrap();

    match *x86_ins.get("instruction_set").unwrap() {
        Value::String(ref v) => {
            match v.as_str() {
                X86_ISET => (),
                _ => panic!("Unsupported instruction set"),
            }
        }
        _ => panic!("Unsupported iset"),
    }

    let mut instructions = Vec::new();

    for (key, value) in x86_ins.iter() {
        match key.as_str() {
            "instructions" => parse_instructions(value, &mut instructions),
            _ => (),
        }
    }

    instructions
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        super::load_instructions();
    }
}
