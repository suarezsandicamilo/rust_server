// Copyright 2022 Camilo Suárez Sandí

use std::fs;
use std::io::Error;
use std::path;

use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

pub trait HttpApp {
    fn handle(
        &mut self,
        http_request: &HttpRequest,
        http_response: &mut HttpResponse,
    ) -> Result<bool, Error>;

    fn serve_static(&self, http_response: &mut HttpResponse, path: &str) -> Result<(), Error> {
        let file = path::Path::new(path);

        let data = fs::read_to_string(file)?;

        http_response.add_body(&data);

        Ok(())
    }

    fn redirect(&self, http_response: &mut HttpResponse, target: &str) {
        http_response.set_code(302);
        http_response.set_message("Found");
        http_response.add_header("Location", target);
    }
}
