use super::worker::Worker;
use std::convert::TryInto;
use std::net::TcpListener;

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
    max_threads_count: u32,
    listener: TcpListener,
    workers: Vec<Worker>,
}

impl ServerManager {
    pub fn new(
        base_threads_count: u32,
        max_threads_count: u32,
        ip_address: &'static str,
        port: u16,
    ) -> Self {
        assert!(base_threads_count > 0);

        assert!(base_threads_count <= max_threads_count);

        let listener = TcpListener::bind(format!("{}:{}", ip_address, port))
            .expect("Failed to Start Listener");

        let mut workers: Vec<Worker> = Vec::new();

        for worker_id in 0..base_threads_count {
            workers.push(Worker::new(worker_id));
        }

        println!(
            "Server started on {}:{} with {} workers",
            ip_address, port, base_threads_count
        );

        ServerManager {
            max_threads_count,
            listener,
            workers,
        }
    }

    ///
    ///
    /// start_server
    /// Description :
    ///     This function is responsible for starting the server
    ///     - start listening for requests
    ///     - assign tasks to an available worker
    ///
    pub fn start_server(&mut self) {
        for incoming in self.listener.incoming() {
            match incoming {
                Ok(mut stream) => {
                    let mut is_handled = false;


                    for worker in &mut self.workers {
                        if worker.is_available() {
                            worker.handle_connection(&mut stream); 
                            is_handled = true;
                            break;
                        }
                    }


                    if !is_handled && (self.workers.len() as u32) < self.max_threads_count {
                        let worker_count: u32 = self.workers.len().try_into().unwrap();
                        println!("No available worker found");
                        println!("Initializing new worker");

                        // Create a new worker
                        let mut new_worker = Worker::new(worker_count + 1);

                        // Handle the request with the new worker
                        new_worker.handle_connection(&mut stream); // Pass mutable reference

                        // Add the new worker to the worker pool
                        self.workers.push(new_worker);
                    } else if !is_handled {
                        println!("All workers are busy");
                        println!("Unable to handle request");
                    }
                }
                Err(e) => println!("Error from listener during receive stream: {}", e),
            }
        }
    }
}
