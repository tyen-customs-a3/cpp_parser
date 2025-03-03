use cpp_parser::{Class, Value, parse_cpp};

pub fn check_property_value(class: &Class, property_name: &str, expected_value: Value) {
    let actual = class.properties.get(property_name).unwrap();
    assert_eq!(actual.value, expected_value, "Property '{}' value mismatch", property_name);
}

pub fn check_array_property<'a>(class: &'a Class, property_name: &str) -> &'a Vec<Value> {
    let prop = class.properties.get(property_name).unwrap();
    if let Value::Array(values) = &prop.value {
        values
    } else {
        panic!("Expected array value for {}", property_name);
    }
}

pub fn find_nested_class<'a>(class: &'a Class, name: &str) -> &'a Class {
    class.nested_classes.iter()
        .find(|c| c.name == name)
        .unwrap_or_else(|| panic!("Nested class '{}' not found", name))
}

pub fn find_class<'a>(classes: &'a [Class], name: &str) -> &'a Class {
    classes.iter()
        .find(|c| c.name == name)
        .unwrap_or_else(|| panic!("Class '{}' not found", name))
}

pub fn class_with_parsed_props(input: &str) -> Vec<Class> {
    cpp_parser::parse_cpp(input).unwrap_or_else(|e| panic!("Failed to parse: {}", e))
} 