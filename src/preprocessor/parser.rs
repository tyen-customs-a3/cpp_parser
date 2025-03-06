use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

/// ConfigParser is the main parser for the config files
#[derive(Parser)]
#[grammar = "preprocessor/config.pest"]
pub struct ConfigParser;

/// Parse a config file into a Pest parse tree
pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    ConfigParser::parse(Rule::file, input)
}

/// Helper function to extract the string value of an identifier
pub fn get_identifier(pair: &Pair<Rule>) -> String {
    pair.as_str().to_string()
}

/// Helper function to extract the value of a string literal
pub fn get_string_value(pair: &Pair<Rule>) -> String {
    // Remove surrounding quotes
    let raw = pair.as_str();
    let without_quotes = &raw[1..raw.len() - 1];
    without_quotes.replace("\\\"", "\"")
}

/// Helper function to extract a numeric value
pub fn get_number_value(pair: &Pair<Rule>) -> f64 {
    pair.as_str().parse().unwrap_or(0.0)
}

/// Helper function to check if a pair matches a specific rule
pub fn is_rule(pair: &Pair<Rule>, rule: Rule) -> bool {
    pair.as_rule() == rule
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let input = r#"
            #define VERSION 1.0
            
            class Vehicle {
                displayName = "Vehicle";
                maxSpeed = 100;
            };
        "#;
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse basic input");
    }
    
    #[test]
    fn test_parse_enum() {
        let input = r#"
            enum {
                RED = 0,
                GREEN = 1,
                BLUE = 2
            };
        "#;
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse enum");
    }
    
    #[test]
    fn test_parse_forward_declaration() {
        let input = "class Vehicle;";
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse forward declaration");
    }
    
    #[test]
    fn test_parse_delete() {
        let input = "delete Vehicle;";
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse delete statement");
    }
    
    #[test]
    fn test_parse_nested_classes() {
        let input = r#"
            class Outer {
                class Inner {
                    value = 1;
                };
            };
        "#;
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse nested classes");
    }
    
    #[test]
    fn test_parse_numeric_property() {
        let input = r#"
            class Vehicle {
                maxSpeed = 100;
            };
        "#;
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse numeric property");
        
        fn print_pairs(pairs: Pairs<Rule>, depth: usize) {
            for pair in pairs {
                let indent = "  ".repeat(depth);
                println!("{}Rule: {:?} - Text: {:?}", indent, pair.as_rule(), pair.as_str());
                print_pairs(pair.into_inner(), depth + 1);
            }
        }
        
        print_pairs(result.unwrap(), 0);
    }
} 