use crate::class_parser::{Class as ParserClass, parse_classes};
use crate::class_parser::class_model::{Class, ClassCollection};

/// Parse a string into a collection of classes
pub fn parse_classes_to_collection(input: &str) -> ClassCollection {
    let parser_classes = parse_classes(input);
    let mut collection = ClassCollection::new();
    
    for parser_class in parser_classes {
        if let Some(class) = parse_class_from_parser_class(&parser_class, input) {
            collection.add_class(class);
        }
    }
    
    collection
}

/// Convert a parser class to a model class
fn parse_class_from_parser_class(parser_class: &ParserClass, input: &str) -> Option<Class> {
    let name = parser_class.name.clone()?;
    let mut class = Class::new(
        name, 
        parser_class.parent.clone(), 
        parser_class.content.clone(),
        parser_class.start_pos,
        parser_class.end_pos
    );
    
    // Parse nested classes
    for child_parser_class in &parser_class.children {
        if let Some(nested_class) = parse_class_from_parser_class(child_parser_class, input) {
            class.add_nested_class(nested_class);
        }
    }
    
    Some(class)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_class() {
        let input = r#"
        class Test {
            value = 123;
            name = "Test Name";
        }
        "#;
        
        let collection = parse_classes_to_collection(input);
        assert_eq!(collection.classes.len(), 1);
        
        let class = &collection.classes[0];
        assert_eq!(class.name, "Test");
        assert_eq!(class.parent, None);
        assert!(class.content.contains("value = 123"));
        assert!(class.content.contains("name = \"Test Name\""));
    }
    
    #[test]
    fn test_parse_inheritance() {
        let input = r#"
        class Parent {
            shared = "Shared Value";
            parentOnly = 100;
        }
        
        class Child : Parent {
            childOnly = "Child Value";
            shared = "Overridden Value";
        }
        "#;
        
        let collection = parse_classes_to_collection(input);
        assert_eq!(collection.classes.len(), 2);
        
        // Check parent class
        let parent = collection.get_class("Parent").unwrap();
        assert_eq!(parent.name, "Parent");
        assert_eq!(parent.parent, None);
        
        // Check child class
        let child = collection.get_class("Child").unwrap();
        assert_eq!(child.name, "Child");
        assert_eq!(child.parent, Some("Parent".to_string()));
        
        // Check content
        assert!(parent.content.contains("shared = \"Shared Value\""));
        assert!(child.content.contains("shared = \"Overridden Value\""));
        
        // Check children lookup
        let children = collection.get_children("Parent");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "Child");
    }
    
    #[test]
    fn test_parse_nested_classes() {
        let input = r#"
        class Outer {
            outerValue = 100;
            
            class Inner {
                innerValue = 200;
            };
        }
        "#;
        
        let collection = parse_classes_to_collection(input);
        assert_eq!(collection.classes.len(), 1);
        
        let outer = &collection.classes[0];
        assert_eq!(outer.name, "Outer");
        assert_eq!(outer.nested_classes.len(), 1);
        
        let inner = &outer.nested_classes[0];
        assert_eq!(inner.name, "Inner");
        assert!(inner.content.contains("innerValue = 200"));
    }
} 