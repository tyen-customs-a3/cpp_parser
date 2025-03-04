pub mod lexical_pass;
pub mod symbols_pass;
pub mod properties_pass;
pub mod references_pass;

// Each pass module now includes logging at key points to help with debugging.
// The logging uses the log crate with the following targets:
// - cpp_parser::lexical_pass
// - cpp_parser::symbols_pass
// - cpp_parser::properties_pass
// - cpp_parser::references_pass
//
// To see the logs, ensure a logger is initialized in your application.
// For example, using env_logger:
// ```
// env_logger::init();
// ```
// Or set the RUST_LOG environment variable:
// ```
// RUST_LOG=cpp_parser=debug
// ```

pub use lexical_pass::*;
pub use symbols_pass::*;
pub use properties_pass::*;
pub use references_pass::*; 