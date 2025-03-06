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
    }

    #[test]
    fn test_parse_array_property() {
        let input = r#"linkedItems[] = {"ItemWatch", "ItemMap"};"#;
        let pairs = PropertyParser::parse(Rule::file, input).unwrap();
        let property_pair = pairs.into_iter().next().unwrap().into_inner().next().unwrap();
        
        let property = parse_property(property_pair, input);
        assert_eq!(property.name, "linkedItems");
        
        if let PropertyValue::Array(values) = property.value {
            assert_eq!(values.len(), 2);
            assert_eq!(values[0], PropertyValue::String("ItemWatch".to_string()));
            assert_eq!(values[1], PropertyValue::String("ItemMap".to_string()));
        } else {
            panic!("Expected array value");
        }
    }
} 