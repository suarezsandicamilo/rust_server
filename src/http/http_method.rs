// Copyright 2022 Camilo Suárez Sandí

/// All the possible http methods
static HTTP_METHODS: [&str; 9] = [
    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
];

/// An http method
pub struct HttpMethod {
    /// The verb of the http method
    verb: String,
}

impl HttpMethod {
    /// HttpMethod constructor
    /// Returns an http method from a verb
    /// It requires that the verb is one of the possible http methods
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

    /// Returns the verb of the http method
    pub fn get_verb(&self) -> &String {
        &self.verb
    }
}
