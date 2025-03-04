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
        println!("DEBUG: displayName raw value: {:?}", display_name);
        assert_eq!(display_name, "Unarmed", "displayName should be exactly 'Unarmed'");
    } else {
        panic!("displayName property not found or not a string");
    }
    
    // Check linkedItems property (array)
    if let Some(Value::Array(linked_items)) = base_man.properties.get("linkedItems") {
        assert_eq!(linked_items.len(), 3, "linkedItems should have 3 elements");
        
        // Print all items in the array for debugging
        for (i, item) in linked_items.iter().enumerate() {
            println!("DEBUG: linkedItems[{}] = {:?}", i, item);
        }
        
        // Check specific items
        if let Some(Value::String(item)) = linked_items.get(0) {
            println!("DEBUG: First item raw value: {:?}", item);
            assert_eq!(item, "ItemWatch", "First item should be exactly 'ItemWatch'");
        } else {
            panic!("First linkedItem not found or not a string");
        }
        
        if let Some(Value::String(item)) = linked_items.get(1) {
            println!("DEBUG: Second item raw value: {:?}", item);
            assert_eq!(item, "ItemMap", "Second item should be exactly 'ItemMap'");
        } else {
            panic!("Second linkedItem not found or not a string");
        }
        
        if let Some(Value::String(item)) = linked_items.get(2) {
            println!("DEBUG: Third item raw value: {:?}", item);
            assert_eq!(item, "ItemCompass", "Third item should be exactly 'ItemCompass'");
        } else {
            panic!("Third linkedItem not found or not a string");
        }
    } else {
        panic!("linkedItems property not found or not an array");
    }
    
    // Verify all classes by name
    let classes = [
        "baseMan", "rm", "ar", "aar", "rm_lat", "gren", "tl", "sl", "co", "rm_fa", "cls"
    ];
    
    for class_name in classes {
        let class = result.iter().find(|c| c.name == Some(class_name.to_string()));
        assert!(class.is_some(), "{} class not found", class_name);
    }
    
    // Verify rm class properties
    let rm = result.iter().find(|c| c.name == Some("rm".to_string())).unwrap();
    
    // Check displayName property
    if let Some(Value::String(display_name)) = rm.properties.get("displayName") {
        println!("DEBUG: rm displayName raw value: {:?}", display_name);
        assert_eq!(display_name, "Rifleman", "displayName should be exactly 'Rifleman'");
    } else {
        panic!("displayName property not found or not a string");
    }
    
    // Check uniform property
    if let Some(Value::Array(uniform)) = rm.properties.get("uniform") {
        // Print all items in the array for debugging
        println!("DEBUG: uniform array length: {}", uniform.len());
        for (i, item) in uniform.iter().enumerate() {
            println!("DEBUG: uniform[{}] = {:?}", i, item);
        }
        
        // Expected uniform values
        let expected_values = [
            "usp_g3c_kp_mx_aor2",
            "usp_g3c_kp_mx_aor2",
            "usp_g3c_rs_kp_mx_aor2",
            "usp_g3c_rs2_kp_mx_aor2"
        ];
        
        assert_eq!(uniform.len(), expected_values.len(), "uniform array should have {} elements", expected_values.len());
        
        for (i, expected) in expected_values.iter().enumerate() {
            if let Some(item) = uniform.get(i) {
                let item_str = format!("{}", item); // Use Display trait
                assert_eq!(item_str, *expected, "uniform[{}] should be exactly '{}'", i, expected);
            } else {
                panic!("uniform[{}] not found", i);
            }
        }
    } else {
        panic!("uniform property not found or not an array");
    }
    
    // Verify ar class properties
    let ar = result.iter().find(|c| c.name == Some("ar".to_string())).unwrap();
    
    // Check displayName property
    if let Some(Value::String(display_name)) = ar.properties.get("displayName") {
        println!("DEBUG: ar displayName raw value: {:?}", display_name);
        assert_eq!(display_name, "Automatic Rifleman", "displayName should be exactly 'Automatic Rifleman'");
    } else {
        panic!("displayName property not found or not a string");
    }
    
    // Check primaryWeapon property
    if let Some(Value::Array(primary_weapon)) = ar.properties.get("primaryWeapon") {
        assert_eq!(primary_weapon.len(), 1, "primaryWeapon should have 1 element");
        
        // Print all items in the array for debugging
        for (i, item) in primary_weapon.iter().enumerate() {
            println!("DEBUG: primaryWeapon[{}] = {:?}", i, item);
        }
        
        if let Some(Value::String(weapon)) = primary_weapon.get(0) {
            println!("DEBUG: primaryWeapon raw value: {:?}", weapon);
            assert_eq!(weapon, "rhs_weap_m249_light_S", "primaryWeapon should be exactly 'rhs_weap_m249_light_S'");
        } else {
            panic!("primaryWeapon not found or not a string");
        }
    } else {
        panic!("primaryWeapon property not found or not an array");
    }
    
    // Verify sl class properties
    let sl = result.iter().find(|c| c.name == Some("sl".to_string())).unwrap();
    
    // Check displayName property
    if let Some(Value::String(display_name)) = sl.properties.get("displayName") {
        println!("DEBUG: sl displayName raw value: {:?}", display_name);
        assert_eq!(display_name, "Squad Leader", "displayName should be exactly 'Squad Leader'");
    } else {
        panic!("displayName property not found or not a string");
    }
    
    // Verify aar class (inherits from rm)
    let aar = result.iter().find(|c| c.name == Some("aar".to_string())).unwrap();
    assert_eq!(aar.parent, Some("rm".to_string()), "aar should inherit from rm");
    
    // Verify rm_lat class (inherits from rm)
    let rm_lat = result.iter().find(|c| c.name == Some("rm_lat".to_string())).unwrap();
    assert_eq!(rm_lat.parent, Some("rm".to_string()), "rm_lat should inherit from rm");
    
    // Verify tl class (inherits from rm)
    let tl = result.iter().find(|c| c.name == Some("tl".to_string())).unwrap();
    assert_eq!(tl.parent, Some("rm".to_string()), "tl should inherit from rm");
    
    // Verify sl class (inherits from tl)
    assert_eq!(sl.parent, Some("tl".to_string()), "sl should inherit from tl");
    
    // Verify co class (inherits from sl)
    let co = result.iter().find(|c| c.name == Some("co".to_string())).unwrap();
    assert_eq!(co.parent, Some("sl".to_string()), "co should inherit from sl");
} 