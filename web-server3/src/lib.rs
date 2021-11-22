use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

enum Message {
    NewTask(Task),
    Terminate,
}

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    sender: mpsc::Sender<Message>,
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
        let message = Message::NewTask(task);
        self.sender.send(message).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
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
