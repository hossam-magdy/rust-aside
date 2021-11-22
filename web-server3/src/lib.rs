use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    sender: mpsc::Sender<Task>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let worker = Worker::new(id, receiver.clone());
            workers.push(worker);
        }

        ThreadPool { sender, workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // TODO
    {
        let task = Box::new(f);
        self.sender.send(task).unwrap();
    }
}

type Task = Box<dyn FnOnce() + Send + 'static>;

struct Worker {}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Self {
        thread::spawn(move || loop {
            let task = receiver.lock().unwrap().recv().unwrap();
            println!("Running task on thread {}", id);
            task();
        });
        Worker {}
    }
}
