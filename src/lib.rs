pub mod parser;

// Re-export commonly used types
pub use parser::{Class, Value, Property, ParseError, parse_cpp, parse_cpp_file};

/// # C++ Parser Library
/// 
/// This library provides functionality to parse C++ configuration files commonly used in Arma and similar games.
/// 
/// ## Basic Usage
/// 
/// ```rust
/// use cpp_parser::{parse_cpp, parse_cpp_file};
/// 
/// // Parse from a string
/// let config_content = r#"
/// class CfgPatches {
///     class MyMod {
///         name = "My Awesome Mod";
///         author = "Me";
///         requiredVersion = 1.0;
///         units[] = {};
///         weapons[] = {};
///     };
/// };
/// "#;
/// 
/// let result = parse_cpp(config_content).unwrap();
/// 
/// // Or parse from a file
/// // let result = parse_cpp_file("path/to/config.cpp").unwrap();
/// 
/// // Access the parsed data
/// for class in &result {
///     println!("Class: {:?}", class.name);
///     
///     // Access properties
///     for (name, value) in &class.properties {
///         println!("  Property: {} = {:?}", name, value);
///     }
///     
///     // Access nested classes
///     for (name, nested_class) in &class.classes {
///         println!("  Nested class: {}", name);
///     }
/// }
/// ```
/// 
/// ## Features
/// 
/// - Parses class definitions with inheritance
/// - Handles nested classes
/// - Supports various property types (strings, numbers, arrays)
/// - Handles preprocessor directives by stripping them out
/// - Supports class names and property names that start with numbers
/// 
/// ## Data Model
/// 
/// The parser produces a list of `Class` objects, each containing:
/// - Name (optional, as some classes might be anonymous)
/// - Parent class name (optional, for inheritance)
/// - Properties (name-value pairs)
/// - Nested classes
/// 
/// Property values can be:
/// - Strings
/// - Numbers (floating point)
/// - Integers
/// - Arrays (containing any of the above types)
/// 
/// ## Error Handling
/// 
/// The parser returns a `Result<Vec<Class>, ParseError>`, where `ParseError` contains
/// information about what went wrong during parsing.
/// 
/// ```rust
/// match parse_cpp(content) {
///     Ok(classes) => {
///         // Process the parsed classes
///     },
///     Err(error) => {
///         eprintln!("Failed to parse: {}", error.message);
///     }
/// }
/// ```

// This is a dummy item to satisfy the linter
#[doc(hidden)]
pub struct _DummyItem;
