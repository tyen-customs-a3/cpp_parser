use std::fmt;
use regex::RegexBuilder;
use rayon::prelude::*;

pub mod class_model;
pub mod class_parser;
pub mod optimized;

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

/// Parse a string into a vector of classes using parallel processing
pub fn parse_classes_parallel(input: &str) -> Vec<Class> {
    // Find all top-level class declarations
    let class_regex = RegexBuilder::new(r"class\s+([A-Za-z0-9_]+)(?:\s*:\s*([A-Za-z0-9_]+))?\s*\{")
        .multi_line(true)
        .build()
        .unwrap();
    
    // Find all matches and their positions
    let mut matches = Vec::new();
    let mut pos = 0;
    
    while pos < input.len() {
        let remaining = &input[pos..];
        
        if let Some(class_match) = class_regex.captures(remaining) {
            let class_name = class_match.get(1).map(|m| m.as_str().to_string());
            let parent_name = class_match.get(2).map(|m| m.as_str().to_string());
            
            // Find the opening brace position
            let match_start = class_match.get(0).unwrap().start();
            let absolute_match_start = pos + match_start;
            
            matches.push((absolute_match_start, class_name, parent_name));
            
            // Move past this match
            pos = absolute_match_start + 1;
        } else {
            // No more matches, we're done
            break;
        }
    }
    
    // Process each match in parallel
    let classes: Vec<Class> = matches.par_iter()
        .filter_map(|(start_pos, class_name, parent_name)| {
            let opening_brace_pos = input[*start_pos..].find('{')? + *start_pos;
            
            // Find the matching closing brace
            if let Some((content, end_pos)) = find_matching_brace(input, opening_brace_pos) {
                let absolute_end_pos = opening_brace_pos + end_pos - opening_brace_pos;
                
                // Create a new class
                let mut class = Class::new(
                    class_name.clone(),
                    parent_name.clone(),
                    content.clone(),
                    *start_pos,
                    absolute_end_pos
                );
                
                // Find nested classes within the content (recursively)
                let nested_classes = find_classes(&content, *start_pos + opening_brace_pos + 1);
                for nested_class in nested_classes {
                    class.add_child(nested_class);
                }
                
                Some(class)
            } else {
                None
            }
        })
        .collect();
    
    // Filter out nested classes (those that are contained within other classes)
    let classes_clone = classes.clone();
    classes.into_iter()
        .filter(|class| {
            !classes_clone.iter().any(|other| 
                other.start_pos < class.start_pos && 
                other.end_pos > class.end_pos && 
                other.start_pos != class.start_pos && 
                other.end_pos != class.end_pos
            )
        })
        .collect()
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
        class Vehicle {
            displayName = "Vehicle";
            maxSpeed = 100;
        };
        "#;
        
        let classes = parse_classes(input);
        assert_eq!(classes.len(), 1);
        
        let vehicle = &classes[0];
        assert_eq!(vehicle.name, Some("Vehicle".to_string()));
        assert_eq!(vehicle.parent, None);
        assert!(vehicle.content.contains("displayName"));
        assert!(vehicle.content.contains("maxSpeed"));
    }
    
    #[test]
    fn test_inherited_class() {
        let input = r#"
        class Car: Vehicle {
            doors = 4;
            wheels = 4;
        };
        "#;
        
        let classes = parse_classes(input);
        assert_eq!(classes.len(), 1);
        
        let car = &classes[0];
        assert_eq!(car.name, Some("Car".to_string()));
        assert_eq!(car.parent, Some("Vehicle".to_string()));
        assert!(car.content.contains("doors"));
        assert!(car.content.contains("wheels"));
    }
    
    #[test]
    fn test_nested_classes() {
        let input = r#"
        class Vehicle {
            class Engine {
                power = 100;
            };
            
            class Wheels {
                count = 4;
            };
        };
        "#;
        
        let classes = parse_classes(input);
        assert_eq!(classes.len(), 1);
        
        let vehicle = &classes[0];
        assert_eq!(vehicle.children.len(), 2);
        
        let engine = &vehicle.children[0];
        let wheels = &vehicle.children[1];
        
        assert_eq!(engine.name, Some("Engine".to_string()));
        assert!(engine.content.contains("power"));
        
        assert_eq!(wheels.name, Some("Wheels".to_string()));
        assert!(wheels.content.contains("count"));
    }
    
    #[test]
    fn test_parallel_class_parsing() {
        let input = r#"
        class Vehicle {
            displayName = "Vehicle";
            
            class Engine {
                power = 100;
                torque = 200;
            };
            
            class Wheels {
                count = 4;
                radius = 0.5;
            };
        };
        
        class Car: Vehicle {
            maxSpeed = 200;
            
            class Transmission {
                gears = 6;
                ratio = 3.5;
            };
        };
        "#;
        
        // Parse with both methods
        let sequential_classes = parse_classes(input);
        let parallel_classes = parse_classes_parallel(input);
        
        // Verify both methods produce the same number of classes
        assert_eq!(sequential_classes.len(), parallel_classes.len());
        assert_eq!(sequential_classes.len(), 2);
        
        // Verify both methods find the same nested classes
        let sequential_nested_count = count_nested_classes(&sequential_classes);
        let parallel_nested_count = count_nested_classes(&parallel_classes);
        assert_eq!(sequential_nested_count, parallel_nested_count);
        assert_eq!(sequential_nested_count, 3); // Engine, Wheels, Transmission
    }
    
    fn count_nested_classes(classes: &[Class]) -> usize {
        let mut count = 0;
        for class in classes {
            count += class.children.len();
            count += count_nested_classes(&class.children);
        }
        count
    }
} 