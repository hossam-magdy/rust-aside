use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() -> () + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        println!("Creating ThreadPool of size {}", size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            let worker = Worker::new(i, Arc::clone(&receiver));
            workers.push(worker);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        let message = Message::NewJob(job);
        self.sender.send(message).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
            .lock()
            .expect("Error locking MutexGuard of channel (to exclusively read a single message in only one thread) ...")
            .recv()
            .expect("Error receiving Message from channel ...");

            match message {
                Message::NewJob(job) => {
                    println!("Worker number {} is runinng a Job", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker number {} is being terminated", id);
                    break;
                }
            };
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
