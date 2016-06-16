#[macro_use]
mod macros;

mod types;
mod group_parser;
mod instruction_parser;
mod loader;
pub mod codegen;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        super::codegen::generate();
    }
}
