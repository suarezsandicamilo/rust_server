// Copyright 2023 Camilo Suárez Sandí

use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type ThreadWork = Option<Box<dyn FnOnce() + Send + 'static>>;

pub struct ThreadPool {
    workers: Vec<ThreadWorker>,
    sender: Sender<ThreadWork>,
}

impl ThreadPool {
    pub fn new(workers_count: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(workers_count);

        for _ in 0..workers_count {
            workers.push(ThreadWorker::new(Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let work = Box::new(f);

        self.sender.send(Some(work)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.sender.send(None).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct ThreadWorker {
    thread: Option<JoinHandle<()>>,
}

impl ThreadWorker {
    fn new(receiver: Arc<Mutex<Receiver<ThreadWork>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let work = receiver.lock().unwrap().recv().unwrap();

            match work {
                Some(message) => {
                    message();
                }
                None => {
                    break;
                }
            }
        });

        Self {
            thread: Some(thread),
        }
    }
}
