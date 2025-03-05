use std::fs;
use cpp_parser::preprocessor::{preprocess, ConfigElementType, ConfigElement};
use cpp_parser::class_parser::{Class, parse_classes};

fn main() {
    // Use our complex config file
    let file_path = "tests/data/complex_config.cpp";
    
    // Read the file
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    
    println!("=== STEP 1: Preprocessing the file ===");
    // First, preprocess the file to identify different elements
    let elements = preprocess(&content);
    
    // Count elements by type
    let defines = elements.iter().filter(|e| e.element_type == ConfigElementType::Define).count();
    let enums = elements.iter().filter(|e| e.element_type == ConfigElementType::Enum).count();
    let classes = elements.iter().filter(|e| e.element_type == ConfigElementType::Class).count();
    let forward_decls = elements.iter().filter(|e| e.element_type == ConfigElementType::ForwardDeclaration).count();
    let deletes = elements.iter().filter(|e| e.element_type == ConfigElementType::Delete).count();
    
    // Print summary
    println!("Found {} elements in the file:", elements.len());
    println!("- {} #define directives", defines);
    println!("- {} enums", enums);
    println!("- {} classes", classes);
    println!("- {} forward declarations", forward_decls);
    println!("- {} delete statements", deletes);
    
    // Print details about enums
    println!("\n=== STEP 2: Analyzing enums ===");
    let enum_elements: Vec<&ConfigElement> = elements.iter()
        .filter(|e| e.element_type == ConfigElementType::Enum)
        .collect();
    
    for (i, enum_element) in enum_elements.iter().enumerate() {
        println!("Enum #{}", i + 1);
        let item_count = count_enum_items(&enum_element.content);
        println!("- Contains {} items", item_count);
        
        // Get the first few items
        let items = get_enum_items(&enum_element.content);
        println!("- First few items:");
        for (j, item) in items.iter().take(5).enumerate() {
            println!("  {}. {}", j + 1, item);
        }
        if items.len() > 5 {
            println!("  ... and {} more", items.len() - 5);
        }
        println!();
    }
    
    // Now focus on classes
    println!("\n=== STEP 3: Detailed parsing of classes ===");
    
    // Get only top-level classes (those that aren't nested inside other classes)
    // We can identify these by checking their position against other classes
    let mut top_level_classes: Vec<&ConfigElement> = Vec::new();
    let class_elements: Vec<&ConfigElement> = elements.iter()
        .filter(|e| e.element_type == ConfigElementType::Class)
        .collect();
    
    // Determine which classes are top-level
    for class_element in &class_elements {
        let is_nested = class_elements.iter().any(|other| 
            other.start_pos < class_element.start_pos && 
            other.end_pos > class_element.end_pos &&
            other.element_type == ConfigElementType::Class
        );
        
        if !is_nested {
            top_level_classes.push(class_element);
        }
    }
    
    println!("Found {} classes in total, {} are top-level", class_elements.len(), top_level_classes.len());
    
    // Process only top-level classes with the class parser
    for (i, class_element) in top_level_classes.iter().enumerate().take(3) {
        println!("\nTop-level class #{}: {}", i + 1, class_element.name.as_deref().unwrap_or("unnamed"));
        
        // Get the full text of this class from the original content
        if let Some(class_text) = class_element.get_raw_text(&content) {
            // Parse this class with the class parser
            let classes = parse_classes(class_text);
            
            if !classes.is_empty() {
                let class = &classes[0];
                println!("- Parent: {:?}", class.parent);
                println!("- Children: {}", class.children.len());
                
                // Print the class structure recursively
                print_class_properties(class, 1);
            }
        }
    }
    
    println!("\n=== STEP 4: Forward declarations ===");
    let forward_decl_elements: Vec<&ConfigElement> = elements.iter()
        .filter(|e| e.element_type == ConfigElementType::ForwardDeclaration)
        .collect();
    
    println!("Found {} forward declarations:", forward_decl_elements.len());
    for (i, decl) in forward_decl_elements.iter().enumerate() {
        if let Some(name) = &decl.name {
            println!("{}. {}", i + 1, name);
        }
    }
}

// Helper function to count items in an enum
fn count_enum_items(content: &str) -> usize {
    content.lines().filter(|line| line.contains('=')).count()
}

// Helper function to get enum items
fn get_enum_items(content: &str) -> Vec<String> {
    content.lines()
        .filter(|line| line.contains('='))
        .map(|line| line.trim().to_string())
        .collect()
}

// Helper function to print class properties recursively
fn print_class_properties(class: &Class, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!("{}* Class: {:?}", indent_str, class.name);
    
    if !class.children.is_empty() {
        println!("{}  Children:", indent_str);
        for child in &class.children {
            print_class_properties(child, indent + 1);
        }
    }
} 