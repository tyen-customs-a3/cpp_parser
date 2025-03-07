use crate::models::Class;
use crate::preprocessor::{CodeElement, CodeElementType};
use rayon::prelude::*;
use crate::models::{Property, PropertyValue};

/// Represents an element in the parsed code
#[derive(Debug, Clone)]
pub enum CodeEntry {
    /// A class with its properties
    Class(Class),
    /// Any other type of code element
    Element(CodeElement),
}

/// Represents an entire parsed codebase with all its elements
#[derive(Debug, Clone)]
pub struct Code {
    /// All elements in their original order
    elements: Vec<CodeEntry>,
}

impl Code {
    /// Create a new empty Code structure
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Create a Code structure from a vector of CodeElements without processing properties
    pub fn from_elements(elements: Vec<CodeElement>) -> Self {
        let mut code = Self::new();
        
        for element in elements {
            let entry = match element.element_type {
                CodeElementType::Class => {
                    // Create a Class with empty properties for now
                    CodeEntry::Class(Class::new(element, Vec::new()))
                }
                _ => CodeEntry::Element(element),
            };
            code.elements.push(entry);
        }
        
        code
    }

    /// Process properties for all classes in parallel
    pub fn process_properties(&mut self) {
        // First collect all class elements into a vector
        let class_indices: Vec<_> = self.elements.iter()
            .enumerate()
            .filter_map(|(i, entry)| match entry {
                CodeEntry::Class(_) => Some(i),
                _ => None,
            })
            .collect();

        // Process properties in parallel
        let property_results: Vec<_> = class_indices.par_iter()
            .filter_map(|&i| {
                if let CodeEntry::Class(class) = &self.elements[i] {
                    let properties = crate::property_parser::parse_properties(&class.element.content)
                        .unwrap_or_default();
                    Some((i, properties))
                } else {
                    None
                }
            })
            .collect();

        // Update the classes with their properties
        for (index, mut properties) in property_results {
            if let CodeEntry::Class(class) = &mut self.elements[index] {
                // Handle property appending
                let mut final_properties = Vec::new();
                
                for prop in properties {
                    if prop.is_append {
                        // If this is an append operation, find the existing property
                        if let Some(existing_prop) = final_properties.iter_mut()
                            .find(|p: &&mut Property| p.name == prop.name) 
                        {
                            // Merge the arrays
                            if let (PropertyValue::Array(ref mut existing_arr), PropertyValue::Array(ref new_arr)) = 
                                (&mut existing_prop.value, &prop.value) 
                            {
                                existing_arr.extend_from_slice(new_arr);
                            }
                        } else {
                            // If no existing property found, just add it as a normal property
                            final_properties.push(prop);
                        }
                    } else {
                        final_properties.push(prop);
                    }
                }
                
                class.properties = final_properties;
            }
        }
    }

    /// Get all elements in their original order
    pub fn elements(&self) -> &[CodeEntry] {
        &self.elements
    }

    /// Get all classes
    pub fn classes(&self) -> Vec<&Class> {
        self.elements.iter()
            .filter_map(|entry| match entry {
                CodeEntry::Class(class) => Some(class),
                _ => None,
            })
            .collect()
    }

    /// Get all defines
    pub fn defines(&self) -> Vec<&CodeElement> {
        self.elements.iter()
            .filter_map(|entry| match entry {
                CodeEntry::Element(e) if e.element_type == CodeElementType::Define => Some(e),
                _ => None,
            })
            .collect()
    }

    /// Get all forward declarations
    pub fn forward_declarations(&self) -> Vec<&CodeElement> {
        self.elements.iter()
            .filter_map(|entry| match entry {
                CodeEntry::Element(e) if e.element_type == CodeElementType::ForwardDeclaration => Some(e),
                _ => None,
            })
            .collect()
    }

    /// Get all delete statements
    pub fn deletes(&self) -> Vec<&CodeElement> {
        self.elements.iter()
            .filter_map(|entry| match entry {
                CodeEntry::Element(e) if e.element_type == CodeElementType::Delete => Some(e),
                _ => None,
            })
            .collect()
    }

    /// Get all enums
    pub fn enums(&self) -> Vec<&CodeElement> {
        self.elements.iter()
            .filter_map(|entry| match entry {
                CodeEntry::Element(e) if e.element_type == CodeElementType::Enum => Some(e),
                _ => None,
            })
            .collect()
    }

    /// Get all other elements
    pub fn other_elements(&self) -> Vec<&CodeElement> {
        self.elements.iter()
            .filter_map(|entry| match entry {
                CodeEntry::Element(e) if e.element_type == CodeElementType::Other => Some(e),
                _ => None,
            })
            .collect()
    }

    /// Get all class names
    pub fn class_names(&self) -> Vec<&str> {
        self.classes().iter().map(|c| c.name()).collect()
    }

    /// Get a class by name
    pub fn get_class(&self, name: &str) -> Option<&Class> {
        self.classes().into_iter().find(|c| c.name() == name)
    }

    /// Get all classes that inherit from a specific parent
    pub fn get_class_children(&self, parent_name: &str) -> Vec<&Class> {
        self.classes().into_iter()
            .filter(|c| c.parent() == Some(parent_name))
            .collect()
    }

    /// Get a define by name
    pub fn get_define(&self, name: &str) -> Option<&CodeElement> {
        self.defines().into_iter()
            .find(|d| d.name.as_deref() == Some(name))
    }

    /// Get an enum by name
    pub fn get_enum(&self, name: &str) -> Option<&CodeElement> {
        self.enums().into_iter()
            .find(|e| e.name.as_deref() == Some(name))
    }

    /// Check if a class is forward declared
    pub fn is_forward_declared(&self, name: &str) -> bool {
        self.forward_declarations().into_iter()
            .any(|fd| fd.name.as_deref() == Some(name))
    }

    /// Check if a class is deleted
    pub fn is_deleted(&self, name: &str) -> bool {
        self.deletes().into_iter()
            .any(|d| d.name.as_deref() == Some(name))
    }
}

impl Default for Code {
    fn default() -> Self {
        Self::new()
    }
} 