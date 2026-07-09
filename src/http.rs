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
                        match item.item_type {
                            crate::model::FormDataType::Text => {
                                multipart = multipart.text(item.key.clone(), item.value.clone());
                            }
                            crate::model::FormDataType::File => {
                                if !item.value.is_empty() {
                                    let path = std::path::Path::new(&item.value);
                                    if path.exists() {
                                        let file_name = path.file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or("file")
                                            .to_string();
                                        match tokio::fs::read(path).await {
                                            Ok(bytes) => {
                                                let part = reqwest::multipart::Part::bytes(bytes)
                                                    .file_name(file_name);
                                                multipart = multipart.part(item.key.clone(), part);
                                            }
                                            Err(e) => {
                                                return Err(anyhow::anyhow!("Failed to read file '{}': {}", item.value, e));
                                            }
                                        }
                                    } else {
                                        return Err(anyhow::anyhow!("File does not exist: {}", item.value));
                                    }
                                }
                            }
                        }
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

pub fn request_to_curl(request: &HttpRequest) -> String {
    let mut parts = vec!["curl".to_string()];

    // Method
    let method = match request.method {
        HttpMethod::GET => "GET",
        HttpMethod::POST => "POST",
        HttpMethod::PUT => "PUT",
        HttpMethod::DELETE => "DELETE",
        HttpMethod::PATCH => "PATCH",
        HttpMethod::HEAD => "HEAD",
        HttpMethod::OPTIONS => "OPTIONS",
    };
    parts.push(format!("-X {}", method));

    // Default Headers (same as execute_request)
    parts.push("-H 'User-Agent: hiposter-gpui/0.1.0'".to_string());
    parts.push("-H 'Accept: */*'".to_string());
    parts.push("-H 'Connection: keep-alive'".to_string());

    // Custom Headers
    for header in &request.headers {
        if !header.key.trim().is_empty() {
            parts.push(format!("-H {}", escape_shell(&format!("{}: {}", header.key.trim(), header.value))));
        }
    }

    // Auth
    match &request.auth.auth_type {
        AuthType::Bearer => {
            if !request.auth.token.is_empty() {
                parts.push(format!("-H {}", escape_shell(&format!("Authorization: Bearer {}", request.auth.token))));
            }
        }
        AuthType::Basic => {
            let encoded = base64_encode(format!("{}:{}", request.auth.username, request.auth.password).as_bytes());
            parts.push(format!("-H {}", escape_shell(&format!("Authorization: Basic {}", encoded))));
        }
        AuthType::None => {}
    }

    // Content Type & Body
    use crate::model::HttpBody;
    if !matches!(request.method, HttpMethod::GET | HttpMethod::HEAD) {
        match &request.body {
            HttpBody::Raw(raw) if !raw.is_empty() => {
                parts.push(format!("-H {}", escape_shell(&format!("Content-Type: {}", request.content_type))));
                parts.push(format!("--data-raw {}", escape_shell(raw)));
            }
            HttpBody::FormData(form) => {
                for item in form {
                    if !item.key.is_empty() {
                        match item.item_type {
                            crate::model::FormDataType::Text => {
                                parts.push(format!("-F {}", escape_shell(&format!("{}={}", item.key, item.value))));
                            }
                            crate::model::FormDataType::File => {
                                if !item.value.is_empty() {
                                    parts.push(format!("-F {}", escape_shell(&format!("{}=@{}", item.key, item.value))));
                                }
                            }
                        }
                    }
                }
            }
            HttpBody::UrlEncoded(form) => {
                let mut urlencoded_parts = Vec::new();
                for item in form {
                    if !item.key.is_empty() {
                        let key_enc = url_encode(&item.key);
                        let val_enc = url_encode(&item.value);
                        urlencoded_parts.push(format!("{}={}", key_enc, val_enc));
                    }
                }
                if !urlencoded_parts.is_empty() {
                    let body_str = urlencoded_parts.join("&");
                    parts.push(format!("-H 'Content-Type: application/x-www-form-urlencoded'"));
                    parts.push(format!("-d {}", escape_shell(&body_str)));
                }
            }
            _ => {}
        }
    }

    // URL (should be at the end, escaped)
    parts.push(escape_shell(&request.url));

    parts.join(" ")
}

fn escape_shell(s: &str) -> String {
    let mut escaped = String::new();
    escaped.push('\'');
    for c in s.chars() {
        if c == '\'' {
            escaped.push_str("'\\''");
        } else {
            escaped.push(c);
        }
    }
    escaped.push('\'');
    escaped
}

fn base64_encode(input: &[u8]) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let mut buf = 0u32;
    let mut bits = 0;
    for &b in input {
        buf = (buf << 8) | b as u32;
        bits += 8;
        while bits >= 6 {
            bits -= 6;
            let idx = (buf >> bits) & 0x3F;
            result.push(CHARSET[idx as usize] as char);
        }
    }
    if bits > 0 {
        buf <<= 6 - bits;
        let idx = buf & 0x3F;
        result.push(CHARSET[idx as usize] as char);
    }
    while result.len() % 4 != 0 {
        result.push('=');
    }
    result
}

fn url_encode(s: &str) -> String {
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => {
                encoded.push('+');
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}
