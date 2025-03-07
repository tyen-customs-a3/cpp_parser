use std::fmt;

/// Represents a property value
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// A string value
    String(String),
    /// A numeric value
    Number(f64),
    /// A boolean value
    Boolean(bool),
    /// An array of values
    Array(Vec<PropertyValue>),
    /// A reference to another property or class
    Reference(String),
    /// A LIST_ macro with count and value
    ListMacro(usize, String),
}

impl fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyValue::String(s) => write!(f, "\"{}\"", s),
            PropertyValue::Number(n) => write!(f, "{}", n),
            PropertyValue::Boolean(b) => write!(f, "{}", b),
            PropertyValue::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            },
            PropertyValue::Reference(r) => write!(f, "{}", r),
            PropertyValue::ListMacro(count, value) => write!(f, "LIST_{}(\"{}\")", count, value),
        }
    }
}

/// Represents a property in a configuration file
#[derive(Debug, Clone)]
pub struct Property {
    /// The name of the property
    pub name: String,
    /// The value of the property
    pub value: PropertyValue,
    /// Start position in the original text
    pub start_pos: usize,
    /// End position in the original text
    pub end_pos: usize,
    /// Whether this property uses the += operator for array appending
    pub is_append: bool,
}

impl Property {
    /// Create a new property
    pub fn new(name: String, value: PropertyValue, start_pos: usize, end_pos: usize) -> Self {
        Self {
            name,
            value,
            start_pos,
            end_pos,
            is_append: false,
        }
    }
    
    /// Get the raw text of this property from the input
    pub fn get_raw_text<'a>(&self, input: &'a str) -> Option<&'a str> {
        if self.start_pos <= self.end_pos && self.end_pos <= input.len() {
            Some(&input[self.start_pos..self.end_pos])
        } else {
            None
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
} 