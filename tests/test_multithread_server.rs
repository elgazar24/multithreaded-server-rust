use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use multithread_server_task::server_manager::ServerManager;

mod client;
use client::Client;

#[test]
// #[ignore]
fn test_multithread_server() {
    //  Define the server details
    let ip_address = "localhost";
    let port = 8080;
    let base_threads_count = 4;

    // Create atomic flag to check if the server is running
    let is_running = Arc::new(AtomicBool::new(true));

    // Clone the `is_running` reference to pass into the thread closure
    let is_running_clone = Arc::clone(&is_running);

    // Start the server
    let mut server_manager = ServerManager::new(base_threads_count, ip_address, port, is_running);

    // Run server in a separate thread
    thread::spawn(move || {
        server_manager.start_server();

        // Allow the server to stop
        thread::sleep(Duration::from_secs(1));
    });

    // Wait for the server to initialize (adjust the sleep duration based on your needs)
    thread::sleep(Duration::from_secs(1));

    // Simulate client using the `Client` struct
    let client = Client::new(ip_address, port);

    let mut stream = match client.connect() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to server: {:?}", e);
            assert!(false, "Failed to connect to server");
            return;
        }
    };

    // Send a basic GET request
    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    if let Err(e) = client.send(request, &mut stream) {
        eprintln!("Failed to send request: {:?}", e);
        assert!(false, "Failed to send request");
        return;
    }

    // Read the response
    match client.receive(&mut stream) {
        Ok(response) => {
            // Assert that the response contains the expected status line for a valid request
            assert!(
                response.contains("HTTP/1.1 200 OK"),
                "Unexpected response: {}",
                response
            );
        }
        Err(e) => {
            eprintln!("Failed to read response: {:?}", e);
            assert!(false, "Failed to read response");
        }
    }
    
    // Change the flag to false to stop the server
    is_running_clone.store(false, std::sync::atomic::Ordering::SeqCst);

}
