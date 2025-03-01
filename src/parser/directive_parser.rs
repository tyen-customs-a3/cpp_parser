use crate::parser::ast::{Directive, DirectiveKind};
use crate::parser::parser::Rule;

/// Parse a #define directive from a Pest pair
pub fn parse_define(pair: pest::iterators::Pair<Rule>) -> Directive {
    let content = pair.as_str();
    let value = content.split_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string();
    
    Directive {
        kind: DirectiveKind::Define,
        value,
    }
}

/// Parse an #include directive from a Pest pair
pub fn parse_include(pair: pest::iterators::Pair<Rule>) -> Directive {
    let value = pair.as_str().trim_start_matches("#include").trim().to_string();
    Directive {
        kind: DirectiveKind::Include,
        value,
    }
}