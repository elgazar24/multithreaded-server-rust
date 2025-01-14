use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;



#[test]
#[ignore = "Simulate multiple clients"]
fn test_multiple_clients() {
    let client_count = 4000; // Number of clients
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
}

