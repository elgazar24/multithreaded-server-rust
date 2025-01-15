use crate::worker_task::WorkerTask;
use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

///
/// Worker
/// Description :
///     This struct is responsible for executing the tasks
///
pub struct Worker {
    pub worker_id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(worker_id: usize, receiver: Arc<Mutex<Receiver<WorkerTask>>>) -> Self {
        // Spawn thread to handle tasks
        let thread = thread::spawn(move || {
            loop {
                let task = {
                    let job = match receiver.lock() {
                        // Receive task within the lock
                        Ok(lock) => lock.recv(),
                        // Exit thread if mutex is poisoned
                        Err(_) => {
                            eprintln!("Worker {}: Mutex was poisoned. Exiting.", worker_id);
                            break;
                        }
                    };
                    match job {
                        // Return task outside lock
                        Ok(task) => task,
                        // Exit thread if channel is closed
                        Err(_) => {
                            eprintln!("Worker {}: Channel closed, exiting.", worker_id);
                            break;
                        }
                    }
                };

                // Process the task
                println!("Worker {} got a task; executing.", worker_id);
                task.run();
            }
        });

        // Return Worker
        Worker {
            worker_id,
            thread: Some(thread),
        }
    }
}
