use pest::Position;
use std::collections::HashMap;

/// Preprocesses C++ content by simply stripping out all preprocessor directives
pub fn preprocess_cpp(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    
    for line in lines {
        let trimmed = line.trim();
        
        // Skip all preprocessor directives
        if !trimmed.starts_with("#") {
            result.push(line.to_string());
        }
    }
    
    result.join("\n")
}

/// Gets a string representation of a pest position for error reporting
pub fn get_location(pos: Position) -> String {
    format!("line {}, column {}", pos.line_col().0, pos.line_col().1)
} 