use std::fs;
use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;
use pest::error::Error as PestError;
use pest::Position;
use crate::parser::ast::{Class, Property, Value};
use super::error::ParseError;

#[derive(Parser)]
#[grammar = "parser/cpp.pest"]
struct CppParser;

fn get_location(pos: Position) -> String {
    format!("line {}, column {}", pos.line_col().0, pos.line_col().1)
}

fn get_snippet(pair: &pest::iterators::Pair<Rule>) -> String {
    let span = pair.as_span();
    let snippet = span.as_str().trim();
    if snippet.lines().count() > 1 {
        snippet.lines().next().unwrap_or("").to_string() + "..."
    } else {
        snippet.to_string()
    }
}

pub fn parse_cpp(content: &str) -> Result<Vec<Class>, ParseError> {
    // Preprocess the content to remove preprocessor directives
    let preprocessed_content = preprocess_cpp(content);
    
    let mut classes = Vec::new();
    let mut enum_values = HashMap::new();
    let mut deleted_items = Vec::new(); // Track deleted items
    let mut global_properties = HashMap::new(); // Track global properties
    
    let pairs = CppParser::parse(Rule::file, &preprocessed_content)
        .map_err(|e| ParseError::PestError {
            message: e.to_string(),
            location: match e.line_col {
                pest::error::LineColLocation::Pos((line, col)) => format!("line {}, column {}", line, col),
                pest::error::LineColLocation::Span((line, col), _) => format!("line {}, column {}", line, col),
            },
            context: "parsing file".to_string(),
            snippet: e.line().to_string(),
        })?;
        
    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::enum_block => {
                            parse_enum_block(inner_pair, &mut enum_values)?;
                        }
                        Rule::class_def => {
                            classes.push(parse_class(inner_pair, &enum_values)?);
                        }
                        Rule::delete_statement => {
                            // Handle delete statement
                            let item_name = inner_pair.into_inner().next()
                                .map(|p| p.as_str().to_string())
                                .unwrap_or_default();
                            deleted_items.push(item_name);
                            // Note: In a real implementation, you would actually remove the item
                            // from the parsed structure. For now, we're just tracking the deleted items.
                        }
                        Rule::property => {
                            // Handle global property
                            let (name, value) = parse_property(inner_pair, &enum_values)?;
                            global_properties.insert(name.clone(), Property { name, value });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    
    // If we have global properties, create a special "global" class to hold them
    if !global_properties.is_empty() {
        let mut global_class = Class::new("__global__".to_string(), None);
        global_class.properties = global_properties;
        classes.push(global_class);
    }
    
    // In a real implementation, you would filter out deleted items here
    // For now, we're just returning all classes
    
    Ok(classes)
}

/// Preprocess C++ content by removing preprocessor directives
fn preprocess_cpp(content: &str) -> String {
    let mut result = String::new();
    let mut if_blocks = Vec::new(); // Stack to track nested if blocks
    let mut skip_content = false;   // Whether to skip content in the current block
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("#") {
            // Special case for #define _ARMA_ which should be ignored but not affect processing
            if trimmed.starts_with("#define") {
                // Skip the line but don't change processing state
                continue;
            } else if trimmed.starts_with("#if") {
                // Push current skip state to stack and set new state
                if_blocks.push(skip_content);
                // If we're already skipping content, continue to skip
                // Otherwise, start skipping (we'll process #else blocks)
                skip_content = true;
            } else if trimmed.starts_with("#else") {
                // In #else block, we want to include content if we were skipping before
                if !if_blocks.is_empty() {
                    skip_content = false;
                }
            } else if trimmed.starts_with("#endif") {
                // Pop the previous skip state
                if !if_blocks.is_empty() {
                    skip_content = if_blocks.pop().unwrap();
                }
            }
            // Skip the preprocessor directive line
            continue;
        }
        
        // Include the line if we're not skipping content
        if !skip_content {
            result.push_str(line);
            result.push('\n');
        }
    }
    
    result
}

fn parse_enum_block(pair: pest::iterators::Pair<Rule>, enum_values: &mut HashMap<String, i32>) -> Result<(), ParseError> {
    for enum_value in pair.into_inner() {
        if enum_value.as_rule() == Rule::enum_value {
            let span = enum_value.as_span();
            let start_pos = span.start_pos();
            let text = enum_value.as_str().trim();
            let parts: Vec<&str> = text.split('=').collect();
            if parts.len() != 2 {
                return Err(ParseError::EnumError {
                    message: format!("Invalid enum format: {}", text),
                    location: get_location(start_pos),
                    context: "parsing enum value".to_string(),
                    snippet: text.to_string(),
                });
            }

            let name_str = parts[0].trim();
            let value_str = parts[1].trim().trim_end_matches(',');
            let value = value_str.parse::<i32>()
                .map_err(|_| ParseError::EnumError {
                    message: format!("Invalid enum value for {}: {}", name_str, value_str),
                    location: get_location(start_pos),
                    context: format!("parsing enum value '{}'", name_str),
                    snippet: text.to_string(),
                })?;
                
            // Store the enum value with its name
            let name = name_str.to_string();
            enum_values.insert(name.clone(), value);
            
            // If the name is a number, also store it as a string representation
            if let Ok(_) = name_str.parse::<i32>() {
                // The name is already a string representation of a number
                // No need to do anything extra since we already inserted it
            }
        }
    }
    Ok(())
}

fn parse_class(pair: pest::iterators::Pair<Rule>, enum_values: &HashMap<String, i32>) -> Result<Class, ParseError> {
    let mut class_name = String::new();
    let mut parent_name = None;
    let mut properties = HashMap::new();
    let mut nested_classes = Vec::new();
    let mut deleted_items = Vec::new(); // Track deleted items
    
    let pair_span = pair.as_span();
    let start_pos = pair_span.start_pos();
    let pair_str = pair.as_str();
    
    let mut pairs = pair.into_inner();
    
    // Parse class header
    if let Some(header) = pairs.next() {
        if header.as_rule() == Rule::class_header {
            let mut header_pairs = header.into_inner();
            
            // Get class name
            if let Some(name_pair) = header_pairs.next() {
                class_name = name_pair.as_str().to_string();
            }
            
            // Get parent class if present
            if let Some(parent_pair) = header_pairs.next() {
                parent_name = Some(parent_pair.as_str().to_string());
            }
        }
    }
    
    // Parse class body
    if let Some(body) = pairs.next() {
        if body.as_rule() == Rule::class_body {
            for item in body.into_inner() {
                match item.as_rule() {
                    Rule::property => {
                        let (name, value) = parse_property(item, enum_values)?;
                        properties.insert(name.clone(), Property { name, value });
                    }
                    Rule::class_def => {
                        let nested_class = parse_class(item, enum_values)?;
                        nested_classes.push(nested_class);
                    }
                    Rule::delete_statement => {
                        // Handle delete statement
                        let item_name = item.into_inner().next()
                            .map(|p| p.as_str().to_string())
                            .unwrap_or_default();
                        deleted_items.push(item_name);
                        // Note: In a real implementation, you would actually remove the item
                        // from the parsed structure. For now, we're just tracking the deleted items.
                    }
                    _ => {}
                }
            }
        }
    }
    
    let mut class = Class::new(class_name.clone(), parent_name.clone());
    class.properties = properties;
    class.nested_classes = nested_classes;
    
    // In a real implementation, you would filter out deleted items here
    
    Ok(class)
}

fn parse_property(pair: pest::iterators::Pair<Rule>, enum_values: &HashMap<String, i32>) -> Result<(String, Value), ParseError> {
    let mut property_name = String::new();
    let mut property_value = None;
    let mut is_array_suffix = false;
    let mut is_append_op = false;
    
    let pair_span = pair.as_span();
    let start_pos = pair_span.start_pos();
    let pair_str = pair.as_str().to_string();
    
    let mut pairs = pair.into_inner();
    
    // Get property name
    if let Some(name_pair) = pairs.next() {
        property_name = name_pair.as_str().to_string();
    }
    
    // Check for array suffix
    if let Some(next_pair) = pairs.next() {
        match next_pair.as_rule() {
            Rule::array_suffix => {
                is_array_suffix = true;
                // Get the assignment operator
                if let Some(op_pair) = pairs.next() {
                    if op_pair.as_rule() == Rule::assignment_op {
                        is_append_op = op_pair.as_str() == "+=";
                        // Get the actual value
                        if let Some(value_pair) = pairs.next() {
                            property_value = Some(parse_value(value_pair, enum_values)?);
                        }
                    } else {
                        // This was the value
                        property_value = Some(parse_value(op_pair, enum_values)?);
                    }
                }
            }
            Rule::assignment_op => {
                is_append_op = next_pair.as_str() == "+=";
                // Get the actual value
                if let Some(value_pair) = pairs.next() {
                    property_value = Some(parse_value(value_pair, enum_values)?);
                }
            }
            _ => {
                // This was the value
                property_value = Some(parse_value(next_pair, enum_values)?);
            }
        }
    }
    
    let value = property_value.ok_or_else(|| ParseError::ValueError {
        message: format!("Missing value for property {}", property_name),
        location: get_location(start_pos),
        context: format!("parsing property '{}'", property_name),
        snippet: pair_str,
    })?;
    
    // Note: For now, we're treating += the same as = since we're just parsing the structure
    // In a real implementation, you might want to handle array concatenation differently
    
    Ok((property_name, value))
}

fn parse_value(pair: pest::iterators::Pair<Rule>, enum_values: &HashMap<String, i32>) -> Result<Value, ParseError> {
    let span = pair.as_span();
    let start_pos = span.start_pos();
    let pair_str = pair.as_str().to_string();
    
    match pair.as_rule() {
        Rule::value => {
            let inner = pair.into_inner().next()
                .ok_or_else(|| ParseError::ValueError {
                    message: "Empty value".to_string(),
                    location: get_location(start_pos),
                    context: "parsing value".to_string(),
                    snippet: pair_str,
                })?;
            parse_value(inner, enum_values)
        }
        Rule::string_literal => {
            let text = pair.as_str();
            // Remove the outer quotes
            let content = &text[1..text.len()-1];
            // Replace escaped quotes with single quotes
            // In Arma 3 config files, quotes are escaped by doubling them
            let content = content.replace("\"\"", "\"");
            Ok(Value::String(content.to_string()))
        }
        Rule::number => {
            let text = pair.as_str();
            
            // Check if this number is actually an enum value
            // This is important for cases like "value = 0;" where 0 is an enum value
            if let Some(value) = enum_values.get(text) {
                return Ok(Value::Enum(*value));
            }
            
            let num = text.parse::<f64>().map_err(|_| ParseError::ValueError {
                message: format!("Failed to parse number: {}", text),
                location: get_location(start_pos),
                context: "parsing number".to_string(),
                snippet: text.to_string(),
            })?;
            Ok(Value::Number(num))
        }
        Rule::identifier => {
            let text = pair.as_str();
            
            // Check if it's an enum value
            if let Some(value) = enum_values.get(text) {
                return Ok(Value::Enum(*value));
            }
            
            // Otherwise, treat it as a string
            Ok(Value::String(text.to_string()))
        }
        Rule::array => {
            let mut values = Vec::new();
            for item_pair in pair.into_inner() {
                // Store the rule before moving the item_pair
                let is_macro_call = item_pair.as_rule() == Rule::macro_call;
                
                let value = parse_value(item_pair, enum_values)?;
                
                // If the value is an array from a macro expansion, flatten it into the current array
                if let Value::Array(inner_values) = value {
                    // Check if this is from a macro expansion
                    if is_macro_call {
                        // Flatten the array
                        for inner_value in inner_values {
                            values.push(inner_value);
                        }
                    } else {
                        // Regular nested array, keep it as is
                        values.push(Value::Array(inner_values));
                    }
                } else {
                    values.push(value);
                }
            }
            Ok(Value::Array(values))
        }
        Rule::macro_call => {
            // Handle macro calls like LIST_10("ACE_fieldDressing")
            let mut inner_pairs = pair.into_inner();
            
            // Get the macro name (e.g., LIST_10)
            let macro_name = inner_pairs.next()
                .ok_or_else(|| ParseError::ValueError {
                    message: "Missing macro name".to_string(),
                    location: get_location(start_pos),
                    context: "parsing macro call".to_string(),
                    snippet: pair_str.clone(),
                })?
                .as_str();
            
            // Get the macro argument
            let macro_arg = inner_pairs.next()
                .ok_or_else(|| ParseError::ValueError {
                    message: "Missing macro argument".to_string(),
                    location: get_location(start_pos),
                    context: "parsing macro call".to_string(),
                    snippet: pair_str.clone(),
                })?;
            
            // Parse the argument value
            let arg_value = parse_value(macro_arg, enum_values)?;
            
            // Extract the count from the macro name (e.g., 10 from LIST_10)
            let count_str = macro_name.strip_prefix("LIST_")
                .ok_or_else(|| ParseError::ValueError {
                    message: format!("Invalid macro name format: {}", macro_name),
                    location: get_location(start_pos),
                    context: "parsing macro call".to_string(),
                    snippet: pair_str.clone(),
                })?;
            
            let count = count_str.parse::<usize>().map_err(|_| ParseError::ValueError {
                message: format!("Invalid count in macro name: {}", count_str),
                location: get_location(start_pos),
                context: "parsing macro call".to_string(),
                snippet: pair_str.clone(),
            })?;
            
            // Create an array with 'count' copies of the argument value
            let mut values = Vec::with_capacity(count);
            for _ in 0..count {
                values.push(arg_value.clone());
            }
            
            Ok(Value::Array(values))
        }
        Rule::macro_arg => {
            // For macro_arg, we just need to parse the inner value
            let inner = pair.into_inner().next()
                .ok_or_else(|| ParseError::ValueError {
                    message: "Empty macro argument".to_string(),
                    location: get_location(start_pos),
                    context: "parsing macro argument".to_string(),
                    snippet: pair_str,
                })?;
            parse_value(inner, enum_values)
        }
        _ => Err(ParseError::ValueError {
            message: format!("Unexpected rule: {:?}", pair.as_rule()),
            location: get_location(start_pos),
            context: "parsing value".to_string(),
            snippet: pair_str,
        })
    }
}

pub fn parse_cpp_file(path: &str) -> Result<Vec<Class>, ParseError> {
    let content = fs::read_to_string(path)?;
    parse_cpp(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scientific_notation_and_syntax() {
        let content = r#"
        class TestClass {
            duration = 1e+11;
            smallValue = 1.23e-5;
            regularNumber = 123.45;
            _generalMacro = "C_UavTerminal";
            muzzles[] += {"APERSMineDispenserMuzzle","TrainingMineMuzzle"};
        };
        
        TrackIR_Developer_ID = 7503;
        TrackIR_Developer_AppKeyHigh = -1231810133;
        TrackIR_Developer_AppKeyLow = -723340391;
        delete Revive;
        "#;

        let result = parse_cpp(content).unwrap();
        assert_eq!(result.len(), 2); // TestClass and __global__
        
        // Check the TestClass
        let test_class = result.iter().find(|c| c.name == "TestClass").unwrap();
        
        // Check scientific notation with positive exponent
        let duration = test_class.properties.get("duration").unwrap();
        if let Value::Number(n) = duration.value {
            assert_eq!(n, 1e+11);
        } else {
            panic!("Expected Number value for duration");
        }
        
        // Check scientific notation with negative exponent
        let small_value = test_class.properties.get("smallValue").unwrap();
        if let Value::Number(n) = small_value.value {
            assert_eq!(n, 1.23e-5);
        } else {
            panic!("Expected Number value for smallValue");
        }
        
        // Check regular decimal number still works
        let regular = test_class.properties.get("regularNumber").unwrap();
        if let Value::Number(n) = regular.value {
            assert_eq!(n, 123.45);
        } else {
            panic!("Expected Number value for regularNumber");
        }
        
        // Check property name with underscore prefix
        let general_macro = test_class.properties.get("_generalMacro").unwrap();
        if let Value::String(s) = &general_macro.value {
            assert_eq!(s, "C_UavTerminal");
        } else {
            panic!("Expected String value for _generalMacro");
        }
        
        // Check array with += operator
        let muzzles = test_class.properties.get("muzzles").unwrap();
        if let Value::Array(arr) = &muzzles.value {
            assert_eq!(arr.len(), 2);
            if let Value::String(s) = &arr[0] {
                assert_eq!(s, "APERSMineDispenserMuzzle");
            } else {
                panic!("Expected String value for first array element");
            }
            if let Value::String(s) = &arr[1] {
                assert_eq!(s, "TrainingMineMuzzle");
            } else {
                panic!("Expected String value for second array element");
            }
        } else {
            panic!("Expected Array value for muzzles");
        }
        
        // Check global properties
        let global_class = result.iter().find(|c| c.name == "__global__").unwrap();
        
        // Check identifiers with multiple underscores
        let track_ir_id = global_class.properties.get("TrackIR_Developer_ID").unwrap();
        if let Value::Number(n) = track_ir_id.value {
            assert_eq!(n, 7503.0);
        } else {
            panic!("Expected Number value for TrackIR_Developer_ID");
        }
        
        let track_ir_high = global_class.properties.get("TrackIR_Developer_AppKeyHigh").unwrap();
        if let Value::Number(n) = track_ir_high.value {
            assert_eq!(n, -1231810133.0);
        } else {
            panic!("Expected Number value for TrackIR_Developer_AppKeyHigh");
        }
        
        let track_ir_low = global_class.properties.get("TrackIR_Developer_AppKeyLow").unwrap();
        if let Value::Number(n) = track_ir_low.value {
            assert_eq!(n, -723340391.0);
        } else {
            panic!("Expected Number value for TrackIR_Developer_AppKeyLow");
        }
        
        // Note: We're not testing the delete statement functionality here
        // since we're not actually implementing the deletion logic
    }

    #[test]
    fn test_numeric_enum_names() {
        let content = r#"
        enum {
            0 = 0,
            1 = 1,
            2 = 2
        };
        
        class TestClass {
            value = 0; // This should be parsed as an enum value
        };
        "#;
        
        let result = parse_cpp(content).unwrap();
        assert_eq!(result.len(), 1);
        
        let class = &result[0];
        assert_eq!(class.name, "TestClass");
        
        // Check that the value is parsed as an enum
        let value_prop = class.properties.get("value").unwrap();
        
        // Print the actual value for debugging
        println!("Actual value: {:?}", value_prop.value);
        
        // Check if the enum values were correctly parsed
        match value_prop.value {
            Value::Enum(n) => {
                assert_eq!(n, 0);
            }
            _ => {
                panic!("Expected Enum value for 'value', got {:?}", value_prop.value);
            }
        }
    }
    
    #[test]
    fn test_macro_expansion() {
        let content = r#"
        class TestClass {
            items[] = {
                LIST_3("FirstItem"),
                LIST_2(123),
                "RegularItem"
            };
        };
        "#;

        let result = parse_cpp(content).unwrap();
        assert_eq!(result.len(), 1);
        
        let class = &result[0];
        assert_eq!(class.name, "TestClass");
        
        // Check that the items array contains the expanded macros
        let items = class.properties.get("items").unwrap();
        if let Value::Array(values) = &items.value {
            assert_eq!(values.len(), 6); // 3 + 2 + 1 = 6 items
            
            // Check the first macro expansion (LIST_3("FirstItem"))
            assert_eq!(values[0], Value::String("FirstItem".to_string()));
            assert_eq!(values[1], Value::String("FirstItem".to_string()));
            assert_eq!(values[2], Value::String("FirstItem".to_string()));
            
            // Check the second macro expansion (LIST_2(123))
            assert_eq!(values[3], Value::Number(123.0));
            assert_eq!(values[4], Value::Number(123.0));
            
            // Check the regular item
            assert_eq!(values[5], Value::String("RegularItem".to_string()));
        } else {
            panic!("Expected Array value for items");
        }
    }
}