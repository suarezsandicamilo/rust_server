// Copyright 2022 Camilo Suárez Sandí

use std::io::Error;

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
            self.serve_static(http_response, "./pages/index.html")?;

            return Ok(true);
        }

        Ok(false)
    }
}
