pub mod block_properties;
mod parser;
mod property_extractor;
mod property_parser;
mod value_parser;

// Re-export the main parsing functions
pub use parser::{PropertyParser, parse_properties, parse_properties_from_element};
pub use parser::Rule;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PropertyValue;

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
            assert_eq!(values.len(), 3);
            
            // Check the LIST_ macros
            assert_eq!(values[0], PropertyValue::ListMacro(2, "ACE_fieldDressing".to_string()));
            assert_eq!(values[1], PropertyValue::ListMacro(10, "ACE_packingBandage".to_string()));
            assert_eq!(values[2], PropertyValue::String("ACRE_PRC343".to_string()));
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