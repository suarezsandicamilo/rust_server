// Copyright 2022 Camilo Suárez Sandí

use crate::http::http_app::HttpApp;
use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

pub struct CalculatorApp {}

impl HttpApp for CalculatorApp {
    fn handle(
        &self,
        http_request: &HttpRequest,
        http_response: &mut HttpResponse,
    ) -> Result<bool, std::io::Error> {
        Ok(false)
    }
}
