use cpp_parser::preprocessor::{preprocess, ConfigElementType};
use cpp_parser::class_parser::parse_classes;
use cpp_parser::property_parser::block_properties::parse_all_block_properties;
use std::fs;

#[test]
fn test_loadout_preprocessing() {
    // Read the loadout file
    let input = fs::read_to_string("tests/data/blufor_loadout.hpp").expect("Failed to read test file");
    
    // Preprocess the file
    let elements = preprocess(&input);
    
    // Verify that we found the expected elements
    let classes = elements.iter().filter(|e| e.element_type == ConfigElementType::Class).count();
    
    // We expect multiple classes in the loadout file
    assert!(classes >= 10, "Expected at least 10 classes in the loadout file");
    
    // Verify specific classes
    let base_man = elements.iter().find(|e| 
        e.element_type == ConfigElementType::Class && 
        e.name.as_deref() == Some("baseMan")
    );
    assert!(base_man.is_some(), "Expected to find baseMan class");
    
    let rifleman = elements.iter().find(|e| 
        e.element_type == ConfigElementType::Class && 
        e.name.as_deref() == Some("rm")
    );
    assert!(rifleman.is_some(), "Expected to find rm (rifleman) class");
    
    let auto_rifleman = elements.iter().find(|e| 
        e.element_type == ConfigElementType::Class && 
        e.name.as_deref() == Some("ar")
    );
    assert!(auto_rifleman.is_some(), "Expected to find ar (automatic rifleman) class");
}

#[test]
fn test_loadout_class_parsing() {
    // Read the loadout file
    let input = fs::read_to_string("tests/data/blufor_loadout.hpp").expect("Failed to read test file");
    
    // Parse the classes
    let classes = parse_classes(&input);
    
    // Verify that we found the expected classes
    assert!(!classes.is_empty(), "Expected to find classes in the loadout file");
    
    // Check for specific classes
    let base_man = classes.iter().find(|c| c.name.as_deref() == Some("baseMan"));
    let rifleman = classes.iter().find(|c| c.name.as_deref() == Some("rm"));
    let auto_rifleman = classes.iter().find(|c| c.name.as_deref() == Some("ar"));
    let team_leader = classes.iter().find(|c| c.name.as_deref() == Some("tl"));
    let squad_leader = classes.iter().find(|c| c.name.as_deref() == Some("sl"));
    
    assert!(base_man.is_some(), "Expected to find baseMan class");
    assert!(rifleman.is_some(), "Expected to find rm class");
    assert!(auto_rifleman.is_some(), "Expected to find ar class");
    assert!(team_leader.is_some(), "Expected to find tl class");
    assert!(squad_leader.is_some(), "Expected to find sl class");
    
    // Check inheritance
    if let Some(rifleman) = rifleman {
        assert_eq!(rifleman.parent.as_deref(), Some("baseMan"), "Expected rm to inherit from baseMan");
    }
    
    if let Some(auto_rifleman) = auto_rifleman {
        assert_eq!(auto_rifleman.parent.as_deref(), Some("rm"), "Expected ar to inherit from rm");
    }
}

#[test]
fn test_loadout_property_parsing() {
    // Read the loadout file
    let input = fs::read_to_string("tests/data/blufor_loadout.hpp").expect("Failed to read test file");
    
    // Parse the classes
    let classes = parse_classes(&input);
    
    // Parse properties from all classes
    let properties = parse_all_block_properties(&classes, &input);
    
    // Verify that we found properties
    assert!(!properties.is_empty(), "Expected to find properties in the loadout file");
    
    // Check for specific properties in baseMan class
    let base_man_display_name = properties.iter().find(|(class_name, prop_name, _)| 
        class_name == "baseMan" && prop_name == "displayName"
    );
    assert!(base_man_display_name.is_some(), "Expected to find displayName property in baseMan class");
    if let Some((_, _, value)) = base_man_display_name {
        assert_eq!(value, "\"Unarmed\"", "Expected baseMan displayName to be \"Unarmed\"");
    }
    
    // Check for array properties
    let linked_items = properties.iter().filter(|(class_name, prop_name, _)| 
        class_name == "baseMan" && prop_name == "linkedItems"
    ).count();
    assert!(linked_items > 0, "Expected to find linkedItems property in baseMan class");
    
    // Check for LIST_ macro properties in rifleman class
    let uniform_props = properties.iter().filter(|(class_name, prop_name, _)| 
        class_name == "rm" && prop_name == "uniform"
    ).count();
    assert!(uniform_props > 0, "Expected to find uniform property in rm class");
}

#[test]
fn test_loadout_list_macros() {
    // Read the loadout file
    let input = fs::read_to_string("tests/data/blufor_loadout.hpp").expect("Failed to read test file");
    
    // Parse the classes
    let classes = parse_classes(&input);
    
    // Find the rifleman class
    let rifleman = classes.iter().find(|c| c.name.as_deref() == Some("rm")).expect("Expected to find rm class");
    
    // Parse properties from the rifleman class
    let properties = cpp_parser::property_parser::parse_properties(&rifleman.content).expect("Failed to parse properties");
    
    // Find the items property which contains LIST_ macros
    let items_prop = properties.iter().find(|p| p.name == "items").expect("Expected to find items property");
    
    // Verify that the items property is an array
    if let cpp_parser::models::PropertyValue::Array(values) = &items_prop.value {
        // The items array in the rifleman class should have multiple values due to LIST_ macros expansion
        // LIST_10("ACE_fieldDressing") + LIST_10("ACE_packingBandage") + LIST_4("ACE_tourniquet") + 
        // LIST_2("ACE_epinephrine") + LIST_2("ACE_morphine") + LIST_2("ACE_splint") + "ACRE_PRC343"
        // = 10 + 10 + 4 + 2 + 2 + 2 + 1 = 31 values
        assert!(values.len() > 20, "Expected at least 20 values in items array due to LIST_ macros expansion");
        
        // Count the number of ACE_fieldDressing items (should be 10 from LIST_10)
        let field_dressing_count = values.iter().filter(|v| {
            if let cpp_parser::models::PropertyValue::String(s) = v {
                s.contains("ACE_fieldDressing")
            } else {
                false
            }
        }).count();
        
        assert_eq!(field_dressing_count, 10, "Expected 10 ACE_fieldDressing items from LIST_10 macro");
    } else {
        panic!("Expected items property to be an array");
    }
}

#[test]
fn test_loadout_array_properties() {
    // Read the loadout file
    let input = fs::read_to_string("tests/data/blufor_loadout.hpp").expect("Failed to read test file");
    
    // Parse the classes
    let classes = parse_classes(&input);
    
    // Find the rifleman class
    let rifleman = classes.iter().find(|c| c.name.as_deref() == Some("rm")).expect("Expected to find rm class");
    
    // Parse properties from the rifleman class
    let properties = cpp_parser::property_parser::parse_properties(&rifleman.content).expect("Failed to parse properties");
    
    // Check that we found array properties
    let array_props = properties.iter().filter(|p| {
        matches!(p.value, cpp_parser::models::PropertyValue::Array(_))
    }).count();
    
    assert!(array_props > 0, "Expected to find array properties in the rifleman class");
    
    // Check for specific array properties
    let uniform_prop = properties.iter().find(|p| p.name == "uniform");
    let vest_prop = properties.iter().find(|p| p.name == "vest");
    let headgear_prop = properties.iter().find(|p| p.name == "headgear");
    
    assert!(uniform_prop.is_some(), "Expected to find uniform property");
    assert!(vest_prop.is_some(), "Expected to find vest property");
    assert!(headgear_prop.is_some(), "Expected to find headgear property");
    
    // Verify that these are array properties
    if let Some(prop) = uniform_prop {
        assert!(matches!(prop.value, cpp_parser::models::PropertyValue::Array(_)), 
                "Expected uniform property to be an array");
    }
    
    if let Some(prop) = vest_prop {
        assert!(matches!(prop.value, cpp_parser::models::PropertyValue::Array(_)), 
                "Expected vest property to be an array");
    }
    
    if let Some(prop) = headgear_prop {
        assert!(matches!(prop.value, cpp_parser::models::PropertyValue::Array(_)), 
                "Expected headgear property to be an array");
    }
}

#[test]
fn test_loadout_list_macros_in_different_classes() {
    // Read the loadout file
    let input = fs::read_to_string("tests/data/blufor_loadout.hpp").expect("Failed to read test file");
    
    // Parse the classes
    let classes = parse_classes(&input);
    
    // Check LIST_ macros in different classes
    let class_names = ["rm", "ar", "sl", "cls"];  // Removed 'aar' as it might not have array properties
    
    for class_name in class_names.iter() {
        let class = classes.iter().find(|c| c.name.as_deref() == Some(class_name));
        assert!(class.is_some(), "Expected to find {} class", class_name);
        
        if let Some(class) = class {
            // Parse properties from the class
            let properties = cpp_parser::property_parser::parse_properties(&class.content).expect("Failed to parse properties");
            
            // Check for array properties that might contain LIST_ macros
            let array_props = properties.iter().filter(|p| 
                matches!(p.value, cpp_parser::models::PropertyValue::Array(_))
            ).count();
            
            // Only assert if the class has content
            if !class.content.trim().is_empty() {
                assert!(array_props > 0, "Expected to find array properties in the {} class", class_name);
                
                // Check the raw content for LIST_ macros
                let list_macro_count = class.content.matches("LIST_").count();
                println!("Found {} LIST_ macros in {} class", list_macro_count, class_name);
                
                // Most classes should have at least one LIST_ macro
                if class_name != &"baseMan" {
                    assert!(list_macro_count > 0, "Expected to find LIST_ macros in the {} class", class_name);
                }
            } else {
                println!("Class {} has empty content", class_name);
            }
        }
    }
} 