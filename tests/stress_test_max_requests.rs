use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;



#[test]
#[ignore = "Stress test is too slow to run on CI"]
fn stress_test_max_requests() {
    let mut client_count = 1; // Start with 1 client and increase
    let failed_requests = Arc::new(Mutex::new(0)); // Thread-safe counter for failed requests
    let max_clients = 10000; // You can modify this depending on your server capacity

    while client_count <= max_clients {
        let failed_requests_clone = Arc::clone(&failed_requests);

        // Spawn a thread to handle each client
        let handle = thread::spawn(move || {
            let mut stream = match TcpStream::connect("localhost:8080") {
                Ok(s) => s,
                Err(_) => {
                    // If connection fails, we mark this request as failed
                    let mut failed = failed_requests_clone.lock().unwrap();
                    *failed += 1;
                    return Err("Connection failed".to_string());
                }
            };

            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            if stream.write_all(request.as_bytes()).is_err() {
                let mut failed = failed_requests_clone.lock().unwrap();
                *failed += 1;
                return Err("Request sending failed".to_string());
            }

            let mut response = String::new();
            if stream.read_to_string(&mut response).is_err() {
                let mut failed = failed_requests_clone.lock().unwrap();
                *failed += 1;
                return Err("Response read failed".to_string());
            }

            // Check if the response contains "200 OK"
            if !response.contains("HTTP/1.1 200 OK") {
                let mut failed = failed_requests_clone.lock().unwrap();
                *failed += 1;
                return Err("Unexpected response".to_string());
            }

            Ok(())
        });

        if let Err(e) = handle.join() {
            eprintln!("Error on client {}: {:#?}", client_count, e);
        }

        // Check the number of failed requests and break the loop if threshold reached
        let failed = failed_requests.lock().unwrap();
        if *failed >= 10 {
            println!("Server failed after {} requests.", client_count);
            break;
        }

        client_count += 1;
        // Optionally, add a small delay to simulate more realistic client behavior
        // thread::sleep(Duration::from_millis(10));
    }
}

