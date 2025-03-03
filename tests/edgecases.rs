#[path = "test_utils.rs"]
mod test_utils;
use test_utils::*;
use cpp_parser::{parse_cpp, parser::parse_cpp_file, Class, Value, Property};
use std::path::Path;
use std::collections::HashMap;

#[test]
fn test_macro_placeholders() {
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
    
    let result = class_with_parsed_props(input);
    assert_eq!(result.len(), 1);
    
    let cfg_patches = &result[0];
    assert_eq!(cfg_patches.name, "CfgPatches");
    
    let addon_class = find_nested_class(cfg_patches, "ADDON");
    
    // Check various macro reference formats
    check_property_value(addon_class, "name", Value::MacroReference("COMPONENT_NAME".to_string()));
    check_property_value(addon_class, "requiredVersion", Value::MacroReference("REQUIRED_VERSION".to_string()));
    check_property_value(addon_class, "author", Value::MacroReference("ECSTRING(common,ACETeam)".to_string()));
    check_property_value(addon_class, "url", Value::MacroReference("ECSTRING(main,URL)".to_string()));
    
    // Check empty arrays and string arrays
    assert_eq!(check_array_property(addon_class, "units").len(), 0);
    assert_eq!(check_array_property(addon_class, "authors").len(), 1);
    assert_eq!(check_array_property(addon_class, "requiredAddons").len(), 1);
    assert_eq!(check_array_property(addon_class, "requiredAddons")[0], Value::String("ace_medical_status".to_string()));
}

#[test]
fn test_special_syntax_and_expressions() {
    let input = r#"
        class ItemInfo: InventoryUnderItem_Base_F {
            deployedPivot = "bipod_out";
            hasBipod = 1;
            mass = 10;
            soundBipodDown[] = {"A3\Sounds_F_Mark\arsenal\sfx\bipods\Bipod_BLU_down", db-3, 1, 20};
            soundBipodUp[] = {"A3\Sounds_F_Mark\arsenal\sfx\bipods\Bipod_BLU_up", db-3, 1, 20};
        };
        
        class NestedExpressions {
            position[] = {
                "(profileNamespace getVariable ['GUI_BCG_RGB_R', 0])",
                1
            };
            color[] = {
                "(((((safezoneW / safezoneH) min 1.2) / 1.2) / 25) * 0.9)",
                0.7
            };
            sound[] = {"filename", db+5, 1.0, 300};
        };
    "#;
    
    let result = class_with_parsed_props(input);
    assert_eq!(result.len(), 2);
    
    let item_info = &result[0];
    assert_eq!(item_info.name, "ItemInfo");
    assert_eq!(item_info.parent, Some("InventoryUnderItem_Base_F".to_string()));
    
    // Check basic property types
    check_property_value(item_info, "deployedPivot", Value::String("bipod_out".to_string()));
    check_property_value(item_info, "hasBipod", Value::Number(1.0));
    check_property_value(item_info, "mass", Value::Number(10.0));
    
    // Check sound arrays with db-3 special syntax
    let sound_bipod_down = check_array_property(item_info, "soundBipodDown");
    assert_eq!(sound_bipod_down.len(), 4);
    assert_eq!(sound_bipod_down[0], Value::String("A3\\Sounds_F_Mark\\arsenal\\sfx\\bipods\\Bipod_BLU_down".to_string()));
    assert!(matches!(&sound_bipod_down[1], Value::Expression(s) if s == "db-3"));
    
    // Check nested expressions class
    let expressions = &result[1];
    
    // Check position array with complex string expressions
    let position = check_array_property(expressions, "position");
    assert_eq!(position[0], Value::String("(profileNamespace getVariable ['GUI_BCG_RGB_R', 0])".to_string()));
    
    // Check db+5 expression in sound array
    let sound = check_array_property(expressions, "sound");
    assert!(matches!(&sound[1], Value::Expression(s) if s == "db+5"));
}

#[test]
fn test_macro_class_names_and_inheritance() {
    let input = r#"
        class GVAR(RscVisionModes) {
            idd = -1;
        };
        class EGVAR(common,RscLabel) {
            displayText = "Label";
            position[] = {0, 0, 1};
        };
        class ADDON_PREFIX_RscButton: GVAR(BaseButton) {
            size = 1;
        };
        
        // Test with nested classes
        class GVAR(Dialog) {
            idd = IDD_DIALOG_MAIN;
            movingEnable = 0;
            
            class controls {
                class GVAR(Background): RscPicture {
                    idc = IDC_BACKGROUND;
                    text = QPATHTOF(ui\images\background.paa);
                };
            };
        };
    "#;
    
    let result = class_with_parsed_props(input);
    assert_eq!(result.len(), 4);
    
    // Check macro class names
    let rsc_vision = find_class(&result, "GVAR(RscVisionModes)");
    let rsc_label = find_class(&result, "EGVAR(common,RscLabel)");
    let button = find_class(&result, "ADDON_PREFIX_RscButton");
    let dialog = find_class(&result, "GVAR(Dialog)");
    
    // Check inheritance from macro class name
    assert_eq!(button.parent, Some("GVAR(BaseButton)".to_string()));
    
    // Check properties within macro named classes
    check_property_value(rsc_vision, "idd", Value::Number(-1.0));
    check_property_value(rsc_label, "displayText", Value::String("Label".to_string()));
    
    // Check array property
    let position = check_array_property(rsc_label, "position");
    assert_eq!(position.len(), 3);
    
    // Check nested class with macro in name
    let controls = find_nested_class(dialog, "controls");
    let background = find_nested_class(controls, "GVAR(Background)");
    
    // Check nested class with inheritance
    assert_eq!(background.parent, Some("RscPicture".to_string()));
    check_property_value(background, "text", Value::MacroReference("QPATHTOF(ui\\images\\background.paa)".to_string()));
}

#[test]
fn test_complex_macro_expressions() {
    let input = r#"
        // Test complex macros like QUOTE, nested expressions and PREP
        PREP(changeBrightness);
        
        class Extended_PreInit_EventHandlers {
            class ADDON {
                init = QUOTE(call COMPILE_SCRIPT(XEH_preInit));
            };
        };
        
        class Presets {
            class Arma3 {
                class Variables {
                    notification[] = {
                        QUOTE(safezoneX + safezoneW - 60 * GUI_GRID_W),
                        QUOTE(GUI_GRID_H)
                    };
                };
            };
        };
    "#;
    
    let result = class_with_parsed_props(input);
    assert_eq!(result.len(), 2);
    
    // Check event handler class with nested macros
    let event_handlers = find_class(&result, "Extended_PreInit_EventHandlers");
    let addon_class = find_nested_class(event_handlers, "ADDON");
    
    // Check complex nested macro references
    check_property_value(addon_class, "init", Value::MacroReference("QUOTE(call COMPILE_SCRIPT(XEH_preInit))".to_string()));
    
    // Check nested array with quote macros
    let presets = find_class(&result, "Presets");
    let arma3 = find_nested_class(presets, "Arma3");
    let variables = find_nested_class(arma3, "Variables");
    
    let notification = check_array_property(variables, "notification");
    assert_eq!(notification.len(), 2);
    assert!(matches!(&notification[0], Value::MacroReference(s) if s == "QUOTE(safezoneX + safezoneW - 60 * GUI_GRID_W)"));
    assert!(matches!(&notification[1], Value::MacroReference(s) if s == "QUOTE(GUI_GRID_H)"));
}

#[test]
fn test_combined_edge_cases() {
    let input = r#"
        class CfgWeapons {
            class ItemCore;
            class MY_ITEM: ItemCore {
                scope = ITEM_SCOPE;
                displayName = "$STR_MY_ITEM";
                model = MODEL_PATH;
                
                class ItemInfo: ITEM_BASE {
                    mass = ITEM_MASS;
                    soundBipodDown[] = {"path\to\sound", db-3, 1, 20};
                    hiddenSelections[] = {};
                };
            };
        }; // Note: This semicolon was causing the parser to include the next class as part of CfgWeapons
        
        class GR_MUH_MK48_1 : CannonCore {
            weight = 10;
            scope = public;
            displayName = MK48; // Unquoted identifier
            shortNameMagazine = 762x51; // Numeric identifier without quotes
        };
    "#;
    
    let result = class_with_parsed_props(input);
    assert_eq!(result.len(), 2, "Expected 2 top-level classes but found {} classes", result.len());
    
    // Test first class with nested structure
    let cfg_weapons = find_class(&result, "CfgWeapons");
    let my_item = find_nested_class(cfg_weapons, "MY_ITEM");
    
    // Check basic properties
    check_property_value(my_item, "scope", Value::MacroReference("ITEM_SCOPE".to_string()));
    check_property_value(my_item, "displayName", Value::String("$STR_MY_ITEM".to_string()));
    
    // Check nested class with inheritance
    let item_info = find_nested_class(my_item, "ItemInfo");
    assert_eq!(item_info.parent, Some("ITEM_BASE".to_string()));
    
    // Check empty array
    assert_eq!(check_array_property(item_info, "hiddenSelections").len(), 0);
    
    // Check array with db-3 expression
    let sound_array = check_array_property(item_info, "soundBipodDown");
    assert!(matches!(&sound_array[1], Value::Expression(s) if s == "db-3"));
    
    // Check second class with unquoted values
    let weapon = find_class(&result, "GR_MUH_MK48_1");
    
    // Check unquoted identifier (should be parsed as MacroReference) 
    check_property_value(weapon, "displayName", Value::MacroReference("MK48".to_string()));
    
    // Check numeric identifier without quotes (should be parsed as String)
    check_property_value(weapon, "shortNameMagazine", Value::String("762x51".to_string()));
}