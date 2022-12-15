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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct AddTaskParameters {
    text: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct CheckTaskParameters {
    check: usize,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct UncheckTaskParameters {
    uncheck: usize,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct RemoveTaskParameters {
    remove: usize,
}

impl HttpApp for TasksApp {
    fn handle(
        &mut self,
        http_request: &HttpRequest,
        http_response: &mut HttpResponse,
    ) -> Result<bool, Error> {
        let target = http_request.get_target();

        if target == "/" {
            self.read_data()?;

            self.serve_index(http_response)?;

            return Ok(true);
        }

        if target.starts_with("/add?") {
            self.serve_add(target)?;

            http_response.set_code(302);
            http_response.set_message("Found");
            http_response.add_header("Location", "/");

            self.write_data()?;

            return Ok(true);
        }

        if target.starts_with("/update") {
            self.serve_update(target)?;

            http_response.set_code(302);
            http_response.set_message("Found");
            http_response.add_header("Location", "/");

            self.write_data()?;

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

        if let Ok(data) = json::parse(&data) {
            if !data.is_object() {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid json"));
            }

            let values = &data["values"];

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

    pub fn write_data(&self) -> Result<(), Error> {
        let mut values = json::array![];

        for task in &self.tasks {
            values.push(task.to_json()).unwrap();
        }

        let file = path::Path::new("./data/tasks.json");

        let data = json::object! {
            values: values
        };

        fs::write(file, json::stringify_pretty(data, 4))?;

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
                            .insert_bool("done", task.is_done())
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

    fn serve_add(&mut self, target: &String) -> Result<(), Error> {
        let target = target.strip_prefix("/add?").unwrap();

        let parameters: AddTaskParameters = serde_qs::from_str(target).unwrap();

        self.tasks.push(Task::new(&parameters.text, false));

        Ok(())
    }

    fn serve_update(&mut self, target: &String) -> Result<(), Error> {
        let target = target.strip_prefix("/update?").unwrap();

        if target.starts_with("check") {
            let parameters: CheckTaskParameters = serde_qs::from_str(target).unwrap();

            let index = parameters.check - 1;

            self.tasks[index].check();

            return Ok(());
        }

        if target.starts_with("uncheck") {
            let parameters: UncheckTaskParameters = serde_qs::from_str(target).unwrap();

            let index = parameters.uncheck - 1;

            self.tasks[index].uncheck();

            return Ok(());
        }

        if target.starts_with("remove") {
            let parameters: RemoveTaskParameters = serde_qs::from_str(target).unwrap();

            let index = parameters.remove - 1;

            self.tasks.remove(index);

            return Ok(());
        }

        Ok(())
    }
}
