use crate::error::Error;
use std::time::Instant;
use log::{debug, trace, warn, error};

/// Token types produced by the lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// 'class' keyword
    Class,
    
    /// Identifier (class name, property name)
    Identifier(String),
    
    /// Opening brace '{'
    OpenBrace,
    
    /// Closing brace '}'
    CloseBrace,
    
    /// Colon ':'
    Colon,
    
    /// Semicolon ';'
    Semicolon,
    
    /// Equal sign '='
    Equals,
    
    /// Plus-equals '+='
    PlusEquals,
    
    /// Opening bracket '['
    OpenBracket,
    
    /// Closing bracket ']'
    CloseBracket,
    
    /// Opening parenthesis '('
    OpenParen,
    
    /// Closing parenthesis ')'
    CloseParen,
    
    /// String literal
    StringLiteral(String),
    
    /// Numeric literal
    NumericLiteral(f64),
    
    /// Comma ','
    Comma,
    
    /// Preprocessor directive like #include, #define, etc.
    Preprocessor(String),
    
    /// End of file
    EOF,
}

/// Performs lexical analysis to convert input text into tokens
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Get the current character if not at the end of the input
    fn current_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }
    
    /// Advance to the next character
    fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            self.position += c.len_utf8();
            
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
    
    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }
    
    /// Process a preprocessor directive
    fn read_preprocessor(&mut self) -> Result<Token, Error> {
        let mut directive = String::new();
        
        // Skip the '#' character
        self.advance();
        
        // Read the directive name
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() {
                directive.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        if directive.is_empty() {
            return Err(Error::Lexer {
                message: "Empty preprocessor directive".to_string(),
                line: self.line,
                column: self.column,
            });
        }
        
        // Skip the rest of the line
        while let Some(c) = self.current_char() {
            if c == '\n' {
                self.advance();
                break;
            }
            self.advance();
        }
        
        Ok(Token::Preprocessor(directive))
    }
    
    /// Skip comments (both // and /* */)
    fn skip_comments(&mut self) -> Result<(), Error> {
        if let Some('/') = self.current_char() {
            self.advance();
            
            match self.current_char() {
                // Line comment
                Some('/') => {
                    self.advance();
                    while let Some(c) = self.current_char() {
                        if c == '\n' {
                            self.advance();
                            break;
                        }
                        self.advance();
                    }
                }
                
                // Block comment
                Some('*') => {
                    self.advance();
                    let mut end_found = false;
                    
                    while let Some(c) = self.current_char() {
                        if c == '*' {
                            self.advance();
                            if let Some('/') = self.current_char() {
                                self.advance();
                                end_found = true;
                                break;
                            }
                        } else {
                            self.advance();
                        }
                    }
                    
                    if !end_found {
                        return Err(Error::Lexer {
                            message: "Unterminated block comment".to_string(),
                            line: self.line,
                            column: self.column,
                        });
                    }
                }
                
                // Not a comment, just a forward slash
                _ => {
                    // Go back since we consumed the slash
                    self.position -= 1;
                    if self.column > 1 {
                        self.column -= 1;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Parse an identifier
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        let mut identifier = String::new();
        
        // First character must be a letter or underscore
        if let Some(c) = self.current_char() {
            if c.is_alphabetic() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                warn!("Invalid first character for identifier: '{}' at position {}", c, start_pos);
                return identifier;
            }
        }
        
        // Subsequent characters can be letters, digits, or underscores
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        if !identifier.is_empty() {
            trace!("Found identifier '{}' at position {}", identifier, start_pos);
        } else {
            // This is a numeric identifier (starts with a digit)
            let mut numeric_identifier = String::new();
            self.position = start_pos; // Reset position
            
            while let Some(c) = self.current_char() {
                if c.is_alphanumeric() || c == '_' {
                    numeric_identifier.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            
            if !numeric_identifier.is_empty() {
                trace!("Found numeric identifier '{}' at position {}", numeric_identifier, start_pos);
                return numeric_identifier;
            } else {
                warn!("No valid identifier found at position {}", start_pos);
            }
        }
        
        identifier
    }
    
    /// Parse a number literal
    fn read_number(&mut self) -> Result<f64, Error> {
        let start_pos = self.position;
        let mut num_str = String::new();
        let mut is_special_format = false;
        
        // Handle negative numbers
        if let Some('-') = self.current_char() {
            num_str.push('-');
            self.advance();
        }
        
        // Read digits before decimal point
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                num_str.push(c);
                self.advance();
            } else if c == '.' {
                // Decimal point
                num_str.push('.');
                self.advance();
                break;
            } else if c == 'e' || c == 'E' {
                // Scientific notation
                num_str.push('e');
                self.advance();
                
                // Optional sign
                if let Some(sign) = self.current_char() {
                    if sign == '+' || sign == '-' {
                        num_str.push(sign);
                        self.advance();
                    }
                }
                
                // Exponent digits
                let mut has_exponent = false;
                while let Some(c) = self.current_char() {
                    if c.is_digit(10) {
                        num_str.push(c);
                        self.advance();
                        has_exponent = true;
                    } else {
                        break;
                    }
                }
                
                if !has_exponent {
                    return Err(Error::Lexer {
                        message: format!("Invalid scientific notation: {}", num_str),
                        line: self.line,
                        column: self.column,
                    });
                }
                
                break;
            } else if c == '*' || c == '/' || c == '+' || c == '-' {
                // This is a mathematical expression, not just a number
                is_special_format = true;
                
                // Read the rest of the expression
                while let Some(c) = self.current_char() {
                    if c.is_digit(10) || c == '.' || c == '*' || c == '/' || c == '+' || c == '-' || c == 'e' || c == 'E' {
                        num_str.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                
                break;
            } else {
                break;
            }
        }
        
        // Read digits after decimal point
        if num_str.ends_with('.') {
            while let Some(c) = self.current_char() {
                if c.is_digit(10) {
                    num_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        if is_special_format {
            // Just return 0.0 for now - we're not doing calculations with these values
            return Ok(0.0);
        }
        
        // Normal decimal number conversion
        match num_str.parse::<f64>() {
            Ok(num) => Ok(num),
            Err(_) => {
                // For complex numerical expressions, just return 0
                // This handles cases like "3.5*0.79432821" without full expression parsing
                if num_str.contains('*') || num_str.contains('/') || num_str.contains('+') || num_str.contains('-') {
                    Ok(0.0)
                } else {
                    return Err(Error::Lexer {
                        message: format!("Invalid number format: '{}'", num_str),
                        line: self.line,
                        column: self.column,
                    });
                }
            }
        }
    }
    
    /// Parse a string literal
    fn read_string(&mut self) -> Result<String, Error> {
        let start_line = self.line;
        let start_column = self.column;
        let mut result = String::new();
        
        // Skip the opening quote
        self.advance();
        
        // Read until closing quote
        while let Some(c) = self.current_char() {
            if c == '"' {
                // Skip the closing quote
                self.advance();
                return Ok(result);
            } else if c == '\\' {
                // Handle escape sequences
                self.advance();
                if let Some(next_char) = self.current_char() {
                    match next_char {
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => {
                            return Err(Error::Lexer {
                                message: format!("Invalid escape sequence: \\{}", next_char),
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                    self.advance();
                } else {
                    return Err(Error::Lexer {
                        message: "Unexpected end of input after escape character".to_string(),
                        line: self.line,
                        column: self.column,
                    });
                }
            } else {
                result.push(c);
                self.advance();
            }
        }
        
        Err(Error::Lexer {
            message: "Unterminated string literal".to_string(),
            line: start_line,
            column: start_column,
        })
    }
    
    /// Get the next token from the input
    pub fn next_token(&mut self) -> Result<Token, Error> {
        // Skip whitespace and comments
        self.skip_whitespace();
        if let Err(e) = self.skip_comments() {
            return Err(e);
        }
        
        // Check if we've reached the end of the input
        if self.position >= self.input.len() {
            return Ok(Token::EOF);
        }
        
        // Get the current character
        let current_char = self.current_char().unwrap();
        let current_line = self.line;
        let current_column = self.column;
        
        // Match on the current character to determine the token type
        match current_char {
            '{' => {
                self.advance();
                Ok(Token::OpenBrace)
            }
            '}' => {
                self.advance();
                Ok(Token::CloseBrace)
            }
            '[' => {
                self.advance();
                Ok(Token::OpenBracket)
            }
            ']' => {
                self.advance();
                Ok(Token::CloseBracket)
            }
            '(' => {
                self.advance();
                Ok(Token::OpenParen)
            }
            ')' => {
                self.advance();
                Ok(Token::CloseParen)
            }
            ':' => {
                self.advance();
                Ok(Token::Colon)
            }
            ';' => {
                self.advance();
                Ok(Token::Semicolon)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            '=' => {
                self.advance();
                Ok(Token::Equals)
            }
            '+' => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Ok(Token::PlusEquals)
                } else {
                    Ok(Token::Identifier("+".to_string()))
                }
            }
            '"' | '\'' => {
                Ok(Token::StringLiteral(self.read_string()?))
            }
            '#' => {
                self.read_preprocessor()
            }
            c if c.is_digit(10) || c == '-' || c == '+' || c == '.' => {
                // Check if this is part of an identifier
                let next_pos = self.position + 1;
                if c == '-' || c == '+' || c == '.' {
                    if next_pos < self.input.len() {
                        let next_char = self.input[next_pos..].chars().next().unwrap();
                        if !next_char.is_digit(10) && next_char != '.' {
                            // This is an identifier or operator, not a number
                            let identifier = self.read_identifier();
                            if !identifier.is_empty() {
                                Ok(Token::Identifier(identifier))
                            } else {
                                Ok(Token::NumericLiteral(self.read_number()?))
                            }
                        } else {
                            // This is a number
                            Ok(Token::NumericLiteral(self.read_number()?))
                        }
                    } else {
                        // End of input, treat as identifier
                        let identifier = self.read_identifier();
                        if !identifier.is_empty() {
                            Ok(Token::Identifier(identifier))
                        } else {
                            Ok(Token::NumericLiteral(self.read_number()?))
                        }
                    }
                } else {
                    // Definitely a number
                    Ok(Token::NumericLiteral(self.read_number()?))
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                let identifier = self.read_identifier();
                
                match identifier.as_str() {
                    "class" => Ok(Token::Class),
                    _ => Ok(Token::Identifier(identifier)),
                }
            }
            _ => {
                self.advance();
                Err(Error::Lexer {
                    message: format!("Unexpected character: '{}'", current_char),
                    line: current_line,
                    column: current_column,
                })
            }
        }
    }
    
    /// Tokenize the entire input and return all tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let start_time = Instant::now();
        let mut tokens = Vec::new();
        let mut token_count = 0;
        
        trace!("Starting tokenization at {:?}", start_time.elapsed());
        
        loop {
            match self.next_token() {
                Ok(token) => {
                    token_count += 1;
                    if let Token::EOF = token {
                        tokens.push(token);
                        break;
                    }
                    tokens.push(token);
                }
                Err(e) => {
                    error!("Error during tokenization at {:?}: {}", start_time.elapsed(), e);
                    return Err(e);
                }
            }
        }
        
        debug!("Tokenization complete at {:?}, {} tokens", start_time.elapsed(), token_count);
        
        Ok(tokens)
    }
} 