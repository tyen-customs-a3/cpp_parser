use crate::models::Property;
use crate::property_parser::parse_properties;
use crate::preprocessor::{CodeElement, CodeElementType};

/// Parse properties from a preprocessed class element
pub fn parse_block_properties(element: &CodeElement) -> Result<Vec<Property>, pest::error::Error<crate::property_parser::Rule>> {
    // Parse properties from the element's content
    parse_properties(&element.content)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_block_properties() {
        let input = r#"
        displayName = "Vehicle";
        maxSpeed = 100;
        "#;
        
        let element = CodeElement {
            content: input.to_string(),
            name: Some("Vehicle".to_string()),
            element_type: CodeElementType::Class,
            start_pos: 0,
            end_pos: input.len(),
            nested_elements: vec![],
            parent: None,
        };
        
        let properties = parse_block_properties(&element).unwrap();
        
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name, "displayName");
        assert_eq!(properties[1].name, "maxSpeed");
    }
} 