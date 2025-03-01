use cpp_parser::parser::file_parser::parse_cpp_file;
use cpp_parser::parser::ast::Value;

#[test]
fn test_parse_complex_config() {
    let path = "tests/data/complex_config.cpp";
    let cpp_file = parse_cpp_file(path).expect("Failed to parse complex_config.cpp");
    
    // Check the number of classes
    assert_eq!(cpp_file.classes.len(), 1);
    
    // Check the class name
    let class = &cpp_file.classes[0];
    assert_eq!(class.name, "CfgPatches");
    
    assert_eq!(class.properties.len(), 5);
    
    // Check specific properties
    let author_property = class.properties.get("author").expect("Missing author property");
    assert_eq!(author_property.value, Value::String("$STR_A3_Bohemia_Interactive".to_string()));
    
    let name_property = class.properties.get("name").expect("Missing name property");
    assert_eq!(name_property.value, Value::String("Arma 3 Eden Update - Eden Editor".to_string()));
    
    let url_property = class.properties.get("url").expect("Missing url property");
    assert_eq!(url_property.value, Value::String("https://www.arma3.com".to_string()));
    
    let required_addons_property = class.properties.get("requiredAddons").expect("Missing requiredAddons property");
    assert_eq!(required_addons_property.value, Value::Array(vec![Value::String("A3_Data_F_Exp_B".to_string())]));
    
    let required_version_property = class.properties.get("requiredVersion").expect("Missing requiredVersion property");
    assert_eq!(required_version_property.value, Value::Number(0.1));
}
