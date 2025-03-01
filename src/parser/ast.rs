use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Identifier(String),
}

impl Value {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            Value::Identifier(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            Value::Identifier(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
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
            let mut parts: Vec<&str> = name.split('.').collect();
            let first = parts.remove(0);
            
            // If the first part matches this class, look deeper
            if first == self.name {
                if parts.is_empty() {
                    return Some(self);
                }
                return self.find_class(&parts.join("."));
            }
            
            // Otherwise look in nested classes
            if let Some(class) = self.nested_classes.iter().find(|c| c.name == first) {
                return class.find_class(&parts.join("."));
            }
        } else {
            // Direct match on this class
            if name == self.name {
                return Some(self);
            }
            // Look in direct children
            return self.nested_classes.iter().find(|c| c.name == name);
        }
        None
    }

    pub fn find_property(&self, name: &str) -> Option<&Property> {
        self.properties.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct Directive {
    pub kind: DirectiveKind,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum DirectiveKind {
    Define,
    Include,
}

/// Represents a parsed CPP file with its directives, classes, and properties
#[derive(Debug)]
pub struct CppFile {
    pub directives: Vec<Directive>,
    pub classes: Vec<Class>,
    pub properties: Vec<Property>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_as_string() {
        let val = Value::String("test".to_string());
        assert_eq!(val.as_string(), Some("test"));
    }

    #[test]
    fn test_value_as_number() {
        let val = Value::Number(42.0);
        assert_eq!(val.as_number(), Some(42.0));
    }

    #[test]
    fn test_value_as_array() {
        let val = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        assert_eq!(val.as_array(), Some(&vec![Value::Number(1.0), Value::Number(2.0)]));
    }

    #[test]
    fn test_value_as_identifier() {
        let val = Value::Identifier("id".to_string());
        assert_eq!(val.as_identifier(), Some("id"));
    }

    #[test]
    fn test_class_find_class() {
        let mut class = Class::new("root".to_string(), None);
        let nested = Class::new("nested".to_string(), None);
        class.nested_classes.push(nested);
        assert!(class.find_class("root").is_some());
        assert!(class.find_class("root.nested").is_some());
        assert!(class.find_class("nonexistent").is_none());
    }

    #[test]
    fn test_class_find_property() {
        let mut class = Class::new("root".to_string(), None);
        class.properties.insert("prop".to_string(), Property { name: "prop".to_string(), value: Value::Number(42.0) });
        assert!(class.find_property("prop").is_some());
        assert!(class.find_property("nonexistent").is_none());
    }
}