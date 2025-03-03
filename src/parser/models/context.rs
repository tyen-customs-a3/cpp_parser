use std::collections::HashMap;
use super::{RawClassDef, RawEnumDef, RawMacroDef, Class};

/// Represents the type of a symbol
#[derive(Debug, Clone)]
pub enum SymbolType {
    Class(usize), // Index in the classes vector
    Enum(String, i32),
    Macro(String, String), // Name and expansion
}

/// Represents an unresolved reference
#[derive(Debug, Clone)]
pub struct UnresolvedReference {
    pub source_class_index: usize,
    pub reference_type: ReferenceType,
    pub name: String,
    pub location: String,
}

impl UnresolvedReference {
    pub fn new(source_class_index: usize, reference_type: ReferenceType, name: String, location: String) -> Self {
        Self {
            source_class_index,
            reference_type,
            name,
            location,
        }
    }
}

/// Represents the type of reference
#[derive(Debug, Clone)]
pub enum ReferenceType {
    Parent,
    PropertyValue,
    MacroExpansion,
}

/// Represents the parsing context
#[derive(Debug, Default)]
pub struct ParseContext {
    // Raw parsed elements from first pass
    pub raw_classes: Vec<RawClassDef>,
    pub raw_enums: Vec<RawEnumDef>,
    pub raw_macros: Vec<RawMacroDef>,
    
    // Symbol tables built in second pass
    pub symbols: HashMap<String, SymbolType>,
    pub enum_values: HashMap<String, i32>,
    pub macro_defs: HashMap<String, String>,
    
    // Final output being built
    pub classes: Vec<Class>,
    
    // Tracking for unresolved items
    pub unresolved_references: Vec<UnresolvedReference>,
}

impl ParseContext {
    pub fn new() -> Self {
        Self {
            raw_classes: Vec::new(),
            raw_enums: Vec::new(),
            raw_macros: Vec::new(),
            symbols: HashMap::new(),
            enum_values: HashMap::new(),
            macro_defs: HashMap::new(),
            classes: Vec::new(),
            unresolved_references: Vec::new(),
        }
    }
}

/// Represents a parse error
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub location: Option<String>,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            location: None,
        }
    }
    
    pub fn with_location(message: String, location: String) -> Self {
        Self {
            message,
            location: Some(location),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(location) = &self.location {
            write!(f, "{} at {}", self.message, location)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for ParseError {} 