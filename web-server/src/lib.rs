use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender,
}

type Job = Box<dyn FnOnce() -> () + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

type SharedReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;
type Sender = mpsc::Sender<Message>;

impl ThreadPool {
    // u8 range is 0-255
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
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
        F: FnOnce() -> () + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(Message::NewJob(job))
            .expect("Error sending Message::NewJob through channel ...");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("Error sending Message::Terminate through channel/mpsc::Sender");
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect("Error joining thread ...");
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: SharedReceiver) -> Worker {
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
