//! # use server::thread::{ [ThreadPool], [Worker] }
//! 
//! This library handles operations to build a multi-threaded system for the web-server.

use std::{sync::{mpsc, Arc, Mutex}, thread};

/// The ThreadPool data structure holds the underlying mechanism for processing concurrent requests.
/// It has a sender which propagates requests through the channel and a vector of workers.
pub struct ThreadPool {
    /// This is an array of workers which constantly listen for requests to the server.
    /// The number of workers defaults to 10 but can be provided with the `threads` option.
    pub workers: Vec<Worker>,

    /// This is a sender type used to propagate requests through a channel that every worker listens on.
    pub sender: mpsc::Sender<Job>
}


impl ThreadPool {
    /// Returns a struct with the type ThreaPool. This method is used to initiate a new threadpool when the server is started.
    /// The workers/threads share ownership of a reciever which is used to accept requests.
    /// 
    /// Args
    /// 
    /// * `pool_size` - This represents the number of workers/threads the server is started with.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use server::thread::ThreadPool;
    /// 
    /// let pool: ThreadPool = ThreadPool::new(5);
    /// assert_eq!(pool.workers.len(), 5);
    /// ```
    /// 
    /// # Panics
    /// 
    /// The associated function panics when the pool size provided is less that 1 or greater than 100.
    /// 
    pub fn new(pool_size: usize) -> ThreadPool {

        assert!(pool_size > 0 && pool_size <= 100, "Pool size must be greater than 0 and less than or equal to 100");
        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(pool_size);

        for id in 0..pool_size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }

    /// This function executes a closure `f` which essentially sends a stream of request down the channel.
    /// A worker in the pool recieves the request and executes it.
    /// 
    /// Args
    /// * `f` - A closure `handle_connection`.
    /// 
    /// The closure is then sent down the channel as a Job.
    /// 
    pub fn execute<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        match self.sender.send(job) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
    }

    // Function signature for thread::spawn
    /* pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,  {} */
}

/// This represents a thread that executes a single request at a time.
pub struct Worker {
    /// This is a unique Identifier for the worker
    pub id: usize,

    /// This is a thread that is used for executing requests.
    pub thread: thread::JoinHandle<()>
}

/// Type alias `Job`.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    /// Function to spawn a thread that continously listens for requests and executes any it gets.
    /// 
    /// Args
    /// 
    /// * `receiver` - This is a Reciever type that recieves requests from the thread-pool channel.
    /// 
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || 
            loop {
                let job = reciever.lock().unwrap().recv().unwrap();
                
                println!("Executing job with worker Id: {}", id);
                job();
            }
        );

        Worker { id, thread }
    }
}
