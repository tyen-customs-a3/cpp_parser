use crate::parser::ast::{Property, Value};
use crate::parser::error::ParseError;
use crate::parser::value_parser::parse_value;
use crate::parser::parser::Rule;

/// Parse a property from a Pest pair
pub fn parse_property(pair: pest::iterators::Pair<Rule>) -> Result<(String, Value), ParseError> {
    let mut property_name = String::new();
    let mut property_value = None;
    
    let mut pairs = pair.into_inner();
    
    if let Some(name_pair) = pairs.next() {
        property_name = name_pair.as_str().to_string();
    }
    
    if let Some(value_pair) = pairs.next() {
        property_value = Some(parse_value(value_pair)?);
    }
    
    let value = property_value.ok_or_else(|| {
        ParseError::ValueError(format!("Missing value for property {}", property_name))
    })?;
    
    Ok((property_name, value))
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use pest::Parser;
    use crate::parser::parser::{CppParser, Rule};
    use crate::parser::property_parser::parse_property;

    #[test]
    fn test_parse_property() {
        let pairs = CppParser::parse(Rule::property, "prop = 42;").unwrap();
        let (name, value) = parse_property(pairs.peek().unwrap()).unwrap();
        assert_eq!(name, "prop");
        assert_eq!(value, Value::Number(42.0));
    }
}