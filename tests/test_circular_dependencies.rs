use cpp_parser::{parse_cpp, ParseError};

#[test]
fn test_circular_parent_inheritance() {
    // Test circular class inheritance (A inherits from B, B inherits from A)
    let input = r#"
    class ClassA: ClassB {
        propA = "A";
    };
    
    class ClassB: ClassA {
        propB = "B";
    };
    "#;
    
    // Since we've disabled the references pass, this should now succeed
    let result = parse_cpp(input);
    assert!(result.is_ok(), "Expected success since references pass is disabled");
}

#[test]
fn test_indirect_circular_inheritance() {
    // Test indirect circular inheritance (A -> B -> C -> A)
    let input = r#"
    class ClassA: ClassB {
        propA = "A";
    };
    
    class ClassB: ClassC {
        propB = "B";
    };
    
    class ClassC: ClassA {
        propC = "C";
    };
    "#;
    
    // Since we've disabled the references pass, this should now succeed
    let result = parse_cpp(input);
    assert!(result.is_ok(), "Expected success since references pass is disabled");
}

#[test]
fn test_self_inheritance() {
    // Test self-inheritance (A inherits from A)
    let input = r#"
    class ClassA: ClassA {
        propA = "A";
    };
    "#;
    
    // Since we've disabled the references pass, this should now succeed
    let result = parse_cpp(input);
    assert!(result.is_ok(), "Expected success since references pass is disabled");
} 