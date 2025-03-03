use cpp_parser::{parse_cpp, Class, Value};
use std::path::Path;

/// A helper function to test any config.cpp file
fn test_config_file(file_path: &str) -> Vec<Class> {
    let path = Path::new(file_path);
    
    // Ensure the file exists
    assert!(path.exists(), "Test file does not exist: {}", file_path);
    
    // Parse the file
    let result = cpp_parser::parse_cpp_file(path.to_str().unwrap()).unwrap();
    
    // Basic validation
    assert!(!result.is_empty(), "No classes were parsed from the file");
    
    // Verify CfgPatches class exists
    let cfg_patches = result.iter().find(|c| c.name == Some("CfgPatches".to_string()));
    assert!(cfg_patches.is_some(), "CfgPatches class not found");
    
    result
}

#[test]
fn test_rhs_headband_config() {
    let result = test_config_file("tests/data/@tc_rhs_headband/config.cpp");
    
    // Verify CfgPatches class
    let cfg_patches = result.iter().find(|c| c.name == Some("CfgPatches".to_string())).unwrap();
    
    // Verify tc_rhs_headband class exists
    assert!(cfg_patches.classes.contains_key("tc_rhs_headband"), 
            "tc_rhs_headband class not found in CfgPatches");
    
    // Verify CfgWeapons class
    let cfg_weapons = result.iter().find(|c| c.name == Some("CfgWeapons".to_string())).unwrap();
    
    // Verify tc_rhs_headband class exists
    assert!(cfg_weapons.classes.contains_key("tc_rhs_headband"),
            "tc_rhs_headband class not found in CfgWeapons");
}

#[test]
fn test_mirrorform_config() {
    let result = test_config_file("tests/data/@tc_mirrorform/config.cpp");
    
    // Verify CfgPatches class
    let cfg_patches = result.iter().find(|c| c.name == Some("CfgPatches".to_string())).unwrap();
    
    // Verify TC_MIRROR class exists
    assert!(cfg_patches.classes.contains_key("TC_MIRROR"), 
            "TC_MIRROR class not found in CfgPatches");
    
    // Verify CfgWeapons class
    let cfg_weapons = result.iter().find(|c| c.name == Some("CfgWeapons".to_string())).unwrap();
    
    // Verify classes exist
    assert!(cfg_weapons.classes.contains_key("TC_U_Mirror_Base"),
            "TC_U_Mirror_Base class not found in CfgWeapons");
    
    assert!(cfg_weapons.classes.contains_key("TC_U_Mirror_1"),
            "TC_U_Mirror_1 class not found in CfgWeapons");
    
    // Verify CfgVehicles class
    let cfg_vehicles = result.iter().find(|c| c.name == Some("CfgVehicles".to_string())).unwrap();
    
    // Verify classes exist
    assert!(cfg_vehicles.classes.contains_key("TC_B_Mirror_Base"),
            "TC_B_Mirror_Base class not found in CfgVehicles");
    
    assert!(cfg_vehicles.classes.contains_key("TC_B_Mirror_1"),
            "TC_B_Mirror_1 class not found in CfgVehicles");
}

/// Test that verifies common patterns in config.cpp files
#[test]
fn test_common_config_patterns() {
    // Test both config files
    let rhs_headband = test_config_file("tests/data/@tc_rhs_headband/config.cpp");
    let mirrorform = test_config_file("tests/data/@tc_mirrorform/config.cpp");
    
    // Verify both have CfgPatches and CfgWeapons
    for result in [&rhs_headband, &mirrorform] {
        assert!(result.iter().any(|c| c.name == Some("CfgPatches".to_string())));
        assert!(result.iter().any(|c| c.name == Some("CfgWeapons".to_string())));
    }
    
    // Verify CfgPatches structure in both files
    for result in [&rhs_headband, &mirrorform] {
        let cfg_patches = result.iter().find(|c| c.name == Some("CfgPatches".to_string())).unwrap();
        
        // Each CfgPatches should have exactly one child class
        assert_eq!(cfg_patches.classes.len(), 1, "CfgPatches should have exactly one child class");
        
        // The child class should have units, weapons, and requiredAddons properties
        let child = cfg_patches.classes.values().next().unwrap();
        
        assert!(child.properties.contains_key("units"), "Child class should have units property");
        assert!(child.properties.contains_key("weapons"), "Child class should have weapons property");
        assert!(child.properties.contains_key("requiredAddons"), "Child class should have requiredAddons property");
        
        // Verify these properties are arrays
        assert!(matches!(child.properties.get("units"), Some(Value::Array(_))));
        assert!(matches!(child.properties.get("weapons"), Some(Value::Array(_))));
        assert!(matches!(child.properties.get("requiredAddons"), Some(Value::Array(_))));
    }
} 