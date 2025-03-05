use crate::class_parser::Class;
use crate::models::Property;
use crate::property_parser::parse_properties;

/// Parse properties from a class
pub fn parse_block_properties(class: &Class, input: &str) -> Result<Vec<Property>, pest::error::Error<crate::property_parser::Rule>> {
    // Get the raw content of the class
    let content = &class.content;
    
    // Parse properties from the content
    parse_properties(content)
}

/// Parse properties from all classes recursively
pub fn parse_all_block_properties(classes: &[Class], input: &str) -> Vec<(String, String, String)> {
    let mut result = Vec::new();
    
    for class in classes {
        if let Some(name) = &class.name {
            if let Ok(properties) = parse_block_properties(class, input) {
                if !properties.is_empty() {
                    for property in properties {
                        // Convert Property to tuple of (name, value as string)
                        let value_str = property.value.to_string();
                        result.push((name.clone(), property.name, value_str));
                    }
                }
            }
            
            // Recursively parse nested classes
            if !class.children.is_empty() {
                let nested_properties = parse_all_block_properties(&class.children, input);
                result.extend(nested_properties);
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class_parser::parse_classes;
    use crate::models::PropertyValue;

    #[test]
    fn test_parse_block_properties() {
        let input = r#"
        class baseMan {
            displayName = "Unarmed";
            uniform = "rhs_uniform_g3_m81";
        };
        "#;
        let classes = parse_classes(input);
        
        assert_eq!(classes.len(), 1);
        let properties = parse_block_properties(&classes[0], input).unwrap();
        
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name, "displayName");
        assert_eq!(properties[0].value, PropertyValue::String("Unarmed".to_string()));
        assert_eq!(properties[1].name, "uniform");
        assert_eq!(properties[1].value, PropertyValue::String("rhs_uniform_g3_m81".to_string()));
    }

    #[test]
    fn test_parse_all_block_properties() {
        let input = r#"
        class baseMan {
            displayName = "Unarmed";
            uniform = "rhs_uniform_g3_m81";
            
            class Inventory {
                class uniform {
                    type = "rhs_uniform_g3_m81";
                };
            };
        };
        
        class rm : baseMan {
            displayName = "Rifleman";
            primaryWeapon = "rhs_weap_m4a1_blockII_KAC";
        };
        "#;
        let classes = parse_classes(input);
        
        let all_properties = parse_all_block_properties(&classes, input);
        
        // We expect 5 entries based on the actual output
        // 2 from baseMan, 1 from uniform, and 2 from rm
        assert_eq!(all_properties.len(), 5);
        
        // Check if properties exist in the result
        let mut found_baseman_displayname = false;
        let mut found_baseman_uniform = false;
        let mut found_uniform_type = false;
        let mut found_rm_displayname = false;
        let mut found_rm_primaryweapon = false;
        
        for prop in &all_properties {
            match (prop.0.as_str(), prop.1.as_str()) {
                ("baseMan", "displayName") => found_baseman_displayname = true,
                ("baseMan", "uniform") => found_baseman_uniform = true,
                ("uniform", "type") => found_uniform_type = true,
                ("rm", "displayName") => found_rm_displayname = true,
                ("rm", "primaryWeapon") => found_rm_primaryweapon = true,
                _ => {}
            }
        }
        
        assert!(found_baseman_displayname, "Missing baseMan.displayName property");
        assert!(found_baseman_uniform, "Missing baseMan.uniform property");
        assert!(found_uniform_type, "Missing uniform.type property");
        assert!(found_rm_displayname, "Missing rm.displayName property");
        assert!(found_rm_primaryweapon, "Missing rm.primaryWeapon property");
    }
} 