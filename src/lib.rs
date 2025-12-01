use std::{sync::{mpsc, Arc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} executing");
                job();
            }
        });
        Worker { id, thread }
        //Worker { id, thread: thread::spawn(|| { receiver }) }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads: Vec<Worker> = Vec::with_capacity(size);
        for i in 0..size {
            threads.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}