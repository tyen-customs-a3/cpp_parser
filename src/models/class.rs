use crate::models::Property;
use crate::preprocessor::CodeElement;

/// Represents a class with its properties
#[derive(Debug, Clone)]
pub struct Class {
    /// The underlying element
    pub element: CodeElement,
    /// The properties of this class
    pub properties: Vec<Property>,
    /// Nested classes with their properties
    pub nested_classes: Vec<Class>,
}

impl Class {
    /// Create a new class with properties
    pub fn new(element: CodeElement, properties: Vec<Property>) -> Self {
        Self {
            element,
            properties,
            nested_classes: Vec::new(),
        }
    }

    /// Add a nested class with properties
    pub fn add_nested_class(&mut self, nested_class: Class) {
        self.nested_classes.push(nested_class);
    }

    /// Get the name of the class
    pub fn name(&self) -> &str {
        self.element.name.as_deref().unwrap_or("")
    }

    /// Get the parent class name, if any
    pub fn parent(&self) -> Option<&str> {
        self.element.parent.as_deref()
    }

    /// Get a property by name
    pub fn get_property(&self, name: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Get a nested class by name
    pub fn get_nested_class(&self, name: &str) -> Option<&Class> {
        self.nested_classes.iter().find(|c| c.name() == name)
    }
} 