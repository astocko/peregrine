#[macro_use]
mod macros;

mod types;
mod group_parser;
mod instruction_parser;
mod loader;
mod code_writer;
pub mod codegen;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        super::codegen::generate();
    }
}
