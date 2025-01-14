use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

#[test]
fn test_multithread_server() {
    // Start the server in a separate thread
    // thread::spawn(|| {
    //     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //     // Assume your server starts here, replace with your actual server code
    //     // server.start(listener); // Replace with actual server startup
    // });

    // Sleep for a short while to let the server start
    thread::sleep(Duration::from_secs(1));

    // Simulate client sending HTTP request to the server
    let mut stream = TcpStream::connect("localhost:8080").unwrap();

    // Send a basic GET request
    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    stream.write_all(request.as_bytes()).unwrap();

    // Read the response
    let mut response = Vec::new();
    if stream.read_to_end(&mut response).is_err() {
        eprintln!("Failed to read response");
        assert!(false, "Failed to read response");
    }

    // Convert the response to a string (Assuming UTF-8)
    let response = String::from_utf8_lossy(&response);

    // Assert that the response contains the expected status line for a valid request
    assert!(response.contains("HTTP/1.1 200 OK") , "Response : {}", response.to_string());
    // assert!(response.contains("index.html")); 

    // Further tests for workers can be added, checking worker handling, concurrency, etc.
}


#[test]
fn test_concurrent_requests() {
    let client_count = 4000; // Number of concurrent clients
    let mut handles = vec![];

    // Start multiple clients in parallel to test server with multiple workers
    for i in 0..client_count {
        let handle = thread::spawn(move || {
            // Connect to the server
            let mut stream = TcpStream::connect("localhost:8080").unwrap();
            
            // Send a GET request
            let request = format!("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
            stream.write_all(request.as_bytes()).unwrap();
            
            // Read the response
            let mut response = Vec::new();
            if stream.read_to_end(&mut response).is_err() {
                eprintln!("Failed to read response from client {}", i);
            }
            
            let response = String::from_utf8_lossy(&response);

            // Assert the response for each client
            assert_eq!(response.contains("HTTP/1.1 200 OK"), true , "Client {} failed to get response", response.to_string());
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        if handle.join().is_err(){
            eprintln!("Failed to join thread");
        }
    }
}


#[test]
fn test_invalid_request() {
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
}