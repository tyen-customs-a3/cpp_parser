use pest::Parser;
use pest::iterators::Pair;
use crate::parser::models::{ParseContext, ParseError, Value, ReferenceType, UnresolvedReference};

// Define the parser
#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar/grammar.pest"]
pub struct PropertiesParser;

// Rule enum is automatically generated by pest_derive

/// Performs the properties pass on the context
pub fn properties_pass(context: &mut ParseContext) -> Result<(), ParseError> {
    // Clone raw_classes to avoid borrowing issues
    let raw_classes = context.raw_classes.clone();
    
    // Process properties for each class
    for (class_index, raw_class) in raw_classes.iter().enumerate() {
        // Process each property
        for raw_property in &raw_class.properties {
            let property_result = process_property(raw_property.raw_text.as_str(), context, class_index)?;
            
            // Add the property to the class
            if let Some(class) = context.classes.get_mut(class_index) {
                class.properties.insert(raw_property.name.clone(), property_result);
            }
        }
        
        // Process nested classes recursively
        process_nested_classes(raw_class, context, class_index)?;
    }
    
    Ok(())
}

/// Processes a property to extract its value
fn process_property(property_text: &str, context: &mut ParseContext, class_index: usize) -> Result<Value, ParseError> {
    // Try to parse the property
    let property = match PropertiesParser::parse(Rule::property, property_text) {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(e) => {
            // If parsing fails, return a default value
            return Ok(Value::String(String::new()));
        }
    };
    
    // Find the value part
    for pair in property.into_inner() {
        if pair.as_rule() == Rule::value {
            return process_value(pair, context, class_index);
        }
    }
    
    // Default value if no value found
    Ok(Value::String(String::new()))
}

/// Processes a value to extract its content
fn process_value(pair: Pair<Rule>, context: &mut ParseContext, class_index: usize) -> Result<Value, ParseError> {
    match pair.as_rule() {
        Rule::value => {
            // Process the first child of the value
            let inner = pair.into_inner().next().unwrap();
            process_value(inner, context, class_index)
        }
        Rule::string_literal => {
            // Extract the string content (remove quotes)
            let text = pair.as_str();
            let content = text[1..text.len() - 1].replace("\"\"", "\"");
            Ok(Value::String(content))
        }
        Rule::number => {
            // Parse as a number
            let text = pair.as_str();
            if text.contains('.') || text.contains('e') || text.contains('E') {
                // Float
                match text.parse::<f64>() {
                    Ok(n) => Ok(Value::Number(n)),
                    Err(_) => Ok(Value::String(text.to_string())),
                }
            } else {
                // Integer
                match text.parse::<i32>() {
                    Ok(n) => Ok(Value::Integer(n)),
                    Err(_) => Ok(Value::String(text.to_string())),
                }
            }
        }
        Rule::array => {
            // Process array items
            let mut items = Vec::new();
            for item_pair in pair.into_inner() {
                let item_value = process_value(item_pair, context, class_index)?;
                items.push(item_value);
            }
            Ok(Value::Array(items))
        }
        Rule::expression => {
            // Store the expression as a string
            Ok(Value::Expression(pair.as_str().to_string()))
        }
        Rule::macro_call => {
            // Store the macro call as a reference for later resolution
            let macro_text = pair.as_str().to_string();
            
            // Add an unresolved reference
            let unresolved = UnresolvedReference::new(
                class_index,
                ReferenceType::MacroExpansion,
                macro_text.clone(),
                format!("macro call: {}", macro_text),
            );
            context.unresolved_references.push(unresolved);
            
            Ok(Value::Reference(macro_text))
        }
        Rule::any_identifier => {
            // Check if it's an enum value
            let identifier = pair.as_str();
            if let Some(value) = context.enum_values.get(identifier) {
                Ok(Value::Integer(*value))
            } else {
                // Add an unresolved reference
                let unresolved = UnresolvedReference::new(
                    class_index,
                    ReferenceType::PropertyValue,
                    identifier.to_string(),
                    format!("identifier: {}", identifier),
                );
                context.unresolved_references.push(unresolved);
                
                Ok(Value::Reference(identifier.to_string()))
            }
        }
        _ => {
            // Default to string for unknown types
            Ok(Value::String(pair.as_str().to_string()))
        }
    }
}

/// Processes nested classes recursively
fn process_nested_classes(raw_class: &crate::parser::models::RawClassDef, context: &mut ParseContext, parent_index: usize) -> Result<(), ParseError> {
    // Process each nested class
    for nested_class in &raw_class.nested_classes {
        // Create a new class
        let mut class = crate::parser::models::Class::new();
        class.name = Some(nested_class.name.clone());
        
        // Process properties for the nested class
        for raw_property in &nested_class.properties {
            let property_result = process_property(raw_property.raw_text.as_str(), context, parent_index)?;
            
            // Add the property to the class
            class.properties.insert(raw_property.name.clone(), property_result);
        }
        
        // Process nested classes recursively for this nested class
        // We'll need to do this directly on the class before adding it to the parent
        let mut nested_raw_class_copy = nested_class.clone();
        nested_raw_class_copy.properties.clear(); // Clear properties as we've already processed them
        process_nested_classes_for_standalone(&nested_raw_class_copy, &mut class)?;
        
        // Add to the parent class
        if let Some(parent) = context.classes.get_mut(parent_index) {
            parent.classes.insert(nested_class.name.clone(), class);
        }
    }
    
    Ok(())
}

/// Helper function to process nested classes for a standalone class (not in context)
fn process_nested_classes_for_standalone(raw_class: &crate::parser::models::RawClassDef, parent_class: &mut crate::parser::models::Class) -> Result<(), ParseError> {
    // Process each nested class
    for nested_class in &raw_class.nested_classes {
        // Create a new class
        let mut class = crate::parser::models::Class::new();
        class.name = Some(nested_class.name.clone());
        
        // Process nested classes recursively
        let mut nested_raw_class_copy = nested_class.clone();
        nested_raw_class_copy.properties.clear(); // We don't process properties here
        process_nested_classes_for_standalone(&nested_raw_class_copy, &mut class)?;
        
        // Add to the parent class
        parent_class.classes.insert(nested_class.name.clone(), class);
    }
    
    Ok(())
} 