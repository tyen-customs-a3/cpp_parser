use cpp_parser::{parse, find_classes_with_string, find_all_matching_strings};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read multiple configuration files
    let file_paths = [
        "tests/data/loadout.hpp",
        "tests/data/ACE_Medical_Treatment.hpp",
    ];
    
    // Parse and collect data from each file
    let mut all_classes = Vec::new();
    for path in &file_paths {
        let content = fs::read_to_string(path)?;
        println!("Parsing file: {}", path);
        match parse(&content) {
            Ok(classes) => {
                println!("  Found {} top-level classes", classes.len());
                all_classes.extend(classes);
            },
            Err(e) => {
                println!("  Error parsing file: {}", e);
            }
        }
    }
    
    println!("\nTotal classes found across all files: {}", all_classes.len());
    
    // Find all references to ACE medical items
    println!("\nClasses referencing ACE medical items:");
    let medical_classes = find_classes_with_string(&all_classes, "ACE_");
    for class in medical_classes {
        println!("  Class: {}", class.name);
        
        // Find all the ACE_ references in this class
        let ace_refs = class.find_strings(|s| s.contains("ACE_"));
        println!("    References:");
        let mut unique_refs = HashSet::new();
        for item in ace_refs {
            unique_refs.insert(item);
        }
        for item in unique_refs {
            println!("      - {}", item);
        }
    }
    
    // Find all unique equipment items across all files
    println!("\nAll unique equipment items:");
    let is_equipment = |s: &str| s.starts_with("rhs_") || s.starts_with("ACE_") || s.starts_with("ACRE_");
    let all_equipment = find_all_matching_strings(&all_classes, is_equipment);
    
    // Count by category
    let mut ace_count = 0;
    let mut rhs_count = 0;
    let mut acre_count = 0;
    
    for item in &all_equipment {
        if item.starts_with("ACE_") {
            ace_count += 1;
        } else if item.starts_with("rhs_") {
            rhs_count += 1;
        } else if item.starts_with("ACRE_") {
            acre_count += 1;
        }
    }
    
    println!("  Total unique equipment items: {}", all_equipment.len());
    println!("    ACE items: {}", ace_count);
    println!("    RHS items: {}", rhs_count);
    println!("    ACRE items: {}", acre_count);
    
    // List some examples
    println!("\nSample equipment items:");
    let mut items: Vec<_> = all_equipment.iter().collect();
    items.sort();
    for (i, item) in items.iter().enumerate().take(10) {
        println!("  {}. {}", i+1, item);
    }
    
    if items.len() > 10 {
        println!("  ... and {} more", items.len() - 10);
    }
    
    Ok(())
} 