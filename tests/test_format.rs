use hiposter_gpui::format_json;
use hiposter_gpui::api_tab::{url_encode, url_decode, parse_url_params};
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

#[test]
fn test_url_decode() {
    assert_eq!(url_decode("foo%20bar"), "foo bar");
    assert_eq!(url_decode("foo+bar"), "foo bar");
    assert_eq!(url_decode("foo%26bar"), "foo&bar");
}

#[test]
fn test_url_encode() {
    assert_eq!(url_encode("foo bar"), "foo+bar");
    assert_eq!(url_encode("foo&bar"), "foo%26bar");
}

#[test]
fn test_parse_url_params() {
    let url = "https://example.com/get?foo=bar&baz=qux&empty=";
    let (base, params) = parse_url_params(url);
    assert_eq!(base, "https://example.com/get");
    assert_eq!(params, vec![
        ("foo".to_string(), "bar".to_string()),
        ("baz".to_string(), "qux".to_string()),
        ("empty".to_string(), "".to_string())
    ]);
    
    let url_no_query = "https://example.com/get";
    let (base2, params2) = parse_url_params(url_no_query);
    assert_eq!(base2, "https://example.com/get");
    assert!(params2.is_empty());
}
