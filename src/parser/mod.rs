mod models;
mod passes;
mod utils;

pub use models::{Class, Value, Property, ParseError};
pub use models::context::ParseContext;
pub use passes::lexical_pass::lexical_pass;
pub use passes::symbols_pass::symbols_pass;
pub use passes::properties_pass::properties_pass;
pub use passes::references_pass::references_pass;
pub use utils::preprocess_cpp;
use std::fs::read_to_string;
use std::time::Instant;
use log::info;

/// Parses a CPP file using a multi-pass approach
pub fn parse_cpp(content: &str) -> Result<Vec<models::Class>, models::ParseError> {
    // Preprocess the content
    let preprocessed = preprocess_cpp(content);
    
    // Create the parsing context
    let mut context = ParseContext::new();
    
    // Pass 1: Lexical analysis
    lexical_pass(&preprocessed, &mut context)?;
    
    // Pass 2: Symbol collection
    symbols_pass(&mut context)?;
    
    // Pass 3: Property processing
    properties_pass(&mut context)?;
    
    // Pass 4: Reference resolution
    // Disabled: We do not care about relinking macros or other properties right now
    // references_pass(&mut context)?;
    
    // Return the parsed classes
    Ok(context.classes)
}

/// Parses a CPP file from a file path
pub fn parse_cpp_file(path: &str) -> Result<Vec<models::Class>, models::ParseError> {

    
    info!(target: "cpp_parser", "Parsing file: {}", path);
    
    // Start timing
    let start_time = Instant::now();
    
    // Read the file
    let content = read_to_string(path)
        .map_err(|e| models::ParseError::new(format!("Failed to read file {}: {}", path, e)))?;
    
    // Parse the content
    let result = parse_cpp(&content);
    
    // Calculate and log the elapsed time
    let elapsed = start_time.elapsed();
    info!(target: "cpp_parser", "Parsed file {} in {:.2?}", path, elapsed);
    
    result
}