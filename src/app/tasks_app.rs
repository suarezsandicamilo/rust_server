// Copyright 2022 Camilo Suárez Sandí

use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path;

use crate::app::task::Task;
use crate::http::http_app::HttpApp;
use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

use regex::Regex;

pub struct TasksApp {
    tasks: Vec<Task>,
}

impl HttpApp for TasksApp {
    fn handle(
        &mut self,
        http_request: &HttpRequest,
        http_response: &mut HttpResponse,
    ) -> Result<bool, Error> {
        let target = http_request.get_target();

        if http_request.get_method().get_verb() == "GET" && target == "/" {
            self.read_data()?;
            self.serve_index(http_response)?;

            return Ok(true);
        }

        if http_request.get_method().get_verb() == "GET" && target.starts_with("/add?") {
            self.serve_add(target)?;
            self.redirect(http_response, "/");
            self.write_data()?;

            return Ok(true);
        }

        if http_request.get_method().get_verb() == "GET" && target.starts_with("/update?") {
            self.serve_update(target)?;
            self.redirect(http_response, "/");
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

        let file = path::Path::new("./data/tasks.txt");

        let data = fs::read_to_string(file)?;

        let mut lines = data.lines();

        let mut index = 1;

        while let Some(line) = lines.next() {
            if !line.is_empty() {
                let mut task = Task::from_string(line)?;

                task.set_index(index);

                self.tasks.push(task);
            }

            index += 1;
        }

        Ok(())
    }

    pub fn write_data(&self) -> Result<(), Error> {
        let mut data = "".to_string();

        let mut index = 0;

        for task in &self.tasks {
            data.push_str(&task.to_string());

            if index < self.tasks.len() - 1 {
                data.push('\n');
            }

            index += 1;
        }

        let file = path::Path::new("./data/tasks.txt");

        fs::write(file, data)?;

        Ok(())
    }

    fn serve_index(&mut self, http_response: &mut HttpResponse) -> Result<(), Error> {
        let file = path::Path::new("./pages/index.html");

        let data = fs::read_to_string(file)?;

        let mut context = tera::Context::new();

        context.insert("tasks", &self.tasks);

        let mut tera = tera::Tera::default();

        let data = tera.render_str(&data, &context);

        if let Ok(data) = data {
            http_response.add_body(&data);
        }

        Ok(())
    }

    fn serve_add(&mut self, target: &String) -> Result<(), Error> {
        let target = target.strip_prefix("/add?").unwrap();

        let re = Regex::new(r"text=([\p{L}\p{M}\p{Z}\p{S}\p{N}\p{P}]+)").unwrap();

        if !re.is_match(target) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid url to add a task",
            ));
        }

        let text = re.captures(target).unwrap().get(1).unwrap().as_str();

        self.tasks.push(Task::new(text, false));

        Ok(())
    }

    fn serve_update(&mut self, target: &String) -> Result<(), Error> {
        let target = target.strip_prefix("/update?").unwrap();

        if target.starts_with("check") {
            let re = Regex::new(r"check=([1-9][0-9]*)").unwrap();

            if !re.is_match(target) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid url to check a task",
                ));
            }

            let text = re.captures(target).unwrap().get(1).unwrap().as_str();

            let index: usize = text.parse::<usize>().unwrap() - 1;

            if index >= self.tasks.len() {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid task index"));
            }

            self.tasks[index].check();

            return Ok(());
        }

        if target.starts_with("uncheck") {
            let re = Regex::new(r"uncheck=([1-9][0-9]*)").unwrap();

            if !re.is_match(target) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid url to uncheck a task",
                ));
            }

            let text = re.captures(target).unwrap().get(1).unwrap().as_str();

            let index: usize = text.parse::<usize>().unwrap() - 1;

            if index >= self.tasks.len() {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid task index"));
            }

            self.tasks[index].uncheck();

            return Ok(());
        }

        if target.starts_with("remove") {
            let re = Regex::new(r"remove=([1-9][0-9]*)").unwrap();

            if !re.is_match(target) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid url to uncheck a task",
                ));
            }

            let text = re.captures(target).unwrap().get(1).unwrap().as_str();

            let index: usize = text.parse::<usize>().unwrap() - 1;

            if index >= self.tasks.len() {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid task index"));
            }

            self.tasks.remove(index);

            return Ok(());
        }

        Ok(())
    }
}
