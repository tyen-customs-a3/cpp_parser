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

/// Parse a LIST_ macro into a PropertyValue
fn parse_list_macro(pair: pest::iterators::Pair<Rule>) -> PropertyValue {
    let text = pair.as_str();
    
    // Extract the count and value from the LIST_N("value") format
    let count_start = text.find('_').unwrap_or(0) + 1;
    let count_end = text.find('(').unwrap_or(text.len());
    let count_str = &text[count_start..count_end];
    
    let count = count_str.parse::<usize>().unwrap_or(0);
    
    // Extract the string value
    let value_start = text.find('"').unwrap_or(0);
    let value_end = text.rfind('"').unwrap_or(text.len());
    
    if value_start < value_end && value_start > 0 {
        let value = text[value_start..=value_end].to_string();
        
        // Create a vector with the value repeated count times
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(PropertyValue::String(value.clone()));
        }
        
        PropertyValue::Array(values)
    } else {
        // Fallback if we can't parse the value
        PropertyValue::String(text.to_string())
    }
}

/// Parse an array value
fn parse_array(pair: pest::iterators::Pair<Rule>) -> PropertyValue {
    let mut values = Vec::new();
    
    for element in pair.into_inner() {
        match element.as_rule() {
            Rule::array_element => {
                let element_inner = element.into_inner().next().unwrap();
                match element_inner.as_rule() {
                    Rule::list_macro => {
                        // Handle LIST_ macro by expanding it
                        if let PropertyValue::Array(list_values) = parse_list_macro(element_inner) {
                            values.extend(list_values);
                        }
                    },
                    _ => {
                        values.push(parse_simple_value(element_inner));
                    }
                }
            },
            _ => {}
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

    #[test]
    fn test_list_macro_property() {
        let input = r#"
        items[] = {
            LIST_2("ACE_fieldDressing"),
            LIST_10("ACE_packingBandage"),
            "ACRE_PRC343"
        };
        "#;
        
        let result = parse_properties(input);
        assert!(result.is_ok());
        
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        
        if let PropertyValue::Array(values) = &properties[0].value {
            // We expect 2 + 10 + 1 = 13 values
            assert_eq!(values.len(), 13);
            
            // Check that the first two values are the same (from LIST_2)
            if let (PropertyValue::String(val1), PropertyValue::String(val2)) = (&values[0], &values[1]) {
                assert_eq!(val1, val2);
                assert!(val1.contains("ACE_fieldDressing"));
            } else {
                panic!("Expected string values");
            }
            
            // Check that the next 10 values are the same (from LIST_10)
            let packing_bandage_count = values.iter().filter(|v| {
                if let PropertyValue::String(s) = v {
                    s.contains("ACE_packingBandage")
                } else {
                    false
                }
            }).count();
            assert_eq!(packing_bandage_count, 10);
            
            // Check the last value
            if let PropertyValue::String(val) = &values[12] {
                assert!(val.contains("ACRE_PRC343"));
            } else {
                panic!("Expected string value");
            }
        } else {
            panic!("Expected array value");
        }
    }

    #[test]
    fn test_array_append_property() {
        let input = r#"
        items[] = {
            "Item1",
            "Item2"
        };
        
        items[] += {
            "Item3",
            "Item4"
        };
        "#;
        
        let result = parse_properties(input);
        assert!(result.is_ok());
        
        let properties = result.unwrap();
        // We expect at least one property
        assert!(!properties.is_empty(), "Expected at least one property");
        
        // Check that we found the items property
        let items_props = properties.iter().filter(|p| p.name == "items").count();
        assert!(items_props > 0, "Expected to find at least one items property");
        
        // Check that at least one property is an array
        let array_props = properties.iter().filter(|p| 
            matches!(p.value, PropertyValue::Array(_))
        ).count();
        assert!(array_props > 0, "Expected at least one array property");
    }
} 