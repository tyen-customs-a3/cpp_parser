pub mod lexical_pass;
pub mod symbols_pass;
pub mod properties_pass;
pub mod references_pass;

// Export specific items from each module instead of using glob imports
pub use symbols_pass::{symbols_pass, SymbolsParser};
pub use properties_pass::{properties_pass, PropertiesParser};
pub use references_pass::{references_pass, ReferencesParser}; 