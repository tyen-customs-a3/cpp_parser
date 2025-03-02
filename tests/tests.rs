#[cfg(test)]
mod tests {
    use cpp_parser::{parse_cpp, parser::parse_cpp_file, Value};

    use super::*;
    use std::path::Path;

    #[test]
    fn test_parse_enums() {
        let input = r#"
            enum {
                _fnc_sizeex = 0,
                destructengine = 2,
                destructtree = 3,
                destructtent = 4
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        // Enum definitions are stored internally and not returned as classes
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_class() {
        let input = r#"
            enum {
                value1 = 0,
                value2 = 1
            };

            class TestClass {
                author = "$STR_A3_Bohemia_Interactive";
                value = 42;
                enumValue = value1;
                array[] = {"test", 1, value2};
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let class = &result[0];
        assert_eq!(class.name, "TestClass");
        
        let author = class.properties.get("author").unwrap();
        assert_eq!(author.value, Value::String("$STR_A3_Bohemia_Interactive".to_string()));
        
        let value = class.properties.get("value").unwrap();
        assert_eq!(value.value, Value::Number(42.0));
        
        let enum_value = class.properties.get("enumValue").unwrap();
        assert_eq!(enum_value.value, Value::Enum(0));
        
        let array = class.properties.get("array").unwrap();
        if let Value::Array(values) = &array.value {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], Value::String("test".to_string()));
            assert_eq!(values[1], Value::Number(1.0));
            assert_eq!(values[2], Value::Enum(1));
        } else {
            panic!("Expected array value");
        }
    }

    #[test]
    fn test_nested_classes() {
        let input = r#"
            class Parent {
                value = 1;
                class Child {
                    value = 2;
                    class GrandChild {
                        value = 3;
                    };
                };
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let parent = &result[0];
        assert_eq!(parent.nested_classes.len(), 1);
        
        let child = &parent.nested_classes[0];
        assert_eq!(child.nested_classes.len(), 1);
        
        let grandchild = &child.nested_classes[0];
        assert_eq!(grandchild.properties.get("value").unwrap().value, Value::Number(3.0));
    }

    #[test]
    fn test_class_inheritance() {
        let input = r#"
            class Parent {
                value = 1;
            };
            
            class Child: Parent {
                childValue = 2;
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 2);
        
        let parent = &result[0];
        let child = &result[1];
        
        assert_eq!(parent.name, "Parent");
        assert_eq!(parent.parent, None);
        
        assert_eq!(child.name, "Child");
        assert_eq!(child.parent, Some("Parent".to_string()));
    }

    #[test]
    fn test_comments() {
        let input = r#"
            // Top level comment
            class Test {
                // Property comment
                value1 = 1; // Inline comment
                /* Block comment
                   Multiple lines */
                value2 = 2;
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let class = &result[0];
        assert_eq!(class.properties.get("value1").unwrap().value, Value::Number(1.0));
        assert_eq!(class.properties.get("value2").unwrap().value, Value::Number(2.0));
    }

    #[test]
    fn test_complex_config() {
        let test_file = Path::new("tests/data/complex_config.cpp");
        let result = parse_cpp_file(test_file.to_str().unwrap()).unwrap();
        
        // Check the enums in the first class
        let cfgpatches = result.iter().find(|c| c.name == "CfgPatches").unwrap();
        let a3_3den = cfgpatches.nested_classes.iter().find(|c| c.name == "A3_3DEN").unwrap();
        
        // Check author property
        assert_eq!(
            a3_3den.properties.get("author").unwrap().value,
            Value::String("$STR_A3_Bohemia_Interactive".to_string())
        );
        
        // Check url property
        assert_eq!(
            a3_3den.properties.get("url").unwrap().value,
            Value::String("https://www.arma3.com".to_string())
        );
        
        // Check arrays
        let units = a3_3den.properties.get("units").unwrap();
        if let Value::Array(values) = &units.value {
            assert_eq!(values.len(), 2);
            assert_eq!(values[0], Value::String("Sphere_3DEN".to_string()));
            assert_eq!(values[1], Value::String("SphereNoGround_3DEN".to_string()));
        } else {
            panic!("Expected array value for units");
        }
    }

    #[test]
    fn test_preprocessor_directives() {
        let test_file = Path::new("tests/data/preprocessor_test.cpp");
        let result = parse_cpp_file(test_file.to_str().unwrap()).unwrap();
        
        assert_eq!(result.len(), 1);
        
        let cfg_patches = &result[0];
        assert_eq!(cfg_patches.name, "CfgPatches");
        
        // Since the preprocessor directives are handled by the C++ compiler before parsing,
        // our parser should ignore them and just parse the content inside the #else block
        assert_eq!(cfg_patches.nested_classes.len(), 1);
        
        let ace_compat = &cfg_patches.nested_classes[0];
        assert_eq!(ace_compat.name, "ace_compat_gm");
        
        // Check some properties
        assert_eq!(
            ace_compat.properties.get("author").unwrap().value,
            Value::String("ACE Team".to_string())
        );
        
        assert_eq!(
            ace_compat.properties.get("name").unwrap().value,
            Value::String("Ace gm not compat".to_string())
        );
        
        // Check arrays
        let ammo = ace_compat.properties.get("ammo").unwrap();
        if let Value::Array(values) = &ammo.value {
            assert_eq!(values.len(), 0);
        } else {
            panic!("Expected array value for ammo");
        }
        
        let required_addons = ace_compat.properties.get("requiredAddons").unwrap();
        if let Value::Array(values) = &required_addons.value {
            assert_eq!(values.len(), 2);
            assert_eq!(values[0], Value::String("ace_common".to_string()));
            assert_eq!(values[1], Value::String("gm_core".to_string()));
        } else {
            panic!("Expected array value for requiredAddons");
        }
        
        let version_ar = ace_compat.properties.get("versionAr").unwrap();
        if let Value::Array(values) = &version_ar.value {
            assert_eq!(values.len(), 4);
            assert_eq!(values[0], Value::Number(3.0));
            assert_eq!(values[1], Value::Number(16.0));
            assert_eq!(values[2], Value::Number(3.0));
            assert_eq!(values[3], Value::Number(79.0));
        } else {
            panic!("Expected array value for versionAr");
        }
    }

    #[test]
    fn test_yellow_farmer_loadout() {
        let test_file = Path::new("tests/data/p_yellow_farmer_loadout.hpp");
        let result = parse_cpp_file(test_file.to_str().unwrap()).unwrap();
        
        // Check that we have the base class and derived classes
        assert!(result.len() > 0);
        
        // Find the baseMan class
        let base_man = result.iter().find(|c| c.name == "baseMan").unwrap();
        
        // Check that baseMan has the expected properties
        assert!(base_man.properties.contains_key("displayName"));
        assert_eq!(
            base_man.properties.get("displayName").unwrap().value,
            Value::String("Unarmed".to_string())
        );
        
        // Check that baseMan has array properties
        assert!(base_man.properties.contains_key("linkedItems"));
        let linked_items = base_man.properties.get("linkedItems").unwrap();
        if let Value::Array(values) = &linked_items.value {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], Value::String("ItemWatch".to_string()));
            assert_eq!(values[1], Value::String("ItemMap".to_string()));
            assert_eq!(values[2], Value::String("ItemCompass".to_string()));
        } else {
            panic!("Expected array value for linkedItems");
        }
        
        // Find the rifleman class
        let rifleman = result.iter().find(|c| c.name == "rm").unwrap();
        
        // Check that rifleman inherits from baseMan
        assert_eq!(rifleman.parent, Some("baseMan".to_string()));
        
        // Check rifleman's uniform
        assert!(rifleman.properties.contains_key("uniform"));
        let uniform = rifleman.properties.get("uniform").unwrap();
        if let Value::Array(values) = &uniform.value {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], Value::String("CUP_U_C_Villager_02".to_string()));
        } else {
            panic!("Expected array value for uniform");
        }
        
        // Check rifleman's primary weapon
        assert!(rifleman.properties.contains_key("primaryWeapon"));
        let primary_weapon = rifleman.properties.get("primaryWeapon").unwrap();
        if let Value::Array(values) = &primary_weapon.value {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], Value::String("rhs_weap_Izh18".to_string()));
        } else {
            panic!("Expected array value for primaryWeapon");
        }
        
        // Find the team leader class
        let team_leader = result.iter().find(|c| c.name == "tl").unwrap();
        
        // Check that team leader inherits from rifleman
        assert_eq!(team_leader.parent, Some("rm".to_string()));
        
        // Check team leader's additional items
        assert!(team_leader.properties.contains_key("items"));
        
        // Find the combat life saver class
        let cls = result.iter().find(|c| c.name == "cls").unwrap();
        
        // Check that cls inherits from rifleman
        assert_eq!(cls.parent, Some("rm".to_string()));
        
        // Check cls's backpack
        assert!(cls.properties.contains_key("backpack"));
        let backpack = cls.properties.get("backpack").unwrap();
        if let Value::Array(values) = &backpack.value {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], Value::String("rhs_sidor".to_string()));
        } else {
            panic!("Expected array value for backpack");
        }
        
        // Check cls's traits
        assert!(cls.properties.contains_key("traits"));
        let traits = cls.properties.get("traits").unwrap();
        if let Value::Array(values) = &traits.value {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], Value::String("medic".to_string()));
        } else {
            panic!("Expected array value for traits");
        }
    }
    
    #[test]
    fn test_blue_farmer_loadout() {
        let test_file = Path::new("tests/data/p_blue_farmer_loadout.hpp");
        let result = parse_cpp_file(test_file.to_str().unwrap()).unwrap();
        
        // Check that we have the base class and derived classes
        assert!(result.len() > 0);
        
        // Find the baseMan class
        let base_man = result.iter().find(|c| c.name == "baseMan").unwrap();
        
        // Check that baseMan has the expected properties
        assert!(base_man.properties.contains_key("displayName"));
        assert_eq!(
            base_man.properties.get("displayName").unwrap().value,
            Value::String("Unarmed".to_string())
        );
        
        // Find the rifleman class
        let rifleman = result.iter().find(|c| c.name == "rm").unwrap();
        
        // Check that rifleman inherits from baseMan
        assert_eq!(rifleman.parent, Some("baseMan".to_string()));
        
        // Check rifleman's uniform - should be different from yellow farmer
        assert!(rifleman.properties.contains_key("uniform"));
        let uniform = rifleman.properties.get("uniform").unwrap();
        if let Value::Array(values) = &uniform.value {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], Value::String("CUP_U_C_Villager_01".to_string()));
        } else {
            panic!("Expected array value for uniform");
        }
        
        // Check rifleman's vest
        assert!(rifleman.properties.contains_key("vest"));
        let vest = rifleman.properties.get("vest").unwrap();
        if let Value::Array(values) = &vest.value {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], Value::String("rhs_6sh46".to_string()));
        } else {
            panic!("Expected array value for vest");
        }
        
        // Check rifleman's headgear options
        assert!(rifleman.properties.contains_key("headgear"));
        let headgear = rifleman.properties.get("headgear").unwrap();
        if let Value::Array(values) = &headgear.value {
            assert_eq!(values.len(), 4);
            assert_eq!(values[0], Value::String("CUP_H_C_Beret_01".to_string()));
            assert_eq!(values[1], Value::String("CUP_H_C_Beret_02".to_string()));
            assert_eq!(values[2], Value::String("CUP_H_C_Beret_03".to_string()));
            assert_eq!(values[3], Value::String("CUP_H_C_Beret_04".to_string()));
        } else {
            panic!("Expected array value for headgear");
        }
        
        // Check rifleman's magazines
        assert!(rifleman.properties.contains_key("magazines"));
        let magazines = rifleman.properties.get("magazines").unwrap();
        if let Value::Array(values) = &magazines.value {
            // The LIST_x macro is likely expanded during preprocessing
            // We can check that there are at least some values
            assert!(values.len() > 0);
        } else {
            panic!("Expected array value for magazines");
        }
    }
    
    #[test]
    fn test_equipment_items_can_be_found() {
        // Test that we can find specific equipment items in both loadout files
        let yellow_file = Path::new("tests/data/p_yellow_farmer_loadout.hpp");
        let yellow_result = parse_cpp_file(yellow_file.to_str().unwrap()).unwrap();
        
        let blue_file = Path::new("tests/data/p_blue_farmer_loadout.hpp");
        let blue_result = parse_cpp_file(blue_file.to_str().unwrap()).unwrap();
        
        // Function to check if an equipment item exists in a loadout
        fn has_equipment_item(classes: &[cpp_parser::Class], item_name: &str) -> bool {
            for class in classes {
                // Check in uniform, vest, backpack, headgear, primaryWeapon, etc.
                for property_name in &["uniform", "vest", "backpack", "headgear", "primaryWeapon", 
                                      "secondaryWeapon", "sidearmWeapon", "items", "linkedItems"] {
                    if let Some(property) = class.properties.get(*property_name) {
                        if let Value::Array(values) = &property.value {
                            for value in values {
                                if let Value::String(s) = value {
                                    if s == item_name {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Check in backpackItems
                if let Some(property) = class.properties.get("backpackItems") {
                    if let Value::Array(values) = &property.value {
                        for value in values {
                            if let Value::String(s) = value {
                                if s == item_name {
                                    return true;
                                }
                            }
                        }
                    }
                }
                
                // Recursively check nested classes
                if !class.nested_classes.is_empty() {
                    if has_equipment_item(&class.nested_classes, item_name) {
                        return true;
                    }
                }
            }
            false
        }
        
        // Check for specific equipment items in yellow farmer loadout
        assert!(has_equipment_item(&yellow_result, "CUP_U_C_Villager_02"));
        assert!(has_equipment_item(&yellow_result, "rhs_6sh46"));
        assert!(has_equipment_item(&yellow_result, "rhs_weap_Izh18"));
        assert!(has_equipment_item(&yellow_result, "ACRE_PRC343"));
        assert!(has_equipment_item(&yellow_result, "ACE_fieldDressing"));
        assert!(has_equipment_item(&yellow_result, "rhs_sidor"));
        
        // Check for specific equipment items in blue farmer loadout
        assert!(has_equipment_item(&blue_result, "CUP_U_C_Villager_01"));
        assert!(has_equipment_item(&blue_result, "rhs_6sh46"));
        assert!(has_equipment_item(&blue_result, "CUP_H_C_Beret_01"));
        assert!(has_equipment_item(&blue_result, "ACRE_PRC343"));
        
        // Check that yellow-specific items are not in blue loadout
        assert!(!has_equipment_item(&blue_result, "CUP_U_C_Villager_02"));
        
        // Check that blue-specific items are not in yellow loadout
        assert!(!has_equipment_item(&yellow_result, "CUP_U_C_Villager_01"));
    }

    #[test]
    fn test_ace_medical_items() {
        let test_file = Path::new("tests/data/ACE_Medical_Treatment.hpp");
        let result = parse_cpp_file(test_file.to_str().unwrap()).unwrap();
        
        // Check that we have the ADDON class
        let addon = result.iter().find(|c| c.name == "ADDON").unwrap();
        
        // Check that we have the Medication class
        let medication = addon.nested_classes.iter().find(|c| c.name == "Medication").unwrap();
        
        // Check that we have the Morphine class
        let morphine = medication.nested_classes.iter().find(|c| c.name == "Morphine").unwrap();
        assert_eq!(morphine.properties.get("painReduce").unwrap().value, Value::Number(0.8));
        
        // Check that we have the Epinephrine class
        let epinephrine = medication.nested_classes.iter().find(|c| c.name == "Epinephrine").unwrap();
        assert_eq!(epinephrine.properties.get("maxDose").unwrap().value, Value::Number(10.0));
        
        // Check that we have the IV class
        let iv = addon.nested_classes.iter().find(|c| c.name == "IV").unwrap();
        
        // Check that we have the BloodIV class
        let blood_iv = iv.nested_classes.iter().find(|c| c.name == "BloodIV").unwrap();
        assert_eq!(blood_iv.properties.get("volume").unwrap().value, Value::Number(1000.0));
    }
}