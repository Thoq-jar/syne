use std::collections::HashMap;

pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, status_text: &str, body: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());

        Response {
            status_code,
            status_text: status_text.to_string(),
            headers,
            body: body.to_string(),
        }
    }

    pub fn json(status_code: u16, json_body: &str) -> Self {
        let mut response = Self::new(status_code, "OK", json_body);
        response.set_header("Content-Type", "application/json");
        response
    }

    pub fn html(status_code: u16, html_body: &str) -> Self {
        let mut response = Self::new(status_code, "OK", html_body);
        response.set_header("Content-Type", "text/html; charset=utf-8");
        response
    }

    pub fn text(status_code: u16, text_body: &str) -> Self {
        let mut response = Self::new(status_code, "OK", text_body);
        response.set_header("Content-Type", "text/plain; charset=utf-8");
        response
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
        if name.to_lowercase() != "content-length" {
            self.headers.insert("Content-Length".to_string(), self.body.len().to_string());
        }
    }

    pub fn headers_string(&self) -> String {
        self.headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}