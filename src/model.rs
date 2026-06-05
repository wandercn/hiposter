use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::GET
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthType {
    None,
    Bearer,
    Basic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthData {
    pub auth_type: AuthType,
    pub token: String,
    pub username: String,
    pub password: String,
}

impl Default for AuthData {
    fn default() -> Self {
        Self {
            auth_type: AuthType::None,
            token: String::new(),
            username: String::new(),
            password: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<Header>,
    pub params: Vec<Header>,
    pub body: String,
    pub content_type: String,
    pub auth: AuthData,
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::GET,
            url: "https://httpbin.org/get".to_string(),
            headers: Vec::new(),
            params: Vec::new(),
            body: String::new(),
            content_type: "application/json".to_string(),
            auth: AuthData::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<Header>,
    pub body: String,
    pub size: usize,
    pub elapsed_ms: u128,
}
