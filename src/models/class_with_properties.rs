use crate::models::Property;
use crate::class_parser::class_model::Class;

/// Represents a class with its properties
#[derive(Debug, Clone)]
pub struct ClassWithProperties {
    /// The underlying block
    pub class: Class,
    /// The properties of this class
    pub properties: Vec<Property>,
    /// Nested classes with their properties
    pub nested_classes: Vec<ClassWithProperties>,
}

impl ClassWithProperties {
    /// Create a new class with properties
    pub fn new(class: Class, properties: Vec<Property>) -> Self {
        Self {
            class,
            properties,
            nested_classes: Vec::new(),
        }
    }

    /// Add a nested class with properties
    pub fn add_nested_class(&mut self, nested_class: ClassWithProperties) {
        self.nested_classes.push(nested_class);
    }

    /// Get the name of the class
    pub fn name(&self) -> &str {
        &self.class.name
    }

    /// Get the parent class name, if any
    pub fn parent(&self) -> Option<&str> {
        self.class.parent.as_deref()
    }

    /// Get a property by name
    pub fn get_property(&self, name: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Get a nested class by name
    pub fn get_nested_class(&self, name: &str) -> Option<&ClassWithProperties> {
        self.nested_classes.iter().find(|c| c.name() == name)
    }
}

/// Represents a collection of classes with their properties
#[derive(Debug, Clone)]
pub struct ClassWithPropertiesCollection {
    /// The top-level classes with their properties
    pub classes: Vec<ClassWithProperties>,
}

impl ClassWithPropertiesCollection {
    /// Create a new collection
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    /// Add a class with properties to the collection
    pub fn add_class(&mut self, class: ClassWithProperties) {
        self.classes.push(class);
    }

    /// Get a class by name
    pub fn get_class(&self, name: &str) -> Option<&ClassWithProperties> {
        self.classes.iter().find(|c| c.name() == name)
    }

    /// Get all classes that inherit from a parent class
    pub fn get_children(&self, parent_name: &str) -> Vec<&ClassWithProperties> {
        self.classes.iter()
            .filter(|c| c.parent() == Some(parent_name))
            .collect()
    }

    /// Get all class names
    pub fn get_class_names(&self) -> Vec<&str> {
        self.classes.iter().map(|c| c.name()).collect()
    }
}

impl Default for ClassWithPropertiesCollection {
    fn default() -> Self {
        Self::new()
    }
} 