pub mod parser;

// Re-export commonly used types
pub use parser::{Class, Value, Property, parse_cpp, ParseError};