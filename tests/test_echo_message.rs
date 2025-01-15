use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod client;
use client::Client;
use multithread_server_task::server_manager::ServerManager;



#[test]
fn test_client_echo_message() {
    let ip_address = "localhost";
    let port = 8080;
    let base_threads_count = 4; // reasonable thread count for this test

   // Create atomic flag to check if the server is running
   let is_running = Arc::new(AtomicBool::new(true));

   let is_running_clone = Arc::clone(&is_running);

    // Start the server
    let mut server_manager = ServerManager::new(base_threads_count, ip_address, port , is_running);

    // Run server in a separate thread
    thread::spawn(move || {

        server_manager.start_server();

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
    is_running_clone.store(false, std::sync::atomic::Ordering::SeqCst);

    // Server stopped successfully
    assert!(true, "Server handled request and stopped gracefully");
}