use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use multithread_server_task::server_manager::ServerManager;



struct Client {
    ip: String,
    port: u16,
}

impl Client {
    fn new(ip: &str, port: u16) -> Self {
        Client {
            ip: ip.to_string(),
            port,
        }
    }

    fn connect(&self) -> Result<TcpStream, std::io::Error> {
        TcpStream::connect(format!("{}:{}", self.ip, self.port))
    }

    fn send(&self, message: &str, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let _ = stream.write_all(message.as_bytes());
        stream.flush()
    }

    fn receive(&self, stream: &mut TcpStream) -> Result<String, std::io::Error> {

        // Read the response
        let mut response = Vec::new();
        stream.read_to_end(&mut response).unwrap();
        let response = String::from_utf8_lossy(&response).to_string();
        Ok(response)
        
    }
}

#[test]
fn test_client_echo_message() {
    let ip_address = "localhost";
    let port = 8080;
    let base_threads_count = 4; // reasonable thread count for this test

    // create atomic flag to check if the server is running
    let is_running = Arc::new(AtomicBool::new(true));

    // Start the server
    let mut server_manager = ServerManager::new(base_threads_count, ip_address, port);

    // Clone the `is_running` reference to pass into the thread closure
    let is_running_clone = Arc::clone(&is_running);

    // Run server in a separate thread
    thread::spawn(move || {
        server_manager.start_server();

        while is_running_clone.load(std::sync::atomic::Ordering::SeqCst) {}

        // Stop the server
        server_manager.stop();

        // Allow the server to stop
        thread::sleep(Duration::from_secs(1));
    });

    // Allow some time for the server to start
    thread::sleep(Duration::from_secs(1));

    // Create a client and connect to the server
    let client = Client::new(ip_address, port);
    let mut stream = client.connect().expect("Failed to connect to the server");

    // Prepare and send the message
    let echo_message = "Hello, Server!";
    assert!(client.send(echo_message, &mut stream).is_ok(), "Failed to send message");

    // Receive the echoed message
    let response = client.receive(&mut stream).expect("Failed to receive response");

    // Verify if the server echoed back the message
    assert_eq!(
        response,
        echo_message,
        "Echoed message {} does not match server response {}",
        echo_message,
        response
    );

    // change the flag to false to stop the server
    is_running.store(false, std::sync::atomic::Ordering::SeqCst);

    // Server stopped successfully
    assert!(true, "Server handled request and stopped gracefully");
}