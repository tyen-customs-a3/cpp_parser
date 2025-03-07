use crate::error::Error;
use crate::lexer::{Lexer, Token};
use crate::model::{Class, Property, clean_string};
use std::time::Instant;
use log::{debug, trace, warn, error};

/// Parser for configuration files
pub struct Parser {
    current_token: Option<Token>,
    tokens: Vec<Token>,
    position: usize,
    depth: usize,  // Track parsing depth
    start_time: Instant,  // Track parsing time
}

impl Parser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            current_token: None,
            tokens: Vec::new(),
            position: 0,
            depth: 0,
            start_time: Instant::now(),
        }
    }
    
    /// Parse a string and return the resulting class structure
    pub fn parse(&mut self, input: &str) -> Result<Vec<Class>, Error> {
        trace!("Starting parse at {:?}", self.start_time.elapsed());
        let mut lexer = Lexer::new(input);
        trace!("Lexer created at {:?}", self.start_time.elapsed());
        
        self.tokens = lexer.tokenize()?;
        debug!("Tokenization complete at {:?}, {} tokens", self.start_time.elapsed(), self.tokens.len());
        
        self.position = 0;
        self.depth = 0;
        self.start_time = Instant::now();
        
        if !self.tokens.is_empty() {
            self.current_token = Some(self.tokens[0].clone());
        }
        
        let result = self.parse_classes();
        
        // Log total parsing time
        debug!("Total parsing time: {:?}", self.start_time.elapsed());
        result
    }
    
    /// Advance to the next token
    fn advance(&mut self) {
        self.position += 1;
        if self.position < self.tokens.len() {
            self.current_token = Some(self.tokens[self.position].clone());
        } else {
            self.current_token = None;
        }
    }
    
    /// Peek at the next token without advancing
    fn peek_token(&self) -> Option<&Token> {
        if self.position + 1 < self.tokens.len() {
            Some(&self.tokens[self.position + 1])
        } else {
            None
        }
    }
    
    /// Consume a token of the expected type
    fn eat(&mut self, token_type: Token) -> Result<(), Error> {
        if let Some(ref token) = self.current_token {
            if std::mem::discriminant(token) == std::mem::discriminant(&token_type) {
                self.advance();
                return Ok(());
            }
        }
        
        // Skip unexpected tokens and try to recover
        self.advance();
        Ok(())
    }
    
    /// Parse one or more classes
    fn parse_classes(&mut self) -> Result<Vec<Class>, Error> {
        trace!("Starting parse_classes at {:?}", self.start_time.elapsed());
        let mut classes = Vec::new();
        let mut class_count = 0;
        
        while let Some(token) = &self.current_token {
            match token {
                Token::Class => {
                    class_count += 1;
                    trace!("Found class #{} at {:?}", class_count, self.start_time.elapsed());
                    match self.parse_class() {
                        Ok(class) => {
                            debug!("Successfully parsed class #{} at {:?}", class_count, self.start_time.elapsed());
                            classes.push(class);
                        }
                        Err(e) => {
                            // Log error but continue parsing other classes
                            error!("Error parsing class #{}: {}", class_count, e);
                            
                            // Skip until we find the next class or EOF
                            while let Some(token) = &self.current_token {
                                if let Token::Class = token {
                                    break;
                                }
                                self.advance();
                            }
                        }
                    }
                },
                Token::Identifier(name) if name == "enum" => {
                    trace!("Skipping enum at {:?}", self.start_time.elapsed());
                    // Skip the entire enum declaration
                    self.advance(); // Skip 'enum'
                    
                    // Skip until we find a semicolon
                    while let Some(token) = &self.current_token {
                        if let Token::Semicolon = token {
                            self.advance();
                            break;
                        }
                        self.advance();
                    }
                },
                _ => {
                    // Skip unrecognized tokens at the top level
                    self.advance();
                }
            }
        }
        
        debug!("Finished parse_classes with {} classes at {:?}", class_count, self.start_time.elapsed());
        Ok(classes)
    }
    
    /// Parse a single class
    fn parse_class(&mut self) -> Result<Class, Error> {
        self.depth += 1;
        if self.depth > 100 {
            let line = 1;
            let column = 1;
            return Err(Error::Parser {
                message: "Maximum parsing depth exceeded".to_string(),
                line,
                column,
            });
        }
        
        // Log parsing progress
        trace!("Parsing class at depth {} at {:?}", self.depth, self.start_time.elapsed());
        
        // Expect 'class' keyword
        let current = self.current_token.clone();
        if let Some(Token::Class) = current {
            self.advance();
        } else {
            let line = 1;
            let column = 1;
            return Err(Error::Parser {
                message: "Expected 'class' keyword".to_string(),
                line,
                column,
            });
        }
        
        // Expect class name (identifier)
        let current = self.current_token.clone();
        let class_name = if let Some(Token::Identifier(name)) = current {
            let name = name.clone();
            self.advance();
            name
        } else {
            let line = 1;
            let column = 1;
            return Err(Error::Parser {
                message: "Expected class name after 'class' keyword".to_string(),
                line,
                column,
            });
        };
        
        trace!("Found class name '{}' at {:?}", class_name, self.start_time.elapsed());
        
        // Check for parent class (optional)
        let mut parent = None;
        let current = self.current_token.clone();
        if let Some(Token::Colon) = current {
            self.advance();
            
            let current = self.current_token.clone();
            if let Some(Token::Identifier(parent_name)) = current {
                parent = Some(parent_name.clone());
                self.advance();
                trace!("Found parent class '{}' at {:?}", parent_name, self.start_time.elapsed());
            } else {
                // If no valid parent after colon, just continue
                warn!("Warning: Expected parent class name after ':' for class '{}'", class_name);
            }
        }
        
        // Create the class
        let mut class = Class::new(class_name.clone(), parent);
        
        // Check for class body or semicolon
        let mut found_brace = false;
        
        // Skip any tokens until we find an opening brace or semicolon
        while let Some(token) = self.current_token.clone() {
            match token {
                Token::OpenBrace => {
                    found_brace = true;
                    self.advance();
                    break;
                }
                Token::Semicolon => {
                    // Empty class declaration
                    self.advance();
                    self.depth -= 1;
                    return Ok(class);
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        if !found_brace {
            let line = 1;
            let column = 1;
            warn!("Warning: Expected '{{' or ';' after class declaration for '{}'", class_name);
            return Err(Error::Parser {
                message: format!("Expected '{{' or ';' after class declaration for '{}'", class_name),
                line,
                column,
            });
        }
        
        // Parse class body (properties and subclasses)
        let mut property_count = 0;
        let mut subclass_count = 0;
        let mut body_parse_limit = 1000; // Limit iterations to prevent infinite loops
        
        loop {
            if body_parse_limit == 0 {
                warn!("Warning: Class body parsing limit reached for class '{}'", class_name);
                break;
            }
            body_parse_limit -= 1;
            
            // Check for end of class body
            let current = self.current_token.clone();
            if let Some(Token::CloseBrace) = current {
                self.advance();
                break;
            }
            
            // Check for end of input
            let current = self.current_token.clone();
            if let Some(Token::EOF) = current {
                let line = 1;
                let column = 1;
                return Err(Error::Parser {
                    message: format!("Unexpected end of input while parsing class '{}'", class_name),
                    line,
                    column,
                });
            }
            
            // Parse class members
            let current = self.current_token.clone();
            match current {
                Some(Token::Class) => {
                    subclass_count += 1;
                    trace!("Found subclass #{} in '{}' at {:?}", subclass_count, class_name, self.start_time.elapsed());
                    match self.parse_class() {
                        Ok(subclass) => {
                            trace!("Successfully parsed subclass #{} in '{}' at {:?}", subclass_count, class_name, self.start_time.elapsed());
                            class.add_subclass(subclass);
                        }
                        Err(e) => {
                            error!("Error parsing subclass #{} of '{}': {}", subclass_count, class_name, e);
                            
                            // Try to recover - skip to next property or class
                            while let Some(token) = self.current_token.clone() {
                                match token {
                                    Token::Semicolon | Token::Class => break,
                                    Token::CloseBrace => {
                                        self.advance();
                                        self.depth -= 1;
                                        return Ok(class);
                                    }
                                    _ => self.advance(),
                                }
                            }
                        }
                    }
                }
                
                Some(Token::Identifier(property_name)) => {
                    let property_name = property_name.clone();
                    property_count += 1;
                    self.advance();
                    
                    trace!("Found property #{} '{}' in '{}' at {:?}", property_count, property_name, class_name, self.start_time.elapsed());
                    
                    // Check if it's an array property
                    let current = self.current_token.clone();
                    let is_array = if let Some(Token::OpenBracket) = current {
                        self.advance();
                        
                        // Skip array index if present
                        while let Some(token) = self.current_token.clone() {
                            if let Token::CloseBracket = token {
                                self.advance();
                                break;
                            }
                            self.advance();
                        }
                        
                        let current = self.current_token.clone();
                        if let Some(Token::CloseBracket) = current {
                            self.advance();
                            true
                        } else {
                            // Missing closing bracket, but assume it's an array
                            warn!("Warning: Expected ']' after array declaration for property '{}'", property_name);
                            true
                        }
                    } else {
                        false
                    };
                    
                    // Check for assignment operator
                    let mut assignment_type = "=";
                    let current = self.current_token.clone();
                    if let Some(token) = current {
                        match token {
                            Token::Equals => {
                                self.advance();
                            }
                            Token::PlusEquals => {
                                assignment_type = "+=";
                                self.advance();
                            }
                            _ => {
                                // If no assignment operator, try to recover or skip
                                warn!("Warning: Expected '=' or '+=' after property '{}', skipping", property_name);
                                
                                // Skip to semicolon
                                while let Some(token) = self.current_token.clone() {
                                    if let Token::Semicolon = token {
                                        self.advance();
                                        break;
                                    }
                                    self.advance();
                                }
                                
                                continue;
                            }
                        }
                    }
                    
                    // Parse property value
                    if is_array {
                        trace!("Parsing array property '{}' in '{}' at {:?}", property_name, class_name, self.start_time.elapsed());
                        match self.parse_array() {
                            Ok(array_values) => {
                                trace!("Successfully parsed array property '{}' in '{}' at {:?}", property_name, class_name, self.start_time.elapsed());
                                // For simplicity, we treat += the same as = for arrays
                                class.add_property(property_name.clone(), Property::Array(array_values));
                            },
                            Err(e) => {
                                error!("Error parsing array for property '{}': {}", property_name, e);
                                
                                // Add an empty array as a fallback
                                class.add_property(property_name.clone(), Property::Array(Vec::new()));
                                
                                // Try to recover - skip to semicolon
                                while let Some(token) = self.current_token.clone() {
                                    if let Token::Semicolon = token {
                                        self.advance();
                                        break;
                                    }
                                    self.advance();
                                }
                            }
                        }
                    } else {
                        // Parse single value
                        if let Some(token) = self.current_token.clone() {
                            match token {
                                Token::StringLiteral(value) => {
                                    let value = value.clone();
                                    self.advance();
                                    class.add_property(property_name.clone(), Property::String(value));
                                }
                                Token::NumericLiteral(value) => {
                                    let value = value;  // No need to dereference, just use the value directly
                                    self.advance();
                                    class.add_property(property_name.clone(), Property::Number(value));
                                }
                                Token::Identifier(value) => {
                                    let value = value.clone();
                                    self.advance();
                                    
                                    // Handle boolean values
                                    if value == "true" {
                                        class.add_property(property_name.clone(), Property::Boolean(true));
                                    } else if value == "false" {
                                        class.add_property(property_name.clone(), Property::Boolean(false));
                                    } else {
                                        // Treat other identifiers as strings
                                        class.add_property(property_name.clone(), Property::String(value));
                                    }
                                }
                                _ => {
                                    // Unknown value, try to recover
                                    warn!("Warning: Expected value after '{}' for property '{}'", assignment_type, property_name);
                                    
                                    // Add a default empty string property
                                    class.add_property(property_name.clone(), Property::String(String::new()));
                                    
                                    // Skip to semicolon
                                    while let Some(token) = self.current_token.clone() {
                                        if let Token::Semicolon = token {
                                            self.advance();
                                            break;
                                        }
                                        self.advance();
                                    }
                                    
                                    continue;
                                }
                            }
                        }
                    }
                    
                    // Expect semicolon after property value
                    if let Some(Token::Semicolon) = self.current_token.clone() {
                        self.advance();
                    } else {
                        warn!("Warning: Expected ';' after property '{}'", property_name);
                        
                        // Try to recover - skip to next property or class
                        while let Some(token) = self.current_token.clone() {
                            match token {
                                Token::Identifier(_) | Token::Class => break,
                                Token::CloseBrace => {
                                    self.advance();
                                    self.depth -= 1;
                                    return Ok(class);
                                }
                                _ => self.advance(),
                            }
                        }
                    }
                }
                
                _ => {
                    // Skip unrecognized tokens
                    self.advance();
                }
            }
        }
        
        trace!("Finished parsing class '{}' with {} properties and {} subclasses at {:?}", 
            class_name, property_count, subclass_count, self.start_time.elapsed());
        self.depth -= 1;
        Ok(class)
    }
    
    /// Parse an array of values
    fn parse_array(&mut self) -> Result<Vec<Property>, Error> {
        let mut values = Vec::new();
        let mut element_count = 0;
        let mut array_parse_limit = 1000; // Limit iterations to prevent infinite loops
        
        // Expect opening brace
        let current = self.current_token.clone();
        if let Some(Token::OpenBrace) = current {
            self.advance();
        } else {
            let line = 1;
            let column = 1;
            return Err(Error::Parser {
                message: "Expected '{' at start of array".to_string(),
                line,
                column,
            });
        }
        
        loop {
            if array_parse_limit == 0 {
                warn!("Warning: Array parsing limit reached");
                break;
            }
            array_parse_limit -= 1;
            
            // Check for end of array
            let current = self.current_token.clone();
            if let Some(Token::CloseBrace) = current {
                self.advance();
                break;
            }
            
            // Check for end of input
            let current = self.current_token.clone();
            if let Some(Token::EOF) = current {
                let line = 1;
                let column = 1;
                return Err(Error::Parser {
                    message: "Unexpected end of input while parsing array".to_string(),
                    line,
                    column,
                });
            }
            
            // Parse array element
            let current = self.current_token.clone();
            if let Some(token) = current {
                match token {
                    Token::StringLiteral(value) => {
                        let value = value.clone();
                        self.advance();
                        values.push(Property::String(value));
                        element_count += 1;
                    }
                    Token::NumericLiteral(value) => {
                        let value = value;  // No need to dereference
                        self.advance();
                        values.push(Property::Number(value));
                        element_count += 1;
                    }
                    Token::Identifier(value) => {
                        let value = value.clone();
                        self.advance();
                        
                        // Special case for LIST_X macros
                        if value.starts_with("LIST_") {
                            // Handle LIST_n macros which expand to arrays
                            let current = self.current_token.clone();
                            if let Some(Token::OpenParen) = current {
                                self.advance();
                                
                                let current = self.current_token.clone();
                                if let Some(Token::StringLiteral(list_content)) = current {
                                    let list_content = list_content.clone();
                                    self.advance();
                                    
                                    // Process the list content
                                    for item in list_content.split(',') {
                                        let item = item.trim();
                                        if !item.is_empty() {
                                            values.push(Property::String(item.to_string()));
                                            element_count += 1;
                                        }
                                    }
                                    
                                    let current = self.current_token.clone();
                                    if let Some(Token::CloseParen) = current {
                                        self.advance();
                                    } else {
                                        // Missing closing parenthesis but try to continue
                                        warn!("Warning: Expected closing parenthesis after LIST_X macro");
                                    }
                                } else {
                                    // Missing string literal but try to continue
                                    warn!("Warning: Expected string literal in LIST_X macro");
                                    self.advance();
                                }
                            }
                        } else if value == "true" {
                            values.push(Property::Boolean(true));
                            element_count += 1;
                        } else if value == "false" {
                            values.push(Property::Boolean(false));
                            element_count += 1;
                        } else {
                            // Treat other identifiers as strings
                            values.push(Property::String(value));
                            element_count += 1;
                        }
                    }
                    _ => {
                        // Skip unrecognized tokens
                        self.advance();
                    }
                }
                
                // Check for comma
                let current = self.current_token.clone();
                if let Some(Token::Comma) = current {
                    self.advance();
                }
            }
        }
        
        trace!("Finished parsing array with {} elements at {:?}", element_count, self.start_time.elapsed());
        Ok(values)
    }
} 