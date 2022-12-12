// Copyright 2022 Camilo Suárez Sandí

use std::collections::HashMap;

use super::http_request::HttpRequest;

/// An http response
pub struct HttpResponse {
    /// The http version
    version: String,
    /// An http code, like 200 or 404
    code: usize,
    /// An http message, like OK
    message: String,
    /// Information for the server
    headers: HashMap<String, String>,
    /// The body
    body: String,
}

impl HttpResponse {
    /// Setter for the code
    pub fn set_code(&mut self, code: usize) {
        self.code = code;
    }

    /// Setter for the message
    pub fn set_message(&mut self, message: &'static str) {
        self.message = message.to_string()
    }

    /// Adds a key and a value as a header
    pub fn add_header(&mut self, key: &'static str, value: &'static str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    /// Concatenates a string to the body
    pub fn add_body(&mut self, string: &String) {
        self.body.push_str(string);
    }

    /// HttpResponse constructor
    /// Returns an http response, it saves a copy of the http version from an http request
    pub fn new(request: &HttpRequest) -> Self {
        Self {
            version: request.version().clone(),
            code: 200,
            message: "OK".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
        }
    }

    /// Returns a string version of the http response to send to the server
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
