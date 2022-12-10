// Copyright 2022 Camilo Suárez Sandí

use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::http::http_request::HttpRequest;

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
        let _http_request = HttpRequest::from_stream(&stream)?;

        stream.write_all("HTTP/1.1 200 OK".as_bytes())?;

        Ok(())
    }
}
