use cpp_parser::preprocessor::{preprocess, ConfigElementType};
use std::fs;

#[test]
fn test_complex_config_preprocessing() {
    // Read the complex config file
    let input = fs::read_to_string("tests/data/complex_config.cpp").expect("Failed to read test file");
    
    // Preprocess the file
    let elements = preprocess(&input);
    
    // Verify that we found the expected elements
    let defines = elements.iter().filter(|e| e.element_type == ConfigElementType::Define).count();
    let enums = elements.iter().filter(|e| e.element_type == ConfigElementType::Enum).count();
    let classes = elements.iter().filter(|e| e.element_type == ConfigElementType::Class).count();
    let forward_decls = elements.iter().filter(|e| e.element_type == ConfigElementType::ForwardDeclaration).count();
    let deletes = elements.iter().filter(|e| e.element_type == ConfigElementType::Delete).count();
    
    // We expect at least one define, one enum, multiple classes, some forward declarations, and at least one delete
    assert!(defines >= 1, "Expected at least one #define directive");
    assert!(enums >= 1, "Expected at least one enum");
    assert!(classes >= 5, "Expected at least 5 classes");
    assert!(forward_decls >= 3, "Expected at least 3 forward declarations");
    assert!(deletes >= 1, "Expected at least one delete statement");
    
    // Print out the elements for debugging
    println!("Found {} elements:", elements.len());
    println!("- {} #define directives", defines);
    println!("- {} enums", enums);
    println!("- {} classes", classes);
    println!("- {} forward declarations", forward_decls);
    println!("- {} delete statements", deletes);
    
    // Verify specific elements
    let arma_define = elements.iter().find(|e| 
        e.element_type == ConfigElementType::Define && 
        e.name.as_deref() == Some("_ARMA_")
    );
    assert!(arma_define.is_some(), "Expected to find #define _ARMA_");
    
    let cfg_patches = elements.iter().find(|e| 
        e.element_type == ConfigElementType::Class && 
        e.name.as_deref() == Some("CfgPatches")
    );
    assert!(cfg_patches.is_some(), "Expected to find class CfgPatches");
    
    let cfg_addons = elements.iter().find(|e| 
        e.element_type == ConfigElementType::Class && 
        e.name.as_deref() == Some("CfgAddons")
    );
    assert!(cfg_addons.is_some(), "Expected to find class CfgAddons");
    
    let rsc_text_decl = elements.iter().find(|e| 
        e.element_type == ConfigElementType::ForwardDeclaration && 
        e.name.as_deref() == Some("RscText")
    );
    assert!(rsc_text_decl.is_some(), "Expected to find forward declaration for RscText");
    
    let revive_delete = elements.iter().find(|e| 
        e.element_type == ConfigElementType::Delete && 
        e.name.as_deref() == Some("Revive")
    );
    assert!(revive_delete.is_some(), "Expected to find delete Revive statement");
    
    // Verify that we can extract the content of elements
    if let Some(cfg_patches) = cfg_patches {
        assert!(cfg_patches.content.contains("A3_3DEN"), "Expected CfgPatches to contain A3_3DEN");
    }
} 