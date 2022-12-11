// Copyright 2022 Camilo Suárez Sandí

use std::fs;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path;

use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

/// An http server
pub struct HttpServer {
    /// The address of the server
    address: String,
    /// The port of the server
    port: String,
    /// A TcpListener
    listener: TcpListener,
}

impl HttpServer {
    /// HttpServer constructor
    /// Returns a new http server from an address and a port, like 127.0.0.1:8080
    pub fn new(address: &'static str, port: &'static str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", address, port))?;

        let server = Self {
            address: address.to_string(),
            port: port.to_string(),
            listener,
        };

        return Ok(server);
    }

    /// Starts listening to client requests and sends the server responses
    pub fn start(&self) -> Result<(), std::io::Error> {
        println!("Server running at {}:{}", self.address, self.port);

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

    /// Handles a single request and sends a single response
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
