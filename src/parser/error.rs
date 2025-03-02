use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error at {location}: {message}\nContext: {context}\nSnippet: {snippet}")]
    PestError {
        message: String,
        location: String,
        context: String,
        snippet: String,
    },
    
    #[error("Value error at {location}: {message}\nContext: {context}\nSnippet: {snippet}")]
    ValueError {
        message: String,
        location: String,
        context: String,
        snippet: String,
    },
    
    #[error("Enum error at {location}: {message}\nContext: {context}\nSnippet: {snippet}")]
    EnumError {
        message: String,
        location: String,
        context: String,
        snippet: String,
    },
}