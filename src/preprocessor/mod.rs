use regex::RegexBuilder;

/// Types of configuration elements that can be found in a config file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigElementType {
    /// A preprocessor directive like #define or #include
    Define,
    /// An enumeration
    Enum,
    /// A class definition
    Class,
    /// A forward declaration (class declaration without a body)
    ForwardDeclaration,
    /// A delete statement (e.g., delete Revive;)
    Delete,
    /// Other content that doesn't match any specific type
    Other,
}

/// Represents a single element in the configuration file
#[derive(Debug, Clone)]
pub struct ConfigElement {
    /// The type of this element
    pub element_type: ConfigElementType,
    /// The name of the element, if applicable
    pub name: Option<String>,
    /// The parent class name, if applicable (for classes with inheritance)
    pub parent: Option<String>,
    /// The raw content of the element
    pub content: String,
    /// Start position in the original text
    pub start_pos: usize,
    /// End position in the original text
    pub end_pos: usize,
}

impl ConfigElement {
    /// Create a new config element
    pub fn new(
        element_type: ConfigElementType,
        name: Option<String>,
        parent: Option<String>,
        content: String,
        start_pos: usize,
        end_pos: usize,
    ) -> Self {
        ConfigElement {
            element_type,
            name,
            parent,
            content,
            start_pos,
            end_pos,
        }
    }

    /// Get the raw text of this element from the original input
    pub fn get_raw_text<'a>(&self, input: &'a str) -> Option<&'a str> {
        if self.start_pos < input.len() && self.end_pos <= input.len() && self.start_pos <= self.end_pos {
            Some(&input[self.start_pos..self.end_pos])
        } else {
            None
        }
    }
}

/// Preprocess a configuration file to identify different elements
pub fn preprocess(input: &str) -> Vec<ConfigElement> {
    let mut elements = Vec::new();
    
    // Find all defines
    find_defines(input, &mut elements);
    
    // Find all enums
    find_enums(input, &mut elements);
    
    // Find all classes
    find_classes(input, &mut elements);
    
    // Find all forward declarations
    find_forward_declarations(input, &mut elements);
    
    // Find all delete statements
    find_delete_statements(input, &mut elements);
    
    // Sort elements by their position in the file
    elements.sort_by_key(|e| e.start_pos);
    
    elements
}

/// Find all #define directives in the input
fn find_defines(input: &str, elements: &mut Vec<ConfigElement>) {
    let define_regex = RegexBuilder::new(r"#define\s+([A-Za-z0-9_]+)(?:\s+(.+))?")
        .multi_line(true)
        .build()
        .unwrap();
    
    for define_match in define_regex.captures_iter(input) {
        let match_start = define_match.get(0).unwrap().start();
        let match_end = define_match.get(0).unwrap().end();
        
        let name = define_match.get(1).map(|m| m.as_str().to_string());
        let content = define_match.get(0).unwrap().as_str().to_string();
        
        elements.push(ConfigElement::new(
            ConfigElementType::Define,
            name,
            None,
            content,
            match_start,
            match_end
        ));
    }
}

/// Find all enum declarations in the input
fn find_enums(input: &str, elements: &mut Vec<ConfigElement>) {
    let enum_regex = RegexBuilder::new(r"enum\s*\{")
        .multi_line(true)
        .build()
        .unwrap();
    
    for enum_match in enum_regex.find_iter(input) {
        let match_start = enum_match.start();
        let opening_brace_pos = input[match_start..].find('{').unwrap() + match_start;
        
        // Find the matching closing brace
        if let Some((content, end_pos)) = find_matching_brace(input, opening_brace_pos) {
            elements.push(ConfigElement::new(
                ConfigElementType::Enum,
                None, // Enums in this format don't have names
                None,
                content,
                match_start,
                end_pos
            ));
        }
    }
}

/// Find all class declarations in the input
fn find_classes(input: &str, elements: &mut Vec<ConfigElement>) {
    let class_regex = RegexBuilder::new(r"class\s+([A-Za-z0-9_]+)(?:\s*:\s*([A-Za-z0-9_]+))?\s*\{")
        .multi_line(true)
        .build()
        .unwrap();
    
    // Track positions of all classes first
    let mut all_classes = Vec::new();
    
    let mut pos = 0;
    while pos < input.len() {
        let remaining = &input[pos..];
        
        if let Some(class_match) = class_regex.captures(remaining) {
            let class_name = class_match.get(1).map(|m| m.as_str().to_string());
            let parent_name = class_match.get(2).map(|m| m.as_str().to_string());
            
            // Find the opening brace position
            let match_start = class_match.get(0).unwrap().start();
            let absolute_match_start = pos + match_start;
            let opening_brace_pos = remaining[match_start..].find('{').unwrap() + match_start;
            let absolute_opening_brace_pos = pos + opening_brace_pos;
            
            // Find the matching closing brace
            if let Some((content, end_pos)) = find_matching_brace(input, absolute_opening_brace_pos) {
                let absolute_end_pos = absolute_opening_brace_pos + end_pos - absolute_opening_brace_pos;
                
                all_classes.push((
                    class_name,
                    parent_name,
                    content,
                    absolute_match_start,
                    absolute_end_pos
                ));
                
                // Move past this class
                pos = absolute_end_pos;
            } else {
                // If we can't find a matching brace, move past this match
                pos = absolute_match_start + 1;
            }
        } else {
            // No more matches, we're done
            break;
        }
    }
    
    // Now determine which classes are top-level (not nested inside other classes)
    for i in 0..all_classes.len() {
        let (ref class_name, ref parent_name, ref content, start_pos, end_pos) = all_classes[i];
        
        // Check if this class is nested inside another class
        let is_nested = all_classes.iter().any(|(_, _, _, other_start, other_end)| 
            *other_start < start_pos && *other_end > end_pos && *other_start != start_pos && *other_end != end_pos
        );
        
        if !is_nested {
            // This is a top-level class
            elements.push(ConfigElement::new(
                ConfigElementType::Class,
                class_name.clone(),
                parent_name.clone(),
                content.clone(),
                start_pos,
                end_pos
            ));
        }
    }
}

/// Find all forward declarations in the input
fn find_forward_declarations(input: &str, elements: &mut Vec<ConfigElement>) {
    let forward_decl_regex = RegexBuilder::new(r"class\s+([A-Za-z0-9_]+)\s*;")
        .multi_line(true)
        .build()
        .unwrap();
    
    for decl_match in forward_decl_regex.captures_iter(input) {
        let match_start = decl_match.get(0).unwrap().start();
        let match_end = decl_match.get(0).unwrap().end();
        
        let class_name = decl_match.get(1).map(|m| m.as_str().to_string());
        let content = decl_match.get(0).unwrap().as_str().to_string();
        
        elements.push(ConfigElement::new(
            ConfigElementType::ForwardDeclaration,
            class_name,
            None,
            content,
            match_start,
            match_end
        ));
    }
}

/// Find all delete statements in the input
fn find_delete_statements(input: &str, elements: &mut Vec<ConfigElement>) {
    let delete_regex = RegexBuilder::new(r"delete\s+([A-Za-z0-9_]+)\s*;")
        .multi_line(true)
        .build()
        .unwrap();
    
    for delete_match in delete_regex.captures_iter(input) {
        let match_start = delete_match.get(0).unwrap().start();
        let match_end = delete_match.get(0).unwrap().end();
        
        let target_name = delete_match.get(1).map(|m| m.as_str().to_string());
        let content = delete_match.get(0).unwrap().as_str().to_string();
        
        elements.push(ConfigElement::new(
            ConfigElementType::Delete,
            target_name,
            None,
            content,
            match_start,
            match_end
        ));
    }
}

/// Find the matching closing brace for an opening brace
fn find_matching_brace(input: &str, opening_brace_pos: usize) -> Option<(String, usize)> {
    let mut brace_count = 1;
    let mut pos = opening_brace_pos + 1;
    
    while pos < input.len() && brace_count > 0 {
        match input.chars().nth(pos) {
            Some('{') => brace_count += 1,
            Some('}') => brace_count -= 1,
            _ => {}
        }
        pos += 1;
    }
    
    if brace_count == 0 {
        // Extract the content between braces (excluding the braces themselves)
        let content = input[opening_brace_pos + 1..pos - 1].to_string();
        Some((content, pos))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_defines() {
        let input = r#"
        #define _ARMA_
        #define DEBUG_MODE_FULL
        #define VERSION 1.0
        "#;
        
        let mut elements = Vec::new();
        find_defines(input, &mut elements);
        
        // Print the elements for debugging
        println!("Found {} defines:", elements.len());
        for (i, element) in elements.iter().enumerate() {
            println!("Define #{}: {:?}", i + 1, element.name);
        }
        
        // Check if we have at least 1 define
        assert!(!elements.is_empty(), "No defines found");
        
        // Check that we have the expected defines
        let mut found_arma = false;
        
        for element in &elements {
            assert_eq!(element.element_type, ConfigElementType::Define);
            if let Some(name) = &element.name {
                if name == "_ARMA_" {
                    found_arma = true;
                }
            }
        }
        
        assert!(found_arma, "Missing _ARMA_ define");
    }

    #[test]
    fn test_find_enums() {
        let input = r#"
        enum {
            destructengine = 2,
            destructdefault = 6,
            destructwreck = 7,
            destructtree = 3,
            destructtent = 4
        };
        "#;
        
        let mut elements = Vec::new();
        find_enums(input, &mut elements);
        
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, ConfigElementType::Enum);
        assert!(elements[0].content.contains("destructengine = 2"));
    }

    #[test]
    fn test_find_classes() {
        let input = r#"
        class CfgPatches
        {
            class A3_3DEN
            {
                author = "$STR_A3_Bohemia_Interactive";
                name = "Arma 3 Eden Update - Eden Editor";
            };
        };
        "#;
        
        let mut elements = Vec::new();
        find_classes(input, &mut elements);
        
        // Now we only identify top-level classes
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, ConfigElementType::Class);
        assert_eq!(elements[0].name, Some("CfgPatches".to_string()));
    }

    #[test]
    fn test_find_forward_declarations() {
        let input = r#"
        class RscText;
        class RscTitle;
        class RscListbox;
        "#;
        
        let mut elements = Vec::new();
        find_forward_declarations(input, &mut elements);
        
        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0].element_type, ConfigElementType::ForwardDeclaration);
        assert_eq!(elements[0].name, Some("RscText".to_string()));
        assert_eq!(elements[1].element_type, ConfigElementType::ForwardDeclaration);
        assert_eq!(elements[1].name, Some("RscTitle".to_string()));
        assert_eq!(elements[2].element_type, ConfigElementType::ForwardDeclaration);
        assert_eq!(elements[2].name, Some("RscListbox".to_string()));
    }

    #[test]
    fn test_find_delete_statements() {
        let input = r#"
        class CfgRespawnTemplates
        {
            delete Revive;
        };
        "#;
        
        let mut elements = Vec::new();
        find_delete_statements(input, &mut elements);
        
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].element_type, ConfigElementType::Delete);
        assert_eq!(elements[0].name, Some("Revive".to_string()));
    }

    #[test]
    fn test_preprocess() {
        let input = r#"
        #define _ARMA_
        
        class RscText;
        
        enum {
            destructengine = 2,
            destructdefault = 6
        };
        
        class CfgPatches
        {
            class A3_3DEN
            {
                author = "$STR_A3_Bohemia_Interactive";
            };
        };
        
        class CfgRespawnTemplates
        {
            delete Revive;
        };
        "#;
        
        let elements = preprocess(input);
        
        assert_eq!(elements.len(), 6);
        assert_eq!(elements[0].element_type, ConfigElementType::Define);
        assert_eq!(elements[1].element_type, ConfigElementType::ForwardDeclaration);
        assert_eq!(elements[2].element_type, ConfigElementType::Enum);
        assert_eq!(elements[3].element_type, ConfigElementType::Class);
        assert_eq!(elements[4].element_type, ConfigElementType::Class);
        assert_eq!(elements[5].element_type, ConfigElementType::Delete);
    }
} 