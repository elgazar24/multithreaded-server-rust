use crate::request_manager::RequestManager;
use crate::worker::Worker;
use crate::worker_task::WorkerTask;

use std::{
    io::ErrorKind,
    net::{TcpListener, TcpStream},
    sync::{atomic::AtomicBool, mpsc, Arc, Mutex},
    thread,
    time::Duration,
};
///
/// ServerManager
/// Desciption :
///     This struct is responsible for managing multiple servers to act as a load balancer
///         - It's responsible for :
///                                - starting and stopping all the workers
///                                - balancing the load between the workers
///
pub struct ServerManager {
    is_running: Arc<AtomicBool>,
    listener: TcpListener,
    workers: Vec<Worker>,
    sender: mpsc::Sender<WorkerTask>,
}

impl ServerManager {
    pub fn new(
        base_threads_count: usize,
        ip_address: &'static str,
        port: u16,
        is_running: Arc<AtomicBool>,
    ) -> Self {
        // Check if base_threads_count is greater than 0
        assert!(base_threads_count > 0);

        // Create a vector to hold the workers
        let mut workers: Vec<Worker> = Vec::new();

        // Create a channel to send tasks to the workers
        let (sender, receiver): (mpsc::Sender<WorkerTask>, mpsc::Receiver<WorkerTask>) =
            mpsc::channel();

        // Create an receiver for the workers
        let receiver: Arc<Mutex<mpsc::Receiver<WorkerTask>>> = Arc::new(Mutex::new(receiver));

        // Create a sender to send tasks to the workers
        let sender: mpsc::Sender<WorkerTask> = sender.clone();

        // Create the workers
        for worker_id in 0..base_threads_count {
            workers.push(Worker::new(worker_id, Arc::clone(&receiver)));
        }

        // Create a TCP listener
        let listener = TcpListener::bind(format!("{}:{}", ip_address, port))
            .expect("Failed to Start Listener");

        // Set the listener to non-blocking
        if listener.set_nonblocking(true).is_err() {
            print!("Failed to set listener to non-blocking");
        }

        println!(
            "Server started on {}:{} with {} workers",
            ip_address, port, base_threads_count
        );

        ServerManager {
            is_running,
            listener,
            workers,
            sender,
        }
    }

    /// start_server
    /// Description :
    ///     This function is responsible for starting the server
    ///     - start listening for requests
    ///     - assign tasks to an available worker
    ///
    pub fn start_server(&mut self) {
        // Ensure the server is running
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            // Listen for incoming connections
            for incoming in self.listener.incoming() {
                match incoming {
                    // Accept the incoming connection if it's not a WouldBlock error or any error
                    Ok(mut stream) => {
                        // Assign the task to an available worker

                        self.send_task(move || {
                            let _ = ServerManager::handle_connection(&mut stream);
                        });
                    }
                    // WouldBlock error
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        // Do nothing to decrease CPU usage
                        thread::sleep(Duration::from_millis(1));
                    }
                    // Any other error
                    Err(e) => println!("Error from listener during receive stream: {}", e),
                }
                // Check if the server is running
                if !self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
            }
        }
        print!("Server stopped.");
    }

    /// Send a task to a worker
    fn send_task<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: WorkerTask = WorkerTask::new(Box::new(f));

        self.sender.send(job).unwrap();
    }

    /// Handle a request the request function  
    /// we could use the RequestManager class to handle the request directly
    /// but this implementation is more efficient
    /// because it gives us more control about what we run before and after the request
    fn handle_connection(mut stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        RequestManager::handle_request(&mut stream)
    }

    /// Check if the server is running
    pub fn check_running(&self) -> bool {
        self.is_running.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Stop the server
    pub fn stop(&self) {
        if self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            self.is_running
                .store(false, std::sync::atomic::Ordering::SeqCst);
            println!("Shutdown signal sent.");
        } else {
            println!("Server was already stopped or not running.");
        }
    }
}

impl Drop for ServerManager {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.worker_id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
