use std::collections::HashMap;

/// Represents a parsed class
#[derive(Debug, Clone)]
pub struct Class {
    /// The name of the class
    pub name: String,
    /// The parent class name, if any
    pub parent: Option<String>,
    /// The raw content of the class
    pub content: String,
    /// Nested classes within this class
    pub nested_classes: Vec<Class>,
    /// Start position in the original text
    pub start_pos: usize,
    /// End position in the original text
    pub end_pos: usize,
}

impl Class {
    /// Create a new class
    pub fn new(name: String, parent: Option<String>, content: String, start_pos: usize, end_pos: usize) -> Self {
        Class {
            name,
            parent,
            content,
            nested_classes: Vec::new(),
            start_pos,
            end_pos,
        }
    }

    /// Add a nested class
    pub fn add_nested_class(&mut self, class: Class) {
        self.nested_classes.push(class);
    }

    /// Get the raw text of this class from the original input
    pub fn get_raw_text<'a>(&self, input: &'a str) -> Option<&'a str> {
        if self.start_pos < input.len() && self.end_pos <= input.len() && self.start_pos <= self.end_pos {
            Some(&input[self.start_pos..self.end_pos])
        } else {
            None
        }
    }

    /// Get a nested class by name
    pub fn get_nested_class(&self, name: &str) -> Option<&Class> {
        self.nested_classes.iter().find(|c| c.name == name)
    }
}

/// Represents a collection of parsed classes
#[derive(Debug, Clone)]
pub struct ClassCollection {
    /// The top-level classes
    pub classes: Vec<Class>,
    /// A map of class names to their indices in the classes vector
    class_map: HashMap<String, usize>,
}

impl ClassCollection {
    /// Create a new class collection
    pub fn new() -> Self {
        ClassCollection {
            classes: Vec::new(),
            class_map: HashMap::new(),
        }
    }

    /// Add a class to the collection
    pub fn add_class(&mut self, class: Class) {
        let name = class.name.clone();
        self.classes.push(class);
        self.class_map.insert(name, self.classes.len() - 1);
    }

    /// Get a class by name
    pub fn get_class(&self, name: &str) -> Option<&Class> {
        self.class_map.get(name).map(|&idx| &self.classes[idx])
    }

    /// Get all classes that inherit from a specific class
    pub fn get_children(&self, parent_name: &str) -> Vec<&Class> {
        self.classes.iter()
            .filter(|c| c.parent.as_deref() == Some(parent_name))
            .collect()
    }

    /// Get all class names
    pub fn get_class_names(&self) -> Vec<&str> {
        self.classes.iter().map(|c| c.name.as_str()).collect()
    }
} 