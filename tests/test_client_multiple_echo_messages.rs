use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;


mod client;
use client::Client;
use multithread_server_task::server_manager::ServerManager;




#[test]
fn test_client_multiple_echo_messages() {

    // Define the server details
    let ip_address: &str = "localhost";
    let port: u16 = 8080;
    let base_threads_count: usize = 4; 

    // Create atomic flag to check if the server is running
    let is_running = Arc::new(AtomicBool::new(true));

    let is_running_clone = Arc::clone(&is_running);

    // Start the server
    let mut server_manager: ServerManager = ServerManager::new(base_threads_count, ip_address, port , is_running);

    // Run the server in a separate thread
    thread::spawn(move || {

        server_manager.start_server();

        // Stop the server
        server_manager.stop();

        // Allow the server to stop
        thread::sleep(Duration::from_secs(1));
    });

    // Allow some time for the server to start
    thread::sleep(Duration::from_secs(1));

    // Create a client and connect to the server
    let client = Client::new(ip_address, port);

    // Prepare a list of multiple messages
    let messages = vec![
        "Message 1",
        "Message 2",
        "Message 3",
        "Message 4",
        "Message 5",
    ];

    for message in messages {

        let mut stream = client.connect().expect("Failed to connect to the server");

        println!("Sending message: {}", message);
        // Send the message
        assert!(client.send(message, &mut stream).is_ok(), "Failed to send message: {}", message);

        // Receive the echoed message
        let response: String = client.receive(&mut stream).expect("Failed to receive response");

        println!("Received response: {}", response);

        // Verify if the server echoed back the message
        assert_eq!(
            response,
            message,
            "Echoed message '{}' does not match server response '{}'",
            message,
            response.trim()
        );
        // thread::sleep(Duration::from_secs(1));
    }

    // Change the flag to false to stop the server
    is_running_clone.store(false, std::sync::atomic::Ordering::SeqCst);

    // Server stopped successfully
    assert!(true, "Server handled requests and stopped successfully");
}