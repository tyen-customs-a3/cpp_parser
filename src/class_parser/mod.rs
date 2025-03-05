use std::fmt;
use regex::RegexBuilder;

pub mod class_model;
pub mod class_parser;

/// Represents a parsed class in the config file
#[derive(Debug, Clone)]
pub struct Class {
    /// The name of the class
    pub name: Option<String>,
    /// The parent class name, if any
    pub parent: Option<String>,
    /// The raw content of the class
    pub content: String,
    /// Nested classes within this class
    pub children: Vec<Class>,
    /// Start position in the original text
    pub start_pos: usize,
    /// End position in the original text
    pub end_pos: usize,
}

impl Class {
    /// Create a new class
    pub fn new(name: Option<String>, parent: Option<String>, content: String, start_pos: usize, end_pos: usize) -> Self {
        Class {
            name,
            parent,
            content,
            children: Vec::new(),
            start_pos,
            end_pos,
        }
    }

    /// Add a child class
    pub fn add_child(&mut self, child: Class) {
        self.children.push(child);
    }

    /// Get the raw text of this class from the original input
    pub fn get_raw_text<'a>(&self, input: &'a str) -> Option<&'a str> {
        if self.start_pos < input.len() && self.end_pos <= input.len() && self.start_pos <= self.end_pos {
            Some(&input[self.start_pos..self.end_pos])
        } else {
            None
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class {{ name: {:?}, parent: {:?}, children: {} }}", 
            self.name, self.parent, self.children.len())
    }
}

/// Parse a string into a vector of classes
pub fn parse_classes(input: &str) -> Vec<Class> {
    let mut top_level_classes = Vec::new();
    
    // Find all classes in the input
    let classes = find_classes(input, 0);
    
    // Add top-level classes to the result
    for class in classes {
        top_level_classes.push(class);
    }
    
    top_level_classes
}

/// Find all classes in the input string
fn find_classes(input: &str, offset: usize) -> Vec<Class> {
    let mut classes = Vec::new();
    
    // Regex to match class declarations with optional inheritance
    // Format: class Name [: Parent] { ... }
    let class_regex = RegexBuilder::new(r"class\s+([A-Za-z0-9_]+)(?:\s*:\s*([A-Za-z0-9_]+))?\s*\{")
        .multi_line(true)
        .build()
        .unwrap();
    
    // Find all class declarations at this level
    let mut pos = 0;
    while pos < input.len() {
        let remaining = &input[pos..];
        
        if let Some(class_match) = class_regex.captures(remaining) {
            let class_name = class_match.get(1).map(|m| m.as_str().to_string());
            let parent_name = class_match.get(2).map(|m| m.as_str().to_string());
            
            // Find the opening brace position
            let match_start = class_match.get(0).unwrap().start();
            let absolute_match_start = pos + match_start;
            let opening_brace_pos = remaining[match_start..].find('{').unwrap() + match_start;
            let absolute_opening_brace_pos = pos + opening_brace_pos;
            
            // Find the matching closing brace
            if let Some((content, end_pos)) = find_matching_brace(input, absolute_opening_brace_pos) {
                let start_pos = offset + absolute_match_start;
                let absolute_end_pos = absolute_opening_brace_pos + end_pos - absolute_opening_brace_pos;
                let end_pos = offset + absolute_end_pos;
                
                // Create a new class
                let mut class = Class::new(
                    class_name,
                    parent_name,
                    content.clone(),
                    start_pos,
                    end_pos
                );
                
                // Find nested classes within the content
                let nested_classes = find_classes(&content, offset + absolute_opening_brace_pos + 1);
                for nested_class in nested_classes {
                    class.add_child(nested_class);
                }
                
                classes.push(class);
                
                // Move past this class
                pos = absolute_end_pos;
            } else {
                // If we can't find a matching brace, move past this match
                pos = absolute_match_start + 1;
            }
        } else {
            // No more matches, we're done
            break;
        }
    }
    
    classes
}

/// Find the matching closing brace for an opening brace
fn find_matching_brace(input: &str, opening_brace_pos: usize) -> Option<(String, usize)> {
    let mut brace_count = 1;
    let mut pos = opening_brace_pos + 1;
    
    while pos < input.len() && brace_count > 0 {
        match input.chars().nth(pos) {
            Some('{') => brace_count += 1,
            Some('}') => brace_count -= 1,
            _ => {}
        }
        pos += 1;
    }
    
    if brace_count == 0 {
        // Extract the content between braces (excluding the braces themselves)
        let content = input[opening_brace_pos + 1..pos - 1].to_string();
        Some((content, pos))
    } else {
        None
    }
}

// Re-export commonly used types
pub use class_model::{Class as ModelClass, ClassCollection};
pub use class_parser::parse_classes_to_collection;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_class() {
        let input = r#"
        class baseMan {
            displayName = "Unarmed";
            uniform = "rhs_uniform_g3_m81";
        };
        "#;
        let result = parse_classes(input);
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, Some("baseMan".to_string()));
        assert_eq!(result[0].parent, None);
        assert!(result[0].get_raw_text(input).is_some());
    }

    #[test]
    fn test_inherited_class() {
        let input = r#"
        class baseMan {
            displayName = "Unarmed";
        };
        class rm : baseMan {
            primaryWeapon = "rhs_weap_m4a1_blockII_KAC";
        };
        "#;
        let result = parse_classes(input);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, Some("baseMan".to_string()));
        assert_eq!(result[0].parent, None);
        assert_eq!(result[1].name, Some("rm".to_string()));
        assert_eq!(result[1].parent, Some("baseMan".to_string()));
        assert!(result[0].get_raw_text(input).is_some());
        assert!(result[1].get_raw_text(input).is_some());
    }

    #[test]
    fn test_nested_classes() {
        let input = r#"
        class baseMan {
            class Inventory {
                class uniform {
                    type = "rhs_uniform_g3_m81";
                };
            };
        };
        "#;
        let result = parse_classes(input);
        
        assert_eq!(result.len(), 1);
        let base_man = &result[0];
        assert_eq!(base_man.name, Some("baseMan".to_string()));
        assert_eq!(base_man.children.len(), 1);
        
        let inventory = &base_man.children[0];
        assert_eq!(inventory.name, Some("Inventory".to_string()));
        assert_eq!(inventory.children.len(), 1);
        
        let uniform = &inventory.children[0];
        assert_eq!(uniform.name, Some("uniform".to_string()));
        assert!(base_man.get_raw_text(input).is_some());
    }
} 