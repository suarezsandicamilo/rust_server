// Copyright 2022 Camilo Suárez Sandí

use std::fs;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path;

use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

pub struct HttpServer {
    address: String,
    host: String,
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(address: &'static str, host: &'static str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", address, host))?;

        let server = Self {
            address: address.to_string(),
            host: host.to_string(),
            listener,
        };

        return Ok(server);
    }

    pub fn start(&self) -> Result<(), std::io::Error> {
        println!("Server running at {}:{}", self.address, self.host);

        for stream in self.listener.incoming() {
            if let Err(e) = stream {
                return Err(e);
            }

            if let Ok(mut stream) = stream {
                self.handle_connection(&mut stream)?;
            }
        }

        Ok(())
    }

    pub fn handle_connection(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let http_request = HttpRequest::from_stream(&stream)?;

        match http_request.target().as_str() {
            "/" => {
                let file = path::Path::new("./public/index.html");

                let data = fs::read_to_string(file)?;

                let mut http_response = HttpResponse::new(&http_request);

                http_response.add_body(&data);

                stream.write_all(http_response.to_string().as_bytes())?;
            }
            _ => {
                let mut http_response = HttpResponse::new(&http_request);

                http_response.set_code(404);
                http_response.add_body(&"Not Found!".to_string());

                stream.write_all(http_response.to_string().as_bytes())?;
            }
        }

        Ok(())
    }
}
