use std::collections::HashMap;
use std::fmt;

/// Represents a parsed class
#[derive(Debug, Clone)]
pub struct Class {
    pub name: Option<String>,
    pub parent: Option<String>,
    pub properties: HashMap<String, Value>,
    pub classes: HashMap<String, Class>,
}

impl Class {
    pub fn new() -> Self {
        Self {
            name: None,
            parent: None,
            properties: HashMap::new(),
            classes: HashMap::new(),
        }
    }
}

impl Default for Class {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a property value
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Integer(i32),
    Array(Vec<Value>),
    Class(Box<Class>),
    Expression(String),
    Reference(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Class(c) => {
                write!(f, "{{")?;
                for (k, v) in &c.properties {
                    write!(f, "{}: {}, ", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Expression(e) => write!(f, "{}", e),
            Value::Reference(r) => write!(f, "{}", r),
        }
    }
}

/// Represents a property
#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

impl Property {
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}

/// Represents an enum value
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub name: String,
    pub value: i32,
}

impl EnumValue {
    pub fn new(name: String, value: i32) -> Self {
        Self { name, value }
    }
} 