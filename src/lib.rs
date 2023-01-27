use std::{
    thread, 
    sync::{
        mpsc, 
        Arc, 
        Mutex
    }
};
use rand::Rng;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + 'static + Send>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size >  0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Send + 'static + FnOnce()
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || { loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                let job_no = rand::thread_rng().gen_range(1..=100000000);
                println!("Executing job: {job_no} in thread: {id}");
                job();
                println!("Completed job: {job_no} in thread: {id}");
            }
        });
        Worker { id, thread }
    }
}