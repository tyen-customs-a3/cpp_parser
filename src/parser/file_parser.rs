use std::fs;
use crate::parser::error::ParseError;
use crate::parser::ast::{Property, Directive, CppFile};
use crate::parser::parser::{CppParser, Rule};
use crate::parser::class_parser::parse_class;
use crate::parser::property_parser::parse_property;
use crate::parser::directive_parser::{parse_define, parse_include};
use pest::Parser;

/// Parse a CPP file from a string
pub fn parse_cpp(content: &str) -> Result<CppFile, ParseError> {
    let mut result = CppFile {
        directives: Vec::new(),
        classes: Vec::new(),
        properties: Vec::new(),
    };
    
    let pairs = CppParser::parse(Rule::file, content)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::class_def => {
                            result.classes.push(parse_class(inner_pair)?);
                        }
                        Rule::property => {
                            let (name, value) = parse_property(inner_pair)?;
                            result.properties.push(Property { name, value });
                        }
                        Rule::define_directive => {
                            result.directives.push(parse_define(inner_pair));
                        }
                        Rule::include_directive => {
                            result.directives.push(parse_include(inner_pair));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    
    Ok(result)
}

/// Helper function to parse a CPP file from a file path
pub fn parse_cpp_file(path: &str) -> Result<CppFile, ParseError> {
    let content = fs::read_to_string(path)?;
    parse_cpp(&content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Value;

    #[test]
    fn test_parse_cpp() {
        let content = r#"
        #define TEST
        #include "test.h"
        class Test { prop = 42; };
        "#;
        let cpp_file = parse_cpp(content).unwrap();
        assert_eq!(cpp_file.directives.len(), 2);
        assert_eq!(cpp_file.classes.len(), 1);
        assert_eq!(cpp_file.properties.len(), 0);
    }

    #[test]
    fn test_parse_cpp_file() {
        let path = "tests/data/complex_config.cpp";
        let cpp_file = parse_cpp_file(path).unwrap();
        assert!(cpp_file.classes.len() > 0);
    }
}