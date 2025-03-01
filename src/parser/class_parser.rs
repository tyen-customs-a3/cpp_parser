use std::collections::HashMap;
use crate::parser::ast::{Class, Property, Value};
use crate::parser::error::ParseError;
use crate::parser::parser::Rule;
use crate::parser::property_parser::parse_property;

/// Parse a class definition from a Pest pair
pub fn parse_class(pair: pest::iterators::Pair<Rule>) -> Result<Class, ParseError> {
    let mut class_name = String::new();
    let mut parent_name = None;
    let mut properties = HashMap::new();
    let mut nested_classes = Vec::new();
    
    let mut pairs = pair.into_inner();
    
    // Parse class header
    if let Some(header) = pairs.next() {
        if header.as_rule() == Rule::class_header {
            let mut header_pairs = header.into_inner();
            
            if let Some(name_pair) = header_pairs.next() {
                class_name = name_pair.as_str().to_string();
            }
            
            if let Some(parent_pair) = header_pairs.next() {
                parent_name = Some(parent_pair.as_str().to_string());
            }
        }
    }
    
    // Parse class body if it exists
    for item in pairs {
        match item.as_rule() {
            Rule::class_body => {
                for body_item in item.into_inner() {
                    match body_item.as_rule() {
                        Rule::property => {
                            let (name, value) = parse_property(body_item)?;
                            properties.insert(name.clone(), Property { name, value });
                        }
                        Rule::class_def => {
                            let nested_class = parse_class(body_item)?;
                            nested_classes.push(nested_class);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    
    let mut class = Class::new(class_name, parent_name);
    class.properties = properties;
    class.nested_classes = nested_classes;
    
    Ok(class)
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use pest::Parser;
    use crate::parser::parser::{CppParser, Rule};
    use super::parse_class;

    #[test]
    fn test_parse_class() {
        let pairs = CppParser::parse(Rule::class_def, "class Test { prop = 42; };").unwrap();
        let class = parse_class(pairs.peek().unwrap()).unwrap();
        assert_eq!(class.name, "Test");
        assert_eq!(class.properties.get("prop").unwrap().value, Value::Number(42.0));
    }
}