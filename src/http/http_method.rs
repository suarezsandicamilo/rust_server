// Copyright 2022 Camilo Suárez Sandí

static HTTP_METHODS: [&str; 9] = [
    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
];

pub struct HttpMethod {
    verb: String,
}

impl HttpMethod {
    pub fn new<'a>(verb: &'a str) -> Result<Self, std::io::Error> {
        for http_method in HTTP_METHODS {
            if verb == http_method {
                return Ok(Self {
                    verb: verb.to_string(),
                });
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid http method",
        ))
    }

    pub fn get_verb(&self) -> &String {
        &self.verb
    }
}
