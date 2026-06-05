use crate::model::{Header, HttpMethod, HttpRequest, HttpResponse, AuthType};
use anyhow::{Context, Result};
use reqwest::{Client, Method};
use std::time::Duration;

pub async fn execute_request(request: &HttpRequest) -> Result<HttpResponse> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .danger_accept_invalid_certs(true)
        .build()
        .context("Failed to build HTTP client")?;

    let method = match request.method {
        HttpMethod::GET => Method::GET,
        HttpMethod::POST => Method::POST,
        HttpMethod::PUT => Method::PUT,
        HttpMethod::DELETE => Method::DELETE,
        HttpMethod::PATCH => Method::PATCH,
        HttpMethod::HEAD => Method::HEAD,
        HttpMethod::OPTIONS => Method::OPTIONS,
    };

    let mut builder = client.request(method, &request.url);

    // Add default headers
    builder = builder.header("User-Agent", "hiposter-gpui/0.1.0");
    builder = builder.header("Accept", "*/*");
    builder = builder.header("Connection", "keep-alive");

    // Add content-type if body is present
    if !request.body.is_empty() {
        builder = builder.header("Content-Type", &request.content_type);
    }

    // Add custom headers
    for header in &request.headers {
        if !header.key.trim().is_empty() {
            builder = builder.header(&header.key, &header.value);
        }
    }

    // Handle Auth
    match &request.auth.auth_type {
        AuthType::Bearer => {
            if !request.auth.token.is_empty() {
                builder = builder.bearer_auth(&request.auth.token);
            }
        }
        AuthType::Basic => {
            builder = builder.basic_auth(&request.auth.username, Some(&request.auth.password));
        }
        AuthType::None => {}
    }

    // Add body if it's not a GET/HEAD request
    if !matches!(request.method, HttpMethod::GET | HttpMethod::HEAD) && !request.body.is_empty() {
        builder = builder.body(request.body.clone());
    }

    let start = std::time::Instant::now();
    let response = builder.send().await.context("Failed to send request")?;
    let elapsed_ms = start.elapsed().as_millis();

    let status = response.status();
    let status_text = status.canonical_reason().unwrap_or("").to_string();
    let status_code = status.as_u16();

    let mut headers = Vec::new();
    for (name, value) in response.headers() {
        headers.push(Header {
            key: name.to_string(),
            value: value.to_str().unwrap_or("").to_string(),
        });
    }

    let bytes = response.bytes().await.context("Failed to read response body")?;
    let size = bytes.len();
    let body = String::from_utf8_lossy(&bytes).to_string();

    Ok(HttpResponse {
        status_code,
        status_text,
        headers,
        body,
        size,
        elapsed_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::HttpMethod;

    #[tokio::test]
    async fn test_get_request() {
        let request = HttpRequest {
            method: HttpMethod::GET,
            url: "https://httpbin.org/get".to_string(),
            ..Default::default()
        };
        let result = execute_request(&request).await;
        assert!(result.is_ok(), "Request failed: {:?}", result.err());
        let response = result.unwrap();
        assert_eq!(response.status_code, 200);
        assert!(response.body.contains("url"));
        assert!(response.elapsed_ms > 0);
    }
}
