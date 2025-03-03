use std::collections::HashMap;

/// Represents the raw parsed class definition from the first pass
#[derive(Debug, Clone)]
pub struct RawClassDef {
    pub name: String,
    pub parent_name: Option<String>,
    pub properties: Vec<RawProperty>,
    pub nested_classes: Vec<RawClassDef>,
    pub location: String, // For error reporting
    pub raw_text: String, // Original text for subsequent passes
}

impl RawClassDef {
    pub fn new(name: String, location: String, raw_text: String) -> Self {
        Self {
            name,
            parent_name: None,
            properties: Vec::new(),
            nested_classes: Vec::new(),
            location,
            raw_text,
        }
    }
}

/// Represents a raw property from the first pass
#[derive(Debug, Clone)]
pub struct RawProperty {
    pub name: String,
    pub is_array: bool,
    pub raw_value: String,
    pub location: String,
    pub raw_text: String, // Original text for subsequent passes
}

impl RawProperty {
    pub fn new(name: String, is_array: bool, raw_value: String, location: String, raw_text: String) -> Self {
        Self {
            name,
            is_array,
            raw_value,
            location,
            raw_text,
        }
    }
}

/// Represents a raw enum definition from the first pass
#[derive(Debug, Clone)]
pub struct RawEnumDef {
    pub values: HashMap<String, i32>,
    pub location: String,
    pub raw_text: String, // Original text for subsequent passes
}

impl RawEnumDef {
    pub fn new(location: String, raw_text: String) -> Self {
        Self {
            values: HashMap::new(),
            location,
            raw_text,
        }
    }
}

/// Represents a raw macro definition from the first pass
#[derive(Debug, Clone)]
pub struct RawMacroDef {
    pub name: String,
    pub params: String,
    pub raw_text: String, // Original text for subsequent passes
}

impl RawMacroDef {
    pub fn new(name: String, params: String, raw_text: String) -> Self {
        Self {
            name,
            params,
            raw_text,
        }
    }
} 