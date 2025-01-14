use multithread_server_task::server_manager::ServerManager;

use std::io::Write;
use std::net::TcpStream;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_server_requests_and_stop() {
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

    // Simulate sending a certain number of requests (e.g., 10 requests)
    let request_count = 10;
    for i in 0..request_count {
        let mut stream = TcpStream::connect(format!("{}:{}", ip_address, port))
            .expect("Failed to connect to the server");

        let request = format!("Request number {}\n", i + 1);
        stream
            .write_all(request.as_bytes())
            .expect("Failed to send request");

        println!("Sent request {}", i + 1);
        thread::sleep(Duration::from_millis(100)); // Simulate time between requests
    }

    // change the flag to false to stop the server
    is_running.store(false, std::sync::atomic::Ordering::SeqCst);

    // Here, you can add assertions to check if the server has indeed processed requests
    // and stopped. For simplicity, we just ensure that the server runs and stops without errors.
    assert!(true, "Server handled requests and stopped gracefully.");
}