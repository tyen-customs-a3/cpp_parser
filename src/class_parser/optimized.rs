use crate::class_parser::Class;
use regex::RegexBuilder;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Parse a string into a vector of classes using an optimized algorithm
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

/// Parse a string into a vector of classes using an optimized parallel algorithm
pub fn parse_classes_parallel(input: &str) -> Vec<Class> {
    // Find all top-level class declarations
    let class_regex = RegexBuilder::new(r"class\s+([A-Za-z0-9_]+)(?:\s*:\s*([A-Za-z0-9_]+))?\s*\{")
        .multi_line(true)
        .build()
        .unwrap();
    
    // Pre-compute a map of matching braces for the entire input
    let brace_map = compute_brace_map(input);
    
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
            let opening_brace_pos = remaining[match_start..].find('{').unwrap() + match_start;
            let absolute_opening_brace_pos = pos + opening_brace_pos;
            
            matches.push((absolute_match_start, absolute_opening_brace_pos, class_name, parent_name));
            
            // Move past this match
            pos = absolute_match_start + 1;
        } else {
            // No more matches, we're done
            break;
        }
    }
    
    // Process each match in parallel
    let brace_map_arc = Arc::new(brace_map);
    let classes: Vec<Class> = matches.par_iter()
        .filter_map(|(start_pos, opening_brace_pos, class_name, parent_name)| {
            let brace_map = brace_map_arc.clone();
            
            // Find the matching closing brace using the pre-computed map
            if let Some(&closing_brace_pos) = brace_map.get(opening_brace_pos) {
                // Extract the content between braces (excluding the braces themselves)
                let content = input[*opening_brace_pos + 1..closing_brace_pos].to_string();
                
                // Create a new class
                let mut class = Class::new(
                    class_name.clone(),
                    parent_name.clone(),
                    content.clone(),
                    *start_pos,
                    closing_brace_pos + 1 // +1 to include the closing brace
                );
                
                // Find nested classes within the content
                let nested_classes = find_classes(&content, *start_pos + *opening_brace_pos + 1);
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

/// Find all classes in the input string using an optimized algorithm
fn find_classes(input: &str, offset: usize) -> Vec<Class> {
    let mut classes = Vec::new();
    
    // Regex to match class declarations with optional inheritance
    // Format: class Name [: Parent] { ... }
    let class_regex = RegexBuilder::new(r"class\s+([A-Za-z0-9_]+)(?:\s*:\s*([A-Za-z0-9_]+))?\s*\{")
        .multi_line(true)
        .build()
        .unwrap();
    
    // Pre-compute a map of matching braces for the entire input
    let brace_map = compute_brace_map(input);
    
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
            
            // Find the matching closing brace using the pre-computed map
            if let Some(&closing_brace_pos) = brace_map.get(&absolute_opening_brace_pos) {
                let start_pos = offset + absolute_match_start;
                let end_pos = offset + closing_brace_pos + 1; // +1 to include the closing brace
                
                // Extract the content between braces (excluding the braces themselves)
                let content = input[absolute_opening_brace_pos + 1..closing_brace_pos].to_string();
                
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
                pos = closing_brace_pos + 1;
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

/// Compute a map of opening brace positions to their matching closing brace positions
/// This is much more efficient than finding matching braces one by one
fn compute_brace_map(input: &str) -> HashMap<usize, usize> {
    let mut brace_map = HashMap::new();
    let mut stack = Vec::new();
    
    // Scan the input once to find all matching braces
    for (i, c) in input.chars().enumerate() {
        match c {
            '{' => stack.push(i),
            '}' => {
                if let Some(opening_pos) = stack.pop() {
                    brace_map.insert(opening_pos, i);
                }
            },
            _ => {}
        }
    }
    
    brace_map
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimized_class_parsing() {
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
        
        let classes = parse_classes(input);
        assert_eq!(classes.len(), 2);
        
        // Check the first class (Vehicle)
        let vehicle = &classes[0];
        assert_eq!(vehicle.name, Some("Vehicle".to_string()));
        assert_eq!(vehicle.parent, None);
        assert_eq!(vehicle.children.len(), 2);
        
        // Check the nested classes in Vehicle
        let engine = &vehicle.children[0];
        let wheels = &vehicle.children[1];
        assert_eq!(engine.name, Some("Engine".to_string()));
        assert_eq!(wheels.name, Some("Wheels".to_string()));
        
        // Check the second class (Car)
        let car = &classes[1];
        assert_eq!(car.name, Some("Car".to_string()));
        assert_eq!(car.parent, Some("Vehicle".to_string()));
        assert_eq!(car.children.len(), 1);
        
        // Check the nested class in Car
        let transmission = &car.children[0];
        assert_eq!(transmission.name, Some("Transmission".to_string()));
    }
    
    #[test]
    fn test_optimized_parallel_class_parsing() {
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
        
        let classes = parse_classes_parallel(input);
        assert_eq!(classes.len(), 2);
        
        // Check the first class (Vehicle)
        let vehicle = &classes[0];
        assert_eq!(vehicle.name, Some("Vehicle".to_string()));
        assert_eq!(vehicle.parent, None);
        assert_eq!(vehicle.children.len(), 2);
        
        // Check the nested classes in Vehicle
        let engine = &vehicle.children[0];
        let wheels = &vehicle.children[1];
        assert_eq!(engine.name, Some("Engine".to_string()));
        assert_eq!(wheels.name, Some("Wheels".to_string()));
        
        // Check the second class (Car)
        let car = &classes[1];
        assert_eq!(car.name, Some("Car".to_string()));
        assert_eq!(car.parent, Some("Vehicle".to_string()));
        assert_eq!(car.children.len(), 1);
        
        // Check the nested class in Car
        let transmission = &car.children[0];
        assert_eq!(transmission.name, Some("Transmission".to_string()));
    }
    
    #[test]
    fn test_compute_brace_map() {
        let input = "{ nested { braces } }";
        let brace_map = compute_brace_map(input);
        
        // Print the actual brace map for debugging
        println!("Brace map: {:?}", brace_map);
        println!("Input: {:?}", input);
        for (i, c) in input.chars().enumerate() {
            println!("Index {}: '{}'", i, c);
        }
        
        assert_eq!(brace_map.len(), 2);
        assert_eq!(brace_map.get(&0), Some(&20));
        assert_eq!(brace_map.get(&9), Some(&18));
    }
} 