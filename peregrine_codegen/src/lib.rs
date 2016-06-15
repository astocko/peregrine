#[macro_use]
mod macros;

mod types;
mod group_parser;
mod instruction_parser;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        super::instruction_parser::load_instructions();
        super::group_parser::load_instruction_groups();
    }
}
