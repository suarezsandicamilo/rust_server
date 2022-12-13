// Copyright 2022 Camilo Suárez Sandí

pub mod app;
pub mod http;

use crate::app::to_to_app::ToDoApp;
use crate::http::http_server::HttpServer;

fn main() -> Result<(), std::io::Error> {
    let mut server = HttpServer::new("127.0.0.1", "8080")?;

    let todo_app = Box::new(ToDoApp {});

    server.add_app(todo_app);

    server.start()?;

    Ok(())
}
