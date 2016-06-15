macro_rules! parse_str_as_enum {
    ($val:ident, $et:ident, $out:expr) => {
        match *$val {
            Value::String(ref s) => {
                match $et::from_str(s) {
                    Ok(res) => $out = res,
                    Err(e) => panic!("{}", e),
                }
            }
            _ => {println!("{:?}", $val); panic!("Unexpected JSON field type")},
        }
    }
}

macro_rules! parse_bool_as_bool{
    ($val:ident, $out:expr) => {
        match *$val {
            Value::Bool(ref b) => {
                $out = *b
            }
            _ => {println!("{:?}", $val); panic!("Unexpected JSON field type")}
        }
    }
}

macro_rules! parse_str_as_hex{
    ($val:ident, $out:expr) => {
        match *$val {
            Value::String(ref hex) => {
                match u8::from_str_radix(hex, 16) {
                    Ok(res) => $out = res,
                    Err(e) => panic!("{}", e),
                }
            }
            _ => {println!("{:?}", $val); panic!("Unexpected JSON field type")}
        }
    }
}

macro_rules! parse_str_as_bin{
    ($val:ident, $out:expr) => {
        match *$val {
            Value::String(ref hex) => {
                match u8::from_str_radix(hex, 2) {
                    Ok(res) => $out = res,
                    Err(e) => panic!("{}", e),
                }
            }
            _ => {println!("{:?}", $val); panic!("Unexpected JSON field type")}
        }
    }
}

macro_rules! parse_str_as_pow2 {
    ($val:ident, $out:expr, $min:expr, $max:expr) => {
        match *$val {
            Value::U64(ref b) => {
                if (*b != 0) && ((*b & (*b - 1)) == 0 && (*b >= $min || *b <= $max)) {
                    $out = *b as u8;
                } else {
                    panic!("Number was not a power of 2!");
                }
            }
            _ => {println!("{:?}", $val); panic!("Unexpected JSON field type")}
        }
    }

}

macro_rules! parse_num_as_u64{
    ($val:ident, $out:expr) => {
        match *$val {
            Value::U64(ref b) => {
                $out = *b
            }
            _ => {println!("{:?}", $val); panic!("Unexpected JSON field type")}
        }
    }
}

macro_rules! parse_num_with_values {
    ($val:ident, $out:expr, $($num:expr),+) => {
        match *$val {
            Value::U64(ref b) => {
                match *b {
                $(
                    $num => $out = *b as u8,
                )*
                    _ => panic!("Invalid number for JSON field type"),
                }
            }
            _ => {println!("{:?}", $val); panic!("Unexpected JSON field type")}
        }
    }

}

macro_rules! parse_str_as_bool{
    ($val:ident, $out:expr) => {
        match *$val {
            Value::String(ref s) => {
                match bool::from_str(s) {
                    Ok(res) => $out = res,
                    Err(e) => panic!("{}", e),
                }
            }
            _ => {println!("{:?}", $val); panic!("Unexpected JSON field type")},
        }
    }
}
