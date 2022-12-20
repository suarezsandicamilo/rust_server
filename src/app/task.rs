// Copyright 2022 Camilo Suárez Sandí

use std::io::Error;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct Task {
    text: String,
    done: bool,
}

impl Task {
    pub fn new(text: &str, done: bool) -> Self {
        Self {
            text: text.to_string(),
            done,
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn check(&mut self) {
        self.done = true;
    }

    pub fn uncheck(&mut self) {
        self.done = false;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn from_string(string: &str) -> Result<Self, Error> {
        if let Some((text, done)) = string.split_once('\t') {
            if done != "false" && done != "true" {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid task property: done",
                ));
            }

            let done = match done {
                "false" => false,
                "true" => true,
                _ => false,
            };

            let task = Task {
                text: text.to_string(),
                done,
            };

            return Ok(task);
        }

        Err(Error::new(ErrorKind::InvalidData, "Invalid task"))
    }

    pub fn to_string(&self) -> String {
        format!("{}\t{}", self.text, self.done)
    }
}
