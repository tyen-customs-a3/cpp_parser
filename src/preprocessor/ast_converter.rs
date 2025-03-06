use pest::iterators::{Pair, Pairs};
use super::parser::{ConfigParser, Rule};
use super::types::{CodeElement, CodeElementType};

/// Convert a Pest parse tree to a vector of CodeElement structures
pub fn convert_to_code_elements(pairs: Pairs<Rule>) -> Vec<CodeElement> {
    let mut elements = Vec::new();
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                // Process the file contents
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::enum_block => {
                            elements.push(convert_enum(inner_pair));
                        },
                        Rule::class_declaration => {
                            elements.push(convert_class(inner_pair));
                        },
                        Rule::forward_declaration => {
                            elements.push(convert_forward_declaration(inner_pair));
                        },
                        Rule::define_statement => {
                            elements.push(convert_define(inner_pair));
                        },
                        Rule::delete_statement => {
                            elements.push(convert_delete(inner_pair));
                        },
                        _ => {}
                    }
                }
            },
            Rule::enum_block => {
                elements.push(convert_enum(pair));
            },
            Rule::class_declaration => {
                elements.push(convert_class(pair));
            },
            Rule::forward_declaration => {
                elements.push(convert_forward_declaration(pair));
            },
            Rule::define_statement => {
                elements.push(convert_define(pair));
            },
            Rule::delete_statement => {
                elements.push(convert_delete(pair));
            },
            _ => {}
        }
    }
    
    elements
}

/// Convert an enum block to a CodeElement
fn convert_enum(pair: Pair<Rule>) -> CodeElement {
    let content = pair.as_str().to_string();
    let span = pair.as_span();
    
    CodeElement::new(
        CodeElementType::Enum,
        None,
        None,
        content,
        span.start(),
        span.end()
    )
}

/// Convert a class declaration to a CodeElement with nested elements
fn convert_class(pair: Pair<Rule>) -> CodeElement {
    let content = pair.as_str().to_string();
    let span = pair.as_span();
    let mut class_name = None;
    let mut parent_class = None;
    let mut nested_elements = Vec::new();
    
    // Extract the class name, parent class, and nested classes
    let mut inner_pairs = pair.into_inner();
    
    // First element should be the class name
    if let Some(name_pair) = inner_pairs.next() {
        if name_pair.as_rule() == Rule::identifier {
            class_name = Some(name_pair.as_str().to_string());
        }
    }
    
    // Check for inheritance
    if let Some(next_pair) = inner_pairs.next() {
        if next_pair.as_rule() == Rule::inheritance {
            // Extract parent class name
            let parent_name = next_pair.into_inner().next().unwrap().as_str().to_string();
            parent_class = Some(parent_name);
        } else if next_pair.as_rule() == Rule::class_content {
            // Process class content
            nested_elements.extend(process_class_content(next_pair));
        }
    }
    
    // If we didn't find inheritance, the next pair should be class content
    if nested_elements.is_empty() {
        if let Some(content_pair) = inner_pairs.next() {
            if content_pair.as_rule() == Rule::class_content {
                nested_elements.extend(process_class_content(content_pair));
            }
        }
    }
    
    // Create the class element
    let mut class_element = CodeElement::new(
        CodeElementType::Class,
        class_name,
        parent_class,
        content,
        span.start(),
        span.end()
    );
    
    // Add nested elements
    for nested in nested_elements {
        class_element.add_nested_element(nested);
    }
    
    class_element
}

/// Process the content of a class and extract nested class declarations
fn process_class_content(pair: Pair<Rule>) -> Vec<CodeElement> {
    let mut nested_elements = Vec::new();
    
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::class_declaration => {
                nested_elements.push(convert_class(inner_pair));
            },
            _ => {}
        }
    }
    
    nested_elements
}

/// Convert a forward declaration to a CodeElement
fn convert_forward_declaration(pair: Pair<Rule>) -> CodeElement {
    let content = pair.as_str().to_string();
    let span = pair.as_span();
    let mut class_name = None;
    
    // Extract the class name
    let inner_pairs = pair.into_inner();
    for inner_pair in inner_pairs {
        if inner_pair.as_rule() == Rule::identifier {
            class_name = Some(inner_pair.as_str().to_string());
            break;
        }
    }
    
    CodeElement::new(
        CodeElementType::ForwardDeclaration,
        class_name,
        None,
        content,
        span.start(),
        span.end()
    )
}

/// Convert a define statement to a CodeElement
fn convert_define(pair: Pair<Rule>) -> CodeElement {
    let content = pair.as_str().to_string();
    let span = pair.as_span();
    
    // Extract the define name (first identifier after #define)
    let define_str = content.trim();
    let parts: Vec<&str> = define_str.splitn(3, ' ').collect();
    
    let name = if parts.len() >= 2 {
        Some(parts[1].to_string())
    } else {
        None
    };
    
    CodeElement::new(
        CodeElementType::Define,
        name,
        None,
        content,
        span.start(),
        span.end()
    )
}

/// Convert a delete statement to a CodeElement
fn convert_delete(pair: Pair<Rule>) -> CodeElement {
    let content = pair.as_str().to_string();
    let span = pair.as_span();
    let mut class_name = None;
    
    // Extract the class name
    let inner_pairs = pair.into_inner();
    for inner_pair in inner_pairs {
        if inner_pair.as_rule() == Rule::identifier {
            class_name = Some(inner_pair.as_str().to_string());
            break;
        }
    }
    
    CodeElement::new(
        CodeElementType::Delete,
        class_name,
        None,
        content,
        span.start(),
        span.end()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parser::parse;
    
    #[test]
    fn test_convert_define() {
        let input = "#define VERSION 1.0";
        let pairs = parse(input).unwrap();
        
        // Debug: print the parse result
        println!("Parse result for: {}", input);
        for pair in pairs.clone() {
            println!("Rule: {:?}, Text: {}", pair.as_rule(), pair.as_str());
            
            // Print inner pairs if any
            let inner: Vec<_> = pair.into_inner().collect();
            if !inner.is_empty() {
                println!("Inner pairs:");
                for inner_pair in inner {
                    println!("  Inner rule: {:?}, Text: {}", inner_pair.as_rule(), inner_pair.as_str());
                }
            }
        }
        
        let elements = convert_to_code_elements(pairs);
        
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, CodeElementType::Define);
        assert_eq!(elements[0].name, Some("VERSION".to_string()));
    }
    
    #[test]
    fn test_convert_enum() {
        let input = r#"enum { RED = 0, GREEN = 1, BLUE = 2 };"#;
        let pairs = parse(input).unwrap();
        let elements = convert_to_code_elements(pairs);
        
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, CodeElementType::Enum);
        assert!(elements[0].content.contains("RED"));
        assert!(elements[0].content.contains("GREEN"));
        assert!(elements[0].content.contains("BLUE"));
    }
    
    #[test]
    fn test_convert_class() {
        let input = r#"class Vehicle { displayName = "Vehicle"; maxSpeed = 100; };"#;
        let pairs = parse(input).unwrap();
        let elements = convert_to_code_elements(pairs);
        
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, CodeElementType::Class);
        assert_eq!(elements[0].name, Some("Vehicle".to_string()));
        assert!(elements[0].content.contains("displayName"));
        assert!(elements[0].content.contains("maxSpeed"));
    }
    
    #[test]
    fn test_convert_nested_classes() {
        let input = r#"
            class Outer {
                class Inner {
                    value = 1;
                };
            };
        "#;
        let pairs = parse(input).unwrap();
        let elements = convert_to_code_elements(pairs);
        
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, CodeElementType::Class);
        assert_eq!(elements[0].name, Some("Outer".to_string()));
        assert_eq!(elements[0].nested_elements.len(), 1);
        assert_eq!(elements[0].nested_elements[0].name, Some("Inner".to_string()));
    }
    
    #[test]
    fn test_convert_forward_declaration() {
        let input = "class Vehicle;";
        let pairs = parse(input).unwrap();
        let elements = convert_to_code_elements(pairs);
        
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, CodeElementType::ForwardDeclaration);
        assert_eq!(elements[0].name, Some("Vehicle".to_string()));
    }
    
    #[test]
    fn test_convert_delete() {
        let input = "delete Vehicle;";
        let pairs = parse(input).unwrap();
        let elements = convert_to_code_elements(pairs);
        
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, CodeElementType::Delete);
        assert_eq!(elements[0].name, Some("Vehicle".to_string()));
    }
} 