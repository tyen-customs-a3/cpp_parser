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
}