use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, Thread}; //channels for passing messages amongs threads

//Job is a triat object. So we use trait object here so all kinds of jobs can be passed to execute method.
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create  a new ThreadPool
    ///
    /// This size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// if the size is less than 1
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel();

        //Mutex is for mutability and Arc is a smartpointer for multiple reference that is thread safe.
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(reciever));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            //create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        //Loop makes the thread work in a continous circle.
        let thread = thread::spawn(move || {
            loop {
                //now we need to get the job here from the receiver

                let job: Box<dyn FnOnce() + Send> = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job();
            }
        });

        Worker { id, thread }
    }
}
