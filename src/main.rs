use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "cpp.pest"]
pub struct CppParser;

#[derive(Debug)]
pub struct Class {
    name: String,
    parent: Option<String>,
    nested_classes: Vec<Class>,
}

impl Class {
    fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
            nested_classes: Vec::new(),
        }
    }
}

fn parse_class(pair: pest::iterators::Pair<Rule>) -> Class {
    let mut class_name = String::new();
    let mut parent_name = None;
    let mut nested_classes = Vec::new();

    let pairs = pair.into_inner();
    for pair in pairs {
        match pair.as_rule() {
            Rule::identifier => {
                class_name = pair.as_str().to_string();
            }
            Rule::parent => {
                parent_name = Some(pair.into_inner().next().unwrap().as_str().to_string());
            }
            Rule::class_content => {
                for inner_pair in pair.into_inner() {
                    if let Rule::class_definition = inner_pair.as_rule() {
                        nested_classes.push(parse_class(inner_pair));
                    }
                }
            }
            _ => {}
        }
    }

    let mut class = Class::new(class_name, parent_name);
    class.nested_classes = nested_classes;
    class
}

fn main() {
    let path = "tests/data/@tc_mirrorform/config.cpp";
    let contents = fs::read_to_string(path).expect("Failed to read file");

    match CppParser::parse(Rule::file, &contents) {
        Ok(pairs) => {
            let mut classes = Vec::new();
            for pair in pairs {
                match pair.as_rule() {
                    Rule::file => {
                        for inner_pair in pair.into_inner() {
                            if let Rule::class_definition = inner_pair.as_rule() {
                                classes.push(parse_class(inner_pair));
                            }
                        }
                    }
                    _ => {}
                }
            }
            println!("Found classes: {:#?}", classes);
        }
        Err(e) => println!("Error: {}", e),
    }
}
