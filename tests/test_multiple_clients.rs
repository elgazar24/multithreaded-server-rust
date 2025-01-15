use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use multithread_server_task::server_manager::ServerManager;

mod client;
use client::Client;

#[test]
// #[ignore = "Simulate multiple clients"]
fn test_multiple_clients() {
    // Number of clients to simulate
    let client_count = 4000;

    // Define the server details
    let ip_address = "localhost";
    let port = 8080;
    let base_threads_count = 4; // Use a reasonable number of threads

    // Create atomic flag to check if the server is running
    let is_running = Arc::new(AtomicBool::new(true));

    // Clone the `is_running` reference to pass into the thread closure
    let is_running_clone = Arc::clone(&is_running);

    // Start the server
    let mut server_manager = ServerManager::new(base_threads_count, ip_address, port , is_running);

    // Run server in a separate thread
    thread::spawn(move || {
        
        server_manager.start_server();

        // Allow the server to stop
        thread::sleep(Duration::from_secs(1));
    });

    // Wait for the server to initialize
    thread::sleep(Duration::from_secs(1));

    let mut handles = vec![];

    for i in 0..client_count {
        let ip = ip_address.to_string();
        let handle = thread::spawn(move || {
            let client = Client::new(&ip, port);
            let mut stream = match client.connect() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Client {} failed to connect: {:?}", i, e);
                    return;
                }
            };

            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            if let Err(e) = client.send(request, &mut stream) {
                eprintln!("Client {} failed to send request: {:?}", i, e);
                return;
            }

            match client.receive(&mut stream) {
                Ok(response) => {
                    assert!(
                        response.contains("HTTP/1.1 200 OK"),
                        "Client {} failed: unexpected response",
                        i
                    );
                }
                Err(e) => {
                    eprintln!("Client {} failed to read response: {:?}", i, e);
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread join error: {:?}", e);
        }
    }

    // Change the flag to false to stop the server
    is_running_clone.store(false, std::sync::atomic::Ordering::SeqCst);
}
