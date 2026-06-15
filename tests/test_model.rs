use hiposter_gpui::model::*;
use serde_json::json;

#[test]
fn test_http_method_default() {
    assert_eq!(HttpMethod::default(), HttpMethod::GET);
}

#[test]
fn test_http_request_serialization() {
    let req = HttpRequest {
        method: HttpMethod::POST,
        url: "https://example.com".to_string(),
        headers: vec![Header { key: "Content-Type".to_string(), value: "application/json".to_string() }],
        params: vec![],
        body: HttpBody::Raw("{\"key\":\"value\"}".to_string()),
        content_type: "application/json".to_string(),
        auth: AuthData::default(),
    };
    
    let serialized = serde_json::to_value(&req).unwrap();
    assert_eq!(serialized["method"], "POST");
    assert_eq!(serialized["url"], "https://example.com");
    assert_eq!(serialized["headers"][0]["key"], "Content-Type");
}

#[test]
fn test_http_body_variants() {
    let none = HttpBody::None;
    assert_eq!(serde_json::to_value(&none).unwrap(), "None");
    
    let raw = HttpBody::Raw("test".to_string());
    assert_eq!(serde_json::to_value(&raw).unwrap()["Raw"], "test");
}
