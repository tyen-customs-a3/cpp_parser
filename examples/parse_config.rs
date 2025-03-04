use cpp_parser::{parse_cpp, parse_cpp_file, Class, Value};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the file path from command line arguments or use a default example
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // Parse a file specified by the user
        let file_path = &args[1];
        println!("Parsing file: {}", file_path);
        
        match parse_cpp_file(file_path) {
            Ok(classes) => {
                println!("Successfully parsed {} top-level classes", classes.len());
                print_classes(&classes, 0);
            },
            Err(error) => {
                eprintln!("Failed to parse file: {}", error.message);
                std::process::exit(1);
            }
        }
    } else {
        // No file specified, use an example string
        println!("No file specified, using example string");
        
        let example = r#"
        class CfgPatches
        {
            class MyMod
            {
                name = "My Example Mod";
                author = "CPP Parser";
                requiredVersion = 1.94;
                requiredAddons[] = {"A3_Data_F", "A3_Weapons_F"};
                units[] = {};
                weapons[] = {"MyWeapon"};
                version = 1.0;
            };
        };
        
        class CfgWeapons
        {
            class ItemCore;
            class MyWeapon: ItemCore
            {
                displayName = "My Custom Weapon";
                model = "\path\to\model.p3d";
                
                class WeaponSlotsInfo
                {
                    mass = 50;
                    allowedSlots[] = {801, 901, 701};
                };
            };
        };
        "#;
        
        match parse_cpp(example) {
            Ok(classes) => {
                println!("Successfully parsed {} top-level classes", classes.len());
                print_classes(&classes, 0);
            },
            Err(error) => {
                eprintln!("Failed to parse example: {}", error.message);
                std::process::exit(1);
            }
        }
    }
}

// Helper function to print classes recursively with indentation
fn print_classes(classes: &[Class], indent: usize) {
    let indent_str = " ".repeat(indent * 2);
    
    for class in classes {
        // Print class name and parent if available
        if let Some(name) = &class.name {
            if let Some(parent) = &class.parent {
                println!("{}class {}: {} {{", indent_str, name, parent);
            } else {
                println!("{}class {} {{", indent_str, name);
            }
        } else {
            println!("{}class {{", indent_str);
        }
        
        // Print properties
        for (name, value) in &class.properties {
            println!("{}{} = {};", indent_str, name, format_value(value));
        }
        
        // Print nested classes
        for (_, nested_class) in &class.classes {
            print_classes(&[nested_class.clone()], indent + 1);
        }
        
        println!("{}}};", indent_str);
    }
}

// Helper function to format values for display
fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Number(n) => n.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_value).collect();
            format!("{{{}}}", items.join(", "))
        },
        Value::Class(c) => {
            let mut result = String::from("{ ");
            for (k, v) in &c.properties {
                result.push_str(&format!("{}: {}, ", k, format_value(v)));
            }
            result.push_str("}");
            result
        },
        Value::Expression(e) => format!("{}", e),
        Value::Reference(r) => format!("{}", r),
        &Value::ListMacro(count, ref content) => format!("LIST_{}(\"{}\")", count, content),
    }
} 