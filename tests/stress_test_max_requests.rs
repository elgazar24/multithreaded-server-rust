use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;


mod  client;

use multithread_server_task::server_manager::ServerManager;
use client::Client;


#[test]
#[ignore = "Stress test is too slow to run on CI"]
fn stress_test_max_requests() {


    // Define the number of clients
    let mut client_count = 1; 
    // Define the number of failed requests ( Threads Safe ) 
    let failed_requests = Arc::new(Mutex::new(0)); 
    // Define the number of requests per second
    let max_clients = 10000;

    // Define the server details
    let ip_address = "localhost";
    let port = 8080;
    let base_threads_count = 4; 

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

    // Wait for the server to initialize (you can adjust the sleep duration based on your needs)
    thread::sleep(Duration::from_secs(1));

    // Start sending requests
    while client_count <= max_clients {

        let failed_requests_clone = Arc::clone(&failed_requests);

        // Create a new client
        let client = Client::new(ip_address, port);

        // Spawn a thread to handle each client
        let handle = thread::spawn(move || {
            let mut stream = match client.connect() {
                Ok(s) => s,
                Err(_) => {
                    // If connection fails, we mark this request as failed
                    let mut failed = failed_requests_clone.lock().unwrap();
                    *failed += 1;
                    return;
                }
            };

            // Send the HTTP request
            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            if client.send(request, &mut stream).is_err() {
                let mut failed = failed_requests_clone.lock().unwrap();
                *failed += 1;
                return;
            }

            // Receive the response
            match client.receive(&mut stream) {
                Ok(response) => {
                    // Check if the response contains "200 OK"
                    if !response.contains("HTTP/1.1 200 OK") {
                        let mut failed = failed_requests_clone.lock().unwrap();
                        *failed += 1;
                    }
                }
                Err(_) => {
                    let mut failed = failed_requests_clone.lock().unwrap();
                    *failed += 1;
                }
            }
        });

        // Wait for the client to finish
        if let Err(e) = handle.join() {
            eprintln!("Error on client {}: {:?}", client_count, e);
        }

        // Check the number of failed requests and break the loop if the threshold is reached
        let failed = failed_requests.lock().unwrap();
        if *failed >= 10 {
            println!("Server failed after {} requests.", client_count);
            break;
        }

        client_count += 1;
    }

    // Change the flag to false to stop the server
    is_running_clone.store(false, std::sync::atomic::Ordering::SeqCst);

}