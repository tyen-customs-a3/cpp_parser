use cpp_parser::{parse, find_classes_by_name, find_classes_with_string, get_all_classes};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a configuration file
    let file_path = Path::new("tests/data/loadout.hpp");
    let content = fs::read_to_string(file_path)?;
    
    // Parse the configuration
    let classes = parse(&content)?;
    println!("Successfully parsed file with {} top-level classes", classes.len());
    
    // Find all classes with a specific name
    let rifleman_classes = find_classes_by_name(&classes, "rm");
    if !rifleman_classes.is_empty() {
        println!("\nRifleman class details:");
        let rm = &rifleman_classes[0];
        println!("  Name: {}", rm.name);
        if let Some(parent) = &rm.parent {
            println!("  Parent: {}", parent);
        }
        
        // List properties of the class
        println!("  Properties:");
        for (name, value) in &rm.properties {
            match value {
                cpp_parser::Property::String(s) => println!("    {}: \"{}\"", name, s),
                cpp_parser::Property::Number(n) => println!("    {}: {}", name, n),
                cpp_parser::Property::Boolean(b) => println!("    {}: {}", name, b),
                cpp_parser::Property::Array(items) => {
                    println!("    {}[] = {{", name);
                    for item in items {
                        if let Some(s) = item.as_string() {
                            println!("        \"{}\"", s);
                        } else if let Some(n) = item.as_number() {
                            println!("        {}", n);
                        } else if let Some(b) = item.as_boolean() {
                            println!("        {}", b);
                        }
                    }
                    println!("    }}");
                }
            }
        }
    } else {
        println!("Rifleman class not found");
    }
    
    // Find all classes that reference a specific string
    println!("\nClasses containing 'ACE_Vector':");
    let vector_classes = find_classes_with_string(&classes, "ACE_Vector");
    for class in vector_classes {
        println!("  - {}", class.name);
    }
    
    // Get all classes (including nested ones) and count them
    let all_classes = get_all_classes(&classes);
    println!("\nTotal number of classes (including nested): {}", all_classes.len());
    
    // Print all unique top-level class names
    println!("\nTop-level class names:");
    let mut names = classes.iter().map(|c| &c.name).collect::<Vec<_>>();
    names.sort();
    names.dedup();
    for name in names {
        println!("  - {}", name);
    }
    
    Ok(())
} 