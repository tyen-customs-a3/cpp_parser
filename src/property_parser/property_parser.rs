use crate::models::{Property, PropertyValue};
use super::Rule;
use super::value_parser::{parse_simple_value, parse_array};

/// Parse a single property
pub fn parse_property(pair: pest::iterators::Pair<Rule>, input: &str) -> Property {
    let span = pair.as_span();
    let start_pos = span.start();
    let end_pos = span.end();
    
    let mut name = String::new();
    let mut value = PropertyValue::String(String::new());
    let mut is_append = false;
    
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
                    // First remove the += operator, then remove the [] suffix
                    // This ensures we handle both cases: "identifier[]+" and "identifier[] +="
                    name = name_pair.as_str()
                        .trim_end_matches("+=")  // Remove += first
                        .trim()                  // Remove any whitespace
                        .trim_end_matches("[]")  // Then remove []
                        .to_string();
                }
                
                // Next part is the array value
                if let Some(array_pair) = parts.next() {
                    value = parse_array(array_pair);
                }
                
                is_append = true;
            }
            _ => {}
        }
    }
    
    let mut property = Property::new(name, value, start_pos, end_pos);
    property.is_append = is_append;
    property
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use super::super::parser::PropertyParser;

    #[test]
    fn test_parse_simple_property() {
        let input = r#"displayName = "Unarmed";"#;
        let pairs = PropertyParser::parse(Rule::file, input).unwrap();
        let property_pair = pairs.into_iter().next().unwrap().into_inner().next().unwrap();
        
        let property = parse_property(property_pair, input);
        assert_eq!(property.name, "displayName");
        assert_eq!(property.value, PropertyValue::String("Unarmed".to_string()));
        assert!(!property.is_append);
    }

    #[test]
    fn test_parse_array_append_property() {
        let input = r#"backpackItems[] += { "item1", "item2" };"#;
        let pairs = PropertyParser::parse(Rule::file, input).unwrap();
        let property_pair = pairs.into_iter().next().unwrap().into_inner().next().unwrap();
        
        let property = parse_property(property_pair, input);
        
        assert_eq!(property.name, "backpackItems");
        if let PropertyValue::Array(values) = &property.value {
            assert_eq!(values.len(), 2);
            assert!(matches!(&values[0], PropertyValue::String(s) if s == "item1"));
            assert!(matches!(&values[1], PropertyValue::String(s) if s == "item2"));
        } else {
            panic!("Expected array value");
        }
        assert!(property.is_append);
    }

    #[test]
    fn test_parse_array_append_property_no_whitespace() {
        let input = r#"backpackItems[]+={ "item1", "item2" };"#;
        let pairs = PropertyParser::parse(Rule::file, input).unwrap();
        let property_pair = pairs.into_iter().next().unwrap().into_inner().next().unwrap();
        
        let property = parse_property(property_pair, input);
        
        assert_eq!(property.name, "backpackItems");
        if let PropertyValue::Array(values) = &property.value {
            assert_eq!(values.len(), 2);
            assert!(matches!(&values[0], PropertyValue::String(s) if s == "item1"));
            assert!(matches!(&values[1], PropertyValue::String(s) if s == "item2"));
        } else {
            panic!("Expected array value");
        }
        assert!(property.is_append);
    }
} 