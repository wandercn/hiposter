use hiposter_gpui::http::execute_request;
use hiposter_gpui::model::{HttpRequest, HttpMethod};

#[tokio::test]
async fn test_get_request_live() {
    // 仅在有网络连接时运行的简单冒烟测试
    let request = HttpRequest {
        method: HttpMethod::GET,
        url: "https://httpbin.org/get".to_string(),
        ..Default::default()
    };
    let result = execute_request(&request).await;
    if let Ok(response) = result {
        assert_eq!(response.status_code, 200);
        assert!(response.body.contains("url"));
    }
}

#[tokio::test]
async fn test_invalid_url() {
    let request = HttpRequest {
        method: HttpMethod::GET,
        url: "http://invalid.url.that.does.not.exist".to_string(),
        ..Default::default()
    };
    let result = execute_request(&request).await;
    assert!(result.is_err());
    let err_msg = result.err().unwrap().to_string();
    assert!(
        err_msg.contains("Connection failed") || err_msg.contains("Network error") || err_msg.contains("Request error"),
        "Unexpected error message: {}",
        err_msg
    );
}

#[test]
fn test_request_to_curl_get() {
    use hiposter_gpui::http::request_to_curl;
    let request = HttpRequest {
        method: HttpMethod::GET,
        url: "https://httpbin.org/get?foo=bar&baz=qux".to_string(),
        ..Default::default()
    };
    let curl = request_to_curl(&request);
    assert!(curl.starts_with("curl -X GET"));
    assert!(curl.contains("-H 'User-Agent: hiposter-gpui/0.1.0'"));
    assert!(curl.contains("-H 'Accept: */*'"));
    assert!(curl.contains("'https://httpbin.org/get?foo=bar&baz=qux'"));
}

#[test]
fn test_request_to_curl_post_json() {
    use hiposter_gpui::http::request_to_curl;
    use hiposter_gpui::model::HttpBody;
    let request = HttpRequest {
        method: HttpMethod::POST,
        url: "https://httpbin.org/post".to_string(),
        headers: vec![
            hiposter_gpui::model::Header {
                key: "X-Custom-Header".to_string(),
                value: "custom-value".to_string(),
            }
        ],
        body: HttpBody::Raw("{\"key\":\"value\"}".to_string()),
        content_type: "application/json".to_string(),
        ..Default::default()
    };
    let curl = request_to_curl(&request);
    assert!(curl.starts_with("curl -X POST"));
    assert!(curl.contains("-H 'X-Custom-Header: custom-value'"));
    assert!(curl.contains("-H 'Content-Type: application/json'"));
    assert!(curl.contains("--data-raw '{\"key\":\"value\"}'"));
    assert!(curl.contains("'https://httpbin.org/post'"));
}

#[test]
fn test_request_to_curl_bearer_auth() {
    use hiposter_gpui::http::request_to_curl;
    use hiposter_gpui::model::{AuthData, AuthType};
    let request = HttpRequest {
        method: HttpMethod::GET,
        url: "https://httpbin.org/bearer".to_string(),
        auth: AuthData {
            auth_type: AuthType::Bearer,
            token: "my-secret-token".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    let curl = request_to_curl(&request);
    assert!(curl.contains("-H 'Authorization: Bearer my-secret-token'"));
}

#[test]
fn test_request_to_curl_basic_auth() {
    use hiposter_gpui::http::request_to_curl;
    use hiposter_gpui::model::{AuthData, AuthType};
    let request = HttpRequest {
        method: HttpMethod::GET,
        url: "https://httpbin.org/basic-auth/user/pass".to_string(),
        auth: AuthData {
            auth_type: AuthType::Basic,
            username: "user".to_string(),
            password: "pass".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    let curl = request_to_curl(&request);
    // basic auth header: Authorization: Basic dXNlcjpwYXNz (user:pass in base64 is dXNlcjpwYXNz)
    assert!(curl.contains("-H 'Authorization: Basic dXNlcjpwYXNz'"));
}

#[test]
fn test_request_to_curl_urlencoded() {
    use hiposter_gpui::http::request_to_curl;
    use hiposter_gpui::model::{HttpBody, Header};
    let request = HttpRequest {
        method: HttpMethod::POST,
        url: "https://httpbin.org/post".to_string(),
        body: HttpBody::UrlEncoded(vec![
            Header {
                key: "name".to_string(),
                value: "John Doe".to_string(),
            },
            Header {
                key: "age".to_string(),
                value: "30".to_string(),
            }
        ]),
        ..Default::default()
    };
    let curl = request_to_curl(&request);
    assert!(curl.contains("-H 'Content-Type: application/x-www-form-urlencoded'"));
    // check data encoding (space becomes + or %20, e.g., John+Doe or John%20Doe)
    assert!(curl.contains("-d 'name=John+Doe&age=30'") || curl.contains("-d 'name=John%20Doe&age=30'"));
}

#[test]
fn test_request_to_curl_form_data() {
    use hiposter_gpui::http::request_to_curl;
    use hiposter_gpui::model::{HttpBody, FormDataItem, FormDataType};
    let request = HttpRequest {
        method: HttpMethod::POST,
        url: "https://httpbin.org/post".to_string(),
        body: HttpBody::FormData(vec![
            FormDataItem {
                key: "field1".to_string(),
                value: "value1".to_string(),
                item_type: FormDataType::Text,
            },
            FormDataItem {
                key: "file1".to_string(),
                value: "/path/to/file.txt".to_string(),
                item_type: FormDataType::File,
            }
        ]),
        ..Default::default()
    };
    let curl = request_to_curl(&request);
    assert!(curl.contains("-F 'field1=value1'"));
    assert!(curl.contains("-F 'file1=@/path/to/file.txt'"));
}

