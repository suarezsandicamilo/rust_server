// Copyright 2022 Camilo Suárez Sandí

use std::fs;
use std::io::Error;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path;

use crate::http::http_app::HttpApp;
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
    /// All the apps connected to the server
    apps: Vec<Box<dyn HttpApp>>,
}

impl HttpServer {
    /// HttpServer constructor
    /// Returns a new http server from an address and a port, like 127.0.0.1:8080
    pub fn new(address: &str, port: &str) -> Result<Self, Error> {
        let listener = TcpListener::bind(format!("{}:{}", address, port))?;

        let server = Self {
            address: address.to_string(),
            port: port.to_string(),
            listener,
            apps: vec![],
        };

        return Ok(server);
    }

    pub fn add_app(&mut self, app: Box<dyn HttpApp>) {
        self.apps.push(app);
    }

    /// Starts listening to client requests and sends the server responses
    pub fn start(&self) -> Result<(), Error> {
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
    fn handle_connection(&self, stream: &mut TcpStream) -> Result<(), Error> {
        let http_request = HttpRequest::from_stream(&stream)?;

        let mut http_response = HttpResponse::new();

        http_response.set_version(http_request.get_version());

        for app in &self.apps {
            if app.handle(&http_request, &mut http_response)? {
                stream.write_all(http_response.to_string().as_bytes())?;
                return Ok(());
            }
        }

        if self.serve_public(&http_request, &mut http_response)? {
            stream.write_all(http_response.to_string().as_bytes())?;
            return Ok(());
        }

        self.serve_not_found(&mut http_response)?;
        stream.write_all(http_response.to_string().as_bytes())?;

        Ok(())
    }

    fn serve_public(
        &self,
        http_request: &HttpRequest,
        http_response: &mut HttpResponse,
    ) -> Result<bool, Error> {
        let mut path = "./public".to_string();

        path.push_str(http_request.get_target());

        let file = path::Path::new(&path);

        if let Ok(data) = fs::read_to_string(file) {
            http_response.add_body(&data);

            return Ok(true);
        }

        Ok(false)
    }

    fn serve_not_found(&self, http_response: &mut HttpResponse) -> Result<(), Error> {
        let file = path::Path::new("./pages/not_found.html");

        let data = fs::read_to_string(file)?;

        http_response.set_code(404);
        http_response.set_message("Not Found");
        http_response.add_body(&data);

        Ok(())
    }
}
