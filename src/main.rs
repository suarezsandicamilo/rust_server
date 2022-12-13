// Copyright 2022 Camilo Suárez Sandí

pub mod app;
pub mod http;

use std::io::Error;
use std::io::ErrorKind;

use crate::app::to_to_app::ToDoApp;
use crate::http::http_server::HttpServer;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        return Err(Error::new(ErrorKind::InvalidInput, "Not enough arguments"));
    }

    let address = &args[1];

    let host = &args[2];

    let mut server = HttpServer::new(address, host)?;

    let todo_app = Box::new(ToDoApp {});

    server.add_app(todo_app);

    server.start()?;

    Ok(())
}
