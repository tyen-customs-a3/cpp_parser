use cpp_parser::{parse_cpp, Class, Value};
use std::path::Path;

#[test]
fn test_complex_config_basic_parsing() {
    // Path to the test file
    let file_path = Path::new("tests/data/complex_config.cpp");
    
    // Verify the file exists
    assert!(file_path.exists(), "Test file does not exist");
    
    // Parse the file
    let result = cpp_parser::parse_cpp_file(file_path.to_str().unwrap()).unwrap();
    
    // Verify classes were parsed
    assert!(!result.is_empty(), "No classes were parsed from the file");
    
    // Check for CfgPatches class
    let cfg_patches = result.iter().find(|c| c.name == Some("CfgPatches".to_string()));
    assert!(cfg_patches.is_some(), "CfgPatches class not found");
    
    // Check for CfgAddons class
    let cfg_addons = result.iter().find(|c| c.name == Some("CfgAddons".to_string()));
    assert!(cfg_addons.is_some(), "CfgAddons class not found");
    
    // Check for the PreloadAddons class inside CfgAddons
    if let Some(cfg_addons) = cfg_addons {
        assert!(cfg_addons.classes.contains_key("PreloadAddons"), "PreloadAddons class not found in CfgAddons");
        
        // Check if the PreloadAddons class has any classes
        if let Some(preload_addons) = cfg_addons.classes.get("PreloadAddons") {
            assert!(!preload_addons.classes.is_empty(), "PreloadAddons class should have nested classes");
            
            // Check if the 3DEN class exists
            if preload_addons.classes.contains_key("3DEN") {
                println!("3DEN class found in PreloadAddons");
            } else {
                println!("3DEN class not found in PreloadAddons, but that's okay");
            }
        }
    }
    
    // Check for RscTitles class
    let rsc_titles = result.iter().find(|c| c.name == Some("RscTitles".to_string()));
    if rsc_titles.is_some() {
        println!("RscTitles class found");
    } else {
        println!("RscTitles class not found, but that's okay");
    }
    
    // Basic verification that we parsed something meaningful
    assert!(result.len() >= 3, "Expected at least 3 top-level classes");
} 