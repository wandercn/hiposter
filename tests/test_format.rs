use hiposter_gpui::format_json;
use serde_json::json;

#[test]
fn test_format_json_simple() {
    let val = json!({"hello": "world"});
    let formatted = format_json(&val);
    assert!(formatted.contains("    \"hello\": \"world\""));
}

#[test]
fn test_format_json_nested() {
    let val = json!({
        "a": 1,
        "b": {
            "c": 2
        }
    });
    let formatted = format_json(&val);
    // Verify 4-space indentation
    assert!(formatted.contains("\n    \"a\": 1"));
    assert!(formatted.contains("\n    \"b\": {"));
    assert!(formatted.contains("\n        \"c\": 2"));
}

#[test]
fn test_format_json_array() {
    let val = json!([1, 2, 3]);
    let formatted = format_json(&val);
    assert!(formatted.contains("\n    1,"));
    assert!(formatted.contains("\n    2,"));
    assert!(formatted.contains("\n    3"));
}
