/// Extract properties from content
pub fn extract_properties_from_content(content: &str) -> String {
    let mut result = String::new();
    let mut in_array = false;
    let mut array_brace_count = 0;
    
    // Split the content by lines for easier processing
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }
        
        // Handle array properties
        if trimmed.contains("[]") && trimmed.contains("=") {
            result.push_str(trimmed);
            result.push('\n');
            
            if trimmed.contains("{") {
                in_array = true;
                array_brace_count = trimmed.chars().filter(|&c| c == '{').count();
                array_brace_count -= trimmed.chars().filter(|&c| c == '}').count();
                
                // If array is complete on this line, reset the flag
                if array_brace_count <= 0 {
                    in_array = false;
                }
            }
            continue;
        }
        
        // Handle array content
        if in_array {
            result.push_str(trimmed);
            result.push('\n');
            
            if trimmed.contains("{") {
                array_brace_count += trimmed.chars().filter(|&c| c == '{').count();
            }
            if trimmed.contains("}") {
                array_brace_count -= trimmed.chars().filter(|&c| c == '}').count();
                if array_brace_count <= 0 {
                    in_array = false;
                }
            }
            continue;
        }
        
        // Handle regular properties
        if trimmed.contains("=") {
            result.push_str(trimmed);
            result.push('\n');
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_properties() {
        let input = r#"
            displayName = "Rifleman";
            uniform = "rhs_uniform_g3_m81";
            primaryWeapon = "rhs_weap_m4a1_blockII_KAC";
        "#;
        let result = extract_properties_from_content(input);
        assert!(result.contains("displayName ="));
        assert!(result.contains("uniform ="));
        assert!(result.contains("primaryWeapon ="));
    }

    #[test]
    fn test_extract_array_property() {
        let input = r#"
            items[] = {
                "Item1",
                "Item2",
                "Item3"
            };
        "#;
        let result = extract_properties_from_content(input);
        assert!(result.contains("items[] ="));
        assert!(result.contains("Item1"));
        assert!(result.contains("Item2"));
        assert!(result.contains("Item3"));
    }

    #[test]
    fn test_extract_array_with_list_macro() {
        let input = r#"
            items[] = {
                LIST_2("ACE_fieldDressing"),
                LIST_10("ACE_packingBandage"),
                "ACRE_PRC343"
            };
        "#;
        let result = extract_properties_from_content(input);
        assert!(result.contains("items[] ="));
        assert!(result.contains("LIST_2"));
        assert!(result.contains("LIST_10"));
        assert!(result.contains("ACRE_PRC343"));
    }
} 