use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewTask(Task),
    Terminate,
}

pub struct ThreadPool {
    sender: mpsc::Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, receiver.clone()));
        }

        ThreadPool { sender, workers }
    }

    pub fn execute<F>(&self, f: F) -> ()
    where
        F: FnOnce() + Send + 'static, // TODO revise this!
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
            println!("Shutting down worker {}", worker.id);
            worker.thread.take().unwrap().join().unwrap();
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
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewTask(task) => {
                    println!("Running taks on thread {}", id);
                    task();
                }
                Message::Terminate => {
                    println!("Terminating thread {}", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
