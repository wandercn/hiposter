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
    assert!(err_msg.contains("Connection failed") || err_msg.contains("Network error"));
}
