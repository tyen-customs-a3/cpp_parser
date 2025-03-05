pub mod class_parser;
pub mod models;
pub mod property_parser;
pub mod preprocessor;

pub use class_parser::{Class, parse_classes};
pub use class_parser::class_model::{Class as ClassModel, ClassCollection};
pub use class_parser::class_parser::parse_classes_to_collection;
pub use models::{Property as ModelProperty, PropertyValue};
pub use property_parser::parse_properties;
pub use property_parser::block_properties::{parse_block_properties, parse_all_block_properties};
pub use preprocessor::{preprocess, ConfigElement, ConfigElementType};

// Keep the old names for backward compatibility
pub use class_parser::{Class as Block, parse_classes as parse_blocks};
pub use class_parser::class_model::{Class as BlockModel, ClassCollection as BlockCollection};
pub use class_parser::class_parser::parse_classes_to_collection as parse_blocks_to_collection;
