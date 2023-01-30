// Copyright 2023 Camilo Suárez Sandí

use std::fs;
use std::io::Error;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use crate::http::http_app::HttpApp;
use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;
use crate::thread::thread_pool::ThreadPool;

/// An http server
pub struct HttpServer {
    /// The address of the server
    address: String,
    /// The port of the server
    port: String,
    /// A TcpListener
    listener: TcpListener,
    /// All the apps connected to the server
    apps: Mutex<Vec<Box<dyn HttpApp + Send + Sync>>>,
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
            apps: Mutex::new(Vec::new()),
        };

        return Ok(server);
    }

    pub fn add_app(&mut self, app: Box<dyn HttpApp + Send + Sync>) {
        self.apps.lock().unwrap().push(app);
    }

    /// Starts listening to client requests and sends the server responses
    pub fn start(&mut self) -> Result<(), Error> {
        println!("Server running at {}:{}", self.address, self.port);

        let workers_count = std::thread::available_parallelism()?.get();

        let thread_pool = ThreadPool::new(workers_count);

        let mut apps = Mutex::new(vec![]);

        std::mem::swap(&mut apps, &mut self.apps);

        let apps = Arc::new(apps);

        for stream in self.listener.incoming() {
            let apps = Arc::clone(&apps);

            let mut stream = stream?;

            thread_pool.execute(move || {
                let apps = apps.lock().unwrap();

                HttpServer::handle_connection(&mut stream, apps).unwrap();
            })
        }

        Ok(())
    }

    /// Handles a single request and sends a single response
    fn handle_connection(
        stream: &mut TcpStream,
        mut apps: MutexGuard<Vec<Box<dyn HttpApp + Send + Sync>>>,
    ) -> Result<(), Error> {
        let http_request = HttpRequest::from_stream(&stream)?;

        let mut http_response = HttpResponse::new();

        http_response.set_version(http_request.get_version());

        for app in apps.iter_mut() {
            if app.handle(&http_request, &mut http_response)? {
                stream.write_all(http_response.to_string().as_bytes())?;
                return Ok(());
            }
        }

        if HttpServer::serve_public(&http_request, &mut http_response)? {
            stream.write_all(http_response.to_string().as_bytes())?;
            return Ok(());
        }

        HttpServer::serve_not_found(&mut http_response)?;
        stream.write_all(http_response.to_string().as_bytes())?;

        Ok(())
    }

    fn serve_public(
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

    fn serve_not_found(http_response: &mut HttpResponse) -> Result<(), Error> {
        let file = path::Path::new("./pages/not_found.html");

        let data = fs::read_to_string(file)?;

        http_response.set_code(404);
        http_response.set_message("Not Found");
        http_response.add_body(&data);

        Ok(())
    }
}
