use std::{
    thread, 
    sync::{
        mpsc, 
        Arc, 
        Mutex
    }
};
use rand::Rng;

type Job = Box<dyn FnOnce() + 'static + Send>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size >  0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Send + 'static + FnOnce()
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || { loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        let job_no = rand::thread_rng().gen_range(1..=100000000);
                        println!("Executing job: {job_no} in thread: {id}");
                        job();
                        println!("Completed job: {job_no} in thread: {id}");
                    },
                    Err(_) => {
                        println!("Disconnecting worker {id}");
                        break;
                    }
                }
                
            }
        });
        Worker { id, thread: Some(thread) }
    }
}