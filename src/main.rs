// Copyright 2022 Camilo Suárez Sandí

pub mod http;

use crate::http::http_server::HttpServer;

fn main() {
    let server = HttpServer::new("127.0.0.1", "8080");

    if let Err(e) = server {
        eprintln!("{e}");
        return;
    }

    if let Ok(server) = server {
        if let Err(e) = server.start() {
            eprintln!("{e}");
            return;
        }
    }
}
