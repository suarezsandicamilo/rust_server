// Copyright 2022 Camilo Suárez Sandí

use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path;

use crate::app::task::Task;
use crate::http::http_app::HttpApp;
use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

pub struct TasksApp {
    tasks: Vec<Task>,
}

impl HttpApp for TasksApp {
    fn handle(
        &self,
        http_request: &HttpRequest,
        http_response: &mut HttpResponse,
    ) -> Result<bool, Error> {
        let target = http_request.get_target();

        if target == "/" {
            // self.read_data();

            self.serve_index(http_response)?;

            return Ok(true);
        }

        Ok(false)
    }
}

impl TasksApp {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn read_data(&mut self) -> Result<(), Error> {
        self.tasks.clear();

        let file = path::Path::new("./data/tasks.json");

        let data = fs::read_to_string(file)?;

        if let Ok(json) = json::parse(&data) {
            if !json.is_object() {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid json"));
            }

            let values = &json["values"];

            if values.is_null() || !values.is_array() {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid json"));
            }

            for value in values.members() {
                let task = Task::from_json(value)?;

                self.tasks.push(task);
            }
        }

        Ok(())
    }

    fn serve_index(&self, http_response: &mut HttpResponse) -> Result<(), Error> {
        let file = path::Path::new("./pages/index.html");

        let data = fs::read_to_string(file)?;

        if let Ok(data) = mustache::compile_str(&data) {
            let map = mustache::MapBuilder::new().insert_vec("tasks", |mut vec| {
                let mut index = 1;

                for task in &self.tasks {
                    vec = vec.push_map(|map| {
                        map.insert_str("index", index.to_string())
                            .insert_str("text", task.get_text())
                    });

                    index += 1;
                }

                vec
            });

            let map = map.build();

            if let Ok(data) = data.render_data_to_string(&map) {
                http_response.add_body(&data);
            }
        }

        Ok(())
    }
}
