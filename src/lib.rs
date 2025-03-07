mod error;
mod lexer;
mod parser;
mod model;
mod utils;

pub use error::Error;
pub use model::{Class, Property};
pub use parser::Parser;
pub use utils::{
    find_classes_by_name,
    find_classes_by_parent,
    find_classes_with_string,
    find_all_matching_strings,
    get_all_classes,
    get_property_values_by_type,
};

/// Parse configuration text and return the parsed class structure
/// 
/// # Arguments
/// 
/// * `input` - The configuration text to parse
/// 
/// # Returns
/// 
/// * `Result<Vec<Class>, Error>` - The parsed classes or an error
/// 
/// # Example
/// 
/// ```
/// use cpp_parser::parse;
/// 
/// let config = r#"
/// class Soldier {
///     displayName = "Infantry";
///     items[] = {"Rifle", "Ammo"};
/// };
/// "#;
/// 
/// let classes = parse(config).unwrap();
/// assert_eq!(classes[0].name, "Soldier");
/// ```
pub fn parse(input: &str) -> Result<Vec<model::Class>, Error> {
    let mut parser = Parser::new();
    parser.parse(input)
}

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use ntest::timeout;
    
    #[test]
    #[timeout(1000)]
    fn test_parse_simple_class() {
        let input = r#"
        class Soldier {
            displayName = "Infantry";
            items[] = {"Rifle", "Ammo"};
        };
        "#;
        
        let classes = parse(input).unwrap();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Soldier");
        assert_eq!(classes[0].parent, None);
        assert_eq!(classes[0].properties.len(), 2);
        
        let display_name = classes[0].get_property("displayName").unwrap();
        assert_eq!(display_name.as_string(), Some("Infantry"));
        
        let items = classes[0].get_property("items").unwrap();
        if let Some(array) = items.as_array() {
            assert_eq!(array.len(), 2);
            assert_eq!(array[0].as_string(), Some("Rifle"));
            assert_eq!(array[1].as_string(), Some("Ammo"));
        } else {
            panic!("items should be an array");
        }
    }
    
    #[test]
    #[timeout(1000)]
    fn test_parse_class_with_parent() {
        let input = r#"
        class Soldier {
            displayName = "Infantry";
        };
        
        class Medic : Soldier {
            displayName = "Combat Medic";
            medKit = true;
        };
        "#;
        
        let classes = parse(input).unwrap();
        assert_eq!(classes.len(), 2);
        
        assert_eq!(classes[0].name, "Soldier");
        assert_eq!(classes[0].parent, None);
        
        assert_eq!(classes[1].name, "Medic");
        assert_eq!(classes[1].parent, Some("Soldier".to_string()));
        
        // Check that the Medic has its own displayName
        let display_name = classes[1].get_property("displayName").unwrap();
        assert_eq!(display_name.as_string(), Some("Combat Medic"));
    }
    
    #[test]
    #[timeout(1000)]
    fn test_parse_nested_class() {
        let input = r#"
        class Weapons {
            class Rifle {
                model = "M4A1";
                caliber = 5.56;
            };
            
            class Pistol {
                model = "M9";
                caliber = 9;
            };
        };
        "#;
        
        let classes = parse(input).unwrap();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Weapons");
        assert_eq!(classes[0].subclasses.len(), 2);
        
        let rifle = &classes[0].subclasses[0];
        assert_eq!(rifle.name, "Rifle");
        assert_eq!(rifle.get_property("model").unwrap().as_string(), Some("M4A1"));
        
        let pistol = &classes[0].subclasses[1];
        assert_eq!(pistol.name, "Pistol");
        assert_eq!(pistol.get_property("model").unwrap().as_string(), Some("M9"));
    }
    
    #[test]
    #[timeout(1000)]
    fn test_parse_array_with_list_macro() {
        let input = r#"
        class Soldier {
            items[] = {
                LIST_2("Bandage"),
                LIST_5("Ammo"),
                "Water"
            };
        };
        "#;
        
        let classes = parse(input).unwrap();
        let items = classes[0].get_property("items").unwrap();
        if let Some(array) = items.as_array() {
            assert_eq!(array.len(), 3);
            assert_eq!(array[0].as_string(), Some("Bandage"));
            assert_eq!(array[1].as_string(), Some("Ammo"));
            assert_eq!(array[2].as_string(), Some("Water"));
        } else {
            panic!("items should be an array");
        }
    }
    
    #[test]
    #[timeout(1000)]
    fn test_find_classes_with_string() {
        let input = r#"
        class Soldier {
            items[] = {"FirstAid", "Rifle"};
        };
        
        class Medic : Soldier {
            items[] = {"Surgery", "Morphine", "Bandage"};
        };
        
        class Engineer {
            items[] = {"Toolkit", "Pliers"};
        };
        "#;
        
        let classes = parse(input).unwrap();
        
        // Find classes with "Morphine"
        let morphine_classes = find_classes_with_string(&classes, "Morphine");
        assert_eq!(morphine_classes.len(), 1);
        assert_eq!(morphine_classes[0].name, "Medic");
        
        // Find classes with "Rifle"
        let rifle_classes = find_classes_with_string(&classes, "Rifle");
        assert_eq!(rifle_classes.len(), 1);
        assert_eq!(rifle_classes[0].name, "Soldier");
    }
}
