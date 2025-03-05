use pest::Parser;
use pest_derive::Parser;

use crate::models::{Property, PropertyValue};

pub mod block_properties;

#[derive(Parser)]
#[grammar = "property_parser/grammar.pest"]
pub struct PropertyParser;

/// Parse properties from a string
pub fn parse_properties(input: &str) -> Result<Vec<Property>, pest::error::Error<Rule>> {
    // Clean up the input by removing any nested classes
    let cleaned_input = remove_nested_classes(input);
    
    let pairs = PropertyParser::parse(Rule::file, &cleaned_input)?;
    let mut properties = Vec::new();

    for pair in pairs.into_iter().next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::property => {
                let property = parse_property(pair, &cleaned_input);
                properties.push(property);
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(properties)
}

/// Remove nested classes from the input
fn remove_nested_classes(input: &str) -> String {
    let mut result = String::new();
    let mut in_class = false;
    let mut brace_count = 0;
    
    let mut lines = input.lines();
    
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("class ") && trimmed.contains("{") {
            in_class = true;
            brace_count = 1;
            continue;
        }
        
        if in_class {
            if trimmed.contains("{") {
                brace_count += 1;
            }
            if trimmed.contains("}") {
                brace_count -= 1;
                if brace_count == 0 {
                    in_class = false;
                }
            }
            continue;
        }
        
        result.push_str(line);
        result.push('\n');
    }
    
    result
}

/// Parse a single property
fn parse_property(pair: pest::iterators::Pair<Rule>, input: &str) -> Property {
    let span = pair.as_span();
    let start_pos = span.start();
    let end_pos = span.end();
    
    let mut name = String::new();
    let mut value = PropertyValue::String(String::new());
    
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::simple_property => {
                let mut parts = inner_pair.into_inner();
                
                // First part is the identifier
                if let Some(name_pair) = parts.next() {
                    name = name_pair.as_str().to_string();
                }
                
                // Skip the assignment operator
                if let Some(_) = parts.next() {
                }
                
                // Next part is the value
                if let Some(value_pair) = parts.next() {
                    value = parse_simple_value(value_pair);
                }
            }
            Rule::array_property => {
                let mut parts = inner_pair.into_inner();
                
                // First part is the array identifier
                if let Some(name_pair) = parts.next() {
                    // Remove the [] suffix
                    name = name_pair.as_str().trim_end_matches("[]").to_string();
                }
                
                // Skip the assignment operator
                if let Some(_) = parts.next() {
                }
                
                // Next part is the array value
                if let Some(array_pair) = parts.next() {
                    value = parse_array(array_pair);
                }
            }
            Rule::array_append_property => {
                let mut parts = inner_pair.into_inner();
                
                // First part is the array identifier with +=
                if let Some(name_pair) = parts.next() {
                    // Remove the []+ suffix
                    name = name_pair.as_str().trim_end_matches("[]+").to_string();
                }
                
                // Next part is the array value
                if let Some(array_pair) = parts.next() {
                    value = parse_array(array_pair);
                }
            }
            _ => {}
        }
    }
    
    Property::new(name, value, start_pos, end_pos)
}

/// Parse a simple value
fn parse_simple_value(pair: pest::iterators::Pair<Rule>) -> PropertyValue {
    match pair.as_rule() {
        Rule::simple_value => {
            // Get the inner value
            let inner = pair.into_inner().next().unwrap();
            
            match inner.as_rule() {
                Rule::string_literal => {
                    // Remove the quotes
                    let s = inner.as_str();
                    let s = &s[1..s.len()-1];
                    PropertyValue::String(s.to_string())
                }
                Rule::number => {
                    let n = inner.as_str().parse::<f64>().unwrap_or(0.0);
                    PropertyValue::Number(n)
                }
                Rule::boolean => {
                    let b = inner.as_str() == "true";
                    PropertyValue::Boolean(b)
                }
                Rule::identifier => {
                    PropertyValue::Reference(inner.as_str().to_string())
                }
                _ => {
                    PropertyValue::String(String::new())
                }
            }
        }
        _ => {
            PropertyValue::String(String::new())
        }
    }
}

/// Parse an array value
fn parse_array(pair: pest::iterators::Pair<Rule>) -> PropertyValue {
    let mut values = Vec::new();
    
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::array_element {
            values.push(parse_simple_value(inner_pair.into_inner().next().unwrap()));
        }
    }
    
    PropertyValue::Array(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_property() {
        let input = r#"displayName = "Unarmed";"#;
        let result = parse_properties(input).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "displayName");
        assert_eq!(result[0].value, PropertyValue::String("Unarmed".to_string()));
    }

    #[test]
    fn test_array_property() {
        let input = r#"linkedItems[] = {"ItemWatch", "ItemMap"};"#;
        let result = parse_properties(input).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "linkedItems");
        
        if let PropertyValue::Array(values) = &result[0].value {
            assert_eq!(values.len(), 2);
            assert_eq!(values[0], PropertyValue::String("ItemWatch".to_string()));
            assert_eq!(values[1], PropertyValue::String("ItemMap".to_string()));
        } else {
            panic!("Expected array value");
        }
    }

    #[test]
    fn test_multiple_properties() {
        let input = r#"
        displayName = "Rifleman";
        uniform = "rhs_uniform_g3_m81";
        primaryWeapon = "rhs_weap_m4a1_blockII_KAC";
        "#;
        let result = parse_properties(input).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "displayName");
        assert_eq!(result[1].name, "uniform");
        assert_eq!(result[2].name, "primaryWeapon");
    }
    
    #[test]
    fn test_with_nested_class() {
        let input = r#"
        displayName = "Rifleman";
        uniform = "rhs_uniform_g3_m81";
        
        class Inventory {
            class uniform {
                type = "rhs_uniform_g3_m81";
            };
        };
        
        primaryWeapon = "rhs_weap_m4a1_blockII_KAC";
        "#;
        let result = parse_properties(input).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "displayName");
        assert_eq!(result[1].name, "uniform");
        assert_eq!(result[2].name, "primaryWeapon");
    }
} 