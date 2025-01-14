use std::net::TcpStream;
use std::io::{Read, Write};


#[test]
#[ignore]
fn test_multithread_server() {
    

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
