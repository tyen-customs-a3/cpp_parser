use thiserror::Error;

/// Error types for the CPP parser
#[derive(Debug, Error)]
pub enum Error {
    /// Error occurred during lexical analysis
    #[error("Lexer error at line {line}, column {column}: {message}")]
    Lexer {
        message: String,
        line: usize,
        column: usize,
    },

    /// Error occurred during parsing
    #[error("Parser error at line {line}, column {column}: {message}")]
    Parser {
        message: String,
        line: usize,
        column: usize,
    },

    /// Error occurred when accessing a property that doesn't exist
    #[error("Property not found: {0}")]
    PropertyNotFound(String),

    /// Error occurred when trying to access a class that doesn't exist
    #[error("Class not found: {0}")]
    ClassNotFound(String),

    /// Error occurred when trying to parse a value with incorrect type
    #[error("Type error: {0}")]
    TypeError(String),

    /// I/O error occurred
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
} 