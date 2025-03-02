use serde::{Serialize, Deserialize};
use super::value::Value;

/// Represents a property in a C++ config file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: Value,
}