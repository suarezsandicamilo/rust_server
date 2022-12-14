// Copyright 2022 Camilo Suárez Sandí

use std::io::Error;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct Task {
    text: String,
    done: bool,
}

impl Task {
    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn is_done(&self) -> &bool {
        &self.done
    }

    pub fn from_json(value: &json::JsonValue) -> Result<Self, Error> {
        let text = &value["text"];
        let done = &value["done"];

        if text.is_null() || !text.is_string() || done.is_null() || !done.is_boolean() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid json"));
        }

        let task = Self {
            text: text.as_str().unwrap_or_default().to_string(),
            done: done.as_bool().unwrap_or_default(),
        };

        Ok(task)
    }
}
