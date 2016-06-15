extern crate serde;
extern crate serde_json;

use std::collections::BTreeMap;
use std::vec;
use std::io::prelude::*;
use std::fs::File;

use self::serde_json::Value;

const X86_GROUP_FILE: &'static str = "x86_64_groups.json";

fn parse_group(value: &Value, group: &String, map: &mut BTreeMap<String, String>) {
    match *value {
        Value::Array(ref ins_list) => {
            for v in ins_list {
                match *v {
                    Value::String(ref ins) => {
                        map.insert(ins.to_owned(), group.to_owned());
                        ()
                    }
                    _ => panic!("Expected instruction as string!"),
                }
            }
        }
        _ => panic!("Expected array of instruction names!"),
    }
}

pub fn load_instruction_groups() -> (Vec<String>, BTreeMap<String, String>) {
    let mut f = File::open(X86_GROUP_FILE).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let x86_groups: Value = serde_json::from_str(&s).unwrap();
    let x86_groups = x86_groups.as_object().unwrap();


    let mut ins_group_map: BTreeMap<String, String> = BTreeMap::new();
    let mut groups = Vec::new();

    for (key, value) in x86_groups.iter() {
        parse_group(value, key, &mut ins_group_map);
        groups.push(key.to_owned());
    }

    (groups, ins_group_map)
}
