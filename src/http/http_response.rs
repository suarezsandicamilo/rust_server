// Copyright 2022 Camilo Suárez Sandí

use std::collections::HashMap;

use super::http_request::HttpRequest;

pub struct HttpResponse {
    version: String,
    code: u32,
    message: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpResponse {
    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }

    pub fn set_message(&mut self, message: &'static str) {
        self.message = message.to_string()
    }

    pub fn add_header(&mut self, key: &'static str, value: &'static str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn add_body(&mut self, string: &String) {
        self.body.push_str(string);
    }

    pub fn new(request: &HttpRequest) -> Self {
        Self {
            version: request.version().clone(),
            code: 200,
            message: "OK".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = "".to_string();

        string.push_str(&self.version);
        string.push(' ');
        string.push_str(&self.code.to_string());
        string.push(' ');
        string.push_str(&self.message);
        string.push('\n');

        for (key, value) in &self.headers {
            string.push_str(key);
            string.push_str(": ");
            string.push_str(value);
        }

        if !self.body.is_empty() {
            string.push_str("\n\n");
            string.push_str(&self.body);
        }

        string
    }
}
