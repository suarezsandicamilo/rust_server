// Copyright 2022 Camilo Suárez Sandí

use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

use crate::http::http_method::HttpMethod;

/// An http request
pub struct HttpRequest {
    /// An http method, like GET or POST
    method: HttpMethod,
    /// A url
    target: String,
    /// The http version
    version: String,
    /// Information for the server
    headers: HashMap<String, String>,
}

impl HttpRequest {
    /// Getter for the method
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    /// Getter for the target
    pub fn target(&self) -> &String {
        &self.target
    }

    /// Getter for the version
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Getter for the headers
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// HttpRequest constructor
    /// Returns an http request from a TcpStream
    pub fn from_stream(stream: &TcpStream) -> Result<Self, std::io::Error> {
        let buf_reader = BufReader::new(stream);

        let lines: Vec<String> = buf_reader
            .lines()
            .map(|line| line.unwrap_or_default())
            .take_while(|line| !line.is_empty())
            .collect();

        if lines.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "The http request is empty",
            ));
        }

        let mut http_request = Self::from_first_line(&lines[0])?;

        http_request.add_headers_from_lines(&lines[1..])?;

        Ok(http_request)
    }

    /// Reads the method, target and version
    fn from_first_line(line: &String) -> Result<Self, std::io::Error> {
        let split: Vec<&str> = line.split(' ').collect();

        if split.len() != 3 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid http request message",
            ));
        }

        let method = HttpMethod::new(&split[0])?;

        // TODO: Validation of target
        let target = split[1].to_string();

        // TODO: Validation of version
        let version = split[2].to_string();

        let http_request = Self {
            method,
            target,
            version,
            headers: HashMap::new(),
        };

        return Ok(http_request);
    }

    /// Reads and adds the headers
    fn add_headers_from_lines(&mut self, lines: &[String]) -> Result<(), std::io::Error> {
        for line in lines {
            if !line.contains(':') {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid http header",
                ));
            }

            let (key, value) = line.split_once(':').unwrap_or_default();

            self.headers.insert(key.to_string(), value.to_string());
        }

        Ok(())
    }
}
