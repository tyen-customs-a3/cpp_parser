/// Types of code elements that can be found in a code file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeElementType {
    /// A preprocessor directive like #define or #include
    Define,
    /// An enumeration
    Enum,
    /// A class definition
    Class,
    /// A forward declaration (class declaration without a body)
    ForwardDeclaration,
    /// A delete statement (e.g., delete Revive;)
    Delete,
    /// Other content that doesn't match any specific type
    Other,
}

/// Represents a single element in the code file
#[derive(Debug, Clone)]
pub struct CodeElement {
    /// The type of this element
    pub element_type: CodeElementType,
    /// The name of the element, if applicable
    pub name: Option<String>,
    /// The parent class name, if applicable (for classes with inheritance)
    pub parent: Option<String>,
    /// The raw content of the element
    pub content: String,
    /// Start position in the original text
    pub start_pos: usize,
    /// End position in the original text
    pub end_pos: usize,
    /// Nested elements (for classes with nested classes)
    pub nested_elements: Vec<CodeElement>,
}

impl CodeElement {
    /// Create a new code element
    pub fn new(
        element_type: CodeElementType,
        name: Option<String>,
        parent: Option<String>,
        content: String,
        start_pos: usize,
        end_pos: usize,
    ) -> Self {
        CodeElement {
            element_type,
            name,
            parent,
            content,
            start_pos,
            end_pos,
            nested_elements: Vec::new(),
        }
    }

    /// Add a nested element (for classes with nested classes)
    pub fn add_nested_element(&mut self, element: CodeElement) {
        self.nested_elements.push(element);
    }

    /// Get the raw text of this element from the original input
    pub fn get_raw_text<'a>(&self, input: &'a str) -> Option<&'a str> {
        if self.start_pos < input.len() && self.end_pos <= input.len() && self.start_pos <= self.end_pos {
            Some(&input[self.start_pos..self.end_pos])
        } else {
            None
        }
    }
}
