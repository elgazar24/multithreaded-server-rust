use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub struct Worker {
    worker_id : u32,
    is_available : bool
}


impl Worker {
    
    pub fn  new( worker_id : u32) -> Self {
        
        Worker {
            worker_id,
            is_available : true
        }
    }
    
    pub fn handle_connection(&mut self, mut stream: &mut TcpStream) {
        // Mark the worker as unavailable
        self.is_available = false;
    
        // Read the incoming request
        let buf_reader = BufReader::new(& mut stream);
        let request_line = match buf_reader.lines().next() {
            Some(Ok(line)) => line,      // Successfully read a line
            Some(Err(err)) => {
                eprintln!("Failed to read request line: {}", err);
                return; // Exit on error
            }
            None => {
                eprintln!("No request line received.");
                return; // Exit if no line received
            }
        };
    
        // Determine response based on request
        let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "static/index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "static/404.html")
        };
    
        // Read the file contents
        let contents = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Failed to read file {}: {}", filename, err);
                let error_message = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nInternal Server Error";
                stream.write_all(error_message.as_bytes()).unwrap_or_else(|err| {
                    eprintln!("Failed to write error response: {}", err);
                });
                return; // Exit on file read error
            }
        };
    
        // Prepare the HTTP response
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        // Send the response
        if let Err(err) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write response: {}", err);
        }
    
        // Uncomment if the worker needs to handle multiple connections
        // self.is_available = true;
    
        println!("Request handled by worker {}: {}", self.worker_id, request_line);
    }

    pub fn is_available( &mut self ) -> bool {
        self.is_available
    }

   
}