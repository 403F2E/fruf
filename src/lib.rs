mod args;
mod config;
mod error;

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub use args::*;
pub use config::*;
pub use error::AppError;
pub use reqwest::blocking::Client;

// #### Thread Pool Logic here #### //

// --- Alias Region --- //

type Job = Box<dyn FnOnce() + Send + 'static>;
type Recv = Arc<Mutex<mpsc::Receiver<Message>>>;

// --- Alias Region Ends --- //

// --- Job State Region --- //

enum Message {
    NewJob(Job),
    Terminate,
}

// --- Job State Region Ends --- //

// --- ThreadPool Region --- //

///
/// # ThreadPool
/// type refers to the number of threads to be spawned executing the fuzzing logic
/// according to the pool number given for the command
///
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: u8) -> Self {
        let size: usize = size as usize;

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver: Recv = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Terminating all workers...");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers...");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// --- ThreadPool Region Ends --- //

// --- Worker Region --- //

///
/// # Worker
/// type unit for the worker that executes one single fuzzing function
///
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Recv) -> Self {
        let thread = thread::spawn(move || loop {
            let message: Message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();

                    println!("Worker {} got sent the request.", id);
                }
                Message::Terminate => break,
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}

// --- Worker Region Ends --- //
