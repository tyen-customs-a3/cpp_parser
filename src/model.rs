use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Represents a class in the configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Class {
    /// Name of the class
    pub name: String,
    
    /// Optional parent class name (note: inheritance chain is not walked for properties)
    pub parent: Option<String>,
    
    /// Properties defined in this class
    pub properties: HashMap<String, Property>,
    
    /// Subclasses defined within this class
    pub subclasses: Vec<Class>,
}

impl Class {
    /// Create a new class with the given name and optional parent
    pub fn new(name: impl Into<String>, parent: Option<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            parent: parent.map(|p| p.into()),
            properties: HashMap::new(),
            subclasses: Vec::new(),
        }
    }
    
    /// Add a property to this class
    pub fn add_property(&mut self, name: impl Into<String>, value: Property) {
        self.properties.insert(name.into(), value);
    }
    
    /// Add a subclass to this class
    pub fn add_subclass(&mut self, class: Class) {
        self.subclasses.push(class);
    }
    
    /// Check if this class contains a property with the given name
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }
    
    /// Get a property by name (note: does not check parent class properties)
    pub fn get_property(&self, name: &str) -> Option<&Property> {
        self.properties.get(name)
    }
    
    /// Find all string values in this class and its properties that match a predicate
    pub fn find_strings<F>(&self, predicate: F) -> Vec<&str>
    where
        F: Fn(&str) -> bool + Copy,
    {
        let mut result = Vec::new();
        
        // Check properties
        for prop in self.properties.values() {
            match prop {
                Property::String(s) if predicate(s) => {
                    result.push(s.as_str());
                },
                Property::Array(items) => {
                    for item in items {
                        if predicate(item.as_string().unwrap_or_default()) {
                            result.push(item.as_string().unwrap_or_default());
                        }
                    }
                },
                _ => {}
            }
        }
        
        // Check subclasses
        for subclass in &self.subclasses {
            result.extend(subclass.find_strings(predicate));
        }
        
        result
    }
}

/// Represents a property value in a class
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Property {
    /// String value
    String(String),
    
    /// Numeric value
    Number(f64),
    
    /// Boolean value
    Boolean(bool),
    
    /// Array of values (strings or numbers)
    Array(Vec<Property>),
}

impl Property {
    /// Try to get this property as a string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Property::String(s) => Some(s),
            _ => None,
        }
    }
    
    /// Try to get this property as a number
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Property::Number(n) => Some(*n),
            _ => None,
        }
    }
    
    /// Try to get this property as a boolean
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Property::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    /// Try to get this property as an array
    pub fn as_array(&self) -> Option<&[Property]> {
        match self {
            Property::Array(a) => Some(a),
            _ => None,
        }
    }
}

/// Utility for cleaning strings (removing LIST_ macros)
pub fn clean_string(input: &str) -> String {
    // Handle LIST_X("item") pattern
    if let Some(content) = input.strip_prefix("LIST_") {
        if let Some(pos) = content.find('(') {
            let remaining = &content[pos+1..];
            if let Some(end) = remaining.rfind(')') {
                // Extract the string from inside the LIST_X macro
                // The content should be in quotes
                let inner_content = &remaining[..end];
                return inner_content.trim_matches('"').trim_matches('\'').to_string();
            }
        }
    }
    
    // Handle regular strings
    input.trim_matches('"').trim_matches('\'').to_string()
} 