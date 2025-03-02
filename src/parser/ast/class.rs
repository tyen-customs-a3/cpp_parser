use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::property::Property;

/// Represents a class in a C++ config file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub parent: Option<String>,
    pub properties: HashMap<String, Property>,
    pub nested_classes: Vec<Class>,
}

impl Class {
    pub fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
            properties: HashMap::new(),
            nested_classes: Vec::new(),
        }
    }

    pub fn find_class(&self, name: &str) -> Option<&Class> {
        if name.contains('.') {
            let parts: Vec<&str> = name.split('.').collect();
            let first = parts[0];
            let rest = parts[1..].join(".");
            
            if let Some(class) = self.nested_classes.iter().find(|c| c.name == first) {
                return class.find_class(&rest);
            }
            None
        } else {
            self.nested_classes.iter().find(|c| c.name == name)
        }
    }

    pub fn find_property(&self, name: &str) -> Option<&Property> {
        if name.contains('.') {
            let parts: Vec<&str> = name.split('.').collect();
            let class_path = parts[..parts.len()-1].join(".");
            let prop_name = parts[parts.len()-1];
            
            if let Some(class) = self.find_class(&class_path) {
                return class.properties.get(prop_name);
            }
            None
        } else {
            self.properties.get(name)
        }
    }
}