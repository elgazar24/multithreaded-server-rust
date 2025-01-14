
use super::worker::Worker;
use crate::worker_task::WorkerTask;
use std::{
    io::{prelude::*, BufReader, ErrorKind},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    fs ,
    sync::mpsc,
    path::Path,
    net::TcpStream
};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;



#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    _version: String,
}

#[derive(Debug)]
struct HttpResponse {
    status_line: String,
    content_type: String,
    content: String,
}
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
    max_threads_count: usize,
    listener: TcpListener,
    workers: Vec<Worker>,
    sender: mpsc::Sender<WorkerTask>,
    receiver: Arc<Mutex<mpsc::Receiver<WorkerTask>>>,
}

impl ServerManager {
    pub fn new(
        base_threads_count: usize,
        max_threads_count: usize,
        ip_address: &'static str,
        port: u16,
    ) -> Self {
        // Check if base_threads_count is greater than 0
        assert!(base_threads_count > 0);

        // Check if base_threads_count is less than or equal to max_threads_count
        assert!(base_threads_count <= max_threads_count);

        // Create a TCP listener
        let listener = TcpListener::bind(format!("{}:{}", ip_address, port))
            .expect("Failed to Start Listener");

        // Set the listener to non-blocking
        listener.set_nonblocking(true).unwrap();

        // Create a vector to hold the workers
        let mut workers: Vec<Worker> = Vec::new();

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let sender = sender.clone();

        // Create the workers
        for worker_id in 0..base_threads_count {
            workers.push(Worker::new(worker_id, Arc::clone(&receiver)));
        }

        println!(
            "Server started on {}:{} with {} workers",
            ip_address, port, base_threads_count
        );

        ServerManager {
            max_threads_count,
            listener,
            workers,
            sender,
            receiver,
        }
    }

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
                    
                    self.execute( move || {
                        ServerManager::handle_connection(&mut stream);
                    });

                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => println!("Error from listener during receive stream: {}", e),
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
        // Parse the HTTP request
        let request = ServerManager::parse_request(&mut stream)?;
                
        // Generate the appropriate response
        let response = ServerManager::generate_response(&request)?;
        
        // Send the response
        ServerManager::send_response(&mut stream, &response)?;
        
        Ok(())
    }
    
    fn parse_request(stream: &mut TcpStream) -> Result<HttpRequest, Box<dyn std::error::Error>> {
        let buf_reader = BufReader::new(stream);
        let request_line = buf_reader
            .lines()
            .next()
            .ok_or("No request line received")??;
    
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid HTTP request format".into());
        }
    
        Ok(HttpRequest {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            _version: parts[2].to_string(),
        })
    }
    
    fn generate_response(request: &HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        match (request.method.as_str(), request.path.as_str()) {
            ("GET", "/") => ServerManager::serve_file("static/index.html", "text/html"),
            ("GET", path) if path.ends_with(".html") => ServerManager::serve_file(&format!("static{}", path), "text/html"),
            ("GET", path) if path.ends_with(".css") => ServerManager::serve_file(&format!("static{}", path), "text/css"),
            ("GET", path) if path.ends_with(".js") => ServerManager::serve_file(&format!("static{}", path), "application/javascript"),
            // ("GET", path) if path.ends_with(".pdf") => ServerManager::serve_pdf(&format!("static{}", path), "application/pdf"),
            ("GET", path) if path.ends_with(".png") => ServerManager::serve_image(&format!("static{}", path), "image/png"),
            ("GET", path) if path.ends_with(".jpg") => ServerManager::serve_image(&format!("static{}", path), "image/jpeg"),
            ("INVALID", _) => ServerManager::serve_400(),
            _ => ServerManager::serve_404()
        }
    }
    fn serve_image(filepath: &str , content_type: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        if !Path::new(filepath).exists() {
            return ServerManager::serve_404();
        }
    
        let content = fs::read(filepath)?;

        let content = STANDARD.encode(content);

    
        Ok(HttpResponse {
            status_line: "HTTP/1.1 200 OK".to_string(),
            content_type: content_type.to_string(),
            content,
        })
    }
    
    fn serve_file(filepath: &str, content_type: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        if !Path::new(filepath).exists() {
            return ServerManager::serve_404();
        }
    
        let content = fs::read_to_string(filepath)?;
        
        Ok(HttpResponse {
            status_line: "HTTP/1.1 200 OK".to_string(),
            content_type: content_type.to_string(),
            content,
        })
    }
    // fn serve_pdf(filepath: &str, content_type: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    //     if !Path::new(filepath).exists() {
    //         return ServerManager::serve_404();
    //     }
    
    //     // Read the file as binary data
    //     let binary_content = fs::read(filepath)?;
    
    //     // Encode the binary data as a Base64 string
    //     let content = STANDARD.encode(binary_content);
    
    //     Ok(HttpResponse {
    //         status_line: "HTTP/1.1 200 OK".to_string(),
    //         content_type: content_type.to_string(),
    //         content, // Base64 encoded string
    //     })
    // }
    
    fn serve_404() -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("static/404.html")
            .unwrap_or_else(|_| "<h1>404 - Page Not Found</h1>".to_string());
    
        Ok(HttpResponse {
            status_line: "HTTP/1.1 404 NOT FOUND".to_string(),
            content_type: "text/html".to_string(),
            content,
        })
    }
    fn  serve_400() -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("static/400.html")
            .unwrap_or_else(|_| "<h1>400 - Bad Request</h1>".to_string());
    
        Ok(HttpResponse {
            status_line: "HTTP/1.1 400 Bad Request".to_string(),
            content_type: "text/html".to_string(),
            content,
        })
        
    }
    
    fn send_response(stream: &mut TcpStream, response: &HttpResponse) -> Result<(), Box<dyn std::error::Error>> {
        let response_string = format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            response.status_line,
            response.content_type,
            response.content.len(),
            response.content
        );
    
        stream.write_all(response_string.as_bytes())?;
        stream.flush()?;
        
        Ok(())
    }

    pub fn ensure_tasks_finished(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
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

