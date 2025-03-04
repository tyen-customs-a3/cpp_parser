pub mod models;
pub mod passes;
mod utils;

use std::path::Path;
use std::fs;
use std::fs::read_to_string;
use std::time::Instant;
use log::{debug, info};

pub use models::{Class, Value, Property, ParseError};
pub use models::context::ParseContext;
use passes::lexical_pass::lexical_pass;
use passes::symbols_pass::symbols_pass;
use passes::properties_pass::properties_pass;
use passes::references_pass::references_pass;
use utils::preprocess_cpp;

/// Parses a CPP file using a multi-pass approach
pub fn parse_cpp(content: &str) -> Result<Vec<Class>, ParseError> {
    info!(target: "cpp_parser", "Parsing CPP content");
    
    // Start timing
    let start_time = Instant::now();
    
    // Preprocess the content to handle includes and defines
    let preprocessed = preprocess_cpp(content);
    
    // Create a parse context to track state across passes
    let mut context = ParseContext::new();
    
    // Run the lexical pass to tokenize the input
    lexical_pass(&preprocessed, &mut context)?;
    
    // Run the symbols pass to identify classes and properties
    symbols_pass(&mut context)?;
    
    // Run the properties pass to process property values
    properties_pass(&mut context)?;
    
    // Run the references pass to resolve references
    references_pass(&mut context)?;
    
    // Calculate and log the elapsed time
    let elapsed = start_time.elapsed();
    info!(target: "cpp_parser", "Parsed CPP content in {:.2?}", elapsed);
    
    // Return the parsed classes
    Ok(context.classes)
}

/// Parses a CPP file from a file path
pub fn parse_cpp_file(file_path: &str) -> Result<Vec<Class>, ParseError> {
    info!(target: "cpp_parser", "Parsing file: {}", file_path);
    
    // Start timing
    let start_time = Instant::now();
    
    // Read the file
    let content = read_to_string(file_path)
        .map_err(|e| ParseError::new(format!("Failed to read file {}: {}", file_path, e)))?;
    
    // Parse the content
    let result = parse_cpp(&content);
    
    // Calculate and log the elapsed time
    let elapsed = start_time.elapsed();
    info!(target: "cpp_parser", "Parsed file {} in {:.2?}", file_path, elapsed);
    
    result
}