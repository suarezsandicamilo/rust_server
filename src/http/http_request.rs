// Copyright 2022 Camilo Suárez Sandí

use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

use crate::http::http_method::HttpMethod;

pub struct HttpRequest {
    method: HttpMethod,
    target: String,
    version: String,
}

impl HttpRequest {
    pub fn from_stream(stream: &TcpStream) -> Result<Self, std::io::Error> {
        let buf_reader = BufReader::new(stream);

        let lines: Vec<String> = buf_reader
            .lines()
            .map(|line| line.unwrap_or_default())
            .take_while(|line| !line.is_empty())
            .collect();

        if lines.is_empty() || lines[0].is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "The http request is empty",
            ));
        }

        return Self::from_string(&lines[0]);
    }

    fn from_string(line: &str) -> Result<Self, std::io::Error> {
        let line_parts: Vec<&str> = line.split(" ").collect();

        if line_parts.len() != 3 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Invalid http request message",
            ));
        }

        let method = HttpMethod::from_string(&line_parts[0])?;

        // TODO: Validation of target
        let target = line_parts[1].to_string();

        // TODO: Validation of version
        let version = line_parts[2].to_string();

        let http_request = Self {
            method,
            target,
            version,
        };

        return Ok(http_request);
    }
}
