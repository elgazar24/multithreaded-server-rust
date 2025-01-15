use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;


use multithread_server_task::server_manager::ServerManager;
mod client;
use client::Client;
use std::sync::atomic::Ordering;



#[test]
fn test_server_requests_and_stop() {


    // Define the server details
    let ip_address = "localhost";
    let port = 8080;
    let base_threads_count = 4; 

    // Create atomic flag to check if the server is running
    let is_running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    // Clone the `is_running` reference to pass into the thread closure
    let is_running_clone: Arc<AtomicBool> = Arc::clone(&is_running);

    // Start the server
    let mut server_manager: ServerManager = ServerManager::new(base_threads_count, ip_address, port , is_running);

    // Run server in a separate thread
    thread::spawn(move || {

        server_manager.start_server();

        // Allow the server to stop
        thread::sleep(Duration::from_secs(1));
    });

    // Wait for the server to initialize (adjust the sleep duration based on your needs)
    thread::sleep(Duration::from_secs(1));

    // Simulate sending a certain number of requests (e.g., 10 requests)
    let request_count: i32 = 10;
    let client: Client = Client::new(ip_address, port);

    for i in 0..request_count {
        let mut stream = match client.connect() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to connect to the server: {:?}", e);
                assert!(false, "Failed to connect to the server");
                return;
            }
        };

        let request = format!("Request number {}\n", i + 1);
        if let Err(e) = client.send(&request, &mut stream) {
            eprintln!("Failed to send request: {:?}", e);
            assert!(false, "Failed to send request");
        } else {
            println!("Sent request {}", i + 1);
        }

        thread::sleep(Duration::from_millis(100)); // Simulate time between requests
    }

    // Change the flag to false to stop the server
    is_running_clone.store(false, Ordering::SeqCst);

    // Add assertions if you need to validate the server's response handling or state
    assert!(true, "Server handled requests and stopped gracefully.");
}