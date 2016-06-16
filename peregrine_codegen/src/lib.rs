#[macro_use]
mod macros;

mod types;
mod group_parser;
mod instruction_parser;
mod loader;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        super::loader::load_instruction_set();
    }
}
