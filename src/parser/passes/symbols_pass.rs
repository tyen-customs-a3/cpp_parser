use pest::Parser;
use pest::iterators::Pair;
use crate::parser::models::{ParseContext, ParseError, SymbolType, Class, ReferenceType, UnresolvedReference};
use log::{debug, info, warn, error, trace};

// Define the parser
#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar/grammar.pest"]
pub struct SymbolsParser;

/// Performs the symbols pass on the context
pub fn symbols_pass(context: &mut ParseContext) -> Result<(), ParseError> {
    debug!(target: "cpp_parser::symbols_pass", "Starting symbols pass with {} raw enums and {} raw classes", 
          context.raw_enums.len(), context.raw_classes.len());
    
    // Process enums first to build the enum values table
    // Clone the raw_enums to avoid borrowing issues
    let raw_enums = context.raw_enums.clone();
    for (i, raw_enum) in raw_enums.iter().enumerate() {
        debug!(target: "cpp_parser::symbols_pass", "Processing enum {}/{}", 
               i+1, raw_enums.len());
        process_enum(raw_enum.raw_text.as_str(), context)?;
    }
    
    // Process classes to build the symbol table
    let raw_classes = context.raw_classes.clone();
    for (index, raw_class) in raw_classes.iter().enumerate() {
        debug!(target: "cpp_parser::symbols_pass", "Processing class {}/{}: {}", 
               index+1, raw_classes.len(), raw_class.name);
        
        // Create a new class
        let mut class = Class::new();
        class.name = Some(raw_class.name.clone());
        
        // If there's a parent, set it directly on the class
        if let Some(parent_name) = &raw_class.parent_name {
            class.parent = Some(parent_name.clone());
        }
        
        // Add to the symbol table
        context.symbols.insert(raw_class.name.clone(), SymbolType::Class(index));
        
        // Add to the classes vector
        context.classes.push(class);
        
        // If there's a parent, add an unresolved reference
        if let Some(parent_name) = &raw_class.parent_name {
            let unresolved = UnresolvedReference::new(
                index,
                ReferenceType::Parent,
                parent_name.clone(),
                raw_class.location.clone()
            );
            context.unresolved_references.push(unresolved);
        }
    }
    
    // Process macros
    for raw_macro in &context.raw_macros {
        context.macro_defs.insert(raw_macro.name.clone(), raw_macro.params.clone());
    }
    
    debug!(target: "cpp_parser::symbols_pass", "Completed symbols pass with {} symbols and {} unresolved references", 
          context.symbols.len(), context.unresolved_references.len());
    Ok(())
}

/// Processes an enum definition to extract enum values
fn process_enum(enum_text: &str, context: &mut ParseContext) -> Result<(), ParseError> {
    debug!(target: "cpp_parser::symbols_pass", "Processing enum text of {} bytes", enum_text.len());
    
    // Simple parsing for now - just extract key=value pairs
    for line in enum_text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("enum") || line.starts_with("{") || line.starts_with("}") {
            continue;
        }
        
        // Remove trailing comma if present
        let line = line.trim_end_matches(',');
        
        // Check for key=value format
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim();
            let value_str = line[pos+1..].trim();
            
            // Parse the value as an integer
            if let Ok(value) = value_str.parse::<i32>() {
                context.enum_values.insert(key.to_string(), value);
            }
        } else {
            // If no value specified, use auto-incrementing value
            let next_value = context.enum_values.len() as i32;
            context.enum_values.insert(line.to_string(), next_value);
        }
    }
    
    trace!(target: "cpp_parser::symbols_pass", "Parsed enum with {} values", context.enum_values.len());
    Ok(())
} 