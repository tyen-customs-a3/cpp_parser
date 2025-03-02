use serde::{Serialize, Deserialize};

/// Represents different types of values that can appear in a C++ config file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Enum(i32),
}

impl Value {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
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

    pub fn as_enum(&self) -> Option<i32> {
        match self {
            Value::Enum(e) => Some(*e),
            _ => None,
        }
    }
}