use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() -> () + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            let worker = Worker::new(i, receiver.clone());
            workers.push(worker);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            println!("Handling request in thread {}", id);
            let job = receiver
              .lock()
              .expect("Error locking MutexGuard of channel (to exclusively read a single message in only one thread) ...")
              .recv()
              .expect("Error receiving Message from channel ...");

            job();
            // let result_mutex = receiver.lock();

            // match result_mutex {
            //     Ok(mutex) => match mutex.recv() {
            //         Ok(job) => {
            //             job();
            //         }
            //         Err(error) => {
            //             println!("ERROR in mutex.recv: {:?}", error);
            //             panic!()
            //         }
            //     },
            //     Err(error) => {
            //         println!("ERROR in receiver.lock: {:?}", error);
            //         panic!()
            //     }
            // }
        });

        Worker { id, thread }
    }
}
