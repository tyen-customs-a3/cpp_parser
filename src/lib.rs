pub mod class_parser;
pub mod models;
pub mod property_parser;
pub mod preprocessor;

// Re-export the optimized parallel implementation as the default
pub use class_parser::optimized::parse_classes_parallel as parse_classes;
pub use class_parser::class_model::{Class, ClassCollection};
pub use class_parser::class_parser::parse_classes_to_collection;
pub use models::{Property as ModelProperty, PropertyValue};
pub use property_parser::parse_properties;
pub use property_parser::block_properties::{parse_block_properties, parse_all_block_properties_parallel as parse_all_block_properties};
pub use preprocessor::{preprocess, ConfigElement, ConfigElementType};

// Keep the old names for backward compatibility
pub use class_parser::Class as Block;
// Use optimized parallel as the default for parse_blocks too
pub use class_parser::optimized::parse_classes_parallel as parse_blocks;
pub use class_parser::class_model::{Class as BlockModel, ClassCollection as BlockCollection};
pub use class_parser::class_parser::parse_classes_to_collection as parse_blocks_to_collection;

// Original implementations are available under the 'original' namespace
pub mod original {
    //! Original sequential parsing functions

    /// Parse classes from a string using the original sequential algorithm
    pub fn parse_classes(input: &str) -> Vec<crate::class_parser::Class> {
        crate::class_parser::parse_classes(input)
    }

    /// Parse properties from classes using the original sequential algorithm
    pub fn parse_properties(classes: &[crate::class_parser::Class], input: &str) -> Vec<(String, String, String)> {
        crate::property_parser::block_properties::parse_all_block_properties(classes, input)
    }

    /// Parse both classes and properties from a string using the original sequential algorithm
    pub fn parse_config(input: &str) -> (Vec<crate::class_parser::Class>, Vec<(String, String, String)>) {
        let classes = parse_classes(input);
        let properties = parse_properties(&classes, input);
        (classes, properties)
    }
}

pub mod parallel {
    //! Parallel parsing functions for improved performance on multi-core systems

    /// Parse classes from a string using parallel processing
    pub fn parse_classes(input: &str) -> Vec<crate::class_parser::Class> {
        crate::class_parser::parse_classes_parallel(input)
    }

    /// Parse properties from classes using parallel processing
    pub fn parse_properties(classes: &[crate::class_parser::Class], input: &str) -> Vec<(String, String, String)> {
        crate::property_parser::block_properties::parse_all_block_properties_parallel(classes, input)
    }

    /// Parse both classes and properties from a string using parallel processing
    pub fn parse_config(input: &str) -> (Vec<crate::class_parser::Class>, Vec<(String, String, String)>) {
        let classes = parse_classes(input);
        let properties = parse_properties(&classes, input);
        (classes, properties)
    }
}

pub mod optimized {
    //! Optimized parsing functions for improved performance

    /// Parse classes from a string using an optimized algorithm
    pub fn parse_classes(input: &str) -> Vec<crate::class_parser::Class> {
        crate::class_parser::optimized::parse_classes(input)
    }

    /// Parse classes from a string using an optimized parallel algorithm
    pub fn parse_classes_parallel(input: &str) -> Vec<crate::class_parser::Class> {
        crate::class_parser::optimized::parse_classes_parallel(input)
    }

    /// Parse both classes and properties from a string using optimized algorithms
    pub fn parse_config(input: &str) -> (Vec<crate::class_parser::Class>, Vec<(String, String, String)>) {
        let classes = parse_classes(input);
        let properties = crate::property_parser::block_properties::parse_all_block_properties(&classes, input);
        (classes, properties)
    }

    /// Parse both classes and properties from a string using optimized parallel algorithms
    pub fn parse_config_parallel(input: &str) -> (Vec<crate::class_parser::Class>, Vec<(String, String, String)>) {
        let classes = parse_classes_parallel(input);
        let properties = crate::property_parser::block_properties::parse_all_block_properties_parallel(&classes, input);
        (classes, properties)
    }
}

// Default parse_config function that uses the optimized parallel implementation
pub fn parse_config(input: &str) -> (Vec<class_parser::Class>, Vec<(String, String, String)>) {
    let classes = parse_classes(input);
    let properties = parse_all_block_properties(&classes, input);
    (classes, properties)
}
