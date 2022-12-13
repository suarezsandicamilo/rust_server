// Copyright 2022 Camilo Suárez Sandí

use std::io::Error;

use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

pub trait HttpApp {
    fn handle(
        &self,
        http_request: &HttpRequest,
        http_response: &mut HttpResponse,
    ) -> Result<bool, Error>;
}
