use crate::parser::ast::{Directive, Class, Property};

/// Represents a parsed CPP file with its directives, classes, and properties
pub struct CppFile {
    pub directives: Vec<Directive>,
    pub classes: Vec<Class>,
    pub properties: Vec<Property>,
}