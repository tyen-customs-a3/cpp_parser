use cpp_parser::{parse_cpp, Class, Value};
use std::path::Path;

#[test]
fn test_blufor_loadout() {
    // Path to the test file
    let file_path = Path::new("tests/data/blufor_loadout.hpp");
    
    // Verify the file exists
    assert!(file_path.exists(), "Test file does not exist");
    
    // Parse the file
    let result = cpp_parser::parse_cpp_file(file_path.to_str().unwrap()).unwrap();
    
    // Verify that classes were parsed
    assert!(!result.is_empty(), "No classes were parsed from the file");
    
    // Verify baseMan class exists
    let base_man = result.iter().find(|c| c.name == Some("baseMan".to_string()));
    assert!(base_man.is_some(), "baseMan class not found");
    
    // Verify baseMan class properties
    let base_man = base_man.unwrap();
    
    // Check displayName property
    if let Some(Value::String(display_name)) = base_man.properties.get("displayName") {
        println!("displayName: '{}'", display_name);
        
        // Strip quotes if present for comparison
        let display_name_unquoted = if display_name.starts_with('"') && display_name.ends_with('"') && display_name.len() >= 2 {
            &display_name[1..display_name.len() - 1]
        } else {
            display_name
        };
        
        assert_eq!(display_name_unquoted, "Unarmed", "displayName should be exactly 'Unarmed'");
    } else {
        panic!("displayName property not found or not a string");
    }
    
    // Check linkedItems property (array)
    if let Some(Value::Array(linked_items)) = base_man.properties.get("linkedItems") {
        assert_eq!(linked_items.len(), 3, "linkedItems should have 3 elements");
        
        // Check specific items
        if let Some(Value::String(item)) = linked_items.get(0) {
            println!("First item: '{}'", item);
            println!("First item raw: {:?}", item);
            
            // Strip quotes if present for comparison
            let item_unquoted = if item.starts_with('"') && item.ends_with('"') && item.len() >= 2 {
                &item[1..item.len() - 1]
            } else {
                item
            };
            
            assert_eq!(item_unquoted, "ItemWatch", "First item should be exactly 'ItemWatch'");
        } else {
            panic!("First linkedItem not found or not a string");
        }
        
        if let Some(Value::String(item)) = linked_items.get(1) {
            println!("Second item: '{}'", item);
            
            // Strip quotes if present for comparison
            let item_unquoted = if item.starts_with('"') && item.ends_with('"') && item.len() >= 2 {
                &item[1..item.len() - 1]
            } else {
                item
            };
            
            assert_eq!(item_unquoted, "ItemMap", "Second item should be exactly 'ItemMap'");
        } else {
            panic!("Second linkedItem not found or not a string");
        }
        
        if let Some(Value::String(item)) = linked_items.get(2) {
            println!("Third item: '{}'", item);
            
            // Strip quotes if present for comparison
            let item_unquoted = if item.starts_with('"') && item.ends_with('"') && item.len() >= 2 {
                &item[1..item.len() - 1]
            } else {
                item
            };
            
            assert_eq!(item_unquoted, "ItemCompass", "Third item should be exactly 'ItemCompass'");
        } else {
            panic!("Third linkedItem not found or not a string");
        }
    } else {
        panic!("linkedItems property not found or not an array");
    }
    
    // Verify rm class (inherits from baseMan)
    let rm = result.iter().find(|c| c.name == Some("rm".to_string()));
    assert!(rm.is_some(), "rm class not found");
    
    // Verify rm class properties
    let rm = rm.unwrap();
    
    // Check displayName property
    if let Some(Value::String(display_name)) = rm.properties.get("displayName") {
        println!("rm displayName: '{}'", display_name);
        
        // Strip quotes if present for comparison
        let display_name_unquoted = if display_name.starts_with('"') && display_name.ends_with('"') && display_name.len() >= 2 {
            &display_name[1..display_name.len() - 1]
        } else {
            display_name
        };
        
        assert_eq!(display_name_unquoted, "Rifleman", "displayName should be exactly 'Rifleman'");
    } else {
        panic!("displayName property not found or not a string");
    }
    
    // Check rm class properties
    if let Some(uniform) = rm.properties.get("uniform") {
        println!("Uniform property type: {:?}", uniform);
        
        if let Value::Array(uniform) = uniform {
            println!("Uniform array length: {}", uniform.len());
            
            // Print all items in the array for debugging
            for (i, item) in uniform.iter().enumerate() {
                println!("Uniform[{}]: {:?}", i, item);
            }
            
            // Check for LIST_2 macro expansion or direct string values
            // We need to check for the exact string values without quotes
            let expected_values = [
                "usp_g3c_kp_mx_aor2",
                "usp_g3c_rs_kp_mx_aor2",
                "usp_g3c_rs2_kp_mx_aor2"
            ];
            
            // Verify each element is in the array
            for expected in &expected_values {
                let found = uniform.iter().any(|v| {
                    match v {
                        Value::String(s) => {
                            // Strip quotes if present for comparison
                            let s_unquoted = if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                                &s[1..s.len() - 1]
                            } else {
                                s
                            };
                            
                            s_unquoted == *expected
                        },
                        Value::ListMacro(_, content) => {
                            // For ListMacro, check if the content matches
                            content == expected
                        },
                        _ => false
                    }
                });
                
                assert!(found, "uniform should contain '{}'", expected);
            }
        } else {
            panic!("uniform property not found or not an array");
        }
    } else {
        panic!("uniform property not found or not an array");
    }
    
    // Verify ar class (inherits from rm)
    let ar = result.iter().find(|c| c.name == Some("ar".to_string()));
    assert!(ar.is_some(), "ar class not found");
    
    // Verify ar class properties
    let ar = ar.unwrap();
    
    // Check displayName property
    if let Some(Value::String(display_name)) = ar.properties.get("displayName") {
        println!("ar displayName: '{}'", display_name);
        
        // Strip quotes if present for comparison
        let display_name_unquoted = if display_name.starts_with('"') && display_name.ends_with('"') && display_name.len() >= 2 {
            &display_name[1..display_name.len() - 1]
        } else {
            display_name
        };
        
        assert_eq!(display_name_unquoted, "Automatic Rifleman", "displayName should be exactly 'Automatic Rifleman'");
    } else {
        panic!("displayName property not found or not a string");
    }
    
    // Check primaryWeapon property (array)
    if let Some(Value::Array(primary_weapon)) = ar.properties.get("primaryWeapon") {
        assert_eq!(primary_weapon.len(), 1, "primaryWeapon should have 1 element");
        
        if let Some(Value::String(weapon)) = primary_weapon.get(0) {
            println!("primaryWeapon: '{}'", weapon);
            
            // Strip quotes if present for comparison
            let weapon_unquoted = if weapon.starts_with('"') && weapon.ends_with('"') && weapon.len() >= 2 {
                &weapon[1..weapon.len() - 1]
            } else {
                weapon
            };
            
            assert_eq!(weapon_unquoted, "rhs_weap_m249_light_S", "primaryWeapon should be exactly 'rhs_weap_m249_light_S'");
        } else {
            panic!("primaryWeapon not found or not a string");
        }
    } else {
        panic!("primaryWeapon property not found or not an array");
    }
    
    // Verify sl class (inherits from tl)
    let sl = result.iter().find(|c| c.name == Some("sl".to_string()));
    assert!(sl.is_some(), "sl class not found");
    
    // Verify sl class properties
    let sl = sl.unwrap();
    
    // Check displayName property
    if let Some(Value::String(display_name)) = sl.properties.get("displayName") {
        println!("sl displayName: '{}'", display_name);
        
        // Strip quotes if present for comparison
        let display_name_unquoted = if display_name.starts_with('"') && display_name.ends_with('"') && display_name.len() >= 2 {
            &display_name[1..display_name.len() - 1]
        } else {
            display_name
        };
        
        assert_eq!(display_name_unquoted, "Squad Leader", "displayName should be exactly 'Squad Leader'");
    } else {
        panic!("displayName property not found or not a string");
    }
} 