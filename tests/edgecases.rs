#[cfg(test)]
mod tests {
    use cpp_parser::{parse_cpp, parser::parse_cpp_file, Value};
    use std::path::Path;

    #[test]
    fn test_macro_placeholders() {
        // Test that macro placeholders are parsed as strings
        let input = r#"
            class CfgPatches {
                class ADDON {
                    name = COMPONENT_NAME;
                    units[] = {};
                    weapons[] = {};
                    requiredVersion = REQUIRED_VERSION;
                    requiredAddons[] = {"ace_medical_status"};
                    author = ECSTRING(common,ACETeam);
                    authors[] = {""};
                    url = ECSTRING(main,URL);
                    VERSION_CONFIG;
                };
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let cfg_patches = &result[0];
        assert_eq!(cfg_patches.name, "CfgPatches");
        
        let addon_class = &cfg_patches.nested_classes[0];
        assert_eq!(addon_class.name, "ADDON");
        
        // Check that macro placeholders are parsed as strings
        let name = addon_class.properties.get("name").unwrap();
        assert_eq!(name.value, Value::MacroReference("COMPONENT_NAME".to_string()));
        
        let req_version = addon_class.properties.get("requiredVersion").unwrap();
        assert_eq!(req_version.value, Value::MacroReference("REQUIRED_VERSION".to_string()));
        
        let author = addon_class.properties.get("author").unwrap();
        assert_eq!(author.value, Value::MacroReference("ECSTRING(common,ACETeam)".to_string()));
        
        let url = addon_class.properties.get("url").unwrap();
        assert_eq!(url.value, Value::MacroReference("ECSTRING(main,URL)".to_string()));
        
        // Check empty arrays
        let units = addon_class.properties.get("units").unwrap();
        if let Value::Array(values) = &units.value {
            assert_eq!(values.len(), 0);
        } else {
            panic!("Expected array value for units");
        }
        
        // Check arrays with single empty string
        let authors = addon_class.properties.get("authors").unwrap();
        if let Value::Array(values) = &authors.value {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], Value::String("".to_string()));
        } else {
            panic!("Expected array value for authors");
        }
        
        // Check arrays with string values
        let required_addons = addon_class.properties.get("requiredAddons").unwrap();
        if let Value::Array(values) = &required_addons.value {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], Value::String("ace_medical_status".to_string()));
        } else {
            panic!("Expected array value for requiredAddons");
        }
    }

    #[test]
    fn test_sound_arrays_with_special_syntax() {
        // Test arrays with special syntax like db-3
        let input = r#"
            class ItemInfo: InventoryUnderItem_Base_F
            {
                deployedPivot="bipod_out";
                hasBipod=1;
                mass=10;
                soundBipodDown[]={"A3\Sounds_F_Mark\arsenal\sfx\bipods\Bipod_BLU_down",db-3, 1, 20};
                soundBipodUp[]={"A3\Sounds_F_Mark\arsenal\sfx\bipods\Bipod_BLU_up",db-3, 1, 20};
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let item_info = &result[0];
        assert_eq!(item_info.name, "ItemInfo");
        assert_eq!(item_info.parent, Some("InventoryUnderItem_Base_F".to_string()));
        
        // Check numeric and string properties
        assert_eq!(item_info.properties.get("hasBipod").unwrap().value, Value::Number(1.0));
        assert_eq!(item_info.properties.get("mass").unwrap().value, Value::Number(10.0));
        assert_eq!(
            item_info.properties.get("deployedPivot").unwrap().value,
            Value::String("bipod_out".to_string())
        );
        
        // Check sound arrays with db-3 special syntax
        let sound_bipod_down = item_info.properties.get("soundBipodDown").unwrap();
        if let Value::Array(values) = &sound_bipod_down.value {
            assert_eq!(values.len(), 4);
            assert_eq!(values[0], Value::String("A3\\Sounds_F_Mark\\arsenal\\sfx\\bipods\\Bipod_BLU_down".to_string()));
            // The db-3 should be parsed as a special token or expression
            assert!(matches!(values[1], Value::Expression(_)));
            assert_eq!(values[2], Value::Number(1.0));
            assert_eq!(values[3], Value::Number(20.0));
        } else {
            panic!("Expected array value for soundBipodDown");
        }
        
        let sound_bipod_up = item_info.properties.get("soundBipodUp").unwrap();
        if let Value::Array(values) = &sound_bipod_up.value {
            assert_eq!(values.len(), 4);
            assert_eq!(values[0], Value::String("A3\\Sounds_F_Mark\\arsenal\\sfx\\bipods\\Bipod_BLU_up".to_string()));
            // The db-3 should be parsed as a special token or expression
            assert!(matches!(values[1], Value::Expression(_)));
            assert_eq!(values[2], Value::Number(1.0));
            assert_eq!(values[3], Value::Number(20.0));
        } else {
            panic!("Expected array value for soundBipodUp");
        }
    }

    #[test]
    fn test_combined_edge_cases() {
        // Test a combination of edge cases including:
        // - Macro placeholders
        // - Empty arrays
        // - Arrays with special syntax
        // - Nested classes with inheritance
        let input = r#"
            class CfgWeapons {
                class ItemCore;
                class MY_ITEM: ItemCore {
                    scope = ITEM_SCOPE;
                    displayName = "$STR_MY_ITEM";
                    model = MODEL_PATH;
                    
                    class ItemInfo: ITEM_BASE {
                        mass = ITEM_MASS;
                        uniformModel = MODEL_PATH;
                        soundBipodDown[]={"path\to\sound",db-3, 1, 20};
                        modelSpecial = "";
                        hiddenSelections[] = {};
                    };
                };
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let cfg_weapons = &result[0];
        assert_eq!(cfg_weapons.name, "CfgWeapons");
        
        let my_item = &cfg_weapons.nested_classes[1];
        assert_eq!(my_item.name, "MY_ITEM");
        assert_eq!(my_item.parent, Some("ItemCore".to_string()));
        
        // Check macro placeholders
        let scope = my_item.properties.get("scope").unwrap();
        assert_eq!(scope.value, Value::MacroReference("ITEM_SCOPE".to_string()));
        
        let display_name = my_item.properties.get("displayName").unwrap();
        assert_eq!(display_name.value, Value::String("$STR_MY_ITEM".to_string()));
        
        let model = my_item.properties.get("model").unwrap();
        assert_eq!(model.value, Value::MacroReference("MODEL_PATH".to_string()));
        
        // Check nested class with inheritance
        let item_info = &my_item.nested_classes[0];
        assert_eq!(item_info.name, "ItemInfo");
        assert_eq!(item_info.parent, Some("ITEM_BASE".to_string()));
        
        // Check numeric and string properties
        let mass = item_info.properties.get("mass").unwrap();
        assert_eq!(mass.value, Value::MacroReference("ITEM_MASS".to_string()));
        
        let uniform_model = item_info.properties.get("uniformModel").unwrap();
        assert_eq!(uniform_model.value, Value::MacroReference("MODEL_PATH".to_string()));
        
        // Check sound arrays with db-3 special syntax
        let sound_bipod_down = item_info.properties.get("soundBipodDown").unwrap();
        if let Value::Array(values) = &sound_bipod_down.value {
            assert_eq!(values.len(), 4);
            assert_eq!(values[0], Value::String("path\\to\\sound".to_string()));
            // The db-3 should be parsed as a special token or expression
            assert!(matches!(values[1], Value::Expression(_)));
            assert_eq!(values[2], Value::Number(1.0));
            assert_eq!(values[3], Value::Number(20.0));
        } else {
            panic!("Expected array value for soundBipodDown");
        }
        
        // Check empty arrays
        let model_special = item_info.properties.get("modelSpecial").unwrap();
        assert_eq!(model_special.value, Value::String("".to_string()));
        
        let hidden_selections = item_info.properties.get("hiddenSelections").unwrap();
        if let Value::Array(values) = &hidden_selections.value {
            assert_eq!(values.len(), 0);
        } else {
            panic!("Expected array value for hiddenSelections");
        }
    }
    
    #[test]
    fn test_complex_macros_and_grid_expressions() {
        // Test complex macros like QUOTE and grid expressions
        let input = r#"
            class Presets {
                class Arma3 {
                    class Variables {
                        pca_suture_medicNotification[] = {
                            {
                                QUOTE((safeZoneX + ((46) * GUI_GRID_W))),
                                QUOTE((safeZoneY + ((0.6) * GUI_GRID_H))),
                                "((safeZoneW / 15)/1.2)",
                                "(safeZoneW / 15)"
                            },
                            QUOTE(GUI_GRID_W),
                            QUOTE(GUI_GRID_H)
                        };
                    };
                };
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let presets = &result[0];
        assert_eq!(presets.name, "Presets");
        
        let arma3 = &presets.nested_classes[0];
        assert_eq!(arma3.name, "Arma3");
        
        let variables = &arma3.nested_classes[0];
        assert_eq!(variables.name, "Variables");
        
        // Check the complex array with nested structures and macros
        let notification = variables.properties.get("pca_suture_medicNotification").unwrap();
        if let Value::Array(values) = &notification.value {
            // Should have two main items: an array and two QUOTE macros
            assert_eq!(values.len(), 3);
            
            // Check first item: nested array with complex expressions
            if let Value::Array(nested_array) = &values[0] {
                assert_eq!(nested_array.len(), 4);
                
                // Check QUOTE macros are parsed as MacroReference
                assert!(matches!(&nested_array[0], Value::MacroReference(s) if s.starts_with("QUOTE")));
                assert!(matches!(&nested_array[1], Value::MacroReference(s) if s.starts_with("QUOTE")));
                
                // Check the string expressions
                assert_eq!(nested_array[2], Value::String("((safeZoneW / 15)/1.2)".to_string()));
                assert_eq!(nested_array[3], Value::String("(safeZoneW / 15)".to_string()));
            } else {
                panic!("Expected array for first item");
            }
            
            // Check QUOTE macros
            assert!(matches!(&values[1], Value::MacroReference(s) if s == "QUOTE(GUI_GRID_W)"));
            assert!(matches!(&values[2], Value::MacroReference(s) if s == "QUOTE(GUI_GRID_H)"));
        } else {
            panic!("Expected array value for pca_suture_medicNotification");
        }
    }
    
    #[test]
    fn test_weapon_class_with_macros() {
        // Test a weapon class with mixed string and macro values
        let input = r#"
            class GR_MUH_MK48_1 : CannonCore {
                weight = 10;
                scope = public;
                displayName = MK48;
                displayNameMagazine = "4000Rnd_762x51_Belt";
                shortNameMagazine = 762x51;
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let weapon = &result[0];
        assert_eq!(weapon.name, "GR_MUH_MK48_1");
        assert_eq!(weapon.parent, Some("CannonCore".to_string()));
        
        // Check numeric value
        assert_eq!(weapon.properties.get("weight").unwrap().value, Value::Number(10.0));
        
        // Check enums/macros without quotes
        assert!(matches!(&weapon.properties.get("scope").unwrap().value, Value::String(s) if s == "public"));
        
        // This is a macro without quotes, should be parsed as a MacroReference
        assert!(matches!(&weapon.properties.get("displayName").unwrap().value, Value::MacroReference(s) if s == "MK48"));
        
        // This has quotes, should be a string
        assert_eq!(
            weapon.properties.get("displayNameMagazine").unwrap().value,
            Value::String("4000Rnd_762x51_Belt".to_string())
        );
        
        // This is numeric without quotes, should be parsed as a string
        assert!(matches!(&weapon.properties.get("shortNameMagazine").unwrap().value, Value::String(s) if s == "762x51"));
    }
    
    #[test]
    fn test_nested_expressions_and_quotes() {
        // Test nested expressions with quotes and parentheses
        let input = r#"
            class NestedExpressions {
                position[] = {
                    "(profileNamespace getVariable ['GUI_BCG_RGB_R', 0])",
                    "(profileNamespace getVariable ['GUI_BCG_RGB_G', 0])",
                    "(profileNamespace getVariable ['GUI_BCG_RGB_B', 0])",
                    1
                };
                color[] = {
                    "(((((safezoneW / safezoneH) min 1.2) / 1.2) / 25) * 0.9)",
                    "safezoneW / 40",
                    1,
                    0.7
                };
                sound[] = {"filename", db+5, 1.0, 300};
            };
        "#;
        
        let result = parse_cpp(input).unwrap();
        assert_eq!(result.len(), 1);
        
        let expressions = &result[0];
        assert_eq!(expressions.name, "NestedExpressions");
        
        // Check position array with profile namespace expressions
        let position = expressions.properties.get("position").unwrap();
        if let Value::Array(values) = &position.value {
            assert_eq!(values.len(), 4);
            
            // Check string expressions
            assert_eq!(values[0], Value::String("(profileNamespace getVariable ['GUI_BCG_RGB_R', 0])".to_string()));
            assert_eq!(values[1], Value::String("(profileNamespace getVariable ['GUI_BCG_RGB_G', 0])".to_string()));
            assert_eq!(values[2], Value::String("(profileNamespace getVariable ['GUI_BCG_RGB_B', 0])".to_string()));
            assert_eq!(values[3], Value::Number(1.0));
        } else {
            panic!("Expected array value for position");
        }
        
        // Check color array with complex expressions
        let color = expressions.properties.get("color").unwrap();
        if let Value::Array(values) = &color.value {
            assert_eq!(values.len(), 4);
            
            // Check string expression with complex math
            assert_eq!(values[0], Value::String("(((((safezoneW / safezoneH) min 1.2) / 1.2) / 25) * 0.9)".to_string()));
            assert_eq!(values[1], Value::String("safezoneW / 40".to_string()));
            assert_eq!(values[2], Value::Number(1.0));
            assert_eq!(values[3], Value::Number(0.7));
        } else {
            panic!("Expected array value for color");
        }
        
        // Check sound array with db+5 expression
        let sound = expressions.properties.get("sound").unwrap();
        if let Value::Array(values) = &sound.value {
            assert_eq!(values.len(), 4);
            
            assert_eq!(values[0], Value::String("filename".to_string()));
            // Check that db+5 is parsed as an expression
            assert!(matches!(&values[1], Value::Expression(s) if s == "db+5"));
            assert_eq!(values[2], Value::Number(1.0));
            assert_eq!(values[3], Value::Number(300.0));
        } else {
            panic!("Expected array value for sound");
        }
    }
}