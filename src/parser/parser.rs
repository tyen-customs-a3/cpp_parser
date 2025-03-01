use pest::Parser;
use pest_derive::Parser;

/// The main parser structure that implements the Pest parser
#[derive(Parser)]
#[grammar = "parser/cpp.pest"]
pub struct CppParser;