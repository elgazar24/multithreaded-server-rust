use std::net::TcpStream;
use std::io::{Read, Write};


#[test]
#[ignore]
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