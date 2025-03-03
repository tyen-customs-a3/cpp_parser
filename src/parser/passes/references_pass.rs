use pest::Parser;
use pest::iterators::Pair;
use crate::parser::models::{ParseContext, ParseError, SymbolType, Value, ReferenceType};

// Define the parser
#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar/grammar.pest"]
pub struct ReferencesParser;

// Rule enum is automatically generated by pest_derive

/// Performs the references pass on the context
pub fn references_pass(context: &mut ParseContext) -> Result<(), ParseError> {
    // Process unresolved references
    let unresolved_refs = std::mem::take(&mut context.unresolved_references);
    
    for unresolved in unresolved_refs {
        match unresolved.reference_type {
            ReferenceType::Parent => {
                resolve_parent_reference(&unresolved, context)?;
            }
            ReferenceType::PropertyValue => {
                resolve_property_reference(&unresolved, context)?;
            }
            ReferenceType::MacroExpansion => {
                resolve_macro_reference(&unresolved, context)?;
            }
        }
    }
    
    Ok(())
}

/// Resolves a parent class reference
fn resolve_parent_reference(unresolved: &crate::parser::models::UnresolvedReference, context: &mut ParseContext) -> Result<(), ParseError> {
    // Get the class that needs its parent resolved
    let class = match context.classes.get_mut(unresolved.source_class_index) {
        Some(c) => c,
        None => return Ok(()),
    };
    
    // Set the parent name
    class.parent = Some(unresolved.name.clone());
    
    // If the parent is in the symbol table, we could also copy properties, etc.
    if let Some(SymbolType::Class(parent_index)) = context.symbols.get(&unresolved.name) {
        // Here we could copy properties from parent to child if needed
        // For now, we just set the parent name
    }
    
    Ok(())
}

/// Resolves a property value reference
fn resolve_property_reference(unresolved: &crate::parser::models::UnresolvedReference, context: &mut ParseContext) -> Result<(), ParseError> {
    // Get the class that has the property
    let class = match context.classes.get_mut(unresolved.source_class_index) {
        Some(c) => c,
        None => return Ok(()),
    };
    
    // Find the property with this reference
    for (_, value) in class.properties.iter_mut() {
        if let Value::Reference(ref name) = value {
            if *name == unresolved.name {
                // Try to resolve the reference
                if let Some(symbol) = context.symbols.get(&unresolved.name) {
                    match symbol {
                        SymbolType::Enum(_, enum_value) => {
                            *value = Value::Integer(*enum_value);
                        }
                        SymbolType::Class(class_index) => {
                            // Reference to a class - could create a class value
                            // For now, keep as a reference
                        }
                        SymbolType::Macro(_, _) => {
                            // Reference to a macro - could expand it
                            // For now, keep as a reference
                        }
                    }
                }
                break;
            }
        }
    }
    
    Ok(())
}

/// Resolves a macro reference
fn resolve_macro_reference(unresolved: &crate::parser::models::UnresolvedReference, context: &mut ParseContext) -> Result<(), ParseError> {
    // Get the class that has the macro
    let class = match context.classes.get_mut(unresolved.source_class_index) {
        Some(c) => c,
        None => return Ok(()),
    };
    
    // Extract the macro name
    let macro_text = &unresolved.name;
    let macro_name = if let Some(open_paren) = macro_text.find('(') {
        &macro_text[..open_paren]
    } else {
        macro_text
    };
    
    // Find the macro definition
    if let Some(expansion) = context.macro_defs.get(macro_name) {
        // For now, we don't actually expand macros, just note that we found it
        // In a real implementation, you would replace the macro with its expansion
    }
    
    Ok(())
} 