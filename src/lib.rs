pub mod parser;

// Re-export commonly used types
pub use parser::models::{Class, Value, Property};
pub use parser::{parse_cpp, parse_cpp_file};
pub use parser::models::ParseError;

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
/// - Supports list macros (e.g., LIST_2("item"))
/// 
/// ## Debugging
/// 
/// The parser includes logging at key points in the parsing process to help with debugging.
/// To enable logging, ensure a logger is initialized in your application and set the appropriate log level.
/// 
/// ```ignore
/// // Initialize a logger (e.g., env_logger)
/// env_logger::init();
/// 
/// // Or set the RUST_LOG environment variable:
/// // RUST_LOG=cpp_parser=debug
/// ```
/// 
/// The parser uses the following log targets:
/// - `cpp_parser::lexical_pass`: Logs from the lexical analysis pass
/// - `cpp_parser::symbols_pass`: Logs from the symbols resolution pass
/// - `cpp_parser::properties_pass`: Logs from the properties processing pass
/// - `cpp_parser::references_pass`: Logs from the references resolution pass
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
/// - List macros (e.g., LIST_2("item"))
/// 
/// ## Error Handling
/// 
/// The parser returns a `Result<Vec<Class>, ParseError>`, where `ParseError` contains
/// information about what went wrong during parsing.
/// 
/// ```ignore
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
