use crate::request_manager::RequestManager;
use crate::worker::Worker;
use crate::worker_task::WorkerTask;

use std::{
    io::ErrorKind,
    net::TcpListener,
    net::TcpStream,
    sync::mpsc,
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
    time::Duration,
};

///
/// ServerManager
/// Desciption :
///     This struct is responsible for managing multiple servers to act as a load balancer
///         - It's responsible for starting and stopping all the servers
///         - It's responsible for balancing the load between the servers
///         - It's responsible for handling the requests
///         - It's responsible for handling the responses
///
pub struct ServerManager {
    is_running: Arc<AtomicBool>,
    listener: TcpListener,
    workers: Vec<Worker>,
    sender: mpsc::Sender<WorkerTask>,
}

impl ServerManager {
    pub fn new(base_threads_count: usize, ip_address: &'static str, port: u16) -> Self {
        // Check if base_threads_count is greater than 0
        assert!(base_threads_count > 0);

        // Create a vector to hold the workers
        let mut workers: Vec<Worker> = Vec::new();

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let sender = sender.clone();

        let is_running = Arc::new(AtomicBool::new(true));

        // Create the workers
        for worker_id in 0..base_threads_count {
            workers.push(Worker::new(worker_id, Arc::clone(&receiver)));
        }

        // Create a TCP listener
        let listener = TcpListener::bind(format!("{}:{}", ip_address, port))
            .expect("Failed to Start Listener");

        // Set the listener to non-blocking
        listener.set_nonblocking(true).unwrap();

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

        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            for incoming in self.listener.incoming() {
                match incoming {
                    Ok(mut stream) => {
                        self.execute(move || {
                            let _ = ServerManager::handle_connection(&mut stream);
                        });
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => println!("Error from listener during receive stream: {}", e),
                }
                if !self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
            }
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = WorkerTask::new(Box::new(f));

        self.sender.send(job).unwrap();
    }

    fn handle_connection(mut stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        // Handle the request
        RequestManager::handle_request(&mut stream)
    }

    pub  fn check_running(&self) -> bool {
        self.is_running.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn stop(&self) {
        println!("Shutting down server");
        self.is_running
            .store(false, std::sync::atomic::Ordering::SeqCst);
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
