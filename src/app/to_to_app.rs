// Copyright 2022 Camilo Suárez Sandí

use std::fs;
use std::io::Error;
use std::path;

use crate::http::http_app::HttpApp;
use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

pub struct ToDoApp {}

impl HttpApp for ToDoApp {
    fn handle(
        &self,
        http_request: &HttpRequest,
        http_response: &mut HttpResponse,
    ) -> Result<bool, Error> {
        let target = http_request.get_target();

        if target == "/" {
            self.serve_index(http_response)?;

            return Ok(true);
        }

        Ok(false)
    }
}

impl ToDoApp {
    fn serve_index(&self, http_response: &mut HttpResponse) -> Result<(), Error> {
        let file = path::Path::new("./pages/index.html");

        let data = fs::read_to_string(file)?;

        if let Ok(data) = mustache::compile_str(&data) {
            let map = mustache::MapBuilder::new()
                .insert_vec("tasks", |a| {
                    a
                    .push_map(|b| {
                        b
                        .insert_str("index", "1")
                        .insert_str("text", "I have to do everything!")
                    })
                    .push_map(|b| {
                        b
                        .insert_str("index", "2")
                        .insert_str("text", "I have to do everything!")
                    })
                    .push_map(|b| {
                        b
                        .insert_str("index", "3")
                        .insert_str("text", "I have to do everything!")
                    })
                })
                .build();

            if let Ok(data) = data.render_data_to_string(&map) {
                http_response.add_body(&data);
            }
        }

        Ok(())
    }
}
