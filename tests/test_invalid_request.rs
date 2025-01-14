use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use multithread_server_task::server_manager::ServerManager;

#[test]
#[ignore]
fn test_invalid_request() {
    let ip_address = "localhost";
    let port = 8080;
    let base_threads_count = 4; // Use a reasonable number of threads

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

    // Wait for the server to initialize (you can adjust the sleep duration based on your needs)
    thread::sleep(Duration::from_secs(1));

    // Connect to the server
    let mut stream = TcpStream::connect("localhost:8080").unwrap();

    // Send an invalid request
    let invalid_request = "INVALID / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    stream.write_all(invalid_request.as_bytes()).unwrap();

    // Read the response
    let mut response = Vec::new();
    assert!(stream.read_to_end(&mut response).is_ok(), "Failed to read response for invalid request");
    
    let response = String::from_utf8_lossy(&response);

    // Assert that the server returns a 400 Bad Request
    assert!(response.contains("HTTP/1.1 400 Bad Request") , "Failed to get 400 Bad Request response for invalid request {}", response.to_string());

    // change the flag to false to stop the server
    is_running.store(false, std::sync::atomic::Ordering::SeqCst);
}