use crate::model::{Header, HttpMethod, HttpRequest, HttpResponse, AuthType};
use anyhow::Result;
use reqwest::{Client, Method};
use std::time::Duration;

use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build HTTP client")
    })
}

pub async fn execute_request(request: &HttpRequest) -> Result<HttpResponse> {
    let client = get_client();

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

    // Handle Body
    use crate::model::HttpBody;
    if !matches!(request.method, HttpMethod::GET | HttpMethod::HEAD) {
        match &request.body {
            HttpBody::Raw(raw) if !raw.is_empty() => {
                builder = builder.header("Content-Type", &request.content_type);
                builder = builder.body(raw.clone());
            }
            HttpBody::FormData(form) => {
                let mut multipart = reqwest::multipart::Form::new();
                for item in form {
                    if !item.key.is_empty() {
                        multipart = multipart.text(item.key.clone(), item.value.clone());
                    }
                }
                builder = builder.multipart(multipart);
            }
            HttpBody::UrlEncoded(form) => {
                let params: Vec<(String, String)> = form.iter()
                    .filter(|h| !h.key.is_empty())
                    .map(|h| (h.key.clone(), h.value.clone()))
                    .collect();
                builder = builder.form(&params);
            }
            _ => {}
        }
    }

    let start = std::time::Instant::now();
    let response = match builder.send().await {
        Ok(resp) => resp,
        Err(e) => {
            let msg = if e.is_timeout() {
                "Request timeout".to_string()
            } else if e.is_connect() {
                "Connection failed (host unreachable or refused)".to_string()
            } else if e.is_request() {
                format!("Request error: {}", e)
            } else {
                format!("Network error: {}", e)
            };
            return Err(anyhow::anyhow!(msg));
        }
    };
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

    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(e) => return Err(anyhow::anyhow!("Failed to read response body: {}", e)),
    };
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
