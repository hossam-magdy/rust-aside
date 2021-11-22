use super::message::Message;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
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
