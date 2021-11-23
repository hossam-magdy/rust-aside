use super::Message;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct Worker {
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewTask(task) => {
                    println!("Running task on thread {}", id);
                    task();
                }
                Message::Terminate => {
                    println!("Terminating thread {}", id);
                    break;
                }
            }
        });
        Worker {
            thread: Some(thread),
        }
    }
}
