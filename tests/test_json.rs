#[test]
fn test_output() {
    let json_str = r#"{"hello":"world","nested":{"arr":[1,2,3]}}"#;
    let val: serde_json::Value = serde_json::from_str(json_str).unwrap();
    println!("DEFAULT PRETTY:\n{}", serde_json::to_string_pretty(&val).unwrap());
}
