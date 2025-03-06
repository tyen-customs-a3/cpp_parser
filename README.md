# CPP Parser

A multi-pass parser for Arma 3 configuration files.

## Features

- Fast, parallel parsing of complex configuration files
- Handles nested classes, inheritance, and other C++ preprocessor features
- Efficient preprocessing with optimized algorithms

## Performance

The parser uses parallel processing to efficiently handle large configuration files:

- Pre-computed brace mapping for fast matching of opening and closing braces
- Parallel processing of class declarations using Rayon
- Efficient handling of nested classes

Benchmark results show that the parallel implementation is up to 25x faster than the sequential implementation for large, complex inputs.

## Usage

```rust
use cpp_parser::preprocessor;

fn main() {
    let input = r#"
    class Vehicle {
        displayName = "Vehicle";
        
        class Engine {
            power = 100;
            torque = 200;
        };
    };
    "#;
    
    // Preprocess the input to find all code elements
    let elements = preprocessor::preprocess(input);
    
    // Process the elements
    for element in elements {
        println!("Found element: {:?}", element.element_type);
        
        // Handle nested elements
        for nested in &element.nested_elements {
            println!("  Nested element: {:?}", nested.element_type);
        }
    }
}
```

## License

MIT 