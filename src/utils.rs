use crate::model::{Class, Property};
use std::collections::HashSet;

/// Find all classes in a class hierarchy with a specific name
pub fn find_classes_by_name<'a>(classes: &'a [Class], name: &str) -> Vec<&'a Class> {
    let mut result = Vec::new();
    
    for class in classes {
        if class.name == name {
            result.push(class);
        }
        
        // Recursively search in subclasses
        result.extend(find_classes_by_name(&class.subclasses, name));
    }
    
    result
}

/// Find all classes that have a specific parent
pub fn find_classes_by_parent<'a>(classes: &'a [Class], parent_name: &str) -> Vec<&'a Class> {
    let mut result = Vec::new();
    
    for class in classes {
        if let Some(parent) = &class.parent {
            if parent == parent_name {
                result.push(class);
            }
        }
        
        // Recursively search in subclasses
        result.extend(find_classes_by_parent(&class.subclasses, parent_name));
    }
    
    result
}

/// Find all classes that contain a specific string in their properties
pub fn find_classes_with_string<'a>(classes: &'a [Class], search_string: &str) -> Vec<&'a Class> {
    let mut result = Vec::new();
    
    for class in classes {
        let has_string = class.find_strings(|s| s.contains(search_string)).len() > 0;
        
        if has_string {
            result.push(class);
        }
        
        // Also search in subclasses
        result.extend(find_classes_with_string(&class.subclasses, search_string));
    }
    
    result
}

/// Find all unique strings in class properties that match a predicate
pub fn find_all_matching_strings<'a, F>(classes: &'a [Class], predicate: F) -> HashSet<&'a str>
where
    F: Fn(&str) -> bool + Copy,
{
    let mut result = HashSet::new();
    
    for class in classes {
        // Check direct properties
        for found in class.find_strings(predicate) {
            result.insert(found);
        }
        
        // Check subclasses
        let subclass_results = find_all_matching_strings(&class.subclasses, predicate);
        result.extend(subclass_results);
    }
    
    result
}

/// Get a flattened list of all classes in the hierarchy
pub fn get_all_classes<'a>(classes: &'a [Class]) -> Vec<&'a Class> {
    let mut result = Vec::new();
    
    for class in classes {
        result.push(class);
        result.extend(get_all_classes(&class.subclasses));
    }
    
    result
}

/// Get all property values of a certain type from a class
pub fn get_property_values_by_type<'a>(class: &'a Class, property_name: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    
    if let Some(property) = class.get_property(property_name) {
        match property {
            Property::String(value) => {
                result.push(value.as_str());
            }
            Property::Array(values) => {
                for value in values {
                    if let Some(s) = value.as_string() {
                        result.push(s);
                    }
                }
            }
            _ => {}
        }
    }
    
    result
} 