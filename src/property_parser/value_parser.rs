use crate::models::PropertyValue;
use super::Rule;

/// Parse a simple value
pub fn parse_simple_value(pair: pest::iterators::Pair<Rule>) -> PropertyValue {
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
        let value = text[value_start+1..value_end].to_string();
        PropertyValue::ListMacro(count, value)
    } else {
        // Fallback if we can't parse the value
        PropertyValue::String(text.to_string())
    }
}

/// Parse an array value
pub fn parse_array(pair: pest::iterators::Pair<Rule>) -> PropertyValue {
    let mut values = Vec::new();
    
    for element in pair.into_inner() {
        match element.as_rule() {
            Rule::array_element => {
                let element_inner = element.into_inner().next().unwrap();
                match element_inner.as_rule() {
                    Rule::list_macro => {
                        values.push(parse_list_macro(element_inner));
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
    use pest::Parser;
    use super::super::parser::PropertyParser;

    #[test]
    fn test_parse_simple_values() {
        let inputs = vec![
            (r#""test string""#, PropertyValue::String("test string".to_string())),
            ("123.45", PropertyValue::Number(123.45)),
            ("true", PropertyValue::Boolean(true)),
            ("false", PropertyValue::Boolean(false)),
        ];

        for (input, expected) in inputs {
            let pairs = PropertyParser::parse(Rule::simple_value, input).unwrap();
            let value = parse_simple_value(pairs.into_iter().next().unwrap());
            assert_eq!(value, expected);
        }
    }

    #[test]
    fn test_parse_list_macro() {
        let input = r#"LIST_2("test_value")"#;
        let pairs = PropertyParser::parse(Rule::list_macro, input).unwrap();
        let value = parse_list_macro(pairs.into_iter().next().unwrap());
        assert!(matches!(value, PropertyValue::ListMacro(2, s) if s == "test_value"));
    }

    #[test]
    fn test_parse_array_with_list_macro() {
        let input = r#"{ LIST_2("item1"), "item2", "item3" }"#;
        let pairs = PropertyParser::parse(Rule::array_value, input).unwrap();
        if let PropertyValue::Array(values) = parse_array(pairs.into_iter().next().unwrap()) {
            assert_eq!(values.len(), 3);
            assert!(matches!(&values[0], PropertyValue::ListMacro(2, s) if s == "item1"));
            assert!(matches!(&values[1], PropertyValue::String(s) if s == "item2"));
            assert!(matches!(&values[2], PropertyValue::String(s) if s == "item3"));
        } else {
            panic!("Expected array value");
        }
    }
} 