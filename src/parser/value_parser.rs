use crate::parser::ast::Value;
use crate::parser::error::ParseError;
use crate::parser::parser::Rule;

/// Parse a value from a Pest pair
pub fn parse_value(pair: pest::iterators::Pair<Rule>) -> Result<Value, ParseError> {
    match pair.as_rule() {
        Rule::value => {
            let inner = pair.into_inner().next().unwrap();
            parse_value(inner)
        }
        Rule::string_literal => {
            let text = pair.as_str();
            let content = &text[1..text.len()-1]
                .replace("\\\"", "\"");
            Ok(Value::String(content.to_string()))
        }
        Rule::number_literal => {
            let text = pair.as_str();
            let num = text.parse::<f64>().map_err(|_| {
                ParseError::ValueError(format!("Failed to parse number: {}", text))
            })?;
            Ok(Value::Number(num))
        }
        Rule::array_literal => {
            let mut values = Vec::new();
            for value_pair in pair.into_inner() {
                let value = match value_pair.as_rule() {
                    Rule::identifier => Value::Identifier(value_pair.as_str().to_string()),
                    _ => parse_value(value_pair)?,
                };
                values.push(value);
            }
            Ok(Value::Array(values))
        }
        Rule::identifier => {
            Ok(Value::Identifier(pair.as_str().to_string()))
        }
        _ => Err(ParseError::ValueError(format!("Unexpected rule: {:?}", pair.as_rule())))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use pest::Parser;
    use crate::parser::parser::{CppParser, Rule};
    use crate::parser::value_parser::parse_value;

    #[test]
    fn test_parse_value_string() {
        let pairs = CppParser::parse(Rule::string_literal, "\"test\"").unwrap();
        let value = parse_value(pairs.peek().unwrap()).unwrap();
        assert_eq!(value, Value::String("test".to_string()));
    }

    #[test]
    fn test_parse_value_number() {
        let pairs = CppParser::parse(Rule::number_literal, "42").unwrap();
        let value = parse_value(pairs.peek().unwrap()).unwrap();
        assert_eq!(value, Value::Number(42.0));
    }

    #[test]
    fn test_parse_value_array() {
        let pairs = CppParser::parse(Rule::array_literal, "{1, 2}").unwrap();
        let value = parse_value(pairs.peek().unwrap()).unwrap();
        assert_eq!(value, Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]));
    }

    #[test]
    fn test_parse_value_identifier() {
        let pairs = CppParser::parse(Rule::identifier, "id").unwrap();
        let value = parse_value(pairs.peek().unwrap()).unwrap();
        assert_eq!(value, Value::Identifier("id".to_string()));
    }
}