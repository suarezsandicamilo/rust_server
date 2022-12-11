// Copyright 2022 Camilo Suárez Sandí

use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

use crate::http::http_method::HttpMethod;

pub struct HttpRequest {
    method: HttpMethod,
    target: String,
    version: String,
    headers: HashMap<String, String>,
}

impl HttpRequest {
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

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
