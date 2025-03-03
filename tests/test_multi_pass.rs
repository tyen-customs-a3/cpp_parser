use cpp_parser::{parse_cpp, Class, Value};

#[test]
fn test_basic_class_parsing() {
    let input = r#"
    class TestClass {
        property1 = "value1";
        property2 = 123;
        property3[] = {1, 2, 3};
    };
    "#;
    
    let result = parse_cpp(input).unwrap();
    
    // Check that we have one class
    assert_eq!(result.len(), 1);
    
    // Check the class properties
    let class = &result[0];
    assert_eq!(class.name, Some("TestClass".to_string()));
    assert_eq!(class.properties.len(), 3);
    
    // Check property values
    if let Some(Value::String(value)) = class.properties.get("property1") {
        assert_eq!(value, "value1");
    } else {
        panic!("property1 not found or not a string");
    }
    
    if let Some(Value::Integer(value)) = class.properties.get("property2") {
        assert_eq!(*value, 123);
    } else {
        panic!("property2 not found or not an integer");
    }
    
    if let Some(Value::Array(values)) = class.properties.get("property3") {
        assert_eq!(values.len(), 3);
    } else {
        panic!("property3 not found or not an array");
    }
}

#[test]
fn test_nested_class_parsing() {
    let input = r#"
    class ParentClass {
        parentProp = "parent";
        
        class NestedClass {
            nestedProp = "nested";
        };
    };
    "#;
    
    let result = parse_cpp(input).unwrap();
    
    // Check that we have one parent class
    assert_eq!(result.len(), 1);
    
    // Check the parent class
    let parent = &result[0];
    assert_eq!(parent.name, Some("ParentClass".to_string()));
    assert_eq!(parent.classes.len(), 1);
    
    // Check the nested class
    if let Some(nested) = parent.classes.get("NestedClass") {
        assert_eq!(nested.name, Some("NestedClass".to_string()));
        
        if let Some(Value::String(value)) = nested.properties.get("nestedProp") {
            assert_eq!(value, "nested");
        } else {
            panic!("nestedProp not found or not a string");
        }
    } else {
        panic!("NestedClass not found");
    }
}

#[test]
fn test_class_inheritance() {
    let input = r#"
    class BaseClass {
        baseProp = "base";
    };
    
    class ChildClass: BaseClass {
        childProp = "child";
    };
    "#;
    
    let result = parse_cpp(input).unwrap();
    
    // Check that we have two classes
    assert_eq!(result.len(), 2);
    
    // Find the child class
    let child = result.iter().find(|c| c.name == Some("ChildClass".to_string())).unwrap();
    
    // Check inheritance
    assert_eq!(child.parent, Some("BaseClass".to_string()));
    assert_eq!(child.properties.len(), 1);
    
    if let Some(Value::String(value)) = child.properties.get("childProp") {
        assert_eq!(value, "child");
    } else {
        panic!("childProp not found or not a string");
    }
} 