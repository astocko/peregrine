use instruction_parser::load_instructions;
use group_parser::load_instruction_groups;
use types::*;

pub fn load_instruction_set() -> Vec<Instruction> {
    let mut instruction_set = load_instructions();

    for ins in instruction_set.iter_mut() {

        let mut extra_forms = Vec::new();

        for form in ins.forms.iter_mut() {
            let match_count = form.operands
                .iter()
                .filter(|&x| x.id == OperandId::_sae_ || x.id == OperandId::_er_)
                .count();

            if match_count > 0 {
                let mut new_form = form.clone();
                new_form.operands = new_form.operands
                    .into_iter()
                    .filter(|x| x.id != OperandId::_sae_ && x.id != OperandId::_er_)
                    .collect::<Vec<Operand>>();

                match new_form.encodings[0].evex {
                    Some(ref mut e) => {
                        e.b = ZeroRef::Zero;
                        e.LL = LLBitRef::Two
                    }
                    None => (),
                }

                extra_forms.push(new_form);

                match form.encodings[0].evex {
                    Some(ref mut e) => {
                        match e.LL {
                            LLBitRef::LastRef(_) => (),
                            LLBitRef::NONE => (),
                            _ => e.LL = LLBitRef::Zero,
                        }
                    }
                    None => (),
                }

                match form.encodings[0].evex {
                    Some(ref mut e) => e.b = ZeroRef::EVEX_b_ONE,
                    None => (),
                }

            }
        }
        ins.forms.append(&mut extra_forms);
    }

    instruction_set
}




fn filter_instruction_forms(forms: &mut Vec<InstructionForm>) -> Vec<&InstructionForm> {
    let mut new_forms = Vec::new();

    for form in forms.iter() {
        if !form.operands
            .iter()
            .any(|x| x.id == OperandId::moffs32 || x.id == OperandId::moffs64) {
            new_forms.push(form);
        }
    }

    new_forms
}
