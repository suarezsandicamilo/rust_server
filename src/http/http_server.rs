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
    fn handle_connection(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let http_request = HttpRequest::from_stream(&stream)?;

        let target = http_request.target().as_str();

        match target {
            "/" => {
                self.serve_static(stream, &http_request, "./pages/index.html")?;
            }
            _ => {
                if self.serve_public(stream, &http_request)? {
                    return Ok(());
                }

                self.serve_not_found(stream, &http_request)?;
            }
        }

        Ok(())
    }

    fn serve_static(
        &self,
        stream: &mut TcpStream,
        http_request: &HttpRequest,
        path: &'static str,
    ) -> Result<(), std::io::Error> {
        let file = path::Path::new(path);

        let data = fs::read_to_string(file)?;

        let mut http_response = HttpResponse::new(&http_request);

        http_response.add_body(&data);

        stream.write_all(http_response.to_string().as_bytes())?;

        Ok(())
    }

    fn serve_public(
        &self,
        stream: &mut TcpStream,
        http_request: &HttpRequest,
    ) -> Result<bool, std::io::Error> {
        let mut path = "./public".to_string();

        path.push_str(http_request.target());

        let file = path::Path::new(&path);

        if let Ok(data) = fs::read_to_string(file) {
            let mut http_response = HttpResponse::new(&http_request);

            http_response.add_body(&data);

            stream.write_all(http_response.to_string().as_bytes())?;

            return Ok(true);
        }

        Ok(false)
    }

    fn serve_not_found(
        &self,
        stream: &mut TcpStream,
        http_request: &HttpRequest,
    ) -> Result<(), std::io::Error> {
        self.serve_static(stream, http_request, "./pages/not_found.html")?;

        Ok(())
    }
}
