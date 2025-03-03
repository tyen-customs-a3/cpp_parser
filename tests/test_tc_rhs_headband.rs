use cpp_parser::{parse_cpp, Class, Value};
use std::path::Path;

#[test]
fn test_tc_rhs_headband_config() {
    // Path to the test file
    let file_path = Path::new("tests/data/@tc_rhs_headband/config.cpp");
    
    // Parse the file
    let result = cpp_parser::parse_cpp_file(file_path.to_str().unwrap()).unwrap();
    
    // Verify CfgPatches class
    let cfg_patches = result.iter().find(|c| c.name == Some("CfgPatches".to_string())).unwrap();
    
    // Verify tc_rhs_headband class inside CfgPatches
    if let Some(tc_rhs_headband) = cfg_patches.classes.get("tc_rhs_headband") {
        assert_eq!(tc_rhs_headband.name, Some("tc_rhs_headband".to_string()));
        
        // Check author property
        if let Some(Value::String(author)) = tc_rhs_headband.properties.get("author") {
            assert!(author.contains("PCA"));
        } else {
            panic!("author property not found or not a string");
        }
        
        // Check requiredVersion property
        if let Some(Value::Number(version)) = tc_rhs_headband.properties.get("requiredVersion") {
            assert_eq!(*version, 1.6);
        } else {
            panic!("requiredVersion property not found or not a number");
        }
        
        // Check requiredAddons property
        if let Some(Value::Array(addons)) = tc_rhs_headband.properties.get("requiredAddons") {
            assert_eq!(addons.len(), 4);
            
            // Check first addon
            if let Some(Value::String(addon)) = addons.get(0) {
                assert!(addon.contains("A3_Characters_F"));
            } else {
                panic!("First addon not found or not a string");
            }
        } else {
            panic!("requiredAddons property not found or not an array");
        }
        
        // Check units property
        if let Some(Value::Array(units)) = tc_rhs_headband.properties.get("units") {
            assert_eq!(units.len(), 0);
        } else {
            panic!("units property not found or not an array");
        }
        
        // Check weapons property
        if let Some(Value::Array(weapons)) = tc_rhs_headband.properties.get("weapons") {
            assert_eq!(weapons.len(), 0);
        } else {
            panic!("weapons property not found or not an array");
        }
    } else {
        panic!("tc_rhs_headband class not found in CfgPatches");
    }
    
    // Verify CfgWeapons class
    let cfg_weapons = result.iter().find(|c| c.name == Some("CfgWeapons".to_string())).unwrap();
    
    // Verify tc_rhs_headband class inside CfgWeapons
    if let Some(tc_rhs_headband) = cfg_weapons.classes.get("tc_rhs_headband") {
        assert_eq!(tc_rhs_headband.name, Some("tc_rhs_headband".to_string()));
        
        // Check displayName property
        if let Some(Value::String(display_name)) = tc_rhs_headband.properties.get("displayName") {
            assert!(display_name.contains("Headband (I <3 Choccy Milk)"));
        } else {
            panic!("displayName property not found or not a string");
        }
        
        // Check hiddenSelectionsTextures property
        if let Some(Value::Array(textures)) = tc_rhs_headband.properties.get("hiddenSelectionsTextures") {
            assert_eq!(textures.len(), 1);
            
            if let Some(Value::String(texture)) = textures.get(0) {
                assert!(texture.contains("headband_choccymilk_co.paa"));
            } else {
                panic!("Texture not found or not a string");
            }
        } else {
            panic!("hiddenSelectionsTextures property not found or not an array");
        }
    } else {
        panic!("tc_rhs_headband class not found in CfgWeapons");
    }
} 