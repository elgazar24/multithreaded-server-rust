use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use multithread_server_task::server_manager::ServerManager;


#[test]
#[ignore = "Simulate multiple clients"]
fn test_multiple_clients() {
    let client_count = 4000; // Number of clients

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

    let mut handles = vec![];

    for i in 0..client_count {
        let handle = thread::spawn(move || {
            let mut stream = match TcpStream::connect("localhost:8080") {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Client {} failed to connect: {:?}", i, e);
                    return;
                }
            };

            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            if stream.write_all(request.as_bytes()).is_err() {
                eprintln!("Client {} failed to send request", i);
                return;
            }

            let mut response = String::new();
            match stream.read_to_string(&mut response) {
                Ok(_) => {
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

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread join error: {:?}", e);
        }
    }

    // change the flag to false to stop the server
    is_running.store(false, std::sync::atomic::Ordering::SeqCst);

}

