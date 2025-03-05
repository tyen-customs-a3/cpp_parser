use std::fs;
use cpp_parser::class_parser::parse_classes;
use cpp_parser::property_parser::block_properties::{parse_block_properties, parse_all_block_properties};

fn main() {
    // Use a simple config file
    let file_path = "examples/simple_config.txt";
    
    // Read the file
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    
    // Parse the classes
    let classes = parse_classes(&content);
    println!("Found {} top-level classes in simple_config.txt", classes.len());
    
    // Process each class
    for (i, class) in classes.iter().enumerate() {
        println!("\nClass #{}: {:?}", i + 1, class.name);
        println!("Parent: {:?}", class.parent);
        println!("Children: {}", class.children.len());
        
        // Parse properties
        if let Ok(properties) = parse_block_properties(class, &content) {
            println!("Properties:");
            for prop in properties {
                println!("  {} = {}", prop.name, prop.value);
            }
        }
        
        // Process nested classes
        if !class.children.is_empty() {
            println!("Nested classes:");
            for child in &class.children {
                println!("  {:?}", child.name);
                
                // Parse properties of nested classes
                if let Ok(properties) = parse_block_properties(child, &content) {
                    println!("  Properties:");
                    for prop in properties {
                        println!("    {} = {}", prop.name, prop.value);
                    }
                }
            }
        }
    }
    
    // Now try with a complex config file
    println!("\n\n--- Complex Config Example ---\n");
    let complex_file_path = "tests/data/complex_config.cpp";
    
    // Read the file
    let complex_content = match fs::read_to_string(complex_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    
    // Parse the classes
    let complex_classes = parse_classes(&complex_content);
    println!("Found {} top-level classes in complex_config.cpp", complex_classes.len());
    
    // Parse all properties from all classes
    let all_properties = parse_all_block_properties(&complex_classes, &complex_content);
    println!("Found {} properties in all classes", all_properties.len());
    
    // Print some example properties
    println!("\nSome example properties:");
    for (i, (class_name, prop_name, prop_value)) in all_properties.iter().enumerate().take(10) {
        println!("  {}: {}.{} = {}", i + 1, class_name, prop_name, prop_value);
    }
    
    // Find a specific class and its properties
    if let Some(cfg_time_trials) = complex_classes.iter().find(|c| c.name.as_deref() == Some("CfgTimeTrials")) {
        println!("\nProperties of CfgTimeTrials:");
        if let Ok(properties) = parse_block_properties(cfg_time_trials, &complex_content) {
            for prop in properties {
                println!("  {} = {}", prop.name, prop.value);
            }
        }
        
        // Find a nested class
        if let Some(helpers) = cfg_time_trials.children.iter().find(|c| c.name.as_deref() == Some("Helpers")) {
            println!("\nProperties of Helpers.Sign_Circle_F:");
            if let Some(sign_circle) = helpers.children.iter().find(|c| c.name.as_deref() == Some("Sign_Circle_F")) {
                if let Ok(properties) = parse_block_properties(sign_circle, &complex_content) {
                    for prop in properties {
                        println!("  {} = {}", prop.name, prop.value);
                    }
                }
            }
        }
    }
} 