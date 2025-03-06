use crate::models::{Class, Code, Property, PropertyValue};
use crate::preprocessor::{preprocess_with_pest, CodeElement, CodeElementType};
use crate::property_parser::parse_properties;
use crate::models::code::{CodeEntry};
use crate::preprocessor::ast_converter::convert_to_code_elements;
use crate::preprocessor::parser::{parse, Rule};
use pest::iterators::Pairs;

/// Result type for the parser operations
pub type ParseResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Parse a complete C++ file containing class definitions and properties
pub fn parse_file(input: &str) -> ParseResult<Code> {
    // First preprocess the input to get all code elements
    let elements = preprocess_with_pest(input)?;
    
    // Debug: print all elements
    println!("Preprocessed elements:");
    for element in &elements {
        println!("  {:?} - name: {:?}, type: {:?}", 
            element.content.lines().next().unwrap_or(""),
            element.name,
            element.element_type
        );
    }
    
    // Create the initial Code structure without processing properties
    let mut code = Code::from_elements(elements);
    
    // Process properties in parallel
    code.process_properties();
    
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CodeEntry;

    #[test]
    fn test_parse_simple_class() {
        let input = r#"
            class Vehicle {
                displayName = "Vehicle";
                maxSpeed = 100;
            };
        "#;

        let code = parse_file(input).unwrap();
        let classes = code.classes();
        assert_eq!(classes.len(), 1);

        let vehicle = &classes[0];
        assert_eq!(vehicle.name(), "Vehicle");
        assert_eq!(vehicle.properties.len(), 2);
        
        let display_name = vehicle.get_property("displayName").unwrap();
        assert!(matches!(display_name.value, PropertyValue::String(ref s) if s == "Vehicle"));
        
        let max_speed = vehicle.get_property("maxSpeed").unwrap();
        assert!(matches!(max_speed.value, PropertyValue::Number(100.0)));
    }

    #[test]
    fn test_parse_inherited_class() {
        let input = r#"
            class Car: Vehicle {
                doors = 4;
                wheels = 4;
            };
        "#;

        let code = parse_file(input).unwrap();
        let classes = code.classes();
        assert_eq!(classes.len(), 1);

        let car = &classes[0];
        assert_eq!(car.name(), "Car");
        assert_eq!(car.parent(), Some("Vehicle"));
        assert_eq!(car.properties.len(), 2);
    }

    #[test]
    fn test_parse_mixed_content() {
        let input = r#"
            #define VERSION 1.0
            
            class RscText;
            
            enum {
                RED = 0,
                GREEN = 1,
                BLUE = 2
            };
            
            class Vehicle {
                maxSpeed = 100;
            };
            
            delete OldClass;
        "#;

        // Debug: Print preprocessed elements
        let result = crate::preprocessor::parser::parse(input).unwrap();
        fn print_pairs(pairs: Pairs<Rule>, depth: usize) {
            for pair in pairs {
                let indent = "  ".repeat(depth);
                println!("{}Rule: {:?} - Text: {:?}", indent, pair.as_rule(), pair.as_str());
                print_pairs(pair.into_inner(), depth + 1);
            }
        }
        print_pairs(result, 0);

        let code = parse_file(input).unwrap();
        
        // Check that all elements are present
        assert_eq!(code.classes().len(), 1);
        assert_eq!(code.defines().len(), 1);
        assert_eq!(code.forward_declarations().len(), 1);
        assert_eq!(code.enums().len(), 1);
        assert_eq!(code.deletes().len(), 1);

        // Verify the order of elements
        let elements = code.elements();
        assert_eq!(elements.len(), 5);

        // First element should be the #define
        match &elements[0] {
            CodeEntry::Element(e) => {
                assert_eq!(e.element_type, CodeElementType::Define);
                assert_eq!(e.name.as_deref(), Some("VERSION"));
            },
            _ => panic!("First element should be a define"),
        }

        // Second element should be the forward declaration
        match &elements[1] {
            CodeEntry::Element(e) => {
                assert_eq!(e.element_type, CodeElementType::ForwardDeclaration);
                assert_eq!(e.name.as_deref(), Some("RscText"));
            },
            _ => panic!("Second element should be a forward declaration"),
        }

        // Third element should be the enum
        match &elements[2] {
            CodeEntry::Element(e) => {
                assert_eq!(e.element_type, CodeElementType::Enum);
            },
            _ => panic!("Third element should be an enum"),
        }

        // Fourth element should be the class
        match &elements[3] {
            CodeEntry::Class(c) => {
                assert_eq!(c.name(), "Vehicle");
                assert_eq!(c.properties.len(), 1);
                let max_speed = c.get_property("maxSpeed").unwrap();
                assert!(matches!(max_speed.value, PropertyValue::Number(100.0)));
            },
            _ => panic!("Fourth element should be a class"),
        }

        // Fifth element should be the delete statement
        match &elements[4] {
            CodeEntry::Element(e) => {
                assert_eq!(e.element_type, CodeElementType::Delete);
                assert_eq!(e.name.as_deref(), Some("OldClass"));
            },
            _ => panic!("Fifth element should be a delete statement"),
        }
    }
} 