//! CPP Parser is a library for parsing Arma 3 mission config files (.cpp)
//! 
//! # Example
//! ```rust
//! use cpp_parser::{parse_cpp_file, CppFile};
//! 
//! let content = r#"
//!     #define _ARMA_
//!     
//!     class CfgPatches {
//!         class my_addon {
//!             author = "Example";
//!             requiredVersion = 1.6;
//!             requiredAddons[] = {"A3_Data_F"};
//!             units[] = {};
//!             weapons[] = {};
//!         };
//!     };
//! "#;
//! 
//! match cpp_parser::parse_cpp(content) {
//!     Ok(cpp_file) => {
//!         for class in cpp_file.classes {
//!             println!("Found class: {}", class.name);
//!         }
//!     }
//!     Err(e) => println!("Error parsing CPP: {}", e)
//! }
//! ```

pub mod parser;

// Re-export commonly used types
pub use parser::{
    Class,
    Value,
    Property,
    Directive,
    DirectiveKind,
    file_parser::parse_cpp_file,
};