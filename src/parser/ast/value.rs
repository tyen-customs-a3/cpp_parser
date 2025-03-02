use serde::{Serialize, Deserialize};

/// Represents different types of values that can appear in a C++ config file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Enum(i32),
    MacroReference(String),  // For macro references like COMPONENT_NAME or ECSTRING(common,ACETeam)
    Expression(String),      // For expressions like db-3, GUI_GRID_W, etc.
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
    
    pub fn as_macro_reference(&self) -> Option<&str> {
        match self {
            Value::MacroReference(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_expression(&self) -> Option<&str> {
        match self {
            Value::Expression(s) => Some(s),
            _ => None,
        }
    }
}