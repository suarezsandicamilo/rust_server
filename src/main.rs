// Copyright 2022 Camilo Suárez Sandí

pub mod app;
pub mod http;

use crate::http::http_server::HttpServer;

fn main() -> Result<(), std::io::Error> {
    let server = HttpServer::new("127.0.0.1", "8080")?;

    server.start()?;

    Ok(())
}
