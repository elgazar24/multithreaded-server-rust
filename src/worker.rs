use crate::worker_task::WorkerTask;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
    time::Duration,
};


pub struct Worker {
    pub worker_id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(worker_id: usize, receiver: Arc<Mutex<Receiver<WorkerTask>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let task = {
                    let job = match receiver.lock() {
                        Ok(lock) => lock.recv(), // Receive task within the lock
                        Err(_) => {
                            eprintln!("Worker {}: Mutex was poisoned. Exiting.", worker_id);
                            break; // Exit thread if mutex is poisoned
                        }
                    };
                    match job {
                        Ok(task) => task,  // Return task outside lock
                        Err(_) => {
                            eprintln!("Worker {}: Channel closed, exiting.", worker_id);
                            break; // Exit thread if channel is closed
                        }
                    }
                };
    
                // Process the task
                println!("Worker {} got a job; executing.", worker_id);
                task.run();
    
                // Sleep after processing
                // thread::sleep(Duration::from_millis(100));
            }
        });
    
        Worker {
            worker_id,
            thread: Some(thread),
        }
    }

}
