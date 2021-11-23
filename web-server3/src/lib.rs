mod thread_pool;
mod worker;

mod message {
    use super::Task;
    pub enum Message {
        NewTask(Task),
        Terminate,
    }
}

mod task {
    pub type Task = Box<dyn FnOnce() + Send + 'static>;
}

use message::Message;
use task::Task;
use worker::Worker;

pub use thread_pool::ThreadPool;
