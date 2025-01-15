use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use multithread_server_task::server_manager::ServerManager;

mod client;
use client::Client;




#[test]
// #[ignore]
fn test_invalid_request() {


    // Define the server details
    let ip_address = "localhost";
    let port = 8080;
    let base_threads_count = 4; // Use a reasonable number of threads

   // Create atomic flag to check if the server is running
   let is_running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

   let is_running_clone: Arc<AtomicBool> = Arc::clone(&is_running);

    // Start the server
    let mut server_manager = ServerManager::new(base_threads_count, ip_address, port , is_running);


    // Run server in a separate thread
    thread::spawn(move || {

        server_manager.start_server();

        // Allow the server to stop
        thread::sleep(Duration::from_secs(1));
    });

    // Wait for the server to initialize (you can adjust the sleep duration based on your needs)
    thread::sleep(Duration::from_secs(1));

    // Use Client to connect to the server and send the invalid request
    let client = Client::new(ip_address, port);
    let mut stream = client.connect().expect("Failed to connect to the server");

    let invalid_request = "INVALID / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    client
        .send(invalid_request, &mut stream)
        .expect("Failed to send invalid request");

    // Receive the response
    let response = client
        .receive(&mut stream)
        .expect("Failed to read response for invalid request");

    // Assert that the server returns a 400 Bad Request
    assert!(
        response.contains("HTTP/1.1 400 Bad Request"),
        "Expected 400 Bad Request response, but got: {}",
        response
    );

    // Change the flag to false to stop the server
    is_running_clone.store(false, std::sync::atomic::Ordering::SeqCst);
    
}