use pest::Parser;
use pest_derive::Parser;

use crate::models::Property;
use crate::preprocessor::CodeElement;

use super::property_extractor::extract_properties_from_content;
use super::property_parser::parse_property;

#[derive(Parser)]
#[grammar = "property_parser/grammar.pest"]
pub struct PropertyParser;

/// Parse properties from a preprocessed CodeElement
pub fn parse_properties_from_element(element: &CodeElement) -> Result<Vec<Property>, pest::error::Error<Rule>> {
    // Extract the content from the element
    let content = &element.content;
    
    // Remove class declarations but keep properties
    let cleaned_content = extract_properties_from_content(content);
    
    parse_properties(&cleaned_content)
}

/// Parse properties from a string (legacy support)
pub fn parse_properties(input: &str) -> Result<Vec<Property>, pest::error::Error<Rule>> {
    let pairs = PropertyParser::parse(Rule::file, input)?;
    let mut properties = Vec::new();

    // Only process property rules, ignore everything else
    for pair in pairs.into_iter().next().unwrap().into_inner() {
        if let Rule::property = pair.as_rule() {
            let property = parse_property(pair, input);
            properties.push(property);
        }
    }

    Ok(properties)
} 