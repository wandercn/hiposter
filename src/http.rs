use crate::model::{Header, HttpMethod, HttpRequest, HttpResponse};
use anyhow::Result;
use reqwest::{Client, Method};
use std::time::Duration;

pub async fn execute_request(request: &HttpRequest) -> Result<HttpResponse> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

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
    builder = builder.header("Content-Type", &request.content_type);

    // Add custom headers
    for header in &request.headers {
        if !header.key.trim().is_empty() {
            builder = builder.header(&header.key, &header.value);
        }
    }

    // Add body if it's not a GET/HEAD request
    if !matches!(request.method, HttpMethod::GET | HttpMethod::HEAD) && !request.body.is_empty() {
        builder = builder.body(request.body.clone());
    }

    let start = std::time::Instant::now();
    let response = builder.send().await?;
    let duration = start.elapsed();

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

    let body = response.text().await?;
    let size = body.len();

    Ok(HttpResponse {
        status_code,
        status_text,
        headers,
        body,
        size,
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
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status_code, 200);
        assert!(response.body.contains("url"));
    }
}
